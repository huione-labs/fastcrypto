const crypto = require("crypto");
const {toBigIntBE} = require('bigint-buffer');

const utils = require('./utils');
const jwtutils = require('./jwtutils');

const nWidth = require("./constants").inWidth;
const outWidth = require("./constants").outWidth;
const pin = require("./constants").pin;
const poseidonHash = require('./utils').poseidonHash;

// https://datatracker.ietf.org/doc/html/rfc4634#section-4.1
function padMessage(bits) {
    const L = bits.length;
    const K = (512 + 448 - (L % 512 + 1)) % 512;

    bits = bits.concat([1]);
    if(K > 0) {
        bits = bits.concat(Array(K).fill(0));
    }
    bits = bits.concat(utils.buffer2BitArray(Buffer.from(L.toString(16).padStart(16, '0'), 'hex')));
    return bits;
}

function genJwtMask(input, fields) {
    const [header, payload] = input.split('.');
    var payloadMask = Array(payload.length).fill(0);
    for(const field of fields) {
        var [start, len] = jwtutils.indicesOfB64(payload, field);
        for(var i = 0; i < len; i++) {
            payloadMask[start + i] = 1;
        }
    }

    return Array(header.length + 1).fill(1).concat(payloadMask);
}
  
function genSha256Inputs(input, nCount, nWidth = 8, inParam = "in") {
    var segments = utils.arrayChunk(padMessage(utils.buffer2BitArray(Buffer.from(input))), nWidth);
    const num_sha2_blocks = (segments.length * nWidth) / 512;

    if ((segments.length * nWidth) % 512 != 0) {
        throw new Error("Padding error: Padded message length is not a multiple of 512");
    }
    
    if(segments.length < nCount) {
        segments = segments.concat(Array(nCount-segments.length).fill(Array(nWidth).fill(0)));
    }
    
    if(segments.length > nCount) {
        throw new Error(`Padded message (${segments.length}) exceeds maximum length supported by circuit (${nCount})`);
    }
    
    return { [inParam]: segments, "num_sha2_blocks": num_sha2_blocks }; 
}

function genNonceInputs() {
    const eph_public_key = 0x0d7dab358c8dadaa4efa0049a75b07436555b10a368219bb680f70571349d775n; // TODO: Fixed for dev
    // const eph_public_key = BigInt("0x" + crypto.randomBytes(32).toString('hex'));
    const max_epoch = 10000; // TODO: Fixed for dev
    const randomness = 100681567828351849884072155819400689117n; // TODO: Fixed for dev
  
    const eph_public_key_0 = eph_public_key / 2n**128n;
    const eph_public_key_1 = eph_public_key % 2n**128n;
  
    return {
      "eph_public_key": [eph_public_key_0, eph_public_key_1],
      "max_epoch": max_epoch,
      "randomness": randomness
    };
}
  
async function genSubInputs(payload, maxSubLength, payloadIndex) {
    const decoded_payload = Buffer.from(payload, 'base64url').toString();
    const sub_claim = jwtutils.getClaimString(decoded_payload, "sub");
    const [start, len] = jwtutils.indicesOfB64(payload, 'sub');

    if (sub_claim.length > maxSubLength) {
        throw new Error(`Subject claim ${sub_claim} exceeds maximum length of ${maxSubLength} characters`);
    }

    const sub_claim_without_last_char = sub_claim.slice(0, -1);
    const subject_id_com = await utils.commitSubID(sub_claim_without_last_char, pin, maxSubLength, outWidth);

    const padded_subject_id = utils.padWithZeroes(sub_claim.split('').map(c => c.charCodeAt()), maxSubLength);
    return [{
        "subject_id": padded_subject_id,
        "sub_length_ascii": sub_claim.length,
        "sub_claim_index_b64": start + payloadIndex,
        "sub_length_b64": len,
        "subject_pin": pin
    }, subject_id_com];
}

async function genJwtProofUAInputs(input, nCount, fields, maxSubLength) {  
    // init poseidon
    const buildPoseidon = require("circomlibjs").buildPoseidon;
    poseidon = await buildPoseidon();

    // set SHA-2 inputs
    var inputs = genSha256Inputs(input, nCount, nWidth, "content");
    inputs["content"] = inputs["content"].map(bits => toBigIntBE(utils.bitArray2Buffer(bits)));
  
    // set indices
    inputs["payload_start_index"] = input.split('.')[0].length + 1; // 4x+1, 4x, 4x-1
    const payload = input.split('.')[1];
    inputs["payload_len"] = payload.length;

    // set sub claim inputs
    const [sub_inputs, subject_id_com] = await genSubInputs(
        payload,
        maxSubLength,
        inputs["payload_start_index"]
    );
    inputs = Object.assign({}, inputs, sub_inputs);

    // set hash
    const hash = BigInt("0x" + crypto.createHash("sha256").update(input).digest("hex"));
    const jwt_sha2_hash = [hash / 2n**128n, hash % 2n**128n];
  
    // masking 
    inputs["mask"] = genJwtMask(input, fields).concat(Array(nCount - input.length).fill(1));
    const masked_content = utils.applyMask(inputs["content"], inputs["mask"]);
    const packed = utils.pack(masked_content, 8, outWidth);
    const masked_content_hash = poseidonHash(packed, poseidon);
  
    // set nonce-related inputs
    inputs = Object.assign({}, inputs, genNonceInputs());
    const nonce = poseidonHash([
        inputs["eph_public_key"][0], 
        inputs["eph_public_key"][1], 
        inputs["max_epoch"], 
        inputs["randomness"]
    ], poseidon);
  
    inputs["all_inputs_hash"] = poseidonHash([
        jwt_sha2_hash[0],
        jwt_sha2_hash[1],
        masked_content_hash,
        inputs["payload_start_index"],
        inputs["payload_len"],
        inputs["eph_public_key"][0], 
        inputs["eph_public_key"][1], 
        inputs["max_epoch"],
        nonce,
        inputs["num_sha2_blocks"],
        subject_id_com
    ], poseidon);

    const auxiliary_inputs = {
        "masked_content": masked_content,
        "jwt_sha2_hash": jwt_sha2_hash.map(e => e.toString()),
        "payload_start_index": inputs["payload_start_index"],
        "payload_len": inputs["payload_len"],
        "eph_public_key": inputs["eph_public_key"].map(e => e.toString()),
        "max_epoch": inputs["max_epoch"],
        "num_sha2_blocks": inputs["num_sha2_blocks"],
        "sub_id_com": subject_id_com.toString()
    }
  
    return [inputs, auxiliary_inputs];
}  

module.exports = {
    padMessage: padMessage,
    genJwtMask: genJwtMask,
    genSha256Inputs: genSha256Inputs,
    genJwtProofUAInputs: genJwtProofUAInputs
}

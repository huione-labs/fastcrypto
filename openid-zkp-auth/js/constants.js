module.exports = {
    P: 21888242871839275222246405745257275088548364400416034343698204186575808495617n,
    flag: 5,
    inWidth: 8,
    packWidth: 248,
    // const eph_public_key = BigInt("0x" + crypto.randomBytes(32).toString('hex'));
    dev: { // NOTE: Constants meant to be used for dev
        pin: 283089722053851751073973683904920435104n,
        ephPK: 0x0d7dab358c8dadaa4efa0049a75b07436555b10a368219bb680f70571349d775n,
        maxEpoch: 10000,
        jwtRand: 100681567828351849884072155819400689117n
    },
    maskValue: '='.charCodeAt(),
    nonceLen: Math.ceil(256 / 6), // 43
    extNonceLen: Math.ceil(256 / 6) + 11, // 11 for prefix and suffix
    claimsToReveal: ["iss", "aud"],
    maxContentLen: 64*12,
    maxExtClaimLen: 66, // name + value + 6 chars (four '"', one ':' and one ',' / '}')
    maxKeyClaimNameLen: 10,
    maxKeyClaimValueLen: 50,
    maxIssValueLen: 31 * 3
}
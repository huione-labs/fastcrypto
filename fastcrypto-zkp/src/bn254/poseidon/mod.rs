// Copyright (c) 2022, Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::bn254::poseidon::constants::{
    POSEIDON_CONSTANTS_U1, POSEIDON_CONSTANTS_U10, POSEIDON_CONSTANTS_U11, POSEIDON_CONSTANTS_U12,
    POSEIDON_CONSTANTS_U13, POSEIDON_CONSTANTS_U14, POSEIDON_CONSTANTS_U15, POSEIDON_CONSTANTS_U16,
    POSEIDON_CONSTANTS_U2, POSEIDON_CONSTANTS_U3, POSEIDON_CONSTANTS_U4, POSEIDON_CONSTANTS_U5,
    POSEIDON_CONSTANTS_U6, POSEIDON_CONSTANTS_U7, POSEIDON_CONSTANTS_U8, POSEIDON_CONSTANTS_U9,
};
use crate::FrRepr;
use ark_bn254::Fr;
use ark_ff::{BigInteger, PrimeField};
use byte_slice_cast::AsByteSlice;
use fastcrypto::error::FastCryptoError;
use fastcrypto::error::FastCryptoError::{InputTooLong, InvalidInput};
use ff::PrimeField as OtherPrimeField;
use neptune::poseidon::HashMode::OptimizedStatic;
use neptune::Poseidon;
use std::cmp::Ordering;

/// The output of the Poseidon hash function is a field element in BN254 which is 254 bits long, so
/// we need 32 bytes to represent it as an integer.
pub const FIELD_ELEMENT_SIZE_IN_BYTES: usize = 32;
mod constants;

macro_rules! define_poseidon_hash {
    ($inputs:expr, $poseidon_constants:expr) => {{
        let mut poseidon = Poseidon::new(&$poseidon_constants);
        poseidon.reset();
        for input in $inputs.iter() {
            poseidon.input(bn254_to_fr(*input)).unwrap();
        }
        poseidon.hash_in_mode(OptimizedStatic);

        // Neptune returns the state element with index 1 but we want the first element.
        poseidon.elements[0]
    }};
}

/// Poseidon hash function over BN254.
pub fn hash(inputs: Vec<Fr>) -> Result<Fr, FastCryptoError> {
    if inputs.is_empty() || inputs.len() > 16 {
        return Err(FastCryptoError::InputLengthWrong(inputs.len()));
    }

    // Instances of Poseidon and PoseidonConstants from neptune have different types depending on
    // the number of inputs, so unfortunately we need to use a macro here.
    let result = match inputs.len() {
        1 => define_poseidon_hash!(inputs, POSEIDON_CONSTANTS_U1),
        2 => define_poseidon_hash!(inputs, POSEIDON_CONSTANTS_U2),
        3 => define_poseidon_hash!(inputs, POSEIDON_CONSTANTS_U3),
        4 => define_poseidon_hash!(inputs, POSEIDON_CONSTANTS_U4),
        5 => define_poseidon_hash!(inputs, POSEIDON_CONSTANTS_U5),
        6 => define_poseidon_hash!(inputs, POSEIDON_CONSTANTS_U6),
        7 => define_poseidon_hash!(inputs, POSEIDON_CONSTANTS_U7),
        8 => define_poseidon_hash!(inputs, POSEIDON_CONSTANTS_U8),
        9 => define_poseidon_hash!(inputs, POSEIDON_CONSTANTS_U9),
        10 => define_poseidon_hash!(inputs, POSEIDON_CONSTANTS_U10),
        11 => define_poseidon_hash!(inputs, POSEIDON_CONSTANTS_U11),
        12 => define_poseidon_hash!(inputs, POSEIDON_CONSTANTS_U12),
        13 => define_poseidon_hash!(inputs, POSEIDON_CONSTANTS_U13),
        14 => define_poseidon_hash!(inputs, POSEIDON_CONSTANTS_U14),
        15 => define_poseidon_hash!(inputs, POSEIDON_CONSTANTS_U15),
        16 => define_poseidon_hash!(inputs, POSEIDON_CONSTANTS_U16),
        _ => return Err(FastCryptoError::InvalidInput),
    };
    Ok(fr_to_bn254fr(result))
}

/// Calculate the poseidon hash of the field element inputs. If the input length is <= 16, calculate
/// H(inputs), if it is <= 32, calculate H(H(inputs[0..16]), H(inputs[16..])), otherwise return an
/// error.
pub fn to_poseidon_hash(inputs: Vec<Fr>) -> Result<Fr, FastCryptoError> {
    if inputs.len() <= 16 {
        hash(inputs)
    } else if inputs.len() <= 32 {
        let hash1 = hash(inputs[0..16].to_vec())?;
        let hash2 = hash(inputs[16..].to_vec())?;
        hash([hash1, hash2].to_vec())
    } else {
        Err(FastCryptoError::GeneralError(format!(
            "Yet to implement: Unable to hash a vector of length {}",
            inputs.len()
        )))
    }
}

/// Convert an ff field element to an arkworks-ff field element.
/// Given a binary representation of a BN254 field element as an integer in little-endian encoding,
/// this function returns the corresponding field element. If the field element is not canonical (is
/// larger than the field size as an integer), an `FastCryptoError::InvalidInput` is returned.
///
/// If more than 32 bytes is given, an `FastCryptoError::InputTooLong` is returned.
fn from_canonical_le_bytes_to_field_element(bytes: &[u8]) -> Result<Fr, FastCryptoError> {
    match bytes.len().cmp(&FIELD_ELEMENT_SIZE_IN_BYTES) {
        Ordering::Less => Ok(Fr::from_le_bytes_mod_order(bytes)),
        Ordering::Equal => {
            let field_element = Fr::from_le_bytes_mod_order(bytes);
            // Unfortunately, there doesn't seem to be a nice way to check if a modular reduction
            // happened without doing the extra work of serializing the field element again.
            let reduced_bytes = field_element.into_bigint().to_bytes_le();
            if reduced_bytes != bytes {
                return Err(InvalidInput);
            }
            Ok(field_element)
        }
        Ordering::Greater => Err(InputTooLong(bytes.len())),
    }
}

/// Calculate the poseidon hash of an array of inputs. Each input is interpreted as a BN254 field
/// element assuming a little-endian encoding. The field elements are then hashed using the poseidon
/// hash function ([to_poseidon_hash]).
///
/// If one of the inputs is in non-canonical form, e.g. it represents an integer greater than the
/// field size or is longer than 32 bytes, an error is returned.
pub fn hash_to_field_element(inputs: &Vec<Vec<u8>>) -> Result<Fr, FastCryptoError> {
    let mut field_elements = Vec::new();
    for input in inputs {
        field_elements.push(from_canonical_le_bytes_to_field_element(input)?);
    }
    to_poseidon_hash(field_elements)
}

/// Calculate the poseidon hash of an array of inputs. Each input is interpreted as a BN254 field
/// element assuming a little-endian encoding. The field elements are then hashed using the poseidon
/// hash function ([to_poseidon_hash]) and the result is serialized as a little-endian integer (32
/// bytes).
///
/// If one of the inputs is in non-canonical form, e.g. it represents an integer greater than the
/// field size or is longer than 32 bytes, an error is returned.
pub fn hash_to_bytes(
    inputs: &Vec<Vec<u8>>,
) -> Result<[u8; FIELD_ELEMENT_SIZE_IN_BYTES], FastCryptoError> {
    let field_element = hash_to_field_element(inputs)?;
    let bytes = field_element.into_bigint().to_bytes_le();
    Ok(bytes
        .try_into()
        .expect("Leading zeros are added in to_bytes_be"))
}

fn fr_to_bn254fr(fr: crate::Fr) -> Fr {
    Fr::from_be_bytes_mod_order(fr.to_repr().as_byte_slice())
}

/// Convert an arkworks-ff field element to an ff field element.
fn bn254_to_fr(fr: Fr) -> crate::Fr {
    let mut bytes = [0u8; 32];
    bytes.clone_from_slice(&fr.into_bigint().to_bytes_be());
    crate::Fr::from_repr_vartime(FrRepr(bytes)).expect("fr is always valid")
}

#[cfg(test)]
mod test {
    use super::Poseidon;
    use crate::bn254::poseidon::bn254_to_fr;
    use crate::bn254::poseidon::constants::load_constants;
    use crate::bn254::poseidon::hash;
    use crate::bn254::poseidon::hash_to_bytes;
    use crate::bn254::{poseidon::to_poseidon_hash, zk_login::Bn254Fr};
    use crate::bn254::{poseidon::to_poseidon_hash, zk_login::Bn254Fr};
    use ark_bn254::Fr;
    use ff::PrimeField;
    use neptune::hash_type::HashType;
    use neptune::poseidon::HashMode::Correct;
    use neptune::poseidon::PoseidonConstants;
    use std::str::FromStr;
    use typenum::U2;

    fn to_bigint_arr(vals: Vec<u8>) -> Vec<Bn254Fr> {
        vals.into_iter().map(Bn254Fr::from).collect()
    }

    #[test]
    fn poseidon_test() {
        let input1 = Fr::from_str("134696963602902907403122104327765350261").unwrap();
        let input2 = Fr::from_str("17932473587154777519561053972421347139").unwrap();
        let input3 = Fr::from_str("10000").unwrap();
        let input4 = Fr::from_str(
            "50683480294434968413708503290439057629605340925620961559740848568164438166",
        )
        .unwrap();
        let hash = hash(vec![input1, input2, input3, input4]).unwrap();
        assert_eq!(
            hash,
            Fr::from_str(
                "2272550810841985018139126931041192927190568084082399473943239080305281957330"
            )
            .unwrap()
        );
    }
    #[test]
    fn test_to_poseidon_hash() {
        assert_eq!(
            to_poseidon_hash(to_bigint_arr(vec![1]))
                .unwrap()
                .to_string(),
            "18586133768512220936620570745912940619677854269274689475585506675881198879027"
        );
        assert_eq!(
            to_poseidon_hash(to_bigint_arr(vec![1, 2]))
                .unwrap()
                .to_string(),
            "7853200120776062878684798364095072458815029376092732009249414926327459813530"
        );
        assert_eq!(
            to_poseidon_hash(to_bigint_arr(vec![
                1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15
            ]))
            .unwrap()
            .to_string(),
            "4203130618016961831408770638653325366880478848856764494148034853759773445968"
        );
        assert_eq!(
            to_poseidon_hash(to_bigint_arr(vec![
                1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16
            ]))
            .unwrap()
            .to_string(),
            "9989051620750914585850546081941653841776809718687451684622678807385399211877"
        );
        assert_eq!(
            to_poseidon_hash(to_bigint_arr(vec![
                0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22,
                23, 24, 25, 26, 27, 28, 29
            ]))
            .unwrap()
            .to_string(),
            "4123755143677678663754455867798672266093104048057302051129414708339780424023"
        );

        assert!(to_poseidon_hash(to_bigint_arr(vec![
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
            24, 25, 26, 27, 28, 29, 30, 31, 32
        ]))
        .is_err());
    }

    #[test]
    fn test_all_inputs_hash() {
        let jwt_sha2_hash_0 = Fr::from_str("248987002057371616691124650904415756047").unwrap();
        let jwt_sha2_hash_1 = Fr::from_str("113498781424543581252500776698433499823").unwrap();
        let masked_content_hash = Fr::from_str(
            "14900420995580824499222150327925943524564997104405553289134597516335134742309",
        )
        .unwrap();
        let payload_start_index = Fr::from_str("103").unwrap();
        let payload_len = Fr::from_str("564").unwrap();
        let eph_public_key_0 = Fr::from_str("17932473587154777519561053972421347139").unwrap();
        let eph_public_key_1 = Fr::from_str("134696963602902907403122104327765350261").unwrap();
        let max_epoch = Fr::from_str("10000").unwrap();
        let num_sha2_blocks = Fr::from_str("11").unwrap();
        let key_claim_name_f = Fr::from_str(
            "18523124550523841778801820019979000409432455608728354507022210389496924497355",
        )
        .unwrap();
        let addr_seed = Fr::from_str(
            "15604334753912523265015800787270404628529489918817818174033741053550755333691",
        )
        .unwrap();

        let hash = hash(vec![
            jwt_sha2_hash_0,
            jwt_sha2_hash_1,
            masked_content_hash,
            payload_start_index,
            payload_len,
            eph_public_key_0,
            eph_public_key_1,
            max_epoch,
            num_sha2_blocks,
            key_claim_name_f,
            addr_seed,
        ])
        .unwrap();
        assert_eq!(
            hash.to_string(),
            "2487117669597822357956926047501254969190518860900347921480370492048882803688"
                .to_string()
        );
    }

    #[test]
    fn test_hash_to_bytes() {
        let inputs: Vec<Vec<u8>> = vec![vec![1u8]];
        let hash = hash_to_bytes(&inputs).unwrap();
        // 18586133768512220936620570745912940619677854269274689475585506675881198879027 in decimal
        let expected =
            hex::decode("33018202c57d898b84338b16d1a4960e133c6a4d656cfec1bd62a9ea00611729")
                .unwrap();
        assert_eq!(hash.as_slice(), &expected);

        // 7853200120776062878684798364095072458815029376092732009249414926327459813530 in decimal
        let inputs: Vec<Vec<u8>> = vec![vec![1u8], vec![2u8]];
        let hash = hash_to_bytes(&inputs).unwrap();
        let expected =
            hex::decode("9a1817447a60199e51453274f217362acfe962966b4cf63d4190d6e7f5c05c11")
                .unwrap();
        assert_eq!(hash.as_slice(), &expected);

        // Input larger than the modulus
        let inputs = vec![vec![255; 32]];
        assert!(hash_to_bytes(&inputs).is_err());

        // Input smaller than the modulus
        let inputs = vec![vec![255; 31]];
        assert!(hash_to_bytes(&inputs).is_ok());
    }

    macro_rules! define_poseidon {
        (
    $pk_length:expr,
    $sig_length:expr,
    $dst_string:expr
) => {};
    }

    fn from_str(string: &str) -> crate::Fr {
        crate::Fr::from_str_vartime(string).unwrap()
    }
}

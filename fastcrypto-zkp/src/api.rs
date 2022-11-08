// Copyright (c) 2022, Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use ark_bls12_381::Bls12_381;
use ark_groth16::{Proof, VerifyingKey};
use ark_serialize::CanonicalDeserialize;
use fastcrypto::error::FastCryptoError;
use untrusted::Input;

use crate::conversions::BlsFr;
use crate::verifier::{process_vk_special, verify_with_processed_vk, PreparedVerifyingKey};

#[cfg(test)]
#[path = "unit_tests/api_tests.rs"]
mod api_tests;

/// Deserialize bytes as an Arkwork representation of a verifying key, and return a serialized prepared verified key.
pub fn prepare_pvk_bytes(vk_bytes: Vec<u8>) -> Result<Vec<Vec<u8>>, FastCryptoError> {
    let vk = VerifyingKey::<Bls12_381>::deserialize(Input::from(&vk_bytes).as_slice_less_safe())
        .map_err(|_| FastCryptoError::InvalidInput)?;

    process_vk_special(&vk).as_serialized()
}

/// Verify Groth16 proof using the serialized representation of a prepared verifying key, proof public input and proof points.
pub fn verify_groth16_in_bytes(
    pvk_bytes: Vec<Vec<u8>>,
    proof_public_inputs_as_bytes: Vec<u8>,
    proof_points_as_bytes: Vec<u8>,
) -> Result<bool, FastCryptoError> {
    let x = BlsFr::deserialize(Input::from(&proof_public_inputs_as_bytes).as_slice_less_safe())
        .map_err(|_| FastCryptoError::InvalidInput)?;

    let proof =
        Proof::<Bls12_381>::deserialize(Input::from(&proof_points_as_bytes).as_slice_less_safe())
            .map_err(|_| FastCryptoError::InvalidInput)?;

    let blst_pvk = PreparedVerifyingKey::deserialize(pvk_bytes)?;

    verify_with_processed_vk(&blst_pvk, &[x], &proof).map_err(|_| FastCryptoError::GeneralError)
}

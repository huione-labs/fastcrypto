// Copyright (c) 2022, Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
use crate::api::{prepare_pvk_bytes, verify_groth16_in_bytes};

#[test]
fn test_verify_groth16_in_bytes_api() {
    // Success case.
    assert!(verify_groth16_in_bytes(
        vec![
            VK_GAMMA_ABC_G1_VECTOR.to_vec(),
            ALPHA_G1_BETA_G2_BYTES.to_vec(),
            GAMMA_G2_NEG_PC_BYTES.to_vec(),
            DELTA_G2_NEG_PC.to_vec()
        ],
        PROOF_INPUTS_BYTES.to_vec(),
        PROOF_POINTS_BYTES.to_vec()
    )
    .is_ok());

    // Length of verifying key is incorrect.
    let mut modified_bytes = VK_GAMMA_ABC_G1_VECTOR.to_vec();
    modified_bytes.pop();
    assert!(verify_groth16_in_bytes(
        vec![
            modified_bytes,
            ALPHA_G1_BETA_G2_BYTES.to_vec(),
            GAMMA_G2_NEG_PC_BYTES.to_vec(),
            DELTA_G2_NEG_PC.to_vec()
        ],
        PROOF_INPUTS_BYTES.to_vec(),
        PROOF_POINTS_BYTES.to_vec()
    )
    .is_err());

    // Length of public inputs is incorrect.
    let mut modified_proof_points_bytes = PROOF_INPUTS_BYTES.to_vec();
    modified_proof_points_bytes.pop();
    assert!(verify_groth16_in_bytes(
        vec![
            VK_GAMMA_ABC_G1_VECTOR.to_vec(),
            ALPHA_G1_BETA_G2_BYTES.to_vec(),
            GAMMA_G2_NEG_PC_BYTES.to_vec(),
            DELTA_G2_NEG_PC.to_vec()
        ],
        modified_proof_points_bytes,
        PROOF_POINTS_BYTES.to_vec()
    )
    .is_err());

    // length of proof is incorrect
    let mut modified_proof_points_bytes = PROOF_POINTS_BYTES.to_vec();
    modified_proof_points_bytes.pop();
    assert!(verify_groth16_in_bytes(
        vec![
            VK_GAMMA_ABC_G1_VECTOR.to_vec(),
            ALPHA_G1_BETA_G2_BYTES.to_vec(),
            GAMMA_G2_NEG_PC_BYTES.to_vec(),
            DELTA_G2_NEG_PC.to_vec()
        ],
        PROOF_INPUTS_BYTES.to_vec(),
        modified_proof_points_bytes
    )
    .is_err());
}

#[test]
fn test_prepare_pvk_bytes() {
    // Success case.
    assert!(prepare_pvk_bytes(VK_BYTES.to_vec()).is_ok());

    // Length of verifying key is incorrect.
    let mut modified_bytes = VK_BYTES.to_vec();
    modified_bytes.pop();
    assert!(prepare_pvk_bytes(modified_bytes).is_err());
}

/// A serialized Arkwork verifying key.
const VK_BYTES: [u8; 440] = [
    136, 200, 65, 247, 1, 62, 145, 188, 97, 130, 122, 100, 218, 95, 55, 40, 66, 233, 190, 82, 37,
    19, 152, 50, 83, 194, 169, 39, 94, 67, 77, 147, 19, 13, 16, 12, 75, 129, 36, 254, 85, 220, 13,
    193, 239, 69, 145, 139, 77, 7, 240, 200, 179, 135, 59, 23, 10, 242, 88, 2, 30, 113, 164, 220,
    80, 122, 202, 79, 222, 175, 213, 220, 47, 62, 232, 89, 129, 23, 134, 58, 87, 252, 37, 239, 196,
    8, 212, 34, 123, 34, 230, 14, 141, 132, 187, 20, 110, 151, 99, 125, 63, 187, 167, 138, 134, 65,
    244, 76, 255, 248, 44, 184, 148, 71, 32, 117, 166, 211, 81, 92, 84, 206, 159, 162, 202, 24,
    111, 45, 87, 128, 116, 123, 91, 124, 133, 232, 141, 167, 190, 26, 129, 90, 57, 4, 246, 59, 153,
    125, 79, 61, 69, 237, 62, 32, 229, 203, 14, 23, 176, 185, 98, 182, 46, 157, 100, 213, 188, 130,
    95, 229, 113, 255, 193, 95, 152, 177, 6, 5, 117, 142, 175, 68, 15, 225, 101, 19, 56, 108, 8,
    108, 158, 11, 11, 234, 28, 48, 248, 248, 191, 22, 103, 220, 196, 117, 20, 169, 173, 196, 205,
    27, 45, 133, 76, 15, 210, 41, 30, 1, 64, 183, 246, 211, 79, 49, 195, 203, 108, 142, 230, 53,
    185, 57, 72, 33, 54, 145, 84, 221, 82, 10, 253, 170, 205, 72, 218, 109, 238, 219, 25, 15, 39,
    245, 157, 151, 64, 195, 96, 123, 191, 203, 44, 15, 138, 89, 11, 78, 233, 7, 26, 155, 218, 149,
    50, 33, 127, 137, 170, 178, 253, 78, 45, 80, 95, 71, 204, 17, 60, 0, 97, 136, 73, 38, 139, 20,
    15, 171, 107, 228, 5, 100, 154, 45, 29, 7, 73, 131, 24, 50, 135, 184, 238, 122, 115, 196, 219,
    178, 171, 78, 123, 163, 186, 183, 250, 0, 90, 5, 90, 61, 210, 107, 71, 135, 254, 17, 181, 133,
    2, 0, 0, 0, 0, 0, 0, 0, 246, 117, 216, 150, 18, 57, 84, 24, 157, 52, 104, 30, 245, 206, 71,
    181, 227, 38, 2, 71, 228, 234, 104, 23, 241, 156, 65, 11, 159, 111, 227, 222, 176, 134, 225,
    101, 5, 108, 2, 33, 106, 14, 18, 17, 74, 155, 65, 13, 118, 128, 94, 144, 97, 147, 200, 205, 68,
    176, 47, 205, 157, 155, 52, 253, 182, 178, 117, 239, 92, 19, 231, 5, 111, 182, 26, 161, 135, 4,
    9, 184, 2, 8, 16, 249, 178, 154, 171, 107, 51, 159, 163, 248, 83, 192, 225, 3,
];

/// Vector representation of vk_gamma_abc_g1 of PreparedVerifyingKey
const VK_GAMMA_ABC_G1_VECTOR: [u8; 96] = [
    246, 117, 216, 150, 18, 57, 84, 24, 157, 52, 104, 30, 245, 206, 71, 181, 227, 38, 2, 71, 228,
    234, 104, 23, 241, 156, 65, 11, 159, 111, 227, 222, 176, 134, 225, 101, 5, 108, 2, 33, 106, 14,
    18, 17, 74, 155, 65, 13, 118, 128, 94, 144, 97, 147, 200, 205, 68, 176, 47, 205, 157, 155, 52,
    253, 182, 178, 117, 239, 92, 19, 231, 5, 111, 182, 26, 161, 135, 4, 9, 184, 2, 8, 16, 249, 178,
    154, 171, 107, 51, 159, 163, 248, 83, 192, 225, 3,
];

/// Serialized representation of alpha_g1_beta_g2 of PreparedVerifyingKey
const ALPHA_G1_BETA_G2_BYTES: [u8; 576] = [
    18, 22, 138, 163, 138, 26, 224, 54, 5, 80, 208, 84, 16, 2, 176, 36, 5, 122, 182, 137, 212, 92,
    232, 9, 248, 234, 54, 213, 40, 110, 202, 158, 47, 24, 231, 9, 36, 172, 105, 220, 212, 50, 34,
    138, 24, 3, 107, 20, 106, 167, 90, 92, 23, 67, 7, 81, 248, 68, 246, 134, 200, 186, 33, 12, 119,
    54, 173, 177, 133, 31, 122, 250, 199, 251, 188, 74, 199, 138, 1, 199, 202, 69, 8, 227, 212, 91,
    93, 211, 30, 135, 92, 153, 176, 201, 210, 0, 4, 244, 179, 173, 142, 60, 136, 66, 182, 173, 201,
    195, 121, 126, 48, 131, 163, 27, 31, 254, 101, 77, 212, 70, 103, 67, 205, 148, 59, 125, 49,
    133, 88, 138, 45, 129, 218, 95, 32, 179, 101, 147, 21, 124, 36, 41, 178, 24, 53, 150, 74, 187,
    147, 103, 12, 129, 244, 169, 242, 48, 85, 109, 206, 220, 200, 122, 92, 54, 86, 19, 130, 14, 34,
    82, 37, 166, 80, 186, 125, 90, 141, 40, 61, 184, 49, 117, 41, 179, 114, 151, 151, 154, 215, 87,
    100, 5, 178, 110, 83, 242, 193, 98, 227, 85, 87, 234, 244, 229, 158, 27, 61, 69, 109, 72, 98,
    145, 166, 68, 254, 9, 143, 13, 41, 192, 67, 93, 70, 227, 93, 17, 77, 115, 87, 24, 142, 216,
    168, 250, 38, 200, 7, 250, 66, 14, 123, 255, 124, 224, 194, 168, 74, 117, 241, 137, 207, 110,
    208, 57, 86, 79, 54, 68, 18, 54, 114, 11, 225, 27, 197, 56, 80, 243, 112, 4, 145, 245, 4, 48,
    254, 71, 41, 103, 101, 100, 18, 143, 11, 243, 38, 230, 122, 0, 56, 151, 91, 57, 108, 111, 209,
    44, 12, 216, 190, 117, 229, 152, 94, 40, 65, 0, 86, 64, 182, 16, 75, 78, 30, 152, 23, 221, 59,
    68, 229, 26, 164, 176, 151, 36, 137, 173, 153, 155, 184, 20, 58, 78, 131, 49, 16, 5, 123, 163,
    45, 31, 249, 28, 103, 7, 176, 126, 171, 6, 5, 185, 214, 162, 116, 90, 234, 213, 79, 22, 169,
    104, 164, 18, 47, 168, 202, 135, 27, 112, 161, 0, 181, 253, 133, 77, 68, 115, 236, 123, 81,
    156, 4, 84, 127, 20, 185, 171, 166, 112, 30, 84, 231, 55, 22, 31, 193, 84, 204, 55, 81, 249,
    149, 192, 195, 61, 126, 247, 75, 137, 62, 107, 197, 81, 72, 145, 215, 58, 245, 84, 60, 78, 212,
    99, 228, 174, 190, 108, 187, 217, 115, 144, 191, 11, 247, 32, 117, 160, 100, 158, 1, 166, 95,
    162, 183, 25, 139, 237, 172, 56, 64, 104, 100, 220, 120, 12, 184, 120, 157, 240, 203, 9, 207,
    83, 34, 1, 213, 137, 188, 64, 248, 75, 246, 165, 129, 108, 203, 211, 30, 168, 93, 12, 242, 224,
    108, 38, 3, 125, 105, 112, 202, 238, 56, 181, 7, 69, 11, 239, 40, 44, 64, 54, 107, 180, 80,
    100, 8, 241, 126, 51, 31, 222, 50, 17, 192, 203, 2, 28, 120, 88, 186, 131, 230, 161, 241, 210,
    75, 223, 85, 11, 136, 77, 133, 127, 240, 53, 90, 216, 60, 208, 19, 70, 198, 45, 202, 113, 151,
    180, 213, 66, 136, 235, 201, 130, 216, 34, 138, 132, 3, 233, 168, 189, 149, 239, 152, 119, 91,
    249, 196, 0, 4, 226, 181, 222, 62, 102, 50, 18,
];

/// Serialized representation of gamma_g2_neg_pc of PreparedVerifyingKey
const GAMMA_G2_NEG_PC_BYTES: [u8; 96] = [
    246, 59, 153, 125, 79, 61, 69, 237, 62, 32, 229, 203, 14, 23, 176, 185, 98, 182, 46, 157, 100,
    213, 188, 130, 95, 229, 113, 255, 193, 95, 152, 177, 6, 5, 117, 142, 175, 68, 15, 225, 101, 19,
    56, 108, 8, 108, 158, 11, 11, 234, 28, 48, 248, 248, 191, 22, 103, 220, 196, 117, 20, 169, 173,
    196, 205, 27, 45, 133, 76, 15, 210, 41, 30, 1, 64, 183, 246, 211, 79, 49, 195, 203, 108, 142,
    230, 53, 185, 57, 72, 33, 54, 145, 84, 221, 82, 138,
];

/// Serialized representation of delta_g2_neg_pc of PreparedVerifyingKey
const DELTA_G2_NEG_PC: [u8; 96] = [
    253, 170, 205, 72, 218, 109, 238, 219, 25, 15, 39, 245, 157, 151, 64, 195, 96, 123, 191, 203,
    44, 15, 138, 89, 11, 78, 233, 7, 26, 155, 218, 149, 50, 33, 127, 137, 170, 178, 253, 78, 45,
    80, 95, 71, 204, 17, 60, 0, 97, 136, 73, 38, 139, 20, 15, 171, 107, 228, 5, 100, 154, 45, 29,
    7, 73, 131, 24, 50, 135, 184, 238, 122, 115, 196, 219, 178, 171, 78, 123, 163, 186, 183, 250,
    0, 90, 5, 90, 61, 210, 107, 71, 135, 254, 17, 181, 5,
];

/// Serialized representation of the input.
const PROOF_INPUTS_BYTES: [u8; 32] = [
    74, 247, 109, 145, 212, 188, 154, 57, 115, 193, 94, 58, 235, 87, 79, 15, 100, 84, 123, 131,
    143, 149, 10, 243, 93, 185, 127, 7, 5, 196, 33, 75,
];

/// Serialized representation of the three points in proof.
const PROOF_POINTS_BYTES: [u8; 192] = [
    207, 67, 33, 174, 120, 198, 30, 222, 247, 157, 208, 196, 178, 230, 196, 164, 140, 36, 145, 78,
    155, 43, 142, 106, 169, 255, 12, 94, 20, 27, 234, 232, 75, 128, 180, 149, 16, 190, 185, 2, 24,
    167, 108, 237, 179, 157, 204, 151, 252, 48, 158, 214, 217, 17, 200, 173, 101, 151, 94, 8, 27,
    81, 192, 137, 201, 90, 112, 234, 109, 213, 22, 202, 9, 201, 165, 156, 78, 228, 246, 36, 214,
    69, 236, 188, 159, 172, 2, 1, 148, 204, 9, 98, 171, 79, 4, 15, 77, 118, 91, 14, 105, 1, 74, 71,
    188, 159, 27, 6, 224, 186, 129, 139, 255, 242, 165, 31, 66, 78, 62, 186, 50, 91, 81, 78, 13,
    168, 140, 78, 10, 174, 57, 146, 49, 191, 216, 218, 162, 149, 54, 207, 45, 220, 160, 152, 111,
    136, 20, 123, 116, 157, 27, 229, 148, 55, 97, 10, 175, 125, 12, 52, 178, 0, 245, 142, 45, 42,
    147, 244, 236, 209, 66, 8, 163, 20, 88, 56, 4, 221, 42, 59, 194, 131, 236, 0, 222, 1, 236, 247,
    137, 56, 69, 7,
];

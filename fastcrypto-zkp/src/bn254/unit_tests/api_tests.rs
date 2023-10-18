// Copyright (c) 2022, Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::bn254::api::{prepare_pvk_bytes, verify_groth16_in_bytes};
use crate::bn254::verifier::process_vk_special;
use crate::bn254::VerifyingKey;
use crate::dummy_circuits::{DummyCircuit, Fibonacci};
use ark_bn254::{Bn254, Fr};
use ark_groth16::Groth16;
use ark_serialize::CanonicalSerialize;
use ark_snark::SNARK;
use ark_std::rand::thread_rng;
use ark_std::UniformRand;
use std::ops::Mul;

#[path = "./utils.rs"]
mod utils;

#[test]
fn test_verify_groth16_in_bytes_api() {
    const PUBLIC_SIZE: usize = 128;
    let rng = &mut thread_rng();
    let c = DummyCircuit::<Fr> {
        a: Some(<Fr>::rand(rng)),
        b: Some(<Fr>::rand(rng)),
        num_variables: PUBLIC_SIZE,
        num_constraints: 10,
    };

    let (pk, vk) = Groth16::<Bn254>::circuit_specific_setup(c, rng).unwrap();
    let proof = Groth16::<Bn254>::prove(&pk, c, rng).unwrap();
    let v = c.a.unwrap().mul(c.b.unwrap());

    let pvk = process_vk_special(&VerifyingKey(vk));

    let bytes = pvk.serialize().unwrap();
    let vk_gamma_abc_g1_bytes = &bytes[0];
    let alpha_g1_beta_g2_bytes = &bytes[1];
    let gamma_g2_neg_pc_bytes = &bytes[2];
    let delta_g2_neg_pc_bytes = &bytes[3];

    let mut proof_inputs_bytes = vec![];
    v.serialize_compressed(&mut proof_inputs_bytes).unwrap();

    // Proof::write serializes uncompressed and also adds a length to each element, so we serialize
    // each individual element here to avoid that.
    let mut proof_points_bytes = Vec::new();
    proof
        .a
        .serialize_compressed(&mut proof_points_bytes)
        .unwrap();
    proof
        .b
        .serialize_compressed(&mut proof_points_bytes)
        .unwrap();
    proof
        .c
        .serialize_compressed(&mut proof_points_bytes)
        .unwrap();

    // Success case.
    assert!(verify_groth16_in_bytes(
        vk_gamma_abc_g1_bytes,
        alpha_g1_beta_g2_bytes,
        gamma_g2_neg_pc_bytes,
        delta_g2_neg_pc_bytes,
        &proof_inputs_bytes,
        &proof_points_bytes
    )
    .is_ok());
}

#[test]
fn test_prepare_pvk_bytes() {
    const PUBLIC_SIZE: usize = 128;
    let rng = &mut thread_rng();
    let c = DummyCircuit::<Fr> {
        a: Some(<Fr>::rand(rng)),
        b: Some(<Fr>::rand(rng)),
        num_variables: PUBLIC_SIZE,
        num_constraints: 10,
    };

    let (_, vk) = Groth16::<Bn254>::circuit_specific_setup(c, rng).unwrap();

    let mut vk_bytes = vec![];
    vk.serialize_compressed(&mut vk_bytes).unwrap();

    // Success case.
    assert!(prepare_pvk_bytes(vk_bytes.as_slice()).is_ok());

    // Length of verifying key is incorrect.
    let mut modified_bytes = vk_bytes.clone();
    modified_bytes.pop();
    assert!(prepare_pvk_bytes(&modified_bytes).is_err());
}

#[test]
fn test_verify_groth16_in_bytes_multiple_inputs() {
    let mut rng = thread_rng();

    let a = Fr::from(123);
    let b = Fr::from(456);

    let params = {
        let circuit = Fibonacci::<Fr>::new(42, a, b);
        Groth16::<Bn254>::generate_random_parameters_with_reduction(circuit, &mut rng).unwrap()
    };

    let proof = {
        let circuit = Fibonacci::<Fr>::new(42, a, b);
        Groth16::<Bn254>::create_random_proof_with_reduction(circuit, &params, &mut rng).unwrap()
    };

    let pvk = process_vk_special(&VerifyingKey(params.vk));

    let inputs: Vec<_> = [a, b].to_vec();
    assert!(
        Groth16::<Bn254>::verify_with_processed_vk(&pvk.as_arkworks_pvk(), &inputs, &proof)
            .unwrap()
    );

    let pvk = pvk.serialize().unwrap();

    // This circuit has two public inputs:
    let mut inputs_bytes = Vec::new();
    a.serialize_compressed(&mut inputs_bytes).unwrap();
    b.serialize_compressed(&mut inputs_bytes).unwrap();

    // Proof::write serializes uncompressed and also adds a length to each element, so we serialize
    // each individual element here to avoid that.
    let mut proof_bytes = Vec::new();
    proof.a.serialize_compressed(&mut proof_bytes).unwrap();
    proof.b.serialize_compressed(&mut proof_bytes).unwrap();
    proof.c.serialize_compressed(&mut proof_bytes).unwrap();

    assert!(verify_groth16_in_bytes(
        &pvk[0],
        &pvk[1],
        &pvk[2],
        &pvk[3],
        &inputs_bytes,
        &proof_bytes
    )
    .unwrap());

    inputs_bytes[0] += 1;
    assert!(!verify_groth16_in_bytes(
        &pvk[0],
        &pvk[1],
        &pvk[2],
        &pvk[3],
        &inputs_bytes,
        &proof_bytes
    )
    .unwrap());
}

// Test for verifying the elusiv send-quadra circuits used for private on-chain transfers.
// This circuit has 14 public inputs and ~22.5k constraints. More info about the exact details of it
// can be found at https://github.com/elusiv-privacy/circuits
#[test]
fn test_verify_groth16_elusiv_proof_in_bytes_api() {
    // (Proof bytes, Public inputs bytes)
    let elusiv_sample_proof = (
        vec![
            20, 245, 104, 221, 130, 235, 123, 204, 177, 114, 10, 110, 46, 183, 48, 120, 9, 170, 51,
            85, 158, 26, 189, 62, 237, 16, 46, 203, 175, 122, 245, 47, 128, 87, 105, 124, 179, 152,
            174, 66, 22, 174, 55, 85, 1, 47, 128, 147, 202, 36, 183, 172, 26, 137, 85, 39, 96, 39,
            212, 31, 124, 4, 168, 13, 1, 33, 72, 218, 200, 115, 180, 44, 146, 88, 182, 241, 65,
            111, 36, 248, 138, 83, 92, 147, 174, 50, 206, 139, 56, 181, 15, 123, 0, 238, 20, 11,
            123, 58, 226, 125, 60, 189, 123, 74, 214, 222, 32, 75, 128, 205, 200, 6, 68, 207, 105,
            214, 219, 76, 6, 205, 20, 198, 213, 119, 205, 236, 13, 21,
        ],
        vec![
            187, 105, 172, 219, 4, 178, 82, 24, 207, 213, 168, 195, 53, 95, 53, 171, 213, 192, 159,
            78, 251, 174, 158, 168, 44, 21, 120, 167, 161, 85, 87, 20, 36, 159, 7, 87, 95, 30, 146,
            132, 86, 227, 151, 100, 176, 167, 157, 142, 13, 251, 220, 165, 141, 225, 145, 119, 207,
            238, 113, 199, 253, 149, 78, 5, 119, 251, 160, 26, 10, 92, 220, 11, 212, 148, 56, 59,
            245, 100, 28, 234, 83, 163, 83, 83, 48, 131, 246, 220, 176, 116, 72, 8, 79, 68, 105,
            11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 123, 125, 40, 198, 133, 246, 224, 5, 103, 244, 188, 245, 155, 180, 187, 99,
            139, 61, 240, 162, 71, 44, 115, 162, 6, 35, 181, 127, 42, 40, 42, 37, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 136, 19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 175, 11, 110, 47, 171, 92, 39, 63, 36, 183, 61, 144, 105, 250,
            193, 22, 180, 65, 101, 199, 47, 151, 12, 147, 158, 66, 62, 51, 147, 86, 89, 34, 4, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 249, 251, 64, 35, 242, 208, 188, 51, 106, 123, 236, 123, 93, 72,
            26, 61, 110, 224, 247, 245, 114, 29, 253, 212, 174, 130, 115, 44, 183, 49, 31, 23,
        ],
    );

    let vk = VerifyingKey(ark_groth16::VerifyingKey {
        alpha_g1: utils::G1Affine_from_str_projective((
            "8057073471822347335074195152835286348058235024870127707965681971765888348219",
            "14493022634743109860560137600871299171677470588934003383462482807829968516757",
            "1",
        )),
        beta_g2: utils::G2Affine_from_str_projective((
            (
                "3572582736973115805854009786889644784414020463323864932822856731322980736092",
                "20796599916820806690555061040933219683613855446136615092456120794141344002056",
            ),
            (
                "6655819316204680004365614375508079580461146204424752037766280753854543388537",
                "21051385956744942198035008062816432434887289184811055343085396392904977398400",
            ),
            ("1", "0"),
        )),
        gamma_g2: utils::G2Affine_from_str_projective((
            (
                "10857046999023057135944570762232829481370756359578518086990519993285655852781",
                "11559732032986387107991004021392285783925812861821192530917403151452391805634",
            ),
            (
                "8495653923123431417604973247489272438418190587263600148770280649306958101930",
                "4082367875863433681332203403145435568316851327593401208105741076214120093531",
            ),
            ("1", "0"),
        )),
        delta_g2: utils::G2Affine_from_str_projective((
            (
                "11998653647826530912022227389593270429577129765091819606672414955204726946137",
                "12850197969502293778482300034606665950383830355768697463743623195959747528569",
            ),
            (
                "3371177482557063281015231215914240035716553874474070718078727302911297506634",
                "12667795686197095991004340383609552078675969789404912385920584439828198138754",
            ),
            ("1", "0"),
        )),
        gamma_abc_g1: [
            [
                "11423936163622682661315257948859256751456935745483672301927753823261895199269",
                "8106299131826030264309317289206035584499915702251874486285904804204850744645",
                "1",
            ],
            [
                "3101734373871983241904605625023311773791709350380811153571118050344636150719",
                "5892752048111020912174143187873113013528793690570548925602265811558514488885",
                "1",
            ],
            [
                "10476231653569587456624794227763775706638536733174066539315272867287760110504",
                "10966166298405300401399180388536732567182096690752823243070979263725671251842",
                "1",
            ],
            [
                "3616644883823724294840639617628786582022507076201411671428851342676842026051",
                "20036054300972762576589546578455562677975529109923089992859054028247449793275",
                "1",
            ],
            [
                "8922146185459718802170954039785431585338226940878465749467742893964332142463",
                "6543899100030899685821688665010402257161600764202006060926513825176262562594",
                "1",
            ],
            [
                "8838880056209295823278313283853562429175894016112442003934942661774390156254",
                "12827213619164270378479427160832201667918020494718807523503415302940668517033",
                "1",
            ],
            [
                "2830281053896850092944028355764636104294475011402565423874976766597400897579",
                "13415270586926186600118105749667385774136247571413308961986554361125375974552",
                "1",
            ],
            [
                "18596510315364411631453906928618372802526744665579937948378160099177646939132",
                "13639164510921866583928930414183864880892036368934098358398305969672652727368",
                "1",
            ],
            [
                "5166155439194150342865876104665292251058885686253625593517703833929767249773",
                "15776325379616919283841092402757993241658241305931554423955510623840777140969",
                "1",
            ],
            [
                "244871576834190719988785477479956000478101720979685216270364011881385785410",
                "5006539956367064800739393540924950096169041851058318954717373683020872268739",
                "1",
            ],
            [
                "3379906259197166810955208903373839920133048860227880343760386881009843909062",
                "20232197429675204807642408172750830052412585778140676948557231371164499652906",
                "1",
            ],
            [
                "5520775405859402378836749033719619657978092778322140710653552702896452870563",
                "2840091105079872357493316251142119838752629278546220113584117974897982339624",
                "1",
            ],
            [
                "520211872811929422003078090188660039184112525356441893145895540025777918752",
                "18510673159743652418577623905535570073301952222198134524503321213201497608215",
                "1",
            ],
            [
                "6431234738107765889030689757699276709534858281277744012577221575246765244517",
                "4178355859219522686761165914894952086513502987193412248095296044093289572534",
                "1",
            ],
            [
                "4759337634951432350348093011115687353434771991388975508607474262950775320629",
                "3583982358135750838996058092244844686884741536705305315993181569552518297411",
                "1",
            ],
        ]
        .into_iter()
        .map(|s| utils::G1Affine_from_str_projective((s[0], s[1], s[2])))
        .collect(),
    });

    let pvk = process_vk_special(&vk);

    let bytes = pvk.serialize().unwrap();
    let vk_gamma_abc_g1_bytes = &bytes[0];
    let alpha_g1_beta_g2_bytes = &bytes[1];
    let gamma_g2_neg_pc_bytes = &bytes[2];
    let delta_g2_neg_pc_bytes = &bytes[3];

    // Success case.
    assert!(verify_groth16_in_bytes(
        vk_gamma_abc_g1_bytes,
        alpha_g1_beta_g2_bytes,
        gamma_g2_neg_pc_bytes,
        delta_g2_neg_pc_bytes,
        &elusiv_sample_proof.1,
        &elusiv_sample_proof.0
    )
    .is_ok());
}

// Test for verifying the elusiv send-quadra circuits used for private on-chain transfers.
// This circuit has 14 public inputs and ~22.5k constraints. More info about the exact details of it
// can be found at https://github.com/elusiv-privacy/circuits
#[test]
fn fail_verify_groth16_invalid_elusiv_proof_in_bytes_api() {
    // (Invalid proof bytes, Valid public inputs bytes) (last 3 bytes changed to 1 2 3)
    let elusiv_sample_proof_invalid_proof = (
        vec![
            187, 105, 172, 219, 4, 178, 82, 24, 207, 213, 168, 195, 53, 95, 53, 171, 213, 192, 159,
            78, 251, 174, 158, 168, 44, 21, 120, 167, 161, 85, 87, 20, 36, 159, 7, 87, 95, 30, 146,
            132, 86, 227, 151, 100, 176, 167, 157, 142, 13, 251, 220, 165, 141, 225, 145, 119, 207,
            238, 113, 199, 253, 149, 78, 5, 119, 251, 160, 26, 10, 92, 220, 11, 212, 148, 56, 59,
            245, 100, 28, 234, 83, 163, 83, 83, 48, 131, 246, 220, 176, 116, 72, 8, 79, 68, 105,
            11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            23, 25, 67, 79,
        ],
        vec![
            187, 105, 172, 219, 4, 178, 82, 24, 207, 213, 168, 195, 53, 95, 53, 171, 213, 192, 159,
            78, 251, 174, 158, 168, 44, 21, 120, 167, 161, 85, 87, 20, 36, 159, 7, 87, 95, 30, 146,
            132, 86, 227, 151, 100, 176, 167, 157, 142, 13, 251, 220, 165, 141, 225, 145, 119, 207,
            238, 113, 199, 253, 149, 78, 5, 119, 251, 160, 26, 10, 92, 220, 11, 212, 148, 56, 59,
            245, 100, 28, 234, 83, 163, 83, 83, 48, 131, 246, 220, 176, 116, 72, 8, 79, 68, 105,
            11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 123, 125, 40, 198, 133, 246, 224, 5, 103, 244, 188, 245, 155, 180, 187, 99,
            139, 61, 240, 162, 71, 44, 115, 162, 6, 35, 181, 127, 42, 40, 42, 37, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 136, 19, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 175, 11, 110, 247, 71, 92, 39, 63, 36, 183, 61, 144, 105, 250,
            193, 22, 180, 65, 101, 199, 247, 151, 12, 147, 158, 66, 62, 51, 147, 86, 89, 34, 4, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 249, 251, 64, 35, 242, 208, 188, 51, 106, 123, 236, 123, 93, 72,
            26, 61, 110, 224, 247, 245, 114, 29, 253, 212, 174, 130, 115, 44, 183, 49, 31, 23,
        ],
    );

    // (Valid proof bytes, Invalid public inputs bytes) (last 3 bytes changed to 1 2 3)
    let elusiv_sample_proof_invalid_pin = (
        vec![
            20, 245, 104, 221, 130, 235, 123, 204, 177, 114, 10, 110, 46, 183, 48, 120, 9, 170, 51,
            85, 158, 26, 189, 62, 237, 16, 46, 203, 175, 122, 245, 47, 128, 87, 105, 124, 179, 152,
            174, 66, 22, 174, 55, 85, 1, 47, 128, 147, 202, 36, 183, 172, 26, 137, 85, 39, 96, 39,
            212, 31, 124, 4, 168, 13, 1, 33, 72, 218, 200, 115, 180, 44, 146, 88, 182, 241, 65,
            111, 36, 248, 138, 83, 92, 147, 174, 50, 206, 139, 56, 181, 15, 123, 0, 238, 20, 11,
            123, 58, 226, 125, 60, 189, 123, 74, 214, 222, 32, 75, 128, 205, 200, 6, 68, 207, 105,
            214, 219, 76, 6, 205, 20, 198, 213, 119, 205, 236, 13, 21,
        ],
        vec![
            193, 22, 180, 65, 101, 199, 47, 151, 12, 147, 158, 66, 62, 51, 147, 86, 89, 34, 4, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 249, 251, 64, 35, 242, 208, 188, 51, 106, 123, 236, 123, 93, 72,
            26, 61, 110, 224, 247, 245, 114, 29, 253, 212, 174, 130, 115, 44, 183, 1, 2, 3, 139,
            187, 105, 172, 219, 4, 178, 82, 24, 207, 213, 168, 195, 53, 95, 53, 171, 213, 192, 159,
            78, 251, 174, 158, 168, 44, 21, 120, 167, 161, 85, 87, 20, 36, 159, 7, 87, 95, 30, 146,
            132, 86, 227, 151, 100, 176, 167, 157, 142, 13, 251, 220, 165, 141, 225, 145, 119, 207,
            238, 113, 199, 253, 149, 78, 5, 119, 251, 160, 26, 10, 92, 220, 11, 212, 148, 56, 59,
            245, 100, 28, 234, 83, 163, 83, 83, 48, 131, 246, 220, 176, 116, 72, 8, 79, 68, 105,
            11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 123, 125, 40, 198, 133, 246, 224, 5, 103, 244, 188, 245, 155, 180, 187, 99,
            61, 240, 162, 71, 44, 115, 162, 6, 35, 181, 127, 42, 40, 42, 37, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 136, 19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 175, 11, 110, 47, 171, 92, 39, 63, 36, 183, 61, 144, 105, 250,
        ],
    );

    let vk = ark_groth16::VerifyingKey {
        alpha_g1: utils::G1Affine_from_str_projective((
            "8057073471822347335074195152835286348058235024870127707965681971765888348219",
            "14493022634743109860560137600871299171677470588934003383462482807829968516757",
            "1",
        )),
        beta_g2: utils::G2Affine_from_str_projective((
            (
                "3572582736973115805854009786889644784414020463323864932822856731322980736092",
                "20796599916820806690555061040933219683613855446136615092456120794141344002056",
            ),
            (
                "6655819316204680004365614375508079580461146204424752037766280753854543388537",
                "21051385956744942198035008062816432434887289184811055343085396392904977398400",
            ),
            ("1", "0"),
        )),
        gamma_g2: utils::G2Affine_from_str_projective((
            (
                "10857046999023057135944570762232829481370756359578518086990519993285655852781",
                "11559732032986387107991004021392285783925812861821192530917403151452391805634",
            ),
            (
                "8495653923123431417604973247489272438418190587263600148770280649306958101930",
                "4082367875863433681332203403145435568316851327593401208105741076214120093531",
            ),
            ("1", "0"),
        )),
        delta_g2: utils::G2Affine_from_str_projective((
            (
                "11998653647826530912022227389593270429577129765091819606672414955204726946137",
                "12850197969502293778482300034606665950383830355768697463743623195959747528569",
            ),
            (
                "3371177482557063281015231215914240035716553874474070718078727302911297506634",
                "12667795686197095991004340383609552078675969789404912385920584439828198138754",
            ),
            ("1", "0"),
        )),
        gamma_abc_g1: [
            [
                "11423936163622682661315257948859256751456935745483672301927753823261895199269",
                "8106299131826030264309317289206035584499915702251874486285904804204850744645",
                "1",
            ],
            [
                "3101734373871983241904605625023311773791709350380811153571118050344636150719",
                "5892752048111020912174143187873113013528793690570548925602265811558514488885",
                "1",
            ],
            [
                "10476231653569587456624794227763775706638536733174066539315272867287760110504",
                "10966166298405300401399180388536732567182096690752823243070979263725671251842",
                "1",
            ],
            [
                "3616644883823724294840639617628786582022507076201411671428851342676842026051",
                "20036054300972762576589546578455562677975529109923089992859054028247449793275",
                "1",
            ],
            [
                "8922146185459718802170954039785431585338226940878465749467742893964332142463",
                "6543899100030899685821688665010402257161600764202006060926513825176262562594",
                "1",
            ],
            [
                "8838880056209295823278313283853562429175894016112442003934942661774390156254",
                "12827213619164270378479427160832201667918020494718807523503415302940668517033",
                "1",
            ],
            [
                "2830281053896850092944028355764636104294475011402565423874976766597400897579",
                "13415270586926186600118105749667385774136247571413308961986554361125375974552",
                "1",
            ],
            [
                "18596510315364411631453906928618372802526744665579937948378160099177646939132",
                "13639164510921866583928930414183864880892036368934098358398305969672652727368",
                "1",
            ],
            [
                "5166155439194150342865876104665292251058885686253625593517703833929767249773",
                "15776325379616919283841092402757993241658241305931554423955510623840777140969",
                "1",
            ],
            [
                "244871576834190719988785477479956000478101720979685216270364011881385785410",
                "5006539956367064800739393540924950096169041851058318954717373683020872268739",
                "1",
            ],
            [
                "3379906259197166810955208903373839920133048860227880343760386881009843909062",
                "20232197429675204807642408172750830052412585778140676948557231371164499652906",
                "1",
            ],
            [
                "5520775405859402378836749033719619657978092778322140710653552702896452870563",
                "2840091105079872357493316251142119838752629278546220113584117974897982339624",
                "1",
            ],
            [
                "520211872811929422003078090188660039184112525356441893145895540025777918752",
                "18510673159743652418577623905535570073301952222198134524503321213201497608215",
                "1",
            ],
            [
                "6431234738107765889030689757699276709534858281277744012577221575246765244517",
                "4178355859219522686761165914894952086513502987193412248095296044093289572534",
                "1",
            ],
            [
                "4759337634951432350348093011115687353434771991388975508607474262950775320629",
                "3583982358135750838996058092244844686884741536705305315993181569552518297411",
                "1",
            ],
        ]
        .into_iter()
        .map(|s| utils::G1Affine_from_str_projective((s[0], s[1], s[2])))
        .collect(),
    }
    .into();

    let pvk = process_vk_special(&vk);

    let bytes = pvk.serialize().unwrap();
    let vk_gamma_abc_g1_bytes = &bytes[0];
    let alpha_g1_beta_g2_bytes = &bytes[1];
    let gamma_g2_neg_pc_bytes = &bytes[2];
    let delta_g2_neg_pc_bytes = &bytes[3];

    // Should fail verification:.
    assert!(verify_groth16_in_bytes(
        vk_gamma_abc_g1_bytes,
        alpha_g1_beta_g2_bytes,
        gamma_g2_neg_pc_bytes,
        delta_g2_neg_pc_bytes,
        &elusiv_sample_proof_invalid_proof.1,
        &elusiv_sample_proof_invalid_proof.0
    )
    .is_err());

    // Should fail verification.
    assert!(verify_groth16_in_bytes(
        vk_gamma_abc_g1_bytes,
        alpha_g1_beta_g2_bytes,
        gamma_g2_neg_pc_bytes,
        delta_g2_neg_pc_bytes,
        &elusiv_sample_proof_invalid_pin.1,
        &elusiv_sample_proof_invalid_pin.0
    )
    .is_err());
}

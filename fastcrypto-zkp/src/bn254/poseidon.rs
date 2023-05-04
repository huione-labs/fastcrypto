// Copyright (c) 2022, Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use ark_bn254::Fr;
use light_poseidon::Poseidon;
use light_poseidon::PoseidonHasher;

/// Wrapper struct for Poseidon hash instance.
pub struct PoseidonWrapper {
    instance: Poseidon<Fr>,
}

impl PoseidonWrapper {
    /// Initialize a Poseidon hash function with the given size.
    pub fn new(size: usize) -> Self {
        Self {
            instance: Poseidon::<Fr>::new_circom(size).unwrap(),
        }
    }

    /// Calculate the hash of the given inputs.
    pub fn hash(&mut self, inputs: &[Fr]) -> Fr {
        self.instance.hash(inputs).unwrap()
    }
}
#[cfg(test)]
mod test {
    use crate::bn254::zk_login::calculate_merklized_hash;

    use super::PoseidonWrapper;
    use ark_bn254::Fr;
    use std::str::FromStr;
    #[test]
    fn poseidon_test() {
        // Test vector generated from circomjs
        // Poseidon([134696963602902907403122104327765350261n,
        // 17932473587154777519561053972421347139n,
        // 10000,
        // 50683480294434968413708503290439057629605340925620961559740848568164438166n])
        // = 2272550810841985018139126931041192927190568084082399473943239080305281957330n
        let mut poseidon = PoseidonWrapper::new(4);
        let input1 = Fr::from_str("134696963602902907403122104327765350261").unwrap();
        let input2 = Fr::from_str("17932473587154777519561053972421347139").unwrap();
        let input3 = Fr::from_str("10000").unwrap();
        let input4 = Fr::from_str(
            "50683480294434968413708503290439057629605340925620961559740848568164438166",
        )
        .unwrap();
        let hash = poseidon.hash(&[input1, input2, input3, input4]);
        assert_eq!(
            hash,
            Fr::from_str(
                "2272550810841985018139126931041192927190568084082399473943239080305281957330"
            )
            .unwrap()
        );
    }
    #[test]
    fn test_merklized_hash() {
        let masked_content = b"eyJhbGciOiJSUzI1NiIsImtpZCI6ImFjZGEzNjBmYjM2Y2QxNWZmODNhZjgzZTE3M2Y0N2ZmYzM2ZDExMWMiLCJ0eXAiOiJKV1QifQ.eyJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20i============================================================================================================LCJhdWQiOiI1NzU1MTkyMDQyMzctbXNvcDllcDQ1dTJ1bzk4aGFwcW1uZ3Y4ZDg0cWRjOGsuYXBwcy5nb29nbGV1c2VyY29udGVudC5jb20i========================================LCJub25jZSI6IjE2NjM3OTE4ODEzOTA4MDYwMjYxODcwNTI4OTAzOTk0MDM4NzIxNjY5Nzk5NjEzODAzNjAxNjE2Njc4MTU1NTEyMTgxMjczMjg5NDc3Iiwi==============================================================================================================\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x13\xe8";
        assert_eq!(
            calculate_merklized_hash(masked_content),
            "913176389931280990736922944363568337161535184534237191455121139930502488757"
        );
    }

    #[test]
    fn test_all_inputs_hash() {
        let mut poseidon = PoseidonWrapper::new(10);
        let jwt_sha2_hash_0 = Fr::from_str("157614805437531375860474698328477116569").unwrap();
        let jwt_sha2_hash_1 = Fr::from_str("241856916848778307783194144668863247861").unwrap();
        let masked_content_hash = Fr::from_str(
            "15574265890121888853134966170838207038528069623841940909502184441509395967684",
        )
        .unwrap();
        let payload_start_index = Fr::from_str("103").unwrap();
        let payload_len = Fr::from_str("534").unwrap();
        let eph_public_key_0 = Fr::from_str("17932473587154777519561053972421347139").unwrap();
        let eph_public_key_1 = Fr::from_str("134696963602902907403122104327765350261").unwrap();
        let max_epoch = Fr::from_str("10000").unwrap();
        let nonce = Fr::from_str(
            "16637918813908060261870528903994038721669799613803601616678155512181273289477",
        )
        .unwrap();
        let num_sha2_blocks = Fr::from_str("11").unwrap();

        let hash = poseidon.hash(&[
            jwt_sha2_hash_0,
            jwt_sha2_hash_1,
            masked_content_hash,
            payload_start_index,
            payload_len,
            eph_public_key_0,
            eph_public_key_1,
            max_epoch,
            nonce,
            num_sha2_blocks,
        ]);
        assert_eq!(
            hash.to_string(),
            "13530254818489404407543803386086438162615837093532202208677673437093411882596"
                .to_string()
        );
    }
}

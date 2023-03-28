// Copyright (c) 2022, Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

//! Implementation of a verifier following RSASSA-PKCS1-v1_5 using SHA-256 (see https://datatracker.ietf.org/doc/rfc3447/).

use crate::error::{FastCryptoError, FastCryptoResult};
use crate::hash::{HashFunction, Sha256};
use rsa::pkcs1::DecodeRsaPublicKey;
use rsa::pkcs1v15::Signature as ExternalSignature;
use rsa::pkcs8::DecodePublicKey;
use rsa::RsaPublicKey as ExternalPublicKey;
use rsa::{Pkcs1v15Sign, PublicKey};

#[derive(Clone)]
pub struct RSAPublicKey(pub ExternalPublicKey);

#[derive(Clone, PartialEq, Eq)]
pub struct RSASignature(pub ExternalSignature);

impl RSAPublicKey {
    /// Parse an `RSAPublicKey` from a ASN.1 DER (Distinguished Encoding Rules) encoding.
    pub fn from_der(der: &[u8]) -> FastCryptoResult<Self> {
        // First try to parse the public key using PKCS#8 format and if this fails, try PKCS#1 format
        Ok(RSAPublicKey(
            rsa::RsaPublicKey::from_public_key_der(der)
                .or_else(|_| rsa::RsaPublicKey::from_pkcs1_der(der))
                .map_err(|_| FastCryptoError::InvalidInput)?,
        ))
    }

    /// Parse an `RSAPublicKey` from a PEM (Privacy-Enhanced Mail) encoding. Both PKCS#1 and PKCS#8
    /// formats are supported.
    pub fn from_pem(pem: &str) -> FastCryptoResult<Self> {
        // First try to parse the public key using PKCS#8 format and if this fails, try PKCS#1 format
        let pem = pem.trim();
        Ok(RSAPublicKey(
            rsa::RsaPublicKey::from_public_key_pem(pem)
                .or_else(|_| rsa::RsaPublicKey::from_pkcs1_pem(pem))
                .map_err(|_| FastCryptoError::InvalidInput)?,
        ))
    }

    /// Verify a signed message. The verification uses SHA-256 for hashing.
    pub fn verify(&self, msg: &[u8], signature: &RSASignature) -> FastCryptoResult<()> {
        self.verify_prehash(&Sha256::digest(msg).digest, signature)
    }

    /// Verify a signed message. The message, `hashed`, must be the output of a cryptographic hash function.
    pub fn verify_prehash(&self, hashed: &[u8], signature: &RSASignature) -> FastCryptoResult<()> {
        self.0
            .verify(
                Pkcs1v15Sign::new::<sha2::Sha256>(),
                hashed,
                signature.0.as_ref(),
            )
            .map_err(|_| FastCryptoError::InvalidSignature)
    }
}

impl RSASignature {
    pub fn from_bytes(bytes: &[u8]) -> FastCryptoResult<Self> {
        Ok(Self(
            ExternalSignature::try_from(bytes).map_err(|_| FastCryptoError::InvalidInput)?,
        ))
    }
}

#[cfg(test)]
mod test {
    use crate::hash::{HashFunction, Sha256};
    use crate::rsa::{RSAPublicKey, RSASignature};
    use base64ct::{Base64UrlUnpadded, Encoding};

    #[test]
    fn jwt_test() {
        // Test vector generated with https://dinochiesa.github.io/jwt/ and signed according to RS256.
        let pk_pem = "-----BEGIN PUBLIC KEY-----\nMIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEA5NXGDXfb1FDuWgAxQPVH\no+DPUkFl8rCjfj0nvQ++iubfMsMpP3UYu229GwYepOtKOpa4JA6uYGVibXql5ldh\nVZKG4LrGO8TL3S5C2qqac1CQbhwyG+DuyKBj1Fe5C7L/TWKmTep3eKEpolhXuaxN\nHR6R5TsxTb90RFToVRX/20rl8tHz/szWyPzmnLIOqae7UCVPFxenb3O7xa8SvSrV\nrPs2Eej3eEgOYORshP3HC6OQ8GV7ouJuM6VXPdRhb8BEWG/sTKmkr9qvrtoh2PpB\nlnEezat+7tbddPdI6LB4z4CIQzYkTu7OFZY5RV064c3skMmkEht3/Qrb7+MQsEWY\nlwIDAQAB\n-----END PUBLIC KEY-----";
        let pk = RSAPublicKey::from_pem(pk_pem).unwrap();

        let digest = Sha256::digest(b"eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCIsImtpZCI6IjE0ZWJjMDRlNmFjM2QzZTk2MDMxZDJjY2QzODZmY2E5NWRkZjMyZGQifQ.eyJpc3MiOiJodHRwczovL3d3dy5mYWNlYm9vay5jb20iLCJhdWQiOiIxMjQxNTU5MzY2NzU1MjE0Iiwic3ViIjoiNzA4NTYyNjExMDA5NTI1IiwiaWF0IjoxNjc5OTMyMDE0LCJleHAiOjE2Nzk5MzU2MTQsImp0aSI6IlRLdnouZGJlYzdjYTMxOTQyYTVkMmU1NmJkMGRiZmI4MjRiMTcxODVlMGYzMGIyMGYyNTczZGU1ZDQ4ZmM5ZjU4M2U0MyIsIm5vbmNlIjoidGVzdCIsImdpdmVuX25hbWUiOiJKb3kiLCJmYW1pbHlfbmFtZSI6IldhbmciLCJuYW1lIjoiSm95IFdhbmciLCJwaWN0dXJlIjoiaHR0cHM6Ly9wbGF0Zm9ybS1sb29rYXNpZGUuZmJzYnguY29tL3BsYXRmb3JtL3Byb2ZpbGVwaWMvP2FzaWQ9NzA4NTYyNjExMDA5NTI1JmhlaWdodD0xMDAmd2lkdGg9MTAwJmV4dD0xNjgyNTI0MDE1Jmhhc2g9QWVTMENxblhPMmNhT3g4WDhRZyJ9").digest;

        let signature = "Z65bdJv-sFO9mNe4i1Tv0fa74rEtSIh3ZzJ29JtojgpA_d40JfE_NVJliYvoZdfqPX85a8NAG-ujKWWzrtv8l3K33r-T0WuUvosai99Y7TrMZt3WtT9pLwoO4s8KPSr9jXjTD94YFhizdKtyHFvaJRVjyUWFTvsQgZP9kyiSPh-7R_CStVan2u0scZRosZeOlZT4dI5xXnt3AFH-vFfaWiZEEunKljIkqvdrtt3x-HLFnjSvKGFi1Ct4LBObdjbNGJULYjQ0-N7yuQevaiYEpSFW1NBfa3p52vMj9XMADhg4wrV7Nuvk7CqERLeL-M8L_KmUGnRXOmMUL-6KTC8Rtw";
        let signature_bytes = Base64UrlUnpadded::decode_vec(signature).unwrap();
        let signature = RSASignature::from_bytes(&signature_bytes).unwrap();

        assert!(pk.verify_prehash(&digest, &signature).is_ok());
    }
}

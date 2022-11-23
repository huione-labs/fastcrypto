// Copyright (c) 2022, Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

//! This module contains an implementation of the [ECDSA signature scheme](https://en.wikipedia.org/wiki/Elliptic_Curve_Digital_Signature_Algorithm) over the
//! [secp256r1 NIST-P1 curve](https://www.secg.org/SEC2-Ver-1.0.pdf).
//!
//! Messages can be signed and the signature can be verified again:
//! # Example
//! ```rust
//! # use fastcrypto::secp256r1::*;
//! # use fastcrypto::{traits::{KeyPair, Signer}, Verifier};
//! use rand::thread_rng;
//! let kp = Secp256r1KeyPair::generate(&mut thread_rng());
//! let message: &[u8] = b"Hello, world!";
//! let signature = kp.sign(message);
//! assert!(kp.public().verify(message, &signature).is_ok());
//! ```

use crate::{
    encoding::{Base64, Encoding},
    error::FastCryptoError,
    pubkey_bytes::PublicKeyBytes,
    serde_helpers::keypair_decode_base64,
    traits::{
        AllowedRng, Authenticator, EncodeDecodeBase64, KeyPair, SigningKey, ToFromBytes,
        VerifyingKey,
    },
};
use fastcrypto_derive::{SilentDebug, SilentDisplay};
use once_cell::sync::OnceCell;
use p256::ecdsa::Signature as ExternalSignature;
use p256::ecdsa::SigningKey as ExternalSecretKey;
use p256::ecdsa::VerifyingKey as ExternalPublicKey;
use p256::elliptic_curve::group::GroupEncoding;

use crate::hash::HashFunction;
use crate::hash::Sha256;
use ecdsa::hazmat::SignPrimitive;
use generic_array::GenericArray;
use p256::elliptic_curve::bigint::ArrayEncoding;
use p256::elliptic_curve::ops::Reduce;
use p256::elliptic_curve::{Curve, DecompactPoint};
use p256::{AffinePoint, FieldBytes, NistP256, ProjectivePoint, Scalar, U256};
use serde::{de, Deserialize, Serialize};
use signature::{Signature, Signer, Verifier};
use std::{
    fmt::{self, Debug, Display},
    str::FromStr,
};
use zeroize::Zeroize;

pub const PUBLIC_KEY_SIZE: usize = 33;
pub const PRIVATE_KEY_SIZE: usize = 32;
pub const SIGNATURE_SIZE: usize = 64;

/// Secp256r1 public key.
#[readonly::make]
#[derive(Debug, Clone)]
pub struct Secp256r1PublicKey {
    pub pubkey: ExternalPublicKey,
    pub bytes: OnceCell<[u8; PUBLIC_KEY_SIZE]>,
}

/// Binary representation of an instance of [Secp256r1PublicKey].
pub type Secp256r1PublicKeyBytes = PublicKeyBytes<Secp256r1PublicKey, { PUBLIC_KEY_SIZE }>;

/// Secp256r1 private key.
#[readonly::make]
#[derive(SilentDebug, SilentDisplay)]
pub struct Secp256r1PrivateKey {
    pub privkey: ExternalSecretKey,
    pub bytes: OnceCell<[u8; PRIVATE_KEY_SIZE]>,
}

/// Secp256r1 signature.
#[readonly::make]
#[derive(Debug, Clone)]
pub struct Secp256r1Signature {
    pub sig: ExternalSignature,
    pub bytes: OnceCell<[u8; SIGNATURE_SIZE]>,
}

impl std::hash::Hash for Secp256r1PublicKey {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.as_ref().hash(state);
    }
}

impl PartialOrd for Secp256r1PublicKey {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.as_ref().partial_cmp(other.as_ref())
    }
}

impl Ord for Secp256r1PublicKey {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.as_ref().cmp(other.as_ref())
    }
}

impl PartialEq for Secp256r1PublicKey {
    fn eq(&self, other: &Self) -> bool {
        self.pubkey == other.pubkey
    }
}

impl Eq for Secp256r1PublicKey {}

impl VerifyingKey for Secp256r1PublicKey {
    type PrivKey = Secp256r1PrivateKey;
    type Sig = Secp256r1Signature;
    const LENGTH: usize = PUBLIC_KEY_SIZE;
}

impl Verifier<Secp256r1Signature> for Secp256r1PublicKey {
    fn verify(&self, msg: &[u8], signature: &Secp256r1Signature) -> Result<(), signature::Error> {
        // TODO: Until we have a recovery id, we recover both candidates for public keys
        let pks = signature
            .recover(msg)
            .map_err(|_| signature::Error::new())?;

        if !pks.contains(self) {
            return Err(signature::Error::new());
        }

        self.pubkey
            .verify(msg, &signature.sig)
            .map_err(|_| signature::Error::new())
    }
}

impl Secp256r1PublicKey {}

impl AsRef<[u8]> for Secp256r1PublicKey {
    fn as_ref(&self) -> &[u8] {
        self.bytes
            .get_or_try_init::<_, eyre::Report>(|| Ok(self.pubkey.as_ref().to_bytes().into()))
            .expect("OnceCell invariant violated")
    }
}

impl ToFromBytes for Secp256r1PublicKey {
    fn from_bytes(bytes: &[u8]) -> Result<Self, FastCryptoError> {
        match ExternalPublicKey::try_from(bytes) {
            Ok(pubkey) => Ok(Secp256r1PublicKey {
                pubkey,
                // If the given bytes is in the right format (compressed), we keep them for next time to_bytes is called
                bytes: match <[u8; PUBLIC_KEY_SIZE]>::try_from(bytes) {
                    Ok(result) => OnceCell::with_value(result),
                    Err(_) => OnceCell::new(),
                },
            }),
            Err(_) => Err(FastCryptoError::InvalidInput),
        }
    }
}

impl Default for Secp256r1PublicKey {
    fn default() -> Self {
        // Default public key is just the generator for the group
        Secp256r1PublicKey {
            pubkey: ExternalPublicKey::from_affine(AffinePoint::GENERATOR).unwrap(),
            bytes: OnceCell::new(),
        }
    }
}

impl Display for Secp256r1PublicKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Base64::encode(self.as_ref()))
    }
}

// There is a strong requirement for this specific impl. in Fab benchmarks
impl Serialize for Secp256r1PublicKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.encode_base64())
    }
}

impl<'de> Deserialize<'de> for Secp256r1PublicKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        let value = Self::decode_base64(&s).map_err(|e| de::Error::custom(e.to_string()))?;
        Ok(value)
    }
}

impl<'a> From<&'a Secp256r1PrivateKey> for Secp256r1PublicKey {
    fn from(secret: &'a Secp256r1PrivateKey) -> Self {
        Secp256r1PublicKey {
            pubkey: ExternalPublicKey::from(&secret.privkey),
            bytes: OnceCell::new(),
        }
    }
}

impl SigningKey for Secp256r1PrivateKey {
    type PubKey = Secp256r1PublicKey;
    type Sig = Secp256r1Signature;
    const LENGTH: usize = PRIVATE_KEY_SIZE;
}

impl ToFromBytes for Secp256r1PrivateKey {
    fn from_bytes(bytes: &[u8]) -> Result<Self, FastCryptoError> {
        match ExternalSecretKey::try_from(bytes) {
            Ok(privkey) => Ok(Secp256r1PrivateKey {
                privkey,
                bytes: OnceCell::with_value(<[u8; 32]>::try_from(bytes).unwrap()),
            }),
            Err(_) => Err(FastCryptoError::InvalidInput),
        }
    }
}

// There is a strong requirement for this specific impl. in Fab benchmarks
impl Serialize for Secp256r1PrivateKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.encode_base64())
    }
}

// There is a strong requirement for this specific impl. in Fab benchmarks
impl<'de> Deserialize<'de> for Secp256r1PrivateKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let value = Self::decode_base64(&s).map_err(|e| de::Error::custom(e.to_string()))?;
        Ok(value)
    }
}

impl AsRef<[u8]> for Secp256r1PrivateKey {
    fn as_ref(&self) -> &[u8] {
        self.bytes
            .get_or_try_init::<_, eyre::Report>(|| Ok(self.privkey.to_bytes().into()))
            .expect("OnceCell invariant violated")
    }
}

impl Serialize for Secp256r1Signature {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.as_ref().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Secp256r1Signature {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let data: Vec<u8> = Vec::deserialize(deserializer)?;
        <Secp256r1Signature as Signature>::from_bytes(&data)
            .map_err(|e| de::Error::custom(e.to_string()))
    }
}

impl Signature for Secp256r1Signature {
    fn from_bytes(bytes: &[u8]) -> Result<Self, signature::Error> {
        match <ExternalSignature as Signature>::from_bytes(bytes) {
            Ok(sig) => Ok(Secp256r1Signature {
                sig,
                // If the given bytes is in the right format (compressed), we keep them for next time to_bytes is called
                bytes: match <[u8; SIGNATURE_SIZE]>::try_from(bytes) {
                    Ok(result) => OnceCell::with_value(result),
                    Err(_) => OnceCell::new(),
                },
            }),
            Err(_) => Err(signature::Error::new()),
        }
    }
}

impl Authenticator for Secp256r1Signature {
    type PubKey = Secp256r1PublicKey;
    type PrivKey = Secp256r1PrivateKey;
    const LENGTH: usize = SIGNATURE_SIZE;
}

impl AsRef<[u8]> for Secp256r1Signature {
    fn as_ref(&self) -> &[u8] {
        let bytes = self.sig.as_ref();
        self.bytes
            .get_or_try_init::<_, eyre::Report>(|| Ok(bytes.try_into().unwrap()))
            .expect("OnceCell invariant violated")
    }
}

impl std::hash::Hash for Secp256r1Signature {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.as_ref().hash(state);
    }
}

impl PartialEq for Secp256r1Signature {
    fn eq(&self, other: &Self) -> bool {
        self.sig == other.sig
    }
}

impl Eq for Secp256r1Signature {}

impl Display for Secp256r1Signature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", Base64::encode(self.as_ref()))
    }
}

impl Default for Secp256r1Signature {
    fn default() -> Self {
        // Return the signature (1,1)
        Secp256r1Signature {
            sig: ExternalSignature::from_scalars(Scalar::ONE.to_bytes(), Scalar::ONE.to_bytes())
                .unwrap(),
            bytes: OnceCell::new(),
        }
    }
}

// There is a strong requirement for this specific impl. in Fab benchmarks
/// Secp256r1 public/private key pair.
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")] // necessary so as not to deser under a != type
pub struct Secp256r1KeyPair {
    pub name: Secp256r1PublicKey,
    pub secret: Secp256r1PrivateKey,
}

impl EncodeDecodeBase64 for Secp256r1KeyPair {
    fn encode_base64(&self) -> String {
        let mut bytes: Vec<u8> = Vec::new();
        bytes.extend_from_slice(self.secret.as_ref());
        bytes.extend_from_slice(self.name.as_ref());
        Base64::encode(&bytes[..])
    }

    fn decode_base64(value: &str) -> Result<Self, eyre::Report> {
        keypair_decode_base64(value)
    }
}

impl KeyPair for Secp256r1KeyPair {
    type PubKey = Secp256r1PublicKey;
    type PrivKey = Secp256r1PrivateKey;
    type Sig = Secp256r1Signature;

    fn public(&'_ self) -> &'_ Self::PubKey {
        &self.name
    }

    fn private(self) -> Self::PrivKey {
        Secp256r1PrivateKey::from_bytes(self.secret.as_ref()).unwrap()
    }

    #[cfg(feature = "copy_key")]
    fn copy(&self) -> Self {
        Secp256r1KeyPair {
            name: self.name.clone(),
            secret: Secp256r1PrivateKey::from_bytes(self.secret.as_ref()).unwrap(),
        }
    }

    fn generate<R: AllowedRng>(rng: &mut R) -> Self {
        let privkey = ExternalSecretKey::random(rng);
        let pubkey = ExternalPublicKey::from(&privkey);

        Secp256r1KeyPair {
            name: Secp256r1PublicKey {
                pubkey,
                bytes: OnceCell::new(),
            },
            secret: Secp256r1PrivateKey {
                privkey,
                bytes: OnceCell::new(),
            },
        }
    }
}

impl FromStr for Secp256r1KeyPair {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let kp = Self::decode_base64(s).map_err(|e| eyre::eyre!("{}", e.to_string()))?;
        Ok(kp)
    }
}

impl Signer<Secp256r1Signature> for Secp256r1KeyPair {
    fn try_sign(&self, msg: &[u8]) -> Result<Secp256r1Signature, signature::Error> {
        let digest = Sha256::digest(msg);
        let sig = self
            .secret
            .privkey
            .as_nonzero_scalar()
            .try_sign_prehashed_rfc6979::<sha2::Sha256>(FieldBytes::from(digest.digest), &[])?;
        Ok(Secp256r1Signature {
            sig: sig.0,
            bytes: OnceCell::new(),
        })
    }
}

impl TryFrom<Secp256r1PublicKeyBytes> for Secp256r1PublicKey {
    type Error = signature::Error;

    fn try_from(bytes: Secp256r1PublicKeyBytes) -> Result<Secp256r1PublicKey, Self::Error> {
        Secp256r1PublicKey::from_bytes(bytes.as_ref()).map_err(|_| Self::Error::new())
    }
}

impl From<&Secp256r1PublicKey> for Secp256r1PublicKeyBytes {
    fn from(pk: &Secp256r1PublicKey) -> Self {
        Secp256r1PublicKeyBytes::from_bytes(pk.as_ref()).unwrap()
    }
}

impl From<Secp256r1PrivateKey> for Secp256r1KeyPair {
    fn from(secret: Secp256r1PrivateKey) -> Self {
        let name = Secp256r1PublicKey::from(&secret);
        Secp256r1KeyPair { name, secret }
    }
}

impl Secp256r1Signature {
    /// Recover public key(s) from signature. Either 2 or 4 potential public keys,
    /// which could be the corresponding public key for the signature are returned.
    /// This is based on section 4.1.6 in https://www.secg.org/sec1-v2.pdf.
    ///
    /// An [FastCryptoError::GeneralError] is returned if no public keys can be recovered.
    pub fn recover(&self, msg: &[u8]) -> Result<Vec<Secp256r1PublicKey>, FastCryptoError> {
        // This implementation is based on recover_verifying_key_from_digest_bytes in the p256 crate,
        // but also handles the case the the x-coordinate is larger than the group order.

        let (r, s) = self.sig.split_scalars();
        let r_plus_n = U256::from(r.as_ref()).wrapping_add(&NistP256::ORDER);

        // Find points with r or r+n as x-coordinate.
        let mut pts = vec![
            AffinePoint::decompact(&r.to_bytes()),
            AffinePoint::decompact(&r_plus_n.to_be_byte_array()),
        ];

        // Only keep valid points and return err if there a no such points
        pts.retain(|pt| pt.is_some().into());
        if pts.is_empty() {
            return Err(FastCryptoError::GeneralError);
        }

        // Hash of the message.
        let e = <Scalar as Reduce<U256>>::from_be_bytes_reduced(GenericArray::from(
            Sha256::digest(msg).digest,
        ));

        // Compute the up to four candidates for public keys
        let r_inv = r.invert().unwrap();
        let g_term = ProjectivePoint::GENERATOR * -r_inv * e;
        let mut candidates: Vec<Secp256r1PublicKey> = Vec::new();
        for affine_pt in pts {
            let r_term = ProjectivePoint::from(affine_pt.unwrap()) * r_inv * *s;
            candidates.push(Secp256r1PublicKey {
                pubkey: ExternalPublicKey::from_affine((g_term + r_term).to_affine()).unwrap(),
                bytes: OnceCell::new(),
            });
            candidates.push(Secp256r1PublicKey {
                pubkey: ExternalPublicKey::from_affine((g_term - r_term).to_affine()).unwrap(),
                bytes: OnceCell::new(),
            });
        }

        Ok(candidates)
    }
}

impl zeroize::Zeroize for Secp256r1PrivateKey {
    fn zeroize(&mut self) {
        self.bytes.take().zeroize();
        // SigningKey from the p256 crate implements zeroize on drop, so we do not need to zeroize it here.
    }
}

impl Drop for Secp256r1PrivateKey {
    fn drop(&mut self) {
        self.bytes.take().zeroize();
    }
}

impl zeroize::ZeroizeOnDrop for Secp256r1PrivateKey {}

impl zeroize::Zeroize for Secp256r1KeyPair {
    fn zeroize(&mut self) {
        self.secret.zeroize()
    }
}

impl zeroize::ZeroizeOnDrop for Secp256r1KeyPair {}

impl Drop for Secp256r1KeyPair {
    fn drop(&mut self) {
        self.secret.zeroize();
    }
}

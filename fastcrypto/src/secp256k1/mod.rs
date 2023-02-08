// Copyright (c) 2022, Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

//! This module contains an implementation of the [ECDSA signature scheme](https://en.wikipedia.org/wiki/Elliptic_Curve_Digital_Signature_Algorithm) over the [secp256k1 curve](http://www.secg.org/sec2-v2.pdf).
//!
//! Messages can be signed and the signature can be verified again:
//! # Example
//! ```rust
//! # use fastcrypto::secp256k1::*;
//! # use fastcrypto::{traits::{KeyPair, Signer, VerifyingKey}};
//! use rand::thread_rng;
//! let kp = Secp256k1KeyPair::generate(&mut thread_rng());
//! let message: &[u8] = b"Hello, world!";
//! let signature = kp.sign(message);
//! assert!(kp.public().verify(message, &signature).is_ok());
//! ```

pub mod recoverable;

use crate::secp256k1::recoverable::Secp256k1RecoverableSignature;
use crate::{
    encoding::{Base64, Encoding},
    error::FastCryptoError,
    serialize_deserialize_with_to_from_bytes,
    traits::{
        AllowedRng, Authenticator, EncodeDecodeBase64, KeyPair, SigningKey, ToFromBytes,
        VerifyingKey,
    },
};
use fastcrypto_derive::{SilentDebug, SilentDisplay};
use once_cell::sync::{Lazy, OnceCell};
use rust_secp256k1::hashes::sha256;
use rust_secp256k1::{
    constants, ecdsa::Signature as NonrecoverableSignature, All, Message, PublicKey, Secp256k1,
    SecretKey,
};
use signature::{Signature, Signer};
use std::{
    fmt::{self, Debug, Display},
    str::FromStr,
};
use zeroize::Zeroize;

pub static SECP256K1: Lazy<Secp256k1<All>> = Lazy::new(rust_secp256k1::Secp256k1::new);

/// The length of a public key in bytes.
pub const SECP256K1_PUBLIC_KEY_LENGTH: usize = constants::PUBLIC_KEY_SIZE;

/// The length of a private key in bytes.
pub const SECP256K1_PRIVATE_KEY_LENGTH: usize = constants::SECRET_KEY_SIZE;

/// The length of a signature in bytes.
pub const SECP256K1_SIGNATURE_LENGTH: usize = constants::COMPACT_SIGNATURE_SIZE;

/// The key pair bytes length is the same as the private key length. This enforces deserialization to always derive the public key from the private key.
pub const SECP256K1_KEYPAIR_LENGTH: usize = constants::SECRET_KEY_SIZE;
/// Secp256k1 public key.
#[readonly::make]
#[derive(Debug, Clone)]
pub struct Secp256k1PublicKey {
    pub pubkey: PublicKey,
    pub bytes: OnceCell<[u8; SECP256K1_PUBLIC_KEY_LENGTH]>,
}

/// Secp256k1 private key.
#[readonly::make]
#[derive(SilentDebug, SilentDisplay, PartialEq, Eq)]
pub struct Secp256k1PrivateKey {
    pub privkey: SecretKey,
    pub bytes: OnceCell<[u8; SECP256K1_PRIVATE_KEY_LENGTH]>,
}

/// Secp256k1 ECDSA signature.
#[readonly::make]
#[derive(Debug, Clone)]
pub struct Secp256k1Signature {
    pub sig: NonrecoverableSignature,
    pub bytes: OnceCell<[u8; SECP256K1_SIGNATURE_LENGTH]>,
}

impl std::hash::Hash for Secp256k1PublicKey {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.as_ref().hash(state);
    }
}

impl PartialOrd for Secp256k1PublicKey {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.as_ref().partial_cmp(other.as_ref())
    }
}

impl Ord for Secp256k1PublicKey {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.as_ref().cmp(other.as_ref())
    }
}

impl PartialEq for Secp256k1PublicKey {
    fn eq(&self, other: &Self) -> bool {
        self.pubkey == other.pubkey
    }
}

impl Eq for Secp256k1PublicKey {}

impl VerifyingKey for Secp256k1PublicKey {
    type PrivKey = Secp256k1PrivateKey;
    type Sig = Secp256k1Signature;
    const LENGTH: usize = constants::PUBLIC_KEY_SIZE;

    fn verify(&self, msg: &[u8], signature: &Secp256k1Signature) -> Result<(), FastCryptoError> {
        // Sha256 is used by default as digest
        let message = Message::from_hashed_data::<sha256::Hash>(msg);
        self.verify_hashed(message.as_ref(), signature)
            .map_err(|_| FastCryptoError::GeneralOpaqueError)
    }
}

impl Secp256k1PublicKey {
    /// Verify the signature over an already hashed message.
    pub fn verify_hashed(
        &self,
        hashed_msg: &[u8],
        signature: &Secp256k1Signature,
    ) -> Result<(), signature::Error> {
        let message = Message::from_slice(hashed_msg).map_err(|_| signature::Error::new())?;
        signature
            .sig
            .verify(&message, &self.pubkey)
            .map_err(|_| signature::Error::new())
    }

    /// util function to parse wycheproof test key from DER format.
    #[cfg(test)]
    pub fn from_uncompressed(uncompressed: &[u8]) -> Self {
        let pubkey = PublicKey::from_slice(uncompressed).unwrap();
        Self {
            pubkey,
            bytes: OnceCell::new(),
        }
    }
}

impl AsRef<[u8]> for Secp256k1PublicKey {
    fn as_ref(&self) -> &[u8] {
        self.bytes
            .get_or_try_init::<_, eyre::Report>(|| Ok(self.pubkey.serialize()))
            .expect("OnceCell invariant violated")
    }
}

impl ToFromBytes for Secp256k1PublicKey {
    fn from_bytes(bytes: &[u8]) -> Result<Self, FastCryptoError> {
        match PublicKey::from_slice(bytes) {
            Ok(pubkey) => Ok(Secp256k1PublicKey {
                pubkey,
                bytes: OnceCell::new(),
            }),
            Err(_) => Err(FastCryptoError::InvalidInput),
        }
    }
}

impl Display for Secp256k1PublicKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Base64::encode(self.as_ref()))
    }
}

serialize_deserialize_with_to_from_bytes!(Secp256k1PublicKey, SECP256K1_PUBLIC_KEY_LENGTH);

impl<'a> From<&'a Secp256k1PrivateKey> for Secp256k1PublicKey {
    fn from(secret: &'a Secp256k1PrivateKey) -> Self {
        Secp256k1PublicKey {
            pubkey: secret.privkey.public_key(&SECP256K1),
            bytes: OnceCell::new(),
        }
    }
}

impl SigningKey for Secp256k1PrivateKey {
    type PubKey = Secp256k1PublicKey;
    type Sig = Secp256k1Signature;
    const LENGTH: usize = constants::SECRET_KEY_SIZE;
}

impl ToFromBytes for Secp256k1PrivateKey {
    fn from_bytes(bytes: &[u8]) -> Result<Self, FastCryptoError> {
        match SecretKey::from_slice(bytes) {
            Ok(privkey) => Ok(Secp256k1PrivateKey {
                privkey,
                bytes: OnceCell::new(),
            }),
            Err(_) => Err(FastCryptoError::InvalidInput),
        }
    }
}

serialize_deserialize_with_to_from_bytes!(Secp256k1PrivateKey, SECP256K1_PRIVATE_KEY_LENGTH);

impl AsRef<[u8]> for Secp256k1PrivateKey {
    fn as_ref(&self) -> &[u8] {
        self.bytes
            .get_or_try_init::<_, eyre::Report>(|| Ok(self.privkey.secret_bytes()))
            .expect("OnceCell invariant violated")
    }
}

impl zeroize::Zeroize for Secp256k1PrivateKey {
    fn zeroize(&mut self) {
        // Unwrap is safe here because we are using a constant and it has been tested
        // (see fastcrypto/src/tests/secp256k1_tests::test_sk_zeroization_on_drop)
        self.privkey = SecretKey::from_slice(&constants::ONE).unwrap();
        self.bytes.take().zeroize();
    }
}

impl zeroize::ZeroizeOnDrop for Secp256k1PrivateKey {}

impl Drop for Secp256k1PrivateKey {
    fn drop(&mut self) {
        self.zeroize();
    }
}

serialize_deserialize_with_to_from_bytes!(Secp256k1Signature, SECP256K1_SIGNATURE_LENGTH);

impl Signature for Secp256k1Signature {
    fn from_bytes(bytes: &[u8]) -> Result<Self, signature::Error> {
        if bytes.len() != SECP256K1_SIGNATURE_LENGTH {
            return Err(signature::Error::new());
        }
        NonrecoverableSignature::from_compact(bytes)
            .map(|sig| Secp256k1Signature {
                sig,
                bytes: OnceCell::new(),
            })
            .map_err(|_| signature::Error::new())
    }
}

impl Authenticator for Secp256k1Signature {
    type PubKey = Secp256k1PublicKey;
    type PrivKey = Secp256k1PrivateKey;
    const LENGTH: usize = SECP256K1_SIGNATURE_LENGTH;
}

impl AsRef<[u8]> for Secp256k1Signature {
    fn as_ref(&self) -> &[u8] {
        self.bytes
            .get_or_try_init::<_, eyre::Report>(|| Ok(self.sig.serialize_compact()))
            .expect("OnceCell invariant violated")
    }
}

impl std::hash::Hash for Secp256k1Signature {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.as_ref().hash(state);
    }
}

impl Display for Secp256k1Signature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", Base64::encode(self.as_ref()))
    }
}

impl PartialEq for Secp256k1Signature {
    fn eq(&self, other: &Self) -> bool {
        self.sig == other.sig
    }
}

impl Eq for Secp256k1Signature {}

impl From<&Secp256k1RecoverableSignature> for Secp256k1Signature {
    fn from(recoverable_signature: &Secp256k1RecoverableSignature) -> Self {
        Secp256k1Signature {
            sig: recoverable_signature.sig.to_standard(),
            bytes: OnceCell::new(),
        }
    }
}

/// Secp256k1 public/private key pair.
#[derive(Debug, PartialEq, Eq)]
pub struct Secp256k1KeyPair {
    pub name: Secp256k1PublicKey,
    pub secret: Secp256k1PrivateKey,
}

/// The bytes form of the keypair always only contain the private key bytes
impl ToFromBytes for Secp256k1KeyPair {
    fn from_bytes(bytes: &[u8]) -> Result<Self, FastCryptoError> {
        Secp256k1PrivateKey::from_bytes(bytes).map(|secret| secret.into())
    }
}

serialize_deserialize_with_to_from_bytes!(Secp256k1KeyPair, SECP256K1_KEYPAIR_LENGTH);

impl AsRef<[u8]> for Secp256k1KeyPair {
    fn as_ref(&self) -> &[u8] {
        self.secret.as_ref()
    }
}

impl KeyPair for Secp256k1KeyPair {
    type PubKey = Secp256k1PublicKey;
    type PrivKey = Secp256k1PrivateKey;
    type Sig = Secp256k1Signature;

    fn public(&'_ self) -> &'_ Self::PubKey {
        &self.name
    }

    fn private(self) -> Self::PrivKey {
        Secp256k1PrivateKey::from_bytes(self.secret.as_ref()).unwrap()
    }

    #[cfg(feature = "copy_key")]
    fn copy(&self) -> Self {
        Secp256k1KeyPair {
            name: self.name.clone(),
            secret: Secp256k1PrivateKey::from_bytes(self.secret.as_ref()).unwrap(),
        }
    }

    fn generate<R: AllowedRng>(rng: &mut R) -> Self {
        let (privkey, pubkey) = SECP256K1.generate_keypair(rng);

        Secp256k1KeyPair {
            name: Secp256k1PublicKey {
                pubkey,
                bytes: OnceCell::new(),
            },
            secret: Secp256k1PrivateKey {
                privkey,
                bytes: OnceCell::new(),
            },
        }
    }
}

impl FromStr for Secp256k1KeyPair {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let kp = Self::decode_base64(s).map_err(|e| eyre::eyre!("{}", e.to_string()))?;
        Ok(kp)
    }
}

impl Signer<Secp256k1Signature> for Secp256k1KeyPair {
    fn try_sign(&self, msg: &[u8]) -> Result<Secp256k1Signature, signature::Error> {
        // Sha256 is used by default
        let message = Message::from_hashed_data::<sha256::Hash>(msg);

        // Creates a 64-bytes signature of shape [r, s].
        // Pseudo-random deterministic nonce generation is used according to RFC6979.
        Ok(Secp256k1Signature {
            sig: Secp256k1::signing_only().sign_ecdsa(&message, &self.secret.privkey),
            bytes: OnceCell::new(),
        })
    }
}

impl From<Secp256k1PrivateKey> for Secp256k1KeyPair {
    fn from(secret: Secp256k1PrivateKey) -> Self {
        let name = Secp256k1PublicKey::from(&secret);
        Secp256k1KeyPair { name, secret }
    }
}

impl zeroize::Zeroize for Secp256k1KeyPair {
    fn zeroize(&mut self) {
        self.secret.zeroize()
    }
}

impl zeroize::ZeroizeOnDrop for Secp256k1KeyPair {}

impl Drop for Secp256k1KeyPair {
    fn drop(&mut self) {
        self.secret.zeroize();
    }
}

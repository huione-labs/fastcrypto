// Copyright (c) 2022, Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

//! Implementation of the Secp256r1 (aka P-256) curve. This is a 256-bit Weirstrass curve of prime order.
//! See "SEC 2: Recommended Elliptic Curve Domain Parameters" for details."

use crate::error::{FastCryptoError, FastCryptoResult};
use crate::groups::multiplier::ToLittleEndianBytes;
use crate::groups::{Doubling, GroupElement, Scalar as ScalarTrait};
use crate::serde_helpers::ToFromByteArray;
use crate::serialize_deserialize_with_to_from_byte_array;
use crate::traits::AllowedRng;
use ark_ec::Group;
use ark_ff::{Field, One, UniformRand, Zero};
use ark_secp256r1::{Fr, Projective};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use derive_more::{Add, From, Neg, Sub};
use fastcrypto_derive::GroupOpsExtend;
use serde::{de, Deserialize};
use std::ops::{Div, Mul};

pub const SCALAR_SIZE_IN_BYTES: usize = 32;

/// A point on the Secp256r1 curve in projective coordinates.
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, From, Add, Sub, Neg, GroupOpsExtend)]
pub struct ProjectivePoint(pub(crate) Projective);

impl GroupElement for ProjectivePoint {
    type ScalarType = Scalar;

    fn zero() -> Self {
        Self(Projective::zero())
    }

    fn generator() -> Self {
        Self(Projective::generator())
    }
}

impl Doubling for ProjectivePoint {
    fn double(self) -> Self {
        ProjectivePoint::from(self.0.double())
    }
}

impl Mul<Scalar> for ProjectivePoint {
    type Output = ProjectivePoint;

    fn mul(self, rhs: Scalar) -> ProjectivePoint {
        ProjectivePoint::from(self.0 * rhs.0)
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl Div<Scalar> for ProjectivePoint {
    type Output = Result<ProjectivePoint, FastCryptoError>;

    fn div(self, rhs: Scalar) -> Result<ProjectivePoint, FastCryptoError> {
        Ok(self * rhs.inverse()?)
    }
}

/// A field element in the prime field of the same order as the curve.
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, From, Add, Sub, Neg, GroupOpsExtend)]
pub struct Scalar(pub(crate) Fr);

impl GroupElement for Scalar {
    type ScalarType = Scalar;

    fn zero() -> Self {
        Scalar(Fr::zero())
    }

    fn generator() -> Self {
        Scalar(Fr::one())
    }
}

impl Mul<Scalar> for Scalar {
    type Output = Scalar;

    fn mul(self, rhs: Scalar) -> Self::Output {
        Scalar(self.0 * rhs.0)
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl Div<Scalar> for Scalar {
    type Output = Result<Scalar, FastCryptoError>;

    fn div(self, rhs: Scalar) -> Result<Scalar, FastCryptoError> {
        Ok(self * rhs.inverse()?)
    }
}

impl From<u128> for Scalar {
    fn from(value: u128) -> Self {
        Scalar(Fr::from(value))
    }
}

impl ScalarTrait for Scalar {
    fn rand<R: AllowedRng>(rng: &mut R) -> Self {
        Scalar(Fr::rand(rng))
    }

    fn inverse(&self) -> FastCryptoResult<Self> {
        Ok(Scalar(
            self.0.inverse().ok_or(FastCryptoError::InvalidInput)?,
        ))
    }
}

impl ToFromByteArray<SCALAR_SIZE_IN_BYTES> for Scalar {
    fn from_byte_array(bytes: &[u8; SCALAR_SIZE_IN_BYTES]) -> Result<Self, FastCryptoError> {
        Ok(Scalar(
            Fr::deserialize_uncompressed(bytes.as_slice())
                .map_err(|_| FastCryptoError::InvalidInput)?,
        ))
    }

    fn to_byte_array(&self) -> [u8; SCALAR_SIZE_IN_BYTES] {
        let mut bytes = [0u8; SCALAR_SIZE_IN_BYTES];
        self.0
            .serialize_uncompressed(&mut bytes[..])
            .expect("Byte array not large enough");
        bytes
    }
}

impl ToLittleEndianBytes for Scalar {
    fn to_le_bytes(&self) -> Vec<u8> {
        self.to_byte_array().to_vec()
    }
}

serialize_deserialize_with_to_from_byte_array!(Scalar);

// Copyright (c) 2022, Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use fastcrypto::error::FastCryptoResult;
use std::ops::{Add, Neg};

#[cfg(any(test, feature = "experimental"))]
pub mod class_group;

#[cfg(any(test, feature = "experimental"))]
pub mod vdf;

mod extended_gcd;
pub mod hash_prime;

pub trait ToBytes {
    fn to_bytes(&self) -> Vec<u8>;
}

pub trait Parameter: Eq + Sized + ToBytes {
    /// Compute a random instance of a given size from a seed.
    fn from_seed(seed: &[u8], size_in_bits: usize) -> FastCryptoResult<Self>;
}

/// Trait implemented by elements of an additive group where the group is parameterized, for example
/// by the modulus in case of the group being Z mod N or the discriminant in case of class groups.
pub trait ParameterizedGroupElement:
    Sized + Clone + for<'a> Add<&'a Self, Output = Self> + Add<Output = Self> + Neg + Eq
{
    /// The type of the parameter which uniquely defines this group.
    type ParameterType: Parameter;

    /// Integer type used for multiplication.
    type ScalarType: From<u64>;

    /// Return an instance of the identity element in this group.
    fn zero(parameters: &Self::ParameterType) -> Self;

    /// Compute 2 * Self.
    fn double(self) -> Self;

    /// Compute scale * self.
    fn mul(&self, scale: &Self::ScalarType) -> Self;

    /// Serialize this group element.
    fn as_bytes(&self) -> Vec<u8>;

    /// Check whether this group element is in the same group as `other`.
    fn same_group(&self, other: &Self) -> bool;
}

/// Trait impl'd by elements of groups where the order is unknown.
pub trait UnknownOrderGroupElement {}

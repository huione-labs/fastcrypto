// Copyright (c) 2022, Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

//! This module contains an implementation of imaginary class groups. Elements are represented by
//! binary quadratic forms which forms a group under composition. Here we use additive notation
//! for the composition.
//!
//! Serialization is compatible with the chiavdf library (https://github.com/Chia-Network/chiavdf).

use crate::error::FastCryptoError::{InputTooLong, InvalidInput};
use crate::error::{FastCryptoError, FastCryptoResult};
use crate::groups::{ParameterizedGroupElement, UnknownOrderGroupElement};
use class_group::{pari_init, BinaryQF};
use curv::arithmetic::{BasicOps, BitManipulation, Integer, Modulo, One, Roots, Zero};
use curv::BigInt;
use std::ops::{Add, Shl};

mod compressed;

/// The maximal size in bits we allow a discriminant to have.
pub const MAX_DISCRIMINANT_SIZE_IN_BITS: usize = 1024;

/// The size of a compressed quadratic form in bytes. We force all forms to have the same size,
/// namely 100 bytes.
pub const QUADRATIC_FORM_SIZE_IN_BYTES: usize = (MAX_DISCRIMINANT_SIZE_IN_BITS + 31) / 32 * 3 + 4;

/// A binary quadratic form, (a, b, c) for arbitrary integers a, b, and c.
///
/// The `partial_gcd_limit` variable is equal to `|discriminant|^{1/4}` and is used to speed up
/// the composition algorithm.
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct QuadraticForm {
    form: BinaryQF,
    partial_gcd_limit: BigInt,
}

impl Add<QuadraticForm> for QuadraticForm {
    type Output = QuadraticForm;

    fn add(self, rhs: QuadraticForm) -> Self::Output {
        QuadraticForm {
            form: self.form.compose(&rhs.form).reduce(),
            partial_gcd_limit: self.partial_gcd_limit,
        }
    }
}

impl QuadraticForm {
    /// Create a new quadratic form given only the a and b coefficients and the discriminant.
    pub fn from_a_b_discriminant(a: BigInt, b: BigInt, discriminant: &Discriminant) -> Self {
        let c = ((&b * &b) - &discriminant.0) / (BigInt::from(4) * &a);
        Self {
            form: BinaryQF { a, b, c },

            // This limit is used for the partial_xgcd algorithm
            partial_gcd_limit: discriminant.0.abs().sqrt().sqrt(),
        }
    }

    /// Return a generator (or, more precisely, an element with a presumed large order) in a class
    /// group with a given discriminant. We use the element `(2, 1, x)` where `x` is determined from
    /// the discriminant.
    pub fn generator(discriminant: &Discriminant) -> Self {
        Self::from_a_b_discriminant(BigInt::from(2), BigInt::one(), discriminant)
    }

    /// Compute the discriminant `b^2 - 4ac` for this quadratic form.
    pub fn discriminant(&self) -> Discriminant {
        Discriminant::try_from(self.form.discriminant())
            .expect("The discriminant is checked in the constructors")
    }
}

impl ParameterizedGroupElement for QuadraticForm {
    /// Type of the discriminant.
    type ParameterType = Discriminant;

    type ScalarType = BigInt;

    fn zero(discriminant: &Self::ParameterType) -> Self {
        Self::from_a_b_discriminant(BigInt::one(), BigInt::one(), discriminant)
    }

    fn double(&self) -> Self {
        // Slightly optimised version of algorithm 2 from Jacobson, Jr, Michael & Poorten, Alfred
        // (2002). "Computational aspects of NUCOMP", Lecture Notes in Computer Science.
        // (https://www.researchgate.net/publication/221451638_Computational_aspects_of_NUCOMP)
        // The paragraph numbers and variable names follow the paper.

        let BinaryQF { a: u, b: v, c: w } = &self.form;

        // 1.
        let xgcd = BigInt::extended_gcd(&u, &v);
        let g = xgcd.gcd;
        let y = xgcd.y;
        let (capital_by, capital_dy) = if g.is_one() {
            (u / &g, v / &g)
        } else {
            (u.clone(), v.clone())
        };

        // 2.
        let capital_bx = (w * &y).modulus(&capital_by);

        // 3. (partial xgcd)
        let mut bx = capital_bx;
        let mut by = capital_by.clone();

        let mut x = BigInt::one();
        let mut y = BigInt::zero();
        let mut z = 0u32;

        while &by.abs() > &self.partial_gcd_limit && !bx.is_zero() {
            let (q, mut t) = by.div_rem(&bx);
            by = bx;
            bx = t;
            t = &y - &q * &x;
            y = x;
            x = t;
            z += 1;
        }

        if z.is_odd() {
            by = -by;
            y = -y;
        }

        // 4. / 5.
        let mut u3 = by.pow(2);
        let mut w3 = bx.pow(2);
        let mut v3 = -(&bx * &by).shl(1);

        if z.is_zero() {
            // 4.
            let mut dx = (&bx * &capital_dy - w) / &capital_by;
            v3 += v;
            if !g.is_one() {
                dx *= &g;
            }
            w3 -= &dx;
        } else {
            // 5.
            let dx = (&bx * &capital_dy - w * &x) / &capital_by;
            let q1 = &dx * &y;
            let dy = (&q1 + &capital_dy) / &x;
            v3 += &g * (&dy + &q1);

            if !g.is_one() {
                x *= &g;
                y *= &g;
            }
            u3 -= &y * &dy;
            w3 -= &x * &dx;
        }

        Self {
            form: BinaryQF {
                a: u3,
                b: v3,
                c: w3,
            }
            .reduce(),
            partial_gcd_limit: self.partial_gcd_limit.clone(),
        }
    }

    fn mul(&self, scale: &BigInt) -> Self {
        Self {
            form: self.form.exp(scale),
            partial_gcd_limit: self.partial_gcd_limit.clone(),
        }
    }

    fn as_bytes(&self) -> Vec<u8> {
        self.serialize().to_vec()
    }

    fn get_group_parameter(&self) -> Self::ParameterType {
        self.discriminant()
    }
}

impl UnknownOrderGroupElement for QuadraticForm {}

/// A discriminant for an imaginary class group. The discriminant is a negative integer which is
/// equal to 1 mod 4.
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Discriminant(BigInt);

impl TryFrom<BigInt> for Discriminant {
    type Error = FastCryptoError;

    fn try_from(value: BigInt) -> FastCryptoResult<Self> {
        if value >= BigInt::zero() || value.modulus(&BigInt::from(4)) != BigInt::from(1) {
            return Err(InvalidInput);
        }

        if value.bit_length() > MAX_DISCRIMINANT_SIZE_IN_BITS {
            return Err(InputTooLong(value.bit_length()));
        }

        Ok(Self(value))
    }
}

#[test]
fn test_double() {
    let d = Discriminant::try_from(BigInt::from(-1255)).unwrap();

    unsafe {
        pari_init(1000000, 0);
    }

    let iterations = 1000;

    let mut x = QuadraticForm::generator(&d);
    let mut y = QuadraticForm::generator(&d).form;
    for _ in 0..iterations {
        x = x.double();
        y = y.exp(&BigInt::from(2));
        assert_eq!(x.form, y);
    }
}

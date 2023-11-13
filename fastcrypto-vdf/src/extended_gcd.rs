// Copyright (c) 2022, Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

//! This module contains an implementation of the extended Euclidean algorithm for [BigInt]`s.
//! Besides the gcd and the Bezout coefficients, it also returns the quotients of the two inputs
//! divided by the GCD since these are often used, for example in the NUCOMP and NUDPL algorithms,
//! and come out for free while computing the Bezout coefficients.

use std::cmp::min;
use num_bigint::{BigInt, BigUint, Sign};
use num_integer::Integer;
use num_traits::{One, Signed, Zero};
use std::mem;
use std::ops::{Mul, Neg, Shr, Sub};
use std::str::FromStr;
use num_modular::ModularUnaryOps;

/// The output of the extended Euclidean algorithm on inputs `a` and `b`: The Bezout coefficients `x`
/// and `y` such that `ax + by = gcd`. The quotients `a / gcd` and `b / gcd` are also returned.
pub struct EuclideanAlgorithmOutput {
    pub gcd: BigInt,
    pub x: BigInt,
    pub y: BigInt,
    pub a_divided_by_gcd: BigInt,
    pub b_divided_by_gcd: BigInt,
}

impl EuclideanAlgorithmOutput {
    fn flip(self) -> Self {
        Self {
            gcd: self.gcd,
            x: self.y,
            y: self.x,
            a_divided_by_gcd: self.b_divided_by_gcd,
            b_divided_by_gcd: self.a_divided_by_gcd,
        }
    }
}

/// Compute the greatest common divisor gcd of a and b. The output also returns the Bezout coefficients
/// x and y such that ax + by = gcd and also the quotients a / gcd and b / gcd.
pub fn extended_euclidean_algorithm(a: &BigInt, b: &BigInt) -> EuclideanAlgorithmOutput {
    if b < a {
        return extended_euclidean_algorithm(b, a).flip();
    }

    let mut s = (BigInt::zero(), BigInt::one());
    let mut t = (BigInt::one(), BigInt::zero());
    let mut r = (a.clone(), b.clone());

    while !r.0.is_zero() {
        let (q, r_prime) = r.1.div_rem(&r.0);
        r.1 = r.0;
        r.0 = r_prime;

        let f = |mut x: (BigInt, BigInt)| {
            mem::swap(&mut x.0, &mut x.1);
            x.0 -= &q * &x.1;
            x
        };
        s = f(s);
        t = f(t);
    }

    // The last coefficients are equal to +/- a / gcd(a,b) and b / gcd(a,b) respectively.
    let a_divided_by_gcd = if a.sign() != s.0.sign() {
        s.0.neg()
    } else {
        s.0
    };
    let b_divided_by_gcd = if b.sign() != t.0.sign() {
        t.0.neg()
    } else {
        t.0
    };

    if !r.1.is_negative() {
        EuclideanAlgorithmOutput {
            gcd: r.1,
            x: t.1,
            y: s.1,
            a_divided_by_gcd,
            b_divided_by_gcd,
        }
    } else {
        EuclideanAlgorithmOutput {
            gcd: r.1.neg(),
            x: t.1.neg(),
            y: s.1.neg(),
            a_divided_by_gcd,
            b_divided_by_gcd,
        }
    }
}


pub fn exact_div(a: &BigUint, b: &BigUint) -> BigUint {
    let divisor_trailing_zeros = b.to_u32_digits()[0].trailing_zeros();

    let mut a_digits = a.shr(divisor_trailing_zeros as usize).to_u32_digits();
    let b_digits = b.shr(divisor_trailing_zeros as usize).to_u32_digits();

    let result_length = a_digits.len() - b_digits.len() + 1;
    let length = min(b_digits.len(), result_length);

    let b_prime = (b_digits[0] as u64).invm(&(1 << 32)).unwrap() as u32;

    let mut q = Vec::new();

    for k in 0..result_length {
        q.push(a_digits[k].wrapping_mul(b_prime));
        if k + 1 < result_length {
            let j = min(length, result_length -k);
            let a_new_digits = BigUint::from_slice(&a_digits[k..result_length]).sub(BigUint::from_slice(&b_digits[0..j]).mul(q[k])).to_u32_digits();
            a_digits[k..k+a_new_digits.len()].copy_from_slice(&a_new_digits);
            a_digits[k+a_new_digits.len()..result_length].fill(0);
        }
    }
    BigUint::from_slice(&q)
}

#[test]
fn test_exact_div() {
    let a = BigUint::from_str("2868257319497634232961664256").unwrap();
    let b = BigUint::from_str("15239746984").unwrap();
    let c = BigUint::from_str("188208985523773984").unwrap();
    assert_eq!(c, exact_div(&a, &b));
}

#[test]
fn test_xgcd() {
    test_xgcd_single(BigInt::from(240), BigInt::from(46));
    test_xgcd_single(BigInt::from(-240), BigInt::from(46));
    test_xgcd_single(BigInt::from(240), BigInt::from(-46));
    test_xgcd_single(BigInt::from(-240), BigInt::from(-46));
}

#[cfg(test)]
fn test_xgcd_single(a: BigInt, b: BigInt) {
    let output = extended_euclidean_algorithm(&a, &b);
    assert_eq!(output.gcd, a.gcd(&b));
    assert_eq!(&output.x * &a + &output.y * &b, output.gcd);
    assert_eq!(output.a_divided_by_gcd, &a / &output.gcd);
    assert_eq!(output.b_divided_by_gcd, &b / &output.gcd);
}
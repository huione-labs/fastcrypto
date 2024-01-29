// Copyright (c) 2022, Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::class_group::Discriminant;
use crate::hash_prime::{DefaultPrimalityCheck, PrimalityCheck};
use crate::vdf::VDF;
use crate::{hash_prime, Parameter, ParameterizedGroupElement, ToBytes, UnknownOrderGroupElement};
use fastcrypto::error::FastCryptoError::{InvalidInput, InvalidProof};
use fastcrypto::error::FastCryptoResult;
use fastcrypto::groups::multiplier::windowed::WindowedScalarMultiplier;
use fastcrypto::groups::multiplier::ScalarMultiplier;
use num_bigint::{BigInt, ToBigInt};
use num_integer::Integer;
use std::marker::PhantomData;
use std::ops::Neg;

/// An implementation of Wesolowski's VDF construction (https://eprint.iacr.org/2018/623) over a
/// group of unknown order.
pub struct WesolowskisVDF<G: ParameterizedGroupElement + UnknownOrderGroupElement, F: FiatShamir<G>>
{
    group_parameter: G::ParameterType,
    iterations: u64,
    _fiat_shamir: PhantomData<F>,
}

impl<G: ParameterizedGroupElement + UnknownOrderGroupElement, F: FiatShamir<G>>
    WesolowskisVDF<G, F>
{
    /// Create a new VDF using the group defined by the given group parameter. Evaluating this VDF
    /// will require computing `2^iterations * input` which requires `iterations` group operations.
    pub fn new(group_parameter: G::ParameterType, iterations: u64) -> Self {
        Self {
            group_parameter,
            iterations,
            _fiat_shamir: PhantomData::<F>,
        }
    }
}

impl<
        G: ParameterizedGroupElement<ScalarType = BigInt> + UnknownOrderGroupElement,
        F: FiatShamir<G>,
    > VDF for WesolowskisVDF<G, F>
{
    type InputType = G;
    type OutputType = G;
    type ProofType = G;

    fn evaluate(&self, input: &G) -> FastCryptoResult<(G, G)> {
        if self.iterations == 0 {
            return Ok((input.clone(), G::zero(&self.group_parameter)));
        }

        // Compute output = 2^iterations * input
        let mut output = input.clone();
        for _ in 0..self.iterations {
            output = output.double();
        }

        let challenge = F::compute_challenge(self, input, &output);

        // Algorithm from page 3 on https://crypto.stanford.edu/~dabo/pubs/papers/VDFsurvey.pdf
        let two = BigInt::from(2);
        let mut quotient_remainder = two.div_mod_floor(&challenge);
        let mut proof = input.mul(&quotient_remainder.0);
        for _ in 1..self.iterations {
            quotient_remainder = (&quotient_remainder.1 * &two).div_mod_floor(&challenge);
            proof = proof.double() + &input.mul(&quotient_remainder.0);
        }

        Ok((output, proof))
    }

    fn verify(&self, input: &G, output: &G, proof: &G) -> FastCryptoResult<()> {
        if !input.same_group(output) || !input.same_group(proof) {
            return Err(InvalidInput);
        }

        let challenge = F::compute_challenge(self, input, output);
        let f1 = proof.mul(&challenge);

        let r = BigInt::modpow(&BigInt::from(2), &BigInt::from(self.iterations), &challenge);
        let f2 = input.mul(&r);

        if f1 + &f2 != *output {
            return Err(InvalidProof);
        }
        Ok(())
    }
}

/// A faster method of verification which uses fast multi-scalar multiplication. The scalar size
/// for the scalar multiplier `M`  must be larger enough to hold the challenge in the Fiat-Shamir
/// construction `F`.
pub struct FastVerifier<
    G: ParameterizedGroupElement + UnknownOrderGroupElement,
    F: FiatShamir<G>,
    M: ScalarMultiplier<G, G::ScalarType>,
> {
    vdf: WesolowskisVDF<G, F>,
    input: G,
    multiplier: M,
}

impl<
        G: ParameterizedGroupElement<ScalarType = BigInt> + UnknownOrderGroupElement,
        F: FiatShamir<G>,
        M: ScalarMultiplier<G, BigInt>,
    > FastVerifier<G, F, M>
{
    /// Create a new FastVerifier for the given VDF instance.
    pub fn new(vdf: WesolowskisVDF<G, F>, input: G) -> Self {
        let multiplier = M::new(input.clone(), G::zero(&vdf.group_parameter));
        Self {
            vdf,
            input,
            multiplier,
        }
    }

    /// Verify the output and proof from a VDF using the input given in [new].
    pub fn verify(&self, output: &G, proof: &G) -> FastCryptoResult<()> {
        if !self.input.same_group(output) || !self.input.same_group(proof) {
            return Err(InvalidInput);
        }

        let challenge = F::compute_challenge(&self.vdf, &self.input, output);
        let r = BigInt::modpow(
            &BigInt::from(2),
            &BigInt::from(self.vdf.iterations),
            &challenge,
        );
        let actual = self.multiplier.two_scalar_mul(&r, proof, &challenge);
        if actual != *output {
            return Err(InvalidProof);
        }
        Ok(())
    }
}

/// Implementation of Wesolowski's VDF construction over a group of unknown order using a strong
/// Fiat-Shamir implementation.
pub type StrongVDF<G> =
    WesolowskisVDF<G, StrongFiatShamir<G, CHALLENGE_SIZE, DefaultPrimalityCheck>>;

pub type StrongVDFVerifier<G> = FastVerifier<
    G,
    StrongFiatShamir<G, CHALLENGE_SIZE, DefaultPrimalityCheck>,
    WindowedScalarMultiplier<G, BigInt, 256, 5>,
>;

pub trait FiatShamir<G: ParameterizedGroupElement + UnknownOrderGroupElement>: Sized {
    /// Compute the prime modulus used in proving and verification. This is a Fiat-Shamir construction
    /// to make the Wesolowski VDF non-interactive.
    fn compute_challenge(vdf: &WesolowskisVDF<G, Self>, input: &G, output: &G) -> G::ScalarType;
}

/// Default size in bytes of the Fiat-Shamir challenge used in proving and verification (same as chiavdf).
pub const CHALLENGE_SIZE: usize = 33;

/// Implementation of the Fiat-Shamir challenge generation for usage with Wesolowski's VDF construction.
/// The implementation is strong, meaning that all public parameters are used in the challenge generation.
/// See https://eprint.iacr.org/2023/691.
pub struct StrongFiatShamir<G, const CHALLENGE_SIZE: usize, P> {
    _group: PhantomData<G>,
    _primality_check: PhantomData<P>,
}

impl<
        G: ParameterizedGroupElement<ScalarType = BigInt> + UnknownOrderGroupElement,
        const CHALLENGE_SIZE: usize,
        P: PrimalityCheck,
    > FiatShamir<G> for StrongFiatShamir<G, CHALLENGE_SIZE, P>
{
    fn compute_challenge(vdf: &WesolowskisVDF<G, Self>, input: &G, output: &G) -> BigInt {
        let mut seed = vec![];

        let input_bytes = input.to_bytes();
        seed.extend_from_slice(&(input_bytes.len() as u64).to_be_bytes());
        seed.extend_from_slice(&input_bytes);

        let output_bytes = output.to_bytes();
        seed.extend_from_slice(&(output_bytes.len() as u64).to_be_bytes());
        seed.extend_from_slice(&output_bytes);

        seed.extend_from_slice(&(vdf.iterations).to_be_bytes());
        seed.extend_from_slice(&vdf.group_parameter.to_bytes());

        hash_prime::hash_prime::<P>(&seed, CHALLENGE_SIZE, &[0, 8 * CHALLENGE_SIZE - 1]).into()
    }
}

impl Parameter for Discriminant {
    /// Compute a valid discriminant (aka a negative prime equal to 3 mod 4) based on the given seed.
    /// The size_in_bits must be divisible by 8.
    fn from_seed(seed: &[u8], size_in_bits: usize) -> FastCryptoResult<Discriminant> {
        if size_in_bits % 8 != 0 {
            return Err(InvalidInput);
        }
        // Set the lower three bits to ensure that the prime is 7 mod 8 which makes the discriminant 1 mod 8.
        Self::try_from(
            hash_prime::hash_prime_default(seed, size_in_bits / 8, &[0, 1, 2, size_in_bits - 1])
                .to_bigint()
                .expect("Never fails")
                .neg(),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::class_group::{Discriminant, QuadraticForm};
    use crate::vdf::wesolowski::{StrongVDF, StrongVDFVerifier};
    use crate::vdf::VDF;
    use crate::{Parameter, ParameterizedGroupElement, ToBytes};
    use num_bigint::BigInt;

    #[test]
    fn test_prove_and_verify() {
        let challenge = hex::decode("99c9e5e3a4449a4b4e15").unwrap();
        let iterations = 1000u64;
        let discriminant = Discriminant::from_seed(&challenge, 512).unwrap();

        let input = QuadraticForm::generator(&discriminant);

        let vdf = StrongVDF::<QuadraticForm>::new(discriminant, iterations);
        let (output, proof) = vdf.evaluate(&input).unwrap();

        // Regression tests
        assert_eq!(output.to_bytes(), hex::decode("002024ff799e22587cc344b83ace4ce2d814d52aba7dd711591fd407e31a55822a1e0020051fa84e09b4bd5a6ee02b6fae24db625863ae31cfaf962f2f490d92dd372e3f").unwrap());
        assert_eq!(proof.to_bytes(), hex::decode("00201c51f96394af8f5aa29a6f954b08e892c12a7382d5149dbf77b872320c0babc40020eb95792dcc7b7f1af1b3c009860ebd1af0a8e4f0952489bb9b36af19d5b36b25").unwrap());

        assert!(vdf.verify(&input, &output, &proof).is_ok());

        // A modified output or proof fails to verify
        let modified_output = output.mul(&BigInt::from(2));
        let modified_proof = proof.mul(&BigInt::from(2));
        assert!(vdf.verify(&input, &modified_output, &proof).is_err());
        assert!(vdf.verify(&input, &output, &modified_proof).is_err());

        let fast_verifier = StrongVDFVerifier::new(vdf, input);
        assert!(fast_verifier.verify(&output, &proof).is_ok());
    }
}

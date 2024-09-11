// Copyright (c) 2022, Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::math::parameterized_group::ParameterizedGroupElement;
use fastcrypto::groups::Doubling;
use fastcrypto::hash::{HashFunction, Keccak256};
use modulus::RSAModulus;
use num_bigint::BigUint;
use num_traits::One;
use serde::Serialize;
use std::ops::{Add, Mul};

/// When generating a random element, we sample uniformly 8 bytes larger than the modulus to limit the bias by 2^{-64}.
const BIAS_BYTES: usize = 16;

pub mod modulus;

/// This represents an element of the subgroup of an RSA group <i>Z<sub>N</sub><sup>*</sup> / <±1></i>
/// where <i>N</i> is the product of two large primes. See also [RSAModulus].
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct RSAGroupElement<'a> {
    value: BigUint,

    // We assume that the modulus is known from the context, so it is not serialized.
    #[serde(skip)]
    modulus: &'a RSAModulus,
}

impl<'a> RSAGroupElement<'a> {
    /// Create a new RSA group element with the given value and modulus. The value will be reduced to
    /// the subgroup <i>Z<sub>N</sub><sup>*</sup> / <±1></i>, so it does not need to be in canonical
    /// representation.
    pub fn new(value: BigUint, modulus: &'a RSAModulus) -> Self {
        Self {
            value: modulus.reduce(value),
            modulus,
        }
    }

    /// Return the canonical representation of this group element.
    pub fn value(&self) -> &BigUint {
        &self.value
    }

    /// Generate a uniformly random element of the subgroup <i>Z<sub>N</sub><sup>*</sup> / <±1></i>
    /// using the given seed.
    pub fn from_seed(seed: &[u8], modulus: &'a RSAModulus) -> Self {
        // The number of 32-byte chunks needed to sample enough bytes.
        let k = (modulus.value.bits().div_ceil(8) as usize + BIAS_BYTES).div_ceil(32);

        let modulus_bytes = modulus.value.to_bytes_be();

        // H(i || k || seed length || seed || N) for i = 0, 1, ..., k-1
        let bytes: Vec<u8> = (0..k)
            .flat_map(|i| {
                let mut hash = Keccak256::new();
                hash.update((i as u64).to_be_bytes());
                hash.update((k as u64).to_be_bytes());
                hash.update((seed.len() as u64).to_be_bytes());
                hash.update(seed);
                hash.update(&modulus_bytes);
                hash.finalize().digest
            })
            .collect();

        Self::new(BigUint::from_bytes_be(&bytes), modulus)
    }
}

impl Add<&Self> for RSAGroupElement<'_> {
    type Output = Self;

    fn add(self, rhs: &Self) -> Self::Output {
        assert_eq!(self.modulus, rhs.modulus);
        Self::new(self.value.mul(&rhs.value), self.modulus)
    }
}

impl Doubling for RSAGroupElement<'_> {
    fn double(self) -> Self {
        Self::new(self.value.pow(2), self.modulus)
    }
}

impl<'a> ParameterizedGroupElement for RSAGroupElement<'a> {
    type ParameterType = &'a RSAModulus;

    fn zero(parameter: &Self::ParameterType) -> Self {
        Self::new(BigUint::one(), parameter)
    }

    fn is_in_group(&self, parameter: &Self::ParameterType) -> bool {
        &self.modulus == parameter
    }
}

#[cfg(test)]
mod tests {
    use crate::math::parameterized_group::ParameterizedGroupElement;
    use crate::rsa_group::modulus::test::{AMAZON_2048, GOOGLE_4096};
    use crate::rsa_group::RSAGroupElement;
    use fastcrypto::groups::Doubling;
    use num_bigint::BigUint;
    use num_integer::Integer;
    use num_traits::One;
    use std::ops::{Add, Shr};
    use std::str::FromStr;

    #[test]
    fn test_group_ops() {
        // Add
        let zero = RSAGroupElement::zero(&GOOGLE_4096);
        let element = RSAGroupElement::new(BigUint::from(7u32), &GOOGLE_4096);
        let sum = element.clone().add(&zero);
        assert_eq!(&sum, &element);
        assert_eq!(sum, RSAGroupElement::new(BigUint::from(7u32), &GOOGLE_4096));

        // Double
        let expected_double = element.clone().add(&element);
        let double = element.double();
        assert_eq!(&double, &expected_double);
        assert_eq!(
            double,
            RSAGroupElement::new(BigUint::from(49u32), &GOOGLE_4096)
        );

        // Double zero
        assert_eq!(
            RSAGroupElement::zero(&GOOGLE_4096),
            RSAGroupElement::zero(&GOOGLE_4096)
        );

        // +1 = -1 in this group
        let minus_one = RSAGroupElement::new(&GOOGLE_4096.value - BigUint::one(), &GOOGLE_4096);
        let one = RSAGroupElement::new(BigUint::one(), &GOOGLE_4096);
        assert_eq!(minus_one, one);

        // Regression tests
        let random_element = RSAGroupElement::from_seed(b"seed", &GOOGLE_4096);
        assert_eq!(random_element.value, BigUint::from_str("3011487423899721558172586084593447091546794013772421891852686427379649360349591071042155080156709732094635259491728460643062755825443155353414490494738520634212220791183517964816462175953769339567912300427657269717804517280026721229247024289650589508125273320556233316398729682032717996724740009957548686134585740491444482294825407803137069328834705130772315201737659496106383388841243339470405453817571072927274678865019568085654561040035395556031634741651466601835568367540457072862889935753450783992957932029406097704966050543728306105510608629319289759212092934510621246630280136765405088366533746881002911352341041408751035351219406231412464122955173404589925134845031385474896158529793181597536218760633377110755349034028975331836355550604701350426883308966678063342333231216600565874489392922238468078753334993619278651043261980609469172464084240398891624715949792552877770376442514136033484393552993041791182645762139356454749145157477579658597531522208110381769532235858561994918362075064755742956146945173550690952383040506740953044108594935188687493707479875137162045643640551489646841572978057306088607708573462503775952755210105003724436312811606456647274176176519694988552870377464121036699254826555278512845709738406").unwrap());
        let other_random_element = RSAGroupElement::from_seed(b"other seed", &GOOGLE_4096);
        assert_eq!(other_random_element.value, BigUint::from_str("176121375248245509201216527732377427229136570771869944827312545891467986648573780763466747120668600804396006238950097064509440649433743917432980460058679360116926233695430711082478267183214046569073661836817333067858456535777610984611492461217999047265148604169428494890142779007826619098199701557745770386896983806177332928564360387375344555482501202749085602478657814214323624182588015379373742512851995141970869901472921268073683833177697639676066775988865454083180302410405669511398402006114340303244305682782537080028919945597430862100948559096716723002702661379146122031557104458939159254713319103494377648572912261268572499798815410790168657396290041907923909416566227346864616699388396728378067647474754176716012660453761764650828034173016330702918192582359342195589117921293514903439929772202817389007539099754046855073187394118678243016580051079974012864428873748318390279505889775297064585339443113365434878025589026853638814957173261259991375386128305325987050657187508082417393858885165955194343616677676992967613710780091010521396784806395968703044627444622907792110421169414483584876950159529200109066174752565084340631717866329914310759734576567308363512375160593246827334246857520635333893713429358249283983488844344").unwrap());
        let product = random_element + &other_random_element;
        assert_eq!(product.value, BigUint::from_str("315909778659335171677596905320398989699828926498141927102472125295597812288422036730881143107058442115128294053839163669241992727322310466266009971608544723191903057142044024705546882443576652427611380933060348222734851906607130754320937381272081767250332044485545658436960691948804726651552219160656321887533090963207215535304557823370484206685542667636197767607570237723310788056182791907739748657056705930514003813656663353584160059601597271744690113172639381527229424454404892614818974712992078976624581678920706266407851556101240791787801578283302927043366972963924590141540813393793008484650154777325186120776498218467665183302594405004894383111763959727679536537029806658494699371188063492444532626197288526076898077943837008084536936355603813098292176978893329712887803758243601422270259145579870511945890249197010654808416937555738056796892661091756241285716218476472542458954987156338985458283459473748450349814443352897569544224292578527798545901256582542979593409557275957198868793366533066540428870194239603695458600870382644681749528947865754623968466452374008564628695011810188818476304412560487663260052635746675909459021015090097309429212502408686584606945734770457537868641334951344336328921610281004245622140614187").unwrap());

        // Modular reduction of new elements
        let large_number = BigUint::from_str("3036574726774676854074014207033343430612266081507239284282798782317571042227965037278950934156694735611210695484310361268558404608267193592214127172354047065735403268979111700392033047010804126832086391966439076431330350374247419917618297006262856696155226628618293034757242356020689237892332233440871475509280609912569923161406282048700185976978134392521001361425887597945020503850821184990626505486306603461873986703869036511634866702369526250399148024834067982831047337042153803607868763371956217428973526669465977516790908505225943570334171057954932061023400781214763048807711360233971697637040843540051164810073408390129527377426443433651590472389267133466042148959242653139526086412946070786720140657635835875713323699908265653879574906967349779294842580887092681914797921763005562279351709683336901191523161823487780857916850791672416252526865341826684661942406749371858482369032127708690896727803767069390093288584032378684194122410651316035020185762056183476958400919610941041023184947635820644508736415622905641385921660301201726207780865694936025072564059000891388970164247300478721858382388009195099408958892192194896015031243023477528739867472304200234331201318403377478375342123264777993278351523205958747379610474196811231241234").unwrap();
        let large_number_reduced = large_number.mod_floor(&GOOGLE_4096.value);
        let large_number_as_group_element =
            RSAGroupElement::new(large_number.clone(), &GOOGLE_4096);
        assert!(large_number_reduced >= (&GOOGLE_4096.value).shr(1));
        assert_eq!(
            large_number_as_group_element.value,
            &GOOGLE_4096.value - &large_number_reduced
        );
        assert_eq!(
            large_number_as_group_element,
            RSAGroupElement::new(large_number_reduced.clone(), &GOOGLE_4096)
        );
    }

    #[test]
    fn test_is_in_group() {
        let element = RSAGroupElement::new(BigUint::from(7u32), &GOOGLE_4096);
        assert!(element.is_in_group(&GOOGLE_4096));
        assert!(!element.is_in_group(&AMAZON_2048));
    }
}

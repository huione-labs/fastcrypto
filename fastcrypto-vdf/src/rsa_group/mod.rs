// Copyright (c) 2022, Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::math::parameterized_group::ParameterizedGroupElement;
use fastcrypto::groups::Doubling;
use modulus::RSAModulus;
use num_bigint::{BigUint, RandBigInt};
use num_traits::One;
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;
use serde::Serialize;
use std::ops::{Add, Mul};

pub mod modulus;

/// This represents an element of the subgroup of an RSA group <i>Z<sub>N</sub><sup>*</sup> / <±1></i>
/// where <i>N</i> is the product of two large primes. See also [RSAModulus].
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct RSAGroupElement<'a> {
    value: BigUint,

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

    /// Generate a random element of the subgroup <i>Z<sub>N</sub><sup>*</sup> / <±1></i>
    /// using the given seed.
    pub fn from_seed(seed: [u8; 32], modulus: &'a RSAModulus) -> Self {
        let mut rng = ChaCha20Rng::from_seed(seed);
        Self::new(rng.gen_biguint_below(&modulus.half), modulus)
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
    use crate::rsa_group::modulus::test::{AMAZON_MODULUS_2048_REF, GOOGLE_MODULUS_4096_REF};
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
        let zero = RSAGroupElement::zero(&GOOGLE_MODULUS_4096_REF);
        let element = RSAGroupElement::new(BigUint::from(7u32), &GOOGLE_MODULUS_4096_REF);
        let sum = element.clone().add(&zero);
        assert_eq!(&sum, &element);
        assert_eq!(
            sum,
            RSAGroupElement::new(BigUint::from(7u32), &GOOGLE_MODULUS_4096_REF)
        );

        // Double
        let expected_double = element.clone().add(&element);
        let double = element.double();
        assert_eq!(&double, &expected_double);
        assert_eq!(
            double,
            RSAGroupElement::new(BigUint::from(49u32), &GOOGLE_MODULUS_4096_REF)
        );

        // Double zero
        assert_eq!(
            RSAGroupElement::zero(&GOOGLE_MODULUS_4096_REF),
            RSAGroupElement::zero(&GOOGLE_MODULUS_4096_REF)
        );

        // +1 = -1 in this group
        let minus_one = RSAGroupElement::new(
            &GOOGLE_MODULUS_4096_REF.value - BigUint::one(),
            &GOOGLE_MODULUS_4096_REF,
        );
        let one = RSAGroupElement::new(BigUint::one(), &GOOGLE_MODULUS_4096_REF);
        assert_eq!(minus_one, one);

        // Regression tests
        let mut seed = [0u8; 32];
        let random_element = RSAGroupElement::from_seed(seed, &GOOGLE_MODULUS_4096_REF);
        assert_eq!(random_element.value, BigUint::from_str("49724756698435813446349195001063835319895925061658917991478169593309925987262409021069664637361644875625785560933895658349525310896373193752482889552031591180831812323180484254063474414240719883130146852072591348644016351091085280354367883537848861873177489747632379280448167648061502002463288717119855792124181819986615509283748321736425774538119117455897985987975824602239956606045819704079343045965396172354233170743503890845377848150952828773734373757276557188912707419587553917160513565316219890049997275718791318005264204560825575828174168489797403177332409249963414060871784601427910608802424623850419024553369422889257026372580733651921464706501501128777429524301823230389898157865110158069090663342254735475065046323797352664286466486744385899147070494131947558844882586840414494645678738098156338951248175818472305628236987505246813717547293384151779827811240117312879414141448334827925577579659948064591471822218071976484059804027258123242821872507657751783031359759654725263522360736477908884415765554840189345965778272694718858536350946130789913652351175904311606541290039228215402060899697865920745965958414261326655424625599128650352223606839454183339240573518293541294452406392003667572654854290187274739259357169692").unwrap());
        seed[0] += 1;
        let other_random_element = RSAGroupElement::from_seed(seed, &GOOGLE_MODULUS_4096_REF);
        assert_eq!(other_random_element.value, BigUint::from_str("74356763275226168699654017087562295921941429390276656240718281421567058890760885578494133117027665964105763529829941338424300552213300370408324301062444781828493867599713468475196366451390842345488238023204898868110092360631971682703162387416840813209407244797623195188295241144648291345869617070388729965894885143884435260480289817086416603508888155990191498396098877657285921271355396613980529812028002516536797194456987133189368580334714820681285032334950025418552639214954613983835472772762035310682433310722262529674917621864084536907361512837788960646731100200738381329956370637573529391685613160398605623036210468452369158717990321477625146373332341353451244412897543902500301570544232464543919776931135977511649213700266716266424539186222462179741884003438867272423217013496445430634356694462442447986813638645395731742462874110092509271472418930701703381664287298901073428651714168413864698924914439626276176348300626306945247438482498060087283724715782269320365820093818378193906816007593467815336126788395061354682774054040256225184765889498467598472662015527670192131767146940112619815003639117569871315275427931336129965918050553900633128797063788509011202762938530070830627379043901498642591942658607357312240093680581").unwrap());
        let product = random_element + &other_random_element;
        assert_eq!(product.value, BigUint::from_str("330868761091144304340368138747124227710940585033948894239571289859775878468608354823177875899340972008029317517013588187117067553517650515706844752675890836982578352903641109645816850898035373305021031118694246404611518975396142627496035567122337459149458750077060196336590923082335484429468839362818090711990053648574944601039346561422852839321961885309267085904945243391652177353142846157539236829503924899948155261902038000733623183646184377773330262095747244591060942088318994373586864254077151517021350171981037244253901534491654065506386452741704443618432210764395645024210943175419828241871611128731688266392417813319113432657876122569265167183085071617493786807046847405961186983218178167434163830224151092947962637904628347433391172785178655598132528626172506652680511377652862799952495615898511467783468381720497694030115059702209701047418696292064799369424582260416368937509379916568216363782770537781680271207805490140676824877922595821907434797238918508481196944604424267391196898072501059744632981849427357649173447677373396973069891632029703209491987284474006767175758803041848602110752535100696578945857962122584763358749723029159748691460984788630173365178337685937863310631857298203347207392160803980012075491583915").unwrap());

        // Modular reduction of new elements
        let large_number = BigUint::from_str("3036574726774676854074014207033343430612266081507239284282798782317571042227965037278950934156694735611210695484310361268558404608267193592214127172354047065735403268979111700392033047010804126832086391966439076431330350374247419917618297006262856696155226628618293034757242356020689237892332233440871475509280609912569923161406282048700185976978134392521001361425887597945020503850821184990626505486306603461873986703869036511634866702369526250399148024834067982831047337042153803607868763371956217428973526669465977516790908505225943570334171057954932061023400781214763048807711360233971697637040843540051164810073408390129527377426443433651590472389267133466042148959242653139526086412946070786720140657635835875713323699908265653879574906967349779294842580887092681914797921763005562279351709683336901191523161823487780857916850791672416252526865341826684661942406749371858482369032127708690896727803767069390093288584032378684194122410651316035020185762056183476958400919610941041023184947635820644508736415622905641385921660301201726207780865694936025072564059000891388970164247300478721858382388009195099408958892192194896015031243023477528739867472304200234331201318403377478375342123264777993278351523205958747379610474196811231241234").unwrap();
        let large_number_reduced = large_number.mod_floor(&GOOGLE_MODULUS_4096_REF.value);
        let large_number_as_group_element =
            RSAGroupElement::new(large_number.clone(), &GOOGLE_MODULUS_4096_REF);
        assert!(large_number_reduced >= (&GOOGLE_MODULUS_4096_REF.value).shr(1));
        assert_eq!(
            large_number_as_group_element.value,
            &GOOGLE_MODULUS_4096_REF.value - &large_number_reduced
        );
        assert_eq!(
            large_number_as_group_element,
            RSAGroupElement::new(large_number_reduced.clone(), &GOOGLE_MODULUS_4096_REF)
        );
    }

    #[test]
    fn test_is_in_group() {
        let element = RSAGroupElement::new(BigUint::from(7u32), &GOOGLE_MODULUS_4096_REF);
        assert!(element.is_in_group(&GOOGLE_MODULUS_4096_REF));
        assert!(!element.is_in_group(&AMAZON_MODULUS_2048_REF));
    }
}

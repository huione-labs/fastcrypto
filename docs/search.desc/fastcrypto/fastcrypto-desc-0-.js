searchState.loadedDescShard("fastcrypto", 0, "This module contains implementations of various AES modes.\nThis module contains an implementation of the BLS …\nThis module contains an implementation of the Ed25519 …\nEncodings of binary data such as Base64 and Hex.\nCollection of errors to be used in fastcrypto.\nMacro for generating a new alias for BytesRepresentation …\nThis module contains a selection of cryptographic hash …\nImplementations of HMAC and HKDF.\nImplementation of a verifier following RSASSA-PKCS1-v1_5 …\nThis module contains an implementation of the ECDSA …\nThis module contains an implementation of the ECDSA …\nMacro for generating Serialize/Deserialize for a type that …\nAES128 in CBC-mode using ANSI X9.23 padding.\nAES128 in CBC-mode using ISO 10126 padding.\nAES128 in CBC-mode using PKCS #7 padding.\nAES128 in CTR-mode.\nAES128 in GCM-mode (authenticated) using the given nonce …\nAES192 in CTR-mode.\nAES256 in CBC-mode using ANSI X9.23 padding.\nAES256 in CBC-mode using ISO 10126 padding.\nAES256 in CBC-mode using PKCS #7 padding.\nAES256 in CTR-mode.\nAES256 in GCM-mode (authenticated) using the given nonce …\nAes in CBC mode\nAes in CTR mode\nAES in GCM mode (authenticated).\nA key of <code>N</code> bytes used with AES ciphers.\nTrait impl’d by symmetric ciphers for authenticated …\nTrait impl’d by symmetric ciphers.\nTrait impl’d by encryption keys in symmetric cryptography\nStruct wrapping an instance of a …\nAn <code>N</code> byte initialization vector used with AES ciphers.\nTrait impl’d by nonces and IV’s used in symmetric …\nDecrypt <code>ciphertext</code> using the given IV and return the …\nDecrypt <code>ciphertext</code> using the given IV and authentication …\nEncrypt <code>plaintext</code> using the given IV and return the result.\nEncrypt <code>plaintext</code> using the given IV and authentication …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nThe length of public keys when using the min_pk module and …\nThe length of public keys when using the min_sig module …\nThe key pair bytes length used by helper is the same as …\nThe length of a private key in bytes.\nModule minimizing the size of public keys. Module …\nModule minimizing the size of signatures. Module …\nAggregation of multiple BLS 12-381 signatures.\nBLS 12-381 key pair.\nBLS 12-381 private key.\nBLS 12-381 public key.\nBLS 12-381 signature.\nHash-to-curve domain separation tag.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nAggregation of multiple BLS 12-381 signatures.\nBLS 12-381 key pair.\nBLS 12-381 private key.\nBLS 12-381 public key.\nBLS 12-381 signature.\nHash-to-curve domain separation tag.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nThe key pair bytes length is the same as the private key …\nThe length of a private key in bytes.\nThe length of a public key in bytes.\nThe length of a signature in bytes.\nAggregation of multiple Ed25519 signatures.\nEd25519 key pair.\nEd25519 private key.\nEd25519 public key.\nEd25519 signature.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nBase64 encoding\nBech32 encoding\nTrait representing a general binary-to-string encoding.\nHex string encoding.\nDecode this encoding into bytes.\nDecodes the Bech32 string to bytes, validating the given …\nDecodes a hex string to bytes. Both upper and lower case …\nDecodes a hex string to bytes. Both upper and lower case …\nEncode bytes into a string.\nEncodes bytes into a Bech32 encoded string, with the given …\nHex encoding is without “0x” prefix. See …\nEncode bytes as a hex string with a “0x” prefix.\nGet a string representation of this Base64 encoding.\nGet a string representation of this Hex encoding with a “…\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nEncodes bytes as a Base64.\nEncodes bytes as a hex string.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nDecodes this Base64 encoding to bytes.\nDecodes this hex encoding to bytes.\nContains the error value\nCollection of errors to be used in fastcrypto.\nGeneral cryptographic error.\nGeneral opaque cryptographic error.\nMessage should be ignored\nInput length is wrong.\nInput is to long.\nInput is to short.\nInvalid value was given to the function\nInvalid message was given to the function\nInvalid proof was given to the function\nInvalid signature was given to the function\nNot enough inputs were given to the function, retry with …\nContains the success value\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nTrait for group elements that has a fast doubling …\nTrait for groups that have a reduction from a random …\nFaster deserialization in case the input is trusted …\nTrait impl’d by elements of an additive cyclic group.\nTrait for groups that have a standardized “hash_to_point…\nTrait for groups that support multi-scalar multiplication.\nTrait impl’d by scalars to be used with GroupElement.\nType of scalars used in the [Self::mul] multiplication …\nCompute 2 * Self = Self + Self.\nReturn an instance of the generator for this group.\nHashes the given message and maps the result to a group …\nMulti-pairing operation that computes the sum of pairings …\nThis module contains implementations of optimised scalar …\nImplementations of the ristretto255 group which is a group …\nImplementation of the Secp256r1 (aka P-256) curve. This is …\nReturn an instance of the identity element in this group.\nElements of the group G_1 in BLS 12-381.\nElements of the group G_2 in BLS 12-381.\nElements of the subgroup G_T of F_q^{12} in BLS 12-381. …\nThis represents a scalar modulo r = …\nSimilar to <code>reduce_mod_uniform_buffer</code>, returns a result of …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nTrait for scalar multiplication for a fixed group element, …\nCompute <code>self.base_element * scalar</code>.\nCreate a new scalar multiplier with the given base element.\nSerialize scalar into a byte vector in little-endian …\nCompute …\nThis scalar multiplier uses pre-computation with the …\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nThis method computes the linear combination of the given …\nRepresents a point in the Ristretto group for Curve25519.\nRepresents a scalar.\nReturn this point in compressed form.\nReturn this point in compressed form.\nReturns the argument unchanged.\nReturns the argument unchanged.\nConstruct a RistrettoScalar by reducing a 32-byte …\nConstruct a RistrettoScalar by reducing a 64-byte …\nConstruct a RistrettoPoint from the given data using an …\nThe order of the base point.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nConstruct a RistrettoPoint from the given data using a …\nDecode a ristretto point in compressed binary form.\nA point on the Secp256r1 curve in projective coordinates.\nA field element in the prime field of the same order as …\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nThe BLAKE2-256 hash function with 256 bit digests.\nRepresents a digest of <code>DIGEST_LEN</code> bytes.\n<code>EllipticCurveMultisetHash</code> (ECMH) is a homomorphic multiset …\nThis trait is implemented by all messages that can be …\nTrait implemented by hash functions providing a output of …\nThis wraps a digest::Digest as a HashFunction.\nThe KECCAK hash function with 256 bit digests.\nA Multiset Hash is a homomorphic hash function, which …\nThe length of this hash functions digests in bytes.\nThis trait allows using a HashFunctionWrapper where a …\nThe SHA-2 hash function with 256 bit digests.\nThe SHA-3 hash function with 256 bit digests.\nThe SHA-3 hash function with 512 bit digests.\nThe SHA-512 hash function with 512 bit digests.\nThe type of the digest when this is hashed.\nGenerate a digest of the current state of this hash …\nCompute the digest of the given data and consume the hash …\nCompute a single digest from all slices in the iterator in …\nRetrieve result and consume hash function.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nInsert an item into this hash function.\nInsert multiple items into this hash function.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCreate a new hash function of the given type\nCreate a new digest containing the given bytes\nRemove an element from this hash function.\nRemove multiple items from this hash function.\nThe size of this digest in bytes.\nCopy the digest into a new vector.\nAdd all the elements of another hash function into this …\nProcess the given data, and update the internal of the …\nType for input keying material in hkdf_sha3_256.\nType for key in hmac_sha3_256.\nHMAC-based Extract-and-Expand Key Derivation Function …\nKeyed-Hash Message Authentication Code (HMAC) using …\nStruct that represents a standard JWT header according to …\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nParse the header base64 string into a [struct JWTHeader].\nPrivate key/seed of any/fixed size.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nURL-safe Base64 encoding <em>without</em> padding.\nBase64 encoding trait.\nDecode a Base64 string into the provided destination …\nDecode a Base64 string in-place.\nDecode a Base64 string into a byte vector.\nEncode the input byte slice as Base64.\nEncode input byte slice into a <code>String</code> containing Base64.\nGet the length of Base64 produced by encoding the given …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nParse signature from binary representation according to …\nParse an <code>RSAPublicKey</code> from an ASN.1 DER (Distinguished …\nParse an <code>RSAPublicKey</code> from its components, eg. the modulus …\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nVerify a signed message. The verification uses SHA-256 for …\nVerify a signed message. The message, <code>hashed</code>, must be the …\nDefault hash function used for signing and verifying …\nThe key pair bytes length is the same as the private key …\nThe length of a private key in bytes.\nThe length of a public key in bytes.\nThe length of a signature in bytes.\nSecp256k1 public/private key pair.\nSecp256k1 private key.\nSecp256k1 public key.\nSecp256k1 ECDSA signature.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nThis module contains an implementation of the ECDSA …\nCreate a new recoverable signature over the given message. …\nCreate a new signature using the given hash function to …\nVerify the signature using the given hash function to hash …\nLength of a compact signature followed by one extra byte …\nSecp256k1 signature.\nAn ECDSA signature\nObtains a raw mutable pointer suitable for use with FFI …\nObtains a raw pointer suitable for use with FFI functions\nLike <code>cmp::Cmp</code> but faster and with no guarantees across …\nLike <code>cmp::Eq</code> but faster and with no guarantees across …\nReturns the argument unchanged.\nReturns the argument unchanged.\nConverts a 64-byte compact-encoded byte slice to a …\nConverts a DER-encoded byte slice to a signature\nConverts a “lax DER”-encoded byte slice to a …\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nNormalizes a signature to a “low S” form. In ECDSA, …\nRecover public key from signature using the given hash …\nSerializes the signature in compact format\nSerializes the signature in DER format\nConvert a non-recoverable signature into a recoverable …\nVerifies an ECDSA signature for <code>msg</code> using <code>pk</code> and the …\nDefault hash function used for signing and verifying …\nThe number of precomputed points used for scalar …\nThe key pair bytes length is the same as the private key …\nThe length of a private key in bytes.\nThe length of a public key in bytes.\nThe length of a signature in bytes.\nThe size of the sliding window used for scalar …\nSecp256r1 public/private key pair.\nSecp256r1 private key.\nSecp256r1 public key.\nSecp256r1 signature.\nThis module contains conversion function between scalars …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nThis module contains an implementation of the ECDSA …\nCreate a new signature using the given hash function to …\nVerify the signature using the given hash function to hash …\nSecp256r1 signature.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nRecover the public key used to create this signature. This …\nExternal types.\nSerialization of internal types.\nGiven a byte array of length <code>N * SIZE_IN_BYTES</code>, …\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nSerialize a vector of elements of type T into a byte array …\nThis service holds the node’s private key. It takes …\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nTrait impl’d by aggregated signatures in asymmetric …\nTrait impl’d by RNG’s accepted by fastcrypto.\nTrait impl’d by signatures in asymmetric cryptography.\nCryptographic material with an immediate conversion …\nTrait impl’d by a keys/secret seeds for generating a …\nTrait impl’d by cryptographic material that can be …\nTrait for objects that support an insecure default value …\nTrait impl’d by a public / private key pair in …\nTrait impl’d by recoverable signatures\nTrait impl’d by public / private keypairs that can …\nTrait impl’d by a key/keypair that can create signatures.\nTrait impl’d by private (secret) keys in asymmetric …\nTrait impl’d by concrete types that represent digital …\nTrait impl’d by public keys in asymmetric cryptography.\nCombine signatures into a single aggregated signature.\nBorrow a byte slice representing the serialized form of …\nVerify a batch of aggregate signatures, each consisting of …\nParse an object from its byte representation\nGenerate a new keypair using the given RNG.\nGenerate a new random instance using the given RNG.\nGet the private key.\nGet the public key.\nRecover the public key from this signature.\nRecover the public key from this signature. Assuming that …\nCreate a new signature over a message.\nSign as a recoverable signature.\nSign as a recoverable signature using the given hash …\nUse Self to verify that the provided signature for a given …\nVerify this aggregate signature assuming that all …\nVerify this aggregate signature where the signatures are …\nVerify a recoverable signature by recovering the public …\nVerify a recoverable signature by recovering the public …\nReturns the log base 2 of b. There is an exception: for …\nA keypair for a verifiable random function (VRF).\nRepresents a private key used to compute outputs for a …\nA proof that the output of a VRF was computed correctly.\nRepresents a public key of which is use to verify outputs …\nAn implementation of an Elliptic Curve VRF (ECVRF) using …\nGenerate a new keypair using the given RNG.\nCompute both hash and proof for the given input.\nGenerate a proof for the given input.\nCompute the output of the VRF with this proof.\nVerify the correctness of this proof.\nVerify the correctness of this proof and VRF output.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.")
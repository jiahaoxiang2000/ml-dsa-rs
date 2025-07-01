# ML-DSA Implementation Roadmap

This document outlines the implementation plan for ML-DSA-44 based on the RustCrypto reference implementation.

## Project Overview

We are implementing a pure Rust ML-DSA (Module-Lattice-Based Digital Signature Standard) library, starting with the ML-DSA-44 parameter set. The implementation follows FIPS 204 specification and is based on the structure of the RustCrypto ML-DSA implementation.

## Implementation Phases

### Phase 1: Foundation Layer (module_lattice)
**Priority: Critical** | **Dependencies: None**

#### 1.1 Basic Utilities (`src/module_lattice/util.rs`)
- [x] Implement `Truncate` trait for safe integer truncation
- [x] Implement `Flatten` trait for array manipulation  
- [x] Implement `Unflatten` trait for array splitting
- [x] Add comprehensive tests for utility functions

#### 1.2 Field Arithmetic (`src/module_lattice/algebra.rs`) 
- [x] Define `Field` trait for modular arithmetic
- [x] Implement basic field element operations (add, sub, mul, neg)
- [x] Implement modular reduction and inversion
- [x] Add field arithmetic tests

#### 1.3 Encoding Framework (`src/module_lattice/encode.rs`)
- [x] Define `ArraySize` trait using typenum
- [x] Implement `EncodingSize` trait for bit-level encoding
- [x] Implement `Encode` trait for polynomial encoding/decoding
- [x] Implement `VectorEncodingSize` for vector encoding
- [x] Add encoding/decoding tests

#### 1.4 Module Integration (`src/module_lattice/mod.rs`)
- [x] Organize module structure and exports
- [x] Add module-level documentation
- [x] Ensure all components work together

### Phase 2: Core Algebra (`src/algebra.rs`)
**Priority: Critical** | **Dependencies: Phase 1**

#### 2.1 Base Field Definition
- [ ] Define `BaseField` for ML-DSA modulus Q = 8380417
- [ ] Implement field operations using module_lattice foundation
- [ ] Create type aliases: `Int`, `Elem`, `Polynomial`, `Vector`

#### 2.2 Extended Algebra Operations
- [ ] Implement `BarrettReduce` trait for efficient modular reduction
- [ ] Implement `Decompose` trait for high/low bit decomposition
- [ ] Implement `AlgebraExt` trait with ML-DSA specific operations:
  - [ ] `mod_plus_minus` - centered modular reduction
  - [ ] `infinity_norm` - infinity norm calculation
  - [ ] `power2round` - power-of-2 rounding (Algorithm 35)
  - [ ] `high_bits`/`low_bits` - bit decomposition (Algorithms 37/38)

#### 2.3 NTT Types
- [ ] Define `NttPolynomial`, `NttVector`, `NttMatrix` types
- [ ] Implement basic arithmetic for NTT domain objects

### Phase 3: Cryptographic Primitives
**Priority: High** | **Dependencies: Phase 1**

#### 3.1 Hash Functions (`src/crypto.rs`)
- [ ] Implement `ShakeState` wrapper for SHAKE-128/256
- [ ] Implement absorb/squeeze interface for XOF functions
- [ ] Create type aliases `G` (SHAKE-128) and `H` (SHAKE-256)
- [ ] Add hash function tests with known test vectors

#### 3.2 Utility Types (`src/util.rs`)
- [ ] Define `B32` and `B64` byte array types
- [ ] Add utility functions as needed

### Phase 4: Number Theoretic Transform (`src/ntt.rs`)
**Priority: High** | **Dependencies: Phase 2**

#### 4.1 NTT Constants
- [ ] Generate precomputed `ZETA_POW_BITREV` table
- [ ] Implement bit-reversal utilities
- [ ] Verify constants match FIPS 204 Appendix B

#### 4.2 NTT Operations
- [ ] Implement `Ntt` trait (Algorithm 41 - forward NTT)
- [ ] Implement `NttInverse` trait (Algorithm 42 - inverse NTT)
- [ ] Implement NTT multiplication (Algorithm 45 - MultiplyNTT)
- [ ] Add comprehensive NTT tests

### Phase 5: Encoding/Decoding (`src/encode.rs`)
**Priority: High** | **Dependencies: Phase 2**

#### 5.1 Range Encoding
- [ ] Implement `RangeEncodingSize` trait
- [ ] Define range encoding type aliases
- [ ] Implement `BitPack` trait (Algorithm 17 - BitPack/BitUnPack)

#### 5.2 Polynomial Encoding
- [ ] Implement polynomial packing for different bit widths
- [ ] Implement vector packing
- [ ] Add encoding round-trip tests

### Phase 6: Sampling Functions (`src/sampling.rs`)
**Priority: High** | **Dependencies: Phase 3, 4**

#### 6.1 Basic Sampling
- [ ] Implement `bit_set` function (Algorithm 13 - BytesToBits)
- [ ] Implement `coeff_from_three_bytes` (Algorithm 14)
- [ ] Implement `coeff_from_half_byte` (Algorithm 15)

#### 6.2 Rejection Sampling
- [ ] Implement `sample_in_ball` (Algorithm 29)
- [ ] Implement `rej_ntt_poly` (Algorithm 30 - RejNTTPoly) 
- [ ] Implement `rej_bounded_poly` (Algorithm 31 - RejBoundedPoly)

#### 6.3 Expansion Functions
- [ ] Implement `expand_a` (Algorithm 32 - ExpandA)
- [ ] Implement `expand_s` (Algorithm 33 - ExpandS)
- [ ] Implement `expand_mask` (Algorithm 34 - ExpandMask)

### Phase 7: Hint Operations (`src/hint.rs`)
**Priority: Medium** | **Dependencies: Phase 2**

#### 7.1 Hint Logic
- [ ] Implement `make_hint` function
- [ ] Implement `use_hint` function
- [ ] Implement `Hint` struct for ML-DSA-44

#### 7.2 Hint Encoding
- [ ] Implement `bit_pack` for hint encoding
- [ ] Implement `bit_unpack` for hint decoding
- [ ] Add hint validation and tests

### Phase 8: Parameter Set Completion (`src/param.rs`)
**Priority: Medium** | **Dependencies: Phase 2, 5**

#### 8.1 Parameter Traits
- [ ] Implement `ParameterSet` trait for MlDsa44
- [ ] Implement `SigningKeyParams` trait
- [ ] Implement `VerifyingKeyParams` trait  
- [ ] Implement `SignatureParams` trait

#### 8.2 ML-DSA-44 Implementation
- [ ] Define MlDsa44 struct with correct type-level parameters
- [ ] Implement all required trait implementations
- [ ] Verify parameter constants match specification

### Phase 9: Core ML-DSA Logic (`src/lib.rs`)
**Priority: Critical** | **Dependencies: All previous phases**

#### 9.1 Key Types
- [ ] Implement `Signature<P>` struct
- [ ] Implement `SigningKey<P>` struct  
- [ ] Implement `VerifyingKey<P>` struct
- [ ] Implement `KeyPair<P>` struct

#### 9.2 Key Generation
- [ ] Implement `KeyGen` trait (Algorithm 1 - ML-DSA.KeyGen)
- [ ] Implement `key_gen_internal` (Algorithm 6 - ML-DSA.KeyGen_internal)
- [ ] Add key generation tests

#### 9.3 Signing
- [ ] Implement `sign_internal` (Algorithm 7 - ML-DSA.Sign_internal)
- [ ] Implement `sign_deterministic` (Algorithm 2 - deterministic variant)
- [ ] Implement `sign_randomized` (Algorithm 2 - randomized variant)
- [ ] Add signing tests

#### 9.4 Verification  
- [ ] Implement `verify_internal` (Algorithm 8 - ML-DSA.Verify_internal)
- [ ] Implement `verify_with_context` (Algorithm 3 - ML-DSA.Verify)
- [ ] Add verification tests

#### 9.5 Encoding/Decoding
- [ ] Implement signature encoding (Algorithm 26 - sigEncode)
- [ ] Implement signature decoding (Algorithm 27 - sigDecode)
- [ ] Implement key encoding (Algorithms 22/24 - pkEncode/skEncode)
- [ ] Implement key decoding (Algorithms 23/25 - pkDecode/skDecode)

### Phase 10: Trait Implementations
**Priority: Medium** | **Dependencies: Phase 9**

#### 10.1 Signature Crate Integration
- [ ] Implement `signature::Signer` trait
- [ ] Implement `signature::Verifier` trait
- [ ] Implement `signature::Keypair` trait
- [ ] Implement `signature::SignatureEncoding` trait

#### 10.2 Optional Features
- [ ] Implement PKCS#8 support (when `pkcs8` feature enabled)
- [ ] Implement `Zeroize` support (when `zeroize` feature enabled)
- [ ] Add proper feature gates

### Phase 11: Testing & Validation
**Priority: High** | **Dependencies: Phase 9**

#### 11.1 Known Answer Tests
- [ ] Add FIPS 204 test vectors for ML-DSA-44
- [ ] Implement comprehensive round-trip tests
- [ ] Add boundary condition tests

#### 11.2 Interoperability Tests
- [ ] Verify compatibility with reference implementation
- [ ] Test key/signature interoperability
- [ ] Add fuzzing tests

#### 11.3 Performance Testing
- [ ] Add benchmarks for key generation
- [ ] Add benchmarks for signing/verification
- [ ] Profile and optimize critical paths

### Phase 12: Documentation & Polish
**Priority: Medium** | **Dependencies: Phase 11**

#### 12.1 Documentation
- [ ] Complete API documentation
- [ ] Add usage examples
- [ ] Update README with features and usage

#### 12.2 Code Quality
- [ ] Run comprehensive linting (clippy)
- [ ] Ensure no unsafe code violations
- [ ] Add CI/CD pipeline

## Implementation Dependencies

```
Phase 1 (Foundation) → Phase 2 (Algebra), Phase 3 (Crypto)
Phase 2 → Phase 4 (NTT), Phase 5 (Encoding)  
Phase 3 → Phase 6 (Sampling)
Phase 4, Phase 5 → Phase 7 (Hints), Phase 8 (Parameters)
All previous phases → Phase 9 (Core Logic)
Phase 9 → Phase 10 (Traits), Phase 11 (Testing)
Phase 11 → Phase 12 (Documentation)
```

## Key Design Decisions

1. **Type-level Programming**: Use `typenum` for compile-time parameter verification
2. **No Unsafe Code**: Maintain `#![forbid(unsafe_code)]` except where absolutely necessary
3. **Modular Design**: Keep components loosely coupled for testability
4. **Performance**: Use precomputed tables and efficient algorithms where possible
5. **Security**: Follow constant-time principles where applicable

## Testing Strategy

- **Unit Tests**: Each component tested independently
- **Integration Tests**: End-to-end signing/verification tests
- **Known Answer Tests**: FIPS 204 test vectors
- **Property Tests**: Round-trip and boundary condition testing
- **Benchmarks**: Performance regression detection

## Success Criteria

- [ ] All ML-DSA-44 operations implemented correctly
- [ ] Passes all FIPS 204 test vectors
- [ ] Compatible with RustCrypto signature traits
- [ ] No unsafe code (except where unavoidable)
- [ ] Comprehensive test coverage (>90%)
- [ ] Performance comparable to reference implementation
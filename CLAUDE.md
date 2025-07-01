# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a pure Rust implementation of ML-DSA (Module-Lattice-Based Digital Signature Standard), formerly known as CRYSTALS-Dilithium, following FIPS 204 specification. The project structure is based on the RustCrypto ML-DSA implementation.

## Architecture

The codebase follows a modular structure with core ML-DSA components:

- `src/lib.rs` - Main library entry point with core types (`Signature`, `SigningKey`, `VerificationKey`)
- `src/param.rs` - Parameter sets for ML-DSA variants (44, 65, 87) with constants N=256, Q=8380417
- `src/algebra.rs` - Ring element operations for lattice-based cryptography
- `src/crypto.rs` - Cryptographic hash functions and primitives
- `src/encode.rs` - Polynomial encoding/decoding utilities
- `src/hint.rs` - Hint generation and verification operations
- `src/ntt.rs` - Number Theoretic Transform (NTT) operations
- `src/sampling.rs` - Random sampling utilities for ML-DSA
- `src/util.rs` - Common utility functions

The library uses const generics for signature sizes and implements the `signature` crate traits for compatibility.

## Key Dependencies

- `signature` - Core signature traits and error handling
- `sha3` - SHA-3 cryptographic hash functions
- `hybrid-array` - Fixed-size arrays with extra sizes
- `num-traits` - Numeric traits for generic programming

## Optional Features

- `rand_core` - Random number generation (default)
- `pkcs8` - PKCS#8 key encoding support (default)
- `zeroize` - Secure memory clearing
- `alloc` - Heap allocation support (default)

## Development Commands

### Building and Checking
```bash
cargo check        # Quick syntax and type checking
cargo build        # Build the library
cargo build --release  # Release build
```

### Testing
```bash
cargo test         # Run all tests (currently none implemented)
cargo test --doc   # Run documentation tests
```

### Benchmarking
```bash
cargo bench        # Run performance benchmarks
```

### Feature Testing
```bash
cargo check --no-default-features  # Check minimal build
cargo check --all-features         # Check with all features
```

## Code Quality

The project enforces:
- `#![forbid(unsafe_code)]` - No unsafe code allowed
- `#![warn(missing_docs, rust_2018_idioms)]` - Documentation and style warnings
- Current warnings exist for missing documentation on Q constants in param.rs

## Reference Implementation

This project follows the structure of the official RustCrypto ML-DSA implementation at https://github.com/RustCrypto/signatures/tree/master/ml-dsa

## Development Memories

- To memorize use the gh to read the github repo context
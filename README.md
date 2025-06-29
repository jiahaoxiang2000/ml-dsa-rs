# ML-DSA

Pure Rust implementation of the Module-Lattice-Based Digital Signature Standard
(ML-DSA) as described in [FIPS 204] (final).

## About

ML-DSA was formerly known as [CRYSTALS-Dilithium].

This project is based on the structure and design of the [RustCrypto ML-DSA implementation].

## Features

- Pure Rust implementation
- No unsafe code
- Support for all ML-DSA parameter sets (44, 65, 87)
- PKCS#8 support (optional)
- Zeroization support (optional)

## Running Benchmarks

To run the benchmarks:

```bash
cargo bench
```

## Reference Implementation

This implementation follows the structure and design patterns from the official [RustCrypto ML-DSA implementation].

## License

Licensed under either of

* [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
* [MIT license](http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

[FIPS 204]: https://csrc.nist.gov/pubs/fips/204/final
[CRYSTALS-Dilithium]: https://pq-crystals.org/dilithium/
[RustCrypto ML-DSA implementation]: https://github.com/RustCrypto/signatures/tree/master/ml-dsa
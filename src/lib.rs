//! Pure Rust implementation of ML-DSA (formerly known as CRYSTALS-Dilithium) as
//! described in FIPS-204 (final)

#![forbid(unsafe_code)]
#![warn(missing_docs, rust_2018_idioms)]

pub mod algebra;
pub mod crypto;
pub mod encode;
pub mod hint;
pub mod ntt;
pub mod param;
pub mod sampling;
pub mod util;

use signature::{Error, SignatureEncoding};

/// ML-DSA signature
#[derive(Clone)]
pub struct Signature<const N: usize>([u8; N]);

/// ML-DSA signing key
pub struct SigningKey<const N: usize>([u8; N]);

/// ML-DSA verification key
pub struct VerificationKey<const N: usize>([u8; N]);

impl<const N: usize> SignatureEncoding for Signature<N> {
    type Repr = [u8; N];
}

impl<const N: usize> AsRef<[u8]> for Signature<N> {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl<const N: usize> From<[u8; N]> for Signature<N> {
    fn from(bytes: [u8; N]) -> Self {
        Self(bytes)
    }
}

impl<const N: usize> From<Signature<N>> for [u8; N] {
    fn from(sig: Signature<N>) -> [u8; N] {
        sig.0
    }
}

impl<const N: usize> TryFrom<&[u8]> for Signature<N> {
    type Error = Error;

    fn try_from(bytes: &[u8]) -> Result<Self, Error> {
        if bytes.len() == N {
            let mut array = [0u8; N];
            array.copy_from_slice(bytes);
            Ok(Self(array))
        } else {
            Err(Error::new())
        }
    }
}

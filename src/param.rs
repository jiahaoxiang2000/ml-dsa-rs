//! Parameter sets for ML-DSA

/// ML-DSA parameter set 44 (Security Category 2)
pub mod ml_dsa_44 {
    /// Ring dimension (256 for all ML-DSA variants)
    pub const N: usize = 256;
    /// Prime modulus Q = 2^23 - 2^13 + 1 = 8380417
    pub const Q: u32 = 8380417;
    /// Number of rows in the A matrix
    pub const K: usize = 4;
    /// Number of columns in the A matrix  
    pub const L: usize = 4;
    /// Private key range parameter
    pub const ETA: u32 = 2;
    /// Error size bound for y (Gamma1 = 2^17)
    pub const GAMMA1: u32 = 131072; // 2^17
    /// Low-order rounding range (Gamma2 = (Q-1)/88)
    pub const GAMMA2: u32 = 95232; // (8380417-1)/88
    /// Collision strength parameter (lambda/4 in bytes)
    pub const LAMBDA: usize = 32;
    /// Max number of true values in the hint
    pub const OMEGA: usize = 80;
    /// Number of nonzero values in the polynomial c
    pub const TAU: usize = 39;
    /// Derived parameter Beta = TAU * ETA
    pub const BETA: u32 = 78; // 39 * 2
}

/// ML-DSA parameter set 65 (Security Category 3)
pub mod ml_dsa_65 {
    /// Ring dimension (256 for all ML-DSA variants)
    pub const N: usize = 256;
    /// Prime modulus Q = 2^23 - 2^13 + 1 = 8380417
    pub const Q: u32 = 8380417;
    /// Number of rows in the A matrix
    pub const K: usize = 6;
    /// Number of columns in the A matrix
    pub const L: usize = 5;
    /// Private key range parameter
    pub const ETA: u32 = 4;
    /// Error size bound for y (Gamma1 = 2^19)
    pub const GAMMA1: u32 = 524288; // 2^19
    /// Low-order rounding range (Gamma2 = (Q-1)/32)
    pub const GAMMA2: u32 = 261888; // (8380417-1)/32
    /// Collision strength parameter (lambda/4 in bytes)
    pub const LAMBDA: usize = 48;
    /// Max number of true values in the hint
    pub const OMEGA: usize = 55;
    /// Number of nonzero values in the polynomial c
    pub const TAU: usize = 49;
    /// Derived parameter Beta = TAU * ETA
    pub const BETA: u32 = 196; // 49 * 4
}

/// ML-DSA parameter set 87 (Security Category 5)
pub mod ml_dsa_87 {
    /// Ring dimension (256 for all ML-DSA variants)
    pub const N: usize = 256;
    /// Prime modulus Q = 2^23 - 2^13 + 1 = 8380417
    pub const Q: u32 = 8380417;
    /// Number of rows in the A matrix
    pub const K: usize = 8;
    /// Number of columns in the A matrix
    pub const L: usize = 7;
    /// Private key range parameter
    pub const ETA: u32 = 2;
    /// Error size bound for y (Gamma1 = 2^19)
    pub const GAMMA1: u32 = 524288; // 2^19
    /// Low-order rounding range (Gamma2 = (Q-1)/32)
    pub const GAMMA2: u32 = 261888; // (8380417-1)/32
    /// Collision strength parameter (lambda/4 in bytes)
    pub const LAMBDA: usize = 64;
    /// Max number of true values in the hint
    pub const OMEGA: usize = 75;
    /// Number of nonzero values in the polynomial c
    pub const TAU: usize = 60;
    /// Derived parameter Beta = TAU * ETA
    pub const BETA: u32 = 120; // 60 * 2
}
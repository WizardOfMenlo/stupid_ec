pub use ff::{Field, PrimeField};

// One example field
#[derive(PrimeField)]
#[PrimeFieldModulus = "52435875175126190479447740508185965837690552500527637822603658699938581184513"]
#[PrimeFieldGenerator = "7"]
#[PrimeFieldReprEndianness = "little"]
struct Fp([u64; 4]);

// Extend a prime field

//! An implementation of a permeuted multiply with carry random number generator.
//! Details about the design see: https://tom-kaitchuck.medium.com/designing-a-new-prng-1c4ffd27124d
//!
//! This library provides two generators:
//!
//! * `Mwc256XXA64` : A Lag-3 64bit MWC generator with two xors and an addition applied to the output.
//!    It has an output size of 64bits, and a state size of 256bits. This algorithm is fastest on 64 bit architectures.
//! * `Mwc128XXA32` : A Lag-3 32bit MWC generator with two xors and an addition applied to the output.
//!    It has an output size of 32bits, and a state size of 128bits. This algorithm is fastest on 32 bit architectures.
//!
//!
//! # Usage
//!
//! ```toml
//! [dependencies]
//! pcg-mwc = "0.3.0"
//! ```
//! # Typename Nomenclature
//! The name describes the algorithm.
//!
//! 1. First Mwc stands for `Multiply with carry` this is the base generator type.
//! 1. This is followed by the state sise in bits.
//! 1. Third the output permutatuon which is used. Where `X` refers to 'xor' and `A` refers to addition.
//! 1. Fourth is the output size in bits
//!
//! # How to Use
//! The generators implement `rand_core` 0.10's `TryRng` (and so `Rng`) and `SeedableRng`, so they
//! work like the other Rng's from the `rand` crate.
//! You can create a prng as follows
//!
//! ```ignore
//! use rand::{RngExt, SeedableRng};
//! use pcg_mwc::Mwc256XXA64;
//!
//! fn main() {
//!     let mut rand = Mwc256XXA64::seed_from_u64(42);
//!
//!     let x : u32 = rand.random();
//! }
//! ```
//!
mod gen32;
mod gen64;

pub use gen32::Mwc128XXA32;
pub use gen64::Mwc256XXA64;


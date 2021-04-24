// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use core::fmt;
pub mod gen32;
mod aesprng;
pub use crate::aesprng::AesPrng;
use rand_core::{Error, le, RngCore, SeedableRng};

#[cfg(feature = "serde1")]
use serde::{Deserialize, Serialize};

// This is the default multiplier used by MWC.
const MULTIPLIER: u64 = 0xfeb3_4465_7c0a_f413; //Best spectra for lag 3
// For testing with a lag of 1, 3, or 4  the following work: 0x7c49_2513_927a_59b3 or 0xa729_8353_f425_0d13

/// A PCG random number generator (MWC X A 256/64 variant).
///
/// Permuted Congruential Generator with 256-bit state, internal multiply
/// with carry Generator, and 64-bit output via a xor and an add.
#[derive(Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
pub struct Mwc256XXA64 {
    x1: u64,
    x2: u64,
    x3: u64,
    c: u64,
}

impl Mwc256XXA64 {
    /// Construct an instance given two keys.
    pub fn new(k1: u64, k2: u64) -> Self {
        // X3 is 0xcafef00dd15ea5e5 (default state from PCG paper because it cannot be 0.
        // C must be initialized to a value > 1 and < MULTIPLIER
        Mwc256XXA64::from_state_incr(k1, k2, 0xcafef00dd15ea5e5, 0x14057B7EF767814F)
    }

    #[inline]
    fn from_state_incr(x1: u64, x2: u64, x3: u64, c: u64) -> Self {
        let mut pcg = Mwc256XXA64 { x1, x2, x3, c };
        //Advance 6 steps to fully mix the keys.
        pcg.step();
        pcg.step();
        pcg.step();
        pcg.step();
        pcg.step();
        pcg.step();
        pcg
    }

    #[inline]
    fn gen4(&mut self) -> [u64; 4] {
        //This is faster than calling `next_u64` 4 times because it avoids the intermediate assignments to the member variables.
        //For some reason the compiler doesn't figure this out automatically.
        let mut result = [0; 4];
        let (low, hi) = multiply(self.x3);
        result[0] = permute(self.x1, self.x2, self.x3, self.c, low, hi);
        let (r1, b) = low.overflowing_add(self.c);
        let c = hi.wrapping_add(b as u64);
        let (low, hi) = multiply(self.x2);
        result[1] = permute(r1, self.x1, self.x2, c, low, hi);
        let (r2, b) = low.overflowing_add(c);
        let c = hi.wrapping_add(b as u64);
        let (low, hi) = multiply(self.x1);
        result[2] = permute(r2, r1, self.x1, c, low, hi);
        let (r3, b) = low.overflowing_add(c);
        let c = hi.wrapping_add(b as u64);
        let (low, hi) = multiply(r1);
        result[3] = permute(r3, r2, r1, c, low, hi);
        let (r4, b) = low.overflowing_add(c);
        let c = hi.wrapping_add(b as u64);
        self.c = c;
        self.x1 = r4;
        self.x2 = r3;
        self.x3 = r2;
        return result;
    }

    #[inline]
    fn step(&mut self) -> u64 {
        // prepare the MCG for the next round
        let (low, hi) = multiply(self.x3);
        let result = permute(self.x1, self.x2, self.x3, self.c, low, hi);
        let (x1, b) = low.overflowing_add(self.c);
        self.x3 = self.x2;
        self.x2 = self.x1;
        self.x1 = x1;
        self.c = hi.wrapping_add(b as u64);
        result
    }
}

#[inline(always)]
fn multiply(val: u64) -> (u64, u64) {
    //While this looks like 128 bit math, it compiles to a 64 bit multiply.
    let t = (val as u128).wrapping_mul(MULTIPLIER as u128);
    return (t as u64, (t >> 64) as u64);
}

#[inline(always)]
fn permute(x1: u64, x2: u64, x3: u64, _c: u64, _low: u64, hi: u64) -> u64 {
    (x3 ^ x2).wrapping_add(x1 ^ hi)
}

// Custom Debug implementation that does not expose the internal state
impl fmt::Debug for Mwc256XXA64 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Mwc256XXA64 {{}}")
    }
}

/// We use a single 249-bit seed to initialise the state and select a stream.
/// One `seed` bit (lowest bit of `seed[8]`) is ignored.
impl SeedableRng for Mwc256XXA64 {
    type Seed = [u8; 32];

    fn from_seed(seed: Self::Seed) -> Self {
        let mut seed_u64 = [0u64; 4];
        le::read_u64_into(&seed, &mut seed_u64);
        // c must be < MULTIPLE and not all 1s or 0s
        let c = (seed_u64[0] & 0x3ffffffffffffff8) | 5;
        // X3 must be non-zero and not all 1s, hence we discard 2 bits
        let x3 = (seed_u64[3] << 2) | 1;
        Mwc256XXA64::from_state_incr(seed_u64[1], seed_u64[2], x3, c)
    }
}

impl RngCore for Mwc256XXA64 {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        self.step()
    }

    #[inline]
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        fill_bytes_impl(self, dest)
    }

    #[inline(always)]
    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
        self.fill_bytes(dest);
        Ok(())
    }
}

#[inline(always)]
fn fill_bytes_impl(rng: &mut Mwc256XXA64, dest: &mut [u8]) {
    let mut left = dest;
    while left.len() > 0 {
        for chunk in rng.gen4().iter() {
            if left.len() >= 8 {
                let (l, r) = left.split_at_mut(8);
                l.copy_from_slice(&chunk.to_le_bytes());
                left = r;
            } else {
                left.copy_from_slice(&chunk.to_le_bytes()[..left.len()]);
                return;
            }
        }
    }
}
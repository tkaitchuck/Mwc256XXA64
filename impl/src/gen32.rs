use rand_core::{RngCore, Error, SeedableRng, le};

#[cfg(feature = "serde1")]
use serde::{Deserialize, Serialize};

// Deliberately poor constants for testing:
// 2562598503 - Lag-2 or 3 Truly awful spectra
// 2487410280 - Lag-2 or 3 Very bad spectra

const MULTIPLIER: u32 = 3487286589; //Suitable for lag-2,3,4 acceptably good spectra

/// A PCG random number generator (MWC X A 128/32 variant).
///
/// Permuted Congruential Generator with 128-bit state, internal multiply
/// with carry Generator, and 32-bit output via a xor and an add.
#[derive(Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
pub struct Mwc128XXA32 {
    pub(crate) x1: u32,
    pub(crate) x2: u32,
    pub(crate) x3: u32,
    pub(crate) c: u32,
}

impl Mwc128XXA32 {
    /// Construct an instance given two keys.
    pub fn new(k1: u32, k2: u32) -> Self {
        // X3 is 0xcafef00d 0xd15ea5e5 (default state from PCG paper because it cannot be 0.
        // C must be initialized to a value > 1 and < MULTIPLIER
        Mwc128XXA32::from_state_incr(k1, k2, 0xcafef00d, 0xd15ea5e5)
    }

    #[inline]
    fn from_state_incr(x1: u32, x2: u32, x3: u32, c: u32) -> Self {
        let mut pcg = Mwc128XXA32 { x1, x2, x3, c };
        //Advance 6 steps to fully mix the keys.
        pcg.gen6();
        pcg
    }

    pub fn next(&mut self) -> u32 {
        self.step()
    }

    #[inline]
    fn step(&mut self) -> u32 {
        // prepare the MCG for the next round
        let (low, hi) = multiply(self.x3);
        let result = permute(self.x1, self.x2, self.x3, self.c, low, hi);
        let (x1, b) = low.overflowing_add(self.c);
        self.x3 = self.x2;
        self.x2 = self.x1;
        self.x1 = x1;
        self.c = hi.wrapping_add(b as u32);
        result
    }

    #[inline]
    fn gen6(&mut self) -> [u32; 6] {
        //This is faster than calling `next_u32` 6 times because it avoids the intermediate assignments to the member variables.
        //For some reason the compiler doesn't figure this out automatically.
        let mut result = [0; 6];
        let (low, hi) = multiply(self.x3);
        result[0] = permute(self.x1, self.x2, self.x3, self.c, low, hi);
        let (r1, b) = low.overflowing_add(self.c);
        let c = hi.wrapping_add(b as u32);
        let (low, hi) = multiply(self.x2);
        result[1] = permute(r1, self.x1, self.x2, c, low, hi);
        let (r2, b) = low.overflowing_add(c);
        let c = hi.wrapping_add(b as u32);
        let (low, hi) = multiply(self.x1);
        result[2] = permute(r2, r1, self.x1, c, low, hi);
        let (r3, b) = low.overflowing_add(c);
        let c = hi.wrapping_add(b as u32);

        let (low, hi) = multiply(r1);
        result[3] = permute(r3, r2, r1, c, low, hi);
        let (r1, b) = low.overflowing_add(c);
        let c = hi.wrapping_add(b as u32);
        let (low, hi) = multiply(r2);
        result[4] = permute(r1, r3, r2, c, low, hi);
        let (r2, b) = low.overflowing_add(c);
        let c = hi.wrapping_add(b as u32);
        let (low, hi) = multiply(r3);
        result[5] = permute(r2, r1, r3, c, low, hi);
        let (r3, b) = low.overflowing_add(c);
        let c = hi.wrapping_add(b as u32);

        self.c = c;
        self.x1 = r3;
        self.x2 = r2;
        self.x3 = r1;
        return result;
    }
}


/// We use a single 121-bit seed to initialise the state and select a stream.
/// Of the 128 `seed` bits 7 are ignored.
impl SeedableRng for Mwc128XXA32 {
    type Seed = [u8; 16];

    fn from_seed(seed: Self::Seed) -> Self {
        let mut seed_u32 = [0u32; 4];
        le::read_u32_into(&seed, &mut seed_u32);
        // c must be < MULTIPLE and not all 1s or 0s
        let c = (seed_u32[0] & 0x3fff_fff8) | 5;
        // X3 must be non-zero and not all 1s, hence we discard 2 bits
        let x3 = (seed_u32[3] << 2) | 1;
        Mwc128XXA32::from_state_incr(seed_u32[1], seed_u32[2], x3, c)
    }
}

impl RngCore for Mwc128XXA32 {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        self.step() as u32
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        let result = self.step() as u64;
        return (result << 32) | (self.step() as u64);
    }

    #[inline]
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        let mut dest_chunks = dest.chunks_exact_mut(6 * 4);
        for mut dest_chunk in &mut dest_chunks {
            for &num in self.gen6().iter() {
                let (l, r) = dest_chunk.split_at_mut(4);
                l.copy_from_slice(&num.to_le_bytes());
                dest_chunk = r;
            }
        }
        for dest_chunk in dest_chunks.into_remainder().chunks_mut(4) {
            dest_chunk.copy_from_slice(&self.step().to_le_bytes()[..dest_chunk.len()]);
        }
    }

    #[inline(always)]
    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
        self.fill_bytes(dest);
        Ok(())
    }
}

#[inline(always)]
fn multiply(val: u32) -> (u32, u32) {
    let t = (val as u64).wrapping_mul(MULTIPLIER as u64);
    return (t as u32, (t >> 32) as u32);
}

#[inline(always)]
fn permute(x1: u32, x2: u32, x3: u32, _c: u32, _low: u32, hi: u32) -> u32 {
    (x3 ^ x2).wrapping_add(x1 ^ hi)
}
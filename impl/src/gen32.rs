use rand_core::{RngCore, Error};

// Deliberately poor constants for testing:
// 2562598503 - Lag-2 or 3 Truly awful spectra
// 2487410280 - Lag-2 or 3 Very bad spectra

const MULTIPLIER: u32 = 3487286589; //Suitable for lag-2,3,4 acceptably good spectra

pub struct Mwc128XXA32 {
    pub(crate) x1: u32,
    pub(crate) x2: u32,
    pub(crate) x3: u32,
    pub(crate) c: u32,
}

impl Default for Mwc128XXA32 {
    fn default() -> Self {
        Mwc128XXA32 { x1: 123, x2: 45, x3: 67, c: 89 }
    }
}

impl Mwc128XXA32 {
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
use rand_core::{RngCore, Error};

const MULTIPLIER: u32 = 3487286589; //Suitable for lag-2,3,4

pub struct Gen32 {
    pub(crate) x1: u32,
    pub(crate) x2: u32,
    pub(crate) x3: u32,
    pub(crate) x4: u32,
    pub(crate) c: u32,
}

impl Default for Gen32 {
    fn default() -> Self {
        Gen32{x1:1, x2:3, x3: 4, x4: 0, c: 12}
    }
}

impl Gen32 {
    #[inline]
    fn step(&mut self) -> u32 {
        // prepare the MCG for the next round
        let (low, hi) = multiply(self.x4);
        let result = (self.x1 ^ self.x2).wrapping_add(self.x3 ^ self.x4);
        let (x1, b) = low.overflowing_add(self.c);
        self.x4 = self.x3;
        self.x3 = self.x2;
        self.x2 = self.x1;
        self.x1 = x1;
        self.c = hi.wrapping_add(b as u32);
        result
    }

    #[inline]
    fn gen4(&mut self) -> [u32; 4] {
        let mut result = [0;4];
        result[0] = self.step();
        result[1] = self.step();
        result[2] = self.step();
        result[3] = self.step();
        result
    }
}


impl RngCore for Gen32 {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        self.step() as u32
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        let result = self.step() as u64;
        return (result << 32) |  (self.step() as u64);
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
fn fill_bytes_impl(rng: &mut Gen32, dest: &mut [u8]) {
    let mut left = dest;
    while left.len() > 0 {
        for chunk in rng.gen4().iter() {
            if left.len() >= 4 {
                let (l, r) = left.split_at_mut(4);
                l.copy_from_slice(&chunk.to_le_bytes());
                left = r;
            } else {
                left.copy_from_slice(&chunk.to_le_bytes()[..left.len()]);
                return;
            }
        }
    }
}

#[inline(always)]
fn multiply(val: u32) -> (u32, u32) {
    let t = (val as u64).wrapping_mul(MULTIPLIER as u64);
    return (t as u32, (t >> 32) as u32);
}
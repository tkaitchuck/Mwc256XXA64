
pub struct PCG {
    state: u32,
}

impl Iterator for PCG {
    type Item = u16;

    fn next(&mut self) -> Option<Self::Item> {
        let old_state = self.state;
        self.state = self.state * 1019135901 + 2739110765;
        let rot = old_state >> (32-4);
        let result = old_state ^ (old_state >> 10); //(32-12)/2
        let result = (result >> 12) as u16; //16-4
        Some(result.rotate_right(rot))
    }
}

impl Default for PCG {
    fn default() -> Self {
        PCG {
            state: 0x12345678,
        }
    }
}
// Alternative constants
// 19530 - lag-2 Really bad spectra
// 65274 - lag-2 bad spectra
// 52563 - lag-2 good spectra
// 26298 - lag-2 or 4. decent spectra for 2
// 62139 - lag-3 decent spectra
// 57984 - lag-4 REALLY BAD spectra
const MULTIPLIER: u16 = 39273; //Lag-3 good spectra

pub struct Gen16 {
    pub(crate) x1: u16,
    pub(crate) x2: u16,
    pub(crate) x3: u16,
    pub(crate) c: u16,
}

impl Gen16 {

    pub(crate) fn next(&mut self) -> u16 {
        // prepare the MCG for the next round
        let (low, hi) = multiply(self.x3);
        let result = (self.x3 ^ self.x2).wrapping_add(self.x1 ^ hi);
        let (x1, b) = low.overflowing_add(self.c);
        self.x3 = self.x2;
        self.x2 = self.x1;
        self.x1 = x1;
        self.c = hi.wrapping_add(b as u16);
        result
    }
}

#[inline(always)]
fn multiply(val: u16) -> (u16, u16) {
    let t = (val as u32).wrapping_mul(MULTIPLIER as u32);
    return (t as u16, (t >> 16) as u16);
}
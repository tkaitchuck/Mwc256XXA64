
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

const MULTIPLIER: u16 = 26298; //lag 2 or 4.

pub struct Gen16 {
    pub(crate) x1: u16,
    pub(crate) x2: u16,
    pub(crate) c: u16,
}

impl Iterator for Gen16 {
    type Item = u16;

    fn next(&mut self) -> Option<Self::Item> {
        // prepare the MCG for the next round
        let (low, hi) = multiply(self.x2);
        let result = (self.x2 ^ self.x1).wrapping_add(self.c ^ hi);
        let (x1, b) = low.overflowing_add(self.c);
        //self.x3 = self.x2;
        self.x2 = self.x1;
        self.x1 = x1;
        self.c = hi.wrapping_add(b as u16);
        Some(result)
    }
}

#[inline(always)]
fn multiply(val: u16) -> (u16, u16) {
    let t = (val as u32).wrapping_mul(MULTIPLIER as u32);
    return (t as u16, (t >> 16) as u16);
}
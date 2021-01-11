pub trait Gen {
    fn reset(&mut self, config: bool);

    fn next(&mut self) -> u8;
}

pub struct Lcg {
    pub(crate) x: u16,
    pub inc: u16,
}

const M16: u16 = 25385;
const INC1: u16 = 771;
const INC2: u16 = 55555;

impl Default for Lcg {
    fn default() -> Self {
        Lcg {
            x: 12345,
            inc: INC1,
        }
    }
}

impl Gen for Lcg {
    fn reset(&mut self, config: bool) {
        self.x = 1;
        if config {
            self.inc = INC1;
        } else {
            self.inc = INC2;
        }
    }

    fn next(&mut self) -> u8 {
        // prepare the MCG for the next round
        self.x = self.x.wrapping_mul(M16).wrapping_add(self.inc);
        let xorshifted = (((self.x >> 5) ^ self.x) >> 5) as u8;
        let rot = self.x >> 13;
        return xorshifted.rotate_right(rot as u32);
    }
}


pub struct Xoshiro {
    s: [u8; 4],
    scramble: bool,
}

impl Default for Xoshiro {
    fn default() -> Self {
        Xoshiro {
            s: [255, 100, 200, 5],
            scramble: true,
        }
    }
}

impl Gen for Xoshiro {
    fn reset(&mut self, _config: bool) {
        self.s = [1, 2, 3, 5];
    }

    fn next(&mut self) -> u8 {
        let result = if self.scramble {
            self.s[0].wrapping_add(self.s[3]).rotate_left(4).wrapping_add(self.s[0])
        } else {
            self.s[0]
        };
        let t = self.s[1] << 3;
        self.s[2] ^= self.s[0];
        self.s[3] ^= self.s[1];
        self.s[1] ^= self.s[2];
        self.s[0] ^= self.s[3];
        self.s[2] ^= t;
        self.s[3] = self.s[3].rotate_left(5);
        return result;
    }
}

// This is the default multiplier used by MWC.
//const MULTIPLIER: u8 = 210; //lag-1
//const MULTIPLIER: u8 = 45; //lag-2
pub(crate) const MULTIPLIER: u8 = 123; //lag-3  123, 228
//const MULTIPLIER: u8 = 227; //Not full period, but standin for 4.

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Gen8 {
    pub(crate) x1: u8,
    pub(crate) x2: u8,
    pub(crate) x3: u8,
    // pub(crate) x4: u8,
    pub(crate) c: u8,
    pub(crate) mul: u8,
}

impl Default for Gen8 {
    fn default() -> Self {
        Gen8 {
            x1: 169,
            x2: 163,
            x3: 213,
            // x4: 217,
            c: 31,
            mul: MULTIPLIER,
        }
    }
}

impl Gen for Gen8 {
    fn reset(&mut self, config: bool) {
        self.x1 = 120;
        self.x2 = 34;
        self.x3 = 56;
        // self.x4 = 99;
        self.c = 78;
        if config {
            self.mul = 123;
        } else {
            self.mul = 228;
        }
    }

    fn next(&mut self) -> u8 {
        // prepare the MCG for the next round
        let t = (self.x3 as u16).wrapping_mul(self.mul as u16);
        let (low, hi) = (t as u8, (t >> 8) as u8);
        let result = (self.x3 ^ self.x2).wrapping_add(self.x1 ^ hi);
        let (x1, b) = low.overflowing_add(self.c);
        // self.x4 = self.x3;
        self.x3 = self.x2;
        self.x2 = self.x1;
        self.x1 = x1;
        self.c = hi.wrapping_add(b as u8);
        result
    }
}


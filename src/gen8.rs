pub trait Gen {
    fn reset(&mut self);

    fn next(&mut self) -> u8;
}

pub struct Pcg16_8 {
    pub(crate) x: u16,
    pub inc: u16,
}

const M16: u16 = 25385;
const INC1: u16 = 771; //Alt: 5555

impl Default for Pcg16_8 {
    fn default() -> Self {
        Pcg16_8 {
            x: 1,
            inc: INC1,
        }
    }
}

impl Gen for Pcg16_8 {
    fn reset(&mut self) {
        self.x = 1;
        self.inc = INC1;
    }

    fn next(&mut self) -> u8 {
        // prepare the MCG for the next round
        self.x = self.x.wrapping_mul(M16).wrapping_add(self.inc);
        let xorshifted = (((self.x >> 5) ^ self.x) >> 5) as u8;
        let rot = self.x >> 13;
        return xorshifted.rotate_right(rot as u32);
    }
}


pub struct Xoshiro32 {
    pub(crate) s: [u8; 4],
    pub(crate) scramble: bool,
}

impl Default for Xoshiro32 {
    fn default() -> Self {
        Xoshiro32 {
            s: [255, 100, 200, 5],
            scramble: true,
        }
    }
}

impl Gen for Xoshiro32 {
    fn reset(&mut self) {
        self.s = [1, 2, 3, 5];
    }

    fn next(&mut self) -> u8 {
        let result = if self.scramble {
            self.s[0].wrapping_add(self.s[3]).rotate_left(7).wrapping_add(self.s[0])
        } else {
            self.s[2]
        };
        let t = self.s[1] << 3;
        self.s[2] ^= self.s[0];
        self.s[3] ^= self.s[1];
        self.s[1] ^= self.s[2];
        self.s[0] ^= self.s[3];
        self.s[2] ^= t;
        self.s[3] = self.s[3].rotate_left(7);
        return result;
    }
}

// Possible constants:
// lag-1: 90
// lag-1: 99
// lag-1: 174
// lag-1: 204
// lag-1: 210
// lag-2: 45
// lag-3: 123
// lag-3: 228
// There are no full period constants for lag-4. 227 can be used as a stand-in.

// This is the default multiplier used by MWC.
pub(crate) const MULTIPLIER: u8 = 228;

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Mcg32_8 {
    pub(crate) x1: u8,
    pub(crate) x2: u8,
    pub(crate) x3: u8,
    pub(crate) c: u8,
}

impl Default for Mcg32_8 {
    fn default() -> Self {
        Mcg32_8 {
            x1: 123,
            x2: 34,
            x3: 56,
            c: 78,
        }
    }
}

impl Gen for Mcg32_8 {
    fn reset(&mut self) {
        self.x1 = 123;
        self.x2 = 34;
        self.x3 = 56;
        self.c = 78;
    }

    fn next(&mut self) -> u8 {
        // prepare the MCG for the next round
        let t = (self.x3 as u16).wrapping_mul(MULTIPLIER as u16);
        let (low, hi) = (t as u8, (t >> 8) as u8);
        let result = (self.x3 ^ self.x2).wrapping_add(self.x1 ^ hi);
        let (x1, b) = low.overflowing_add(self.c);
        self.x3 = self.x2;
        self.x2 = self.x1;
        self.x1 = x1;
        self.c = hi.wrapping_add(b as u8);
        result
    }
}


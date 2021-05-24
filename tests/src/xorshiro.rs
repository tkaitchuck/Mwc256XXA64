use rand_core::impls::{fill_bytes_via_next, next_u64_via_u32};
use rand_core::le::{read_u64_into, read_u32_into};
use rand_core::{SeedableRng, RngCore, Error};


/// Apply the ** scrambler used by some RNGs from the xoshiro family.
macro_rules! starstar {
    ($x:expr) => {
        $x.wrapping_mul(5).rotate_left(7).wrapping_mul(9)
    }
}

/// Apply the ++ scrambler used by some RNGs from the xoshiro family.
macro_rules! plusplus {
    ($x:expr, $y:expr, $rot:expr) => {
        $x.wrapping_add($y).rotate_left($rot).wrapping_add($x)
    }
}

/// Implement the xoshiro iteration for `u8` output.
macro_rules! impl_xoshiro_u8 {
    ($self:expr) => {
        let t = $self.s[1] << 3;

        $self.s[2] ^= $self.s[0];
        $self.s[3] ^= $self.s[1];
        $self.s[1] ^= $self.s[2];
        $self.s[0] ^= $self.s[3];

        $self.s[2] ^= t;

        $self.s[3] = $self.s[3].rotate_left(7);
    }
}

/// Implement the xoshiro iteration for `u16` output.
macro_rules! impl_xoshiro_u16 {
    ($self:expr) => {
        let t = $self.s[1] << 5;

        $self.s[2] ^= $self.s[0];
        $self.s[3] ^= $self.s[1];
        $self.s[1] ^= $self.s[2];
        $self.s[0] ^= $self.s[3];

        $self.s[2] ^= t;

        $self.s[3] = $self.s[3].rotate_left(11);
    }
}

/// Implement the xoshiro iteration for `u32` output.
macro_rules! impl_xoshiro_u32 {
    ($self:expr) => {
        let t = $self.s[1] << 9;

        $self.s[2] ^= $self.s[0];
        $self.s[3] ^= $self.s[1];
        $self.s[1] ^= $self.s[2];
        $self.s[0] ^= $self.s[3];

        $self.s[2] ^= t;

        $self.s[3] = $self.s[3].rotate_left(11);
    }
}

/// Implement the xoshiro iteration for `u64` output.
macro_rules! impl_xoshiro_u64 {
    ($self:expr) => {
        let t = $self.s[1] << 17;

        $self.s[2] ^= $self.s[0];
        $self.s[3] ^= $self.s[1];
        $self.s[1] ^= $self.s[2];
        $self.s[0] ^= $self.s[3];

        $self.s[2] ^= t;

        $self.s[3] = $self.s[3].rotate_left(45);
    }
}

/// Map an all-zero seed to a different one.
macro_rules! deal_with_zero_seed {
    ($seed:expr, $Self:ident) => {
        if $seed.iter().all(|&x| x == 0) {
            return $Self::seed_from_u64(0);
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Xoshiro32 {
    s: [u8; 4],
}

impl Xoshiro32 {
    pub(crate) fn from_seeds(seed: u8, seed2: u8, seed3:  u8, seed4: u8) -> Xoshiro32 {
        Xoshiro32 { s: [seed, seed2, seed3, seed4] }
    }
}

impl Xoshiro32 {
    #[inline]
    pub(crate) fn next(&mut self) -> u8 {
        //let result = self.s[1];
        //let result = (self.s[0].wrapping_add(self.s[3]));
        //let result = starstar!(self.s[1]);
        let result = plusplus!(self.s[0], self.s[3], 7);
        impl_xoshiro_u8!(self);
        result
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Xoshiro321 {
    s: [u8; 4],
}

impl Xoshiro321 {
    pub(crate) fn from_seeds(seed: u8, seed2: u8, seed3:  u8, seed4: u8) -> Xoshiro321 {
        Xoshiro321 { s: [seed, seed2, seed3, seed4] }
    }
}

impl Xoshiro321 {
    #[inline]
    pub(crate) fn next(&mut self) -> u8 {
        //let result = self.s[1];
        //let result = (self.s[0].wrapping_add(self.s[3]));
        //let result = starstar!(self.s[1]);
        //let result = plusplus!(self.s[0], self.s[3], 7);
        impl_xoshiro_u8!(self);
        self.s[2]
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Xoshiro64 {
    s: [u16; 4],
}

impl Xoshiro64 {
    pub(crate) fn from_seeds(seed: u16, seed2: u16, seed3: u16, seed4: u16) -> Xoshiro64 {
        Xoshiro64 { s: [seed, seed2, seed3, seed4] }
    }
}

impl Xoshiro64 {
    #[inline]
    pub(crate) fn next(&mut self) -> u16 {
        //let result = self.s[1];
        //let result = (self.s[0].wrapping_add(self.s[3]));
        //let result = starstar!(self.s[1]);
        let result = plusplus!(self.s[0], self.s[3], 7);
        impl_xoshiro_u16!(self);
        result
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Xoshiro641 {
    s: [u16; 4],
}

impl Xoshiro641 {
    pub(crate) fn from_seeds(seed: u16, seed2: u16, seed3: u16, seed4: u16) -> Xoshiro641 {
        Xoshiro641 { s: [seed, seed2, seed3, seed4] }
    }
}

impl Xoshiro641 {
    #[inline]
    pub(crate) fn next(&mut self) -> u16 {
        //let result = self.s[1];
        //let result = (self.s[0].wrapping_add(self.s[3]));
        //let result = starstar!(self.s[1]);
        //let result = plusplus!(self.s[0], self.s[3], 7);
        impl_xoshiro_u16!(self);
        self.s[2]
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Xoshiro128 {
    s: [u32; 4],
}

impl Xoshiro128 {
    pub(crate) fn from_seeds(seed: u32, seed2: u32, seed3:  u32, seed4: u32) -> Xoshiro128 {
        Xoshiro128 { s: [seed, seed2, seed3, seed4] }
    }
}

impl Xoshiro128 {
    #[inline]
    pub(crate) fn next(&mut self) -> u32 {
        //let result = self.s[1];
        let result = (self.s[0].wrapping_add(self.s[3]));
        //let result = starstar!(self.s[1]);
        //let result = plusplus!(self.s[0], self.s[3], 7);
        impl_xoshiro_u32!(self);
        result
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Xoshiro256 {
    s: [u64; 4],
}

impl Xoshiro256 {
    pub(crate) fn from_seeds(seed: u64, seed2: u64, seed3:  u64, seed4: u64) -> Xoshiro256 {
        Xoshiro256 { s: [seed, seed2, seed3, seed4] }
    }
}

impl Xoshiro256 {
    #[inline]
    pub(crate) fn next(&mut self) -> u64 {
        // let result = self.s[1];
        // let result = self.s[0].wrapping_add(self.s[3]);
        //let result = starstar!(self.s[1]);
         let result = plusplus!(self.s[0], self.s[3], 23);
        impl_xoshiro_u64!(self);
       result
    }
}

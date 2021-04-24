const INCREMENT: u128 = 0xEA7_A7_CAFE_5EEDBED_4_C0FFEE_7EA_5A1AD5;
const KEY: u128 = 0x_BEA75_CAFE_15_A150_A_D15C0_175_A_B1A57;

pub struct AesPrng {
    counter: u128,
}

impl AesPrng {
    pub fn new(seed: u128) -> Self {
        AesPrng { counter: seed }
    }

    #[inline]
    fn next(&mut self) -> u64 {
        use std::arch::x86_64::*;
        unsafe {
            let counter: __m128i = mem::transmute(self.counter);
            let key: __m128i = mem::transmute(KEY);
            let r1 = _mm_aesenc_si128(counter, key);
            let r2 = _mm_aesenc_si128(r1, key);
            let r3 = _mm_aesenc_si128(r2, r1);
            let r4 = _mm_aesenc_si128(r3, counter);
            self.counter += INCREMENT;
            let result: u128 = mem::transmute(r4);
            result as u64
        }
    }

    #[inline]
    pub fn fill_bytes(&mut self, dest: &mut [u8]) {
        let mut left = dest;
        while left.len() >= 8 {
            let (l, r) = { left }.split_at_mut(8);
            left = r;
            let chunk: [u8; 8] = self.next().to_le_bytes();
            l.copy_from_slice(&chunk);
        }
        let n = left.len();
        if n > 0 {
            let chunk: [u8; 8] = self.next().to_le_bytes();
            left.copy_from_slice(&chunk[..n]);
        }
    }

}
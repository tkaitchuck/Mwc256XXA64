use std::sync::atomic::{AtomicU32, Ordering};
use std::ops::{Shl, Sub};
use rayon::prelude::*;
use glass_pumpkin::num_bigint::BigUint;

// 7c492513927a59b3 and a7298353f4250d13 both pass for 64, 192, and 256.
// feb344657c0af413 passes for 192 and has a good spectrum.
fn is_good_prime(candidate: &u64) -> bool {
    let value = BigUint::from(*candidate);
    // if !glass_pumpkin::safe_prime::check(&value.clone().shl(64u32).sub(1u32)) {
    //     return false;
    // }
    if !glass_pumpkin::safe_prime::check(&value.clone().shl(192u32).sub(1u32)) {
        return false;
    }
    if !glass_pumpkin::safe_prime::check(&value.clone().shl(256u32).sub(1u32)) {
        return false;
    }
    return true;
}

pub(crate) fn gen_8_bit_candidates() {
    for v in 1..=255u8 {
        let value = BigUint::from(v);
        if glass_pumpkin::safe_prime::check(&value.clone().shl(8u32).sub(1u32)) {
            println!("lag-1: {}", v);
        }
        if glass_pumpkin::safe_prime::check(&value.clone().shl(16u32).sub(1u32)) {
            println!("lag-2: {}", v);
        }
        if glass_pumpkin::safe_prime::check(&value.clone().shl(24u32).sub(1u32)) {
            println!("lag-3: {}", v);
        }
        if glass_pumpkin::safe_prime::check(&value.clone().shl(32u32).sub(1u32)) {
            println!("lag-4: {}", v);
        }
    }
}

pub(crate) fn gen_16_bit_candidates() {
    for v in 1..=u16::max_value() {
        let value = BigUint::from(v);
        if glass_pumpkin::safe_prime::check(&value.clone().shl(32u32).sub(1u32)) {
            println!("lag-2: {}", v);
            if glass_pumpkin::safe_prime::check(&value.clone().shl(48u32).sub(1u32)) {
                println!("lag-3: {}", v);
            }
            if glass_pumpkin::safe_prime::check(&value.clone().shl(64u32).sub(1u32)) {
                println!("lag-4: {}", v);
            }
        }
    }
}


//Outputs:
// lag-4: 205533153
// lag-4: 1131874548
// lag-4: 2233997409
// lag-4: 2483802489
// lag-4: 2544691569
// lag-4: 3487286589
pub(crate) fn gen_32_bit_candidates() {
    for v in 1..=u32::max_value() {
        let value = BigUint::from(v);
        if glass_pumpkin::safe_prime::check(&value.clone().shl(64u32).sub(1u32)) {
            if glass_pumpkin::safe_prime::check(&value.clone().shl(96u32).sub(1u32)) {
                println!("lag-3: {}", v);
                if glass_pumpkin::safe_prime::check(&value.clone().shl(128u32).sub(1u32)) {
                    println!("lag-4: {}", v);
                }
            }
        }
    }
}

fn no_dups(value: u64) -> bool {
    let bytes = value.to_ne_bytes();
    for &byte in &bytes {
        let mut count = 0;
        for &to_match in &bytes {
            if to_match == byte {
                count+=1;
            }
        }
        if count > 1 {
            return false;
        }
    }
    true
}

pub(crate) fn gen_64_bit_candidates() -> Vec<u64> {
    let sieve = primal::Sieve::new(u32::max_value() as usize);
    let mut c64s = Vec::new();
    let mut c32s2s = Vec::new();
    let mut c32s1s = Vec::new();
    let mut c16s = Vec::new();
    let mut c8s = Vec::new();
    for v in sieve.primes_from(0).take_while(|x| *x < 256) {
        c8s.push(v as u8);
        if v < 128 {
            c8s.push(v as u8 * 2);
        }
        if v < 64 {
            c8s.push(v as u8 * 4);
        }
    }
    for &a in c8s.iter() {
        for &b in c8s.iter() {
            let v = ((a as u16) << 8) | (b as u16);
            if sieve.is_prime(v as usize) {
                c16s.push(v);
                if v < 32768 {
                    c16s.push(v as u16 * 2);
                }
            }
        }
    }
    for &a in c16s.iter() {
        for &b in c16s.iter() {
            let v = ((a as u32) << 16) | (b as u32);
            if v % 3 == 1 {
                c32s1s.push(v);
            } else if v % 3 == 2 {
                c32s2s.push(v);
            }
        }
    }
    dbg!(c32s1s.len());
    dbg!(c32s2s.len());
    let counter = AtomicU32::new(0);
    let good : Vec<_> = c32s1s.par_iter().map(|&a| {
        let mut candidates = Vec::new();
        for &b in c32s2s.iter() {
            let v1 = ((a as u64) << 32) | (b as u64);
            if  v1 > 0xfe00_0000_0000_0000 && ((v1 & 7 == 5) || (v1 & 7 == 3)) {
                assert_eq!(v1 % 3, 0);
                let value = glass_pumpkin::num_bigint::BigUint::from(v1 / 3);
                if glass_pumpkin::prime::check(&value) {
                    if no_dups(v1) && is_good_prime(&v1) {
                        candidates.push(v1);
                        println!("0x{:x}", v1);
                    }
                }
            }
            let v2 = ((b as u64) << 32) | (a as u64);
            if  v2 > 0xfe00_0000_0000_0000 && ((v2 & 7 == 5) || (v2 & 7 == 3)) {
                assert_eq!(v2 % 3, 0);
                let value = glass_pumpkin::num_bigint::BigUint::from(v2 / 3);
                if glass_pumpkin::prime::check(&value) {
                    if no_dups(v1) && is_good_prime(&v2) {
                        candidates.push(v2);
                        println!("0x{:x}", v2);
                    }
                }
            }
        }
        let val = counter.fetch_add(1, Ordering::SeqCst);
        if val % 100 == 0 {
            dbg!(val);
        }
        candidates
    }).collect();
    for mut set in good {
        c64s.append(&mut set);
    }
    dbg!(c64s.len());
    for v in c64s.iter() {
        println!("{:x}", v);
    }
    c64s
}

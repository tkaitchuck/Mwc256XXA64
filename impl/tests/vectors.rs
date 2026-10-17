//! Known outputs, recorded from pcg-mwc 0.2.1 (on rand_core 0.6), so that changes to the
//! `rand_core` integration can't change the generated values.

use pcg_mwc::{Mwc128XXA32, Mwc256XXA64};
use rand_core::{Rng, SeedableRng};

fn counting_seed<const N: usize>() -> [u8; N] {
    let mut seed = [0u8; N];
    for (i, b) in seed.iter_mut().enumerate() {
        *b = i as u8;
    }
    seed
}

#[test]
fn mwc256_new() {
    let mut rng = Mwc256XXA64::new(42, (7u64 << 32) | 0x1234abcd);
    assert_eq!(rng.next_u64(), 0x58f1ce86742207b0);
    assert_eq!(rng.next_u64(), 0x9f851010a51aecb8);
    assert_eq!(rng.next_u64(), 0xca5547faa599e800);
    assert_eq!(rng.next_u32(), 0x82432cce);
}

#[test]
fn mwc256_seeded() {
    let mut rng = Mwc256XXA64::seed_from_u64(7);
    assert_eq!(rng.next_u64(), 0xa0ef6c104bb590e8);
    assert_eq!(rng.next_u64(), 0x0674eea97327aa56);
    let mut rng = Mwc256XXA64::from_seed(counting_seed());
    assert_eq!(rng.next_u64(), 0xc0254efd76eca57f);
    assert_eq!(rng.next_u64(), 0x559ab5a328916d56);
}

#[test]
fn mwc256_fill_bytes() {
    let mut rng = Mwc256XXA64::new(1, 2);
    let mut buf = [0u8; 53];
    rng.fill_bytes(&mut buf);
    assert_eq!(
        buf,
        [
            25, 153, 221, 165, 3, 64, 62, 197, 147, 128, 205, 22, 219, 20, 175, 66, 232, 217, 107,
            30, 215, 50, 56, 24, 235, 120, 33, 80, 185, 134, 168, 99, 127, 217, 152, 1, 222, 26,
            68, 220, 0, 189, 164, 246, 194, 157, 246, 72, 94, 80, 207, 218, 50
        ]
    );
    assert_eq!(rng.next_u64(), 0x3422f19ce2b0b6d8);
}

#[test]
fn mwc128_new() {
    let mut rng = Mwc128XXA32::new(42, 0x1234abcd);
    assert_eq!(rng.next_u32(), 0xbccef4da);
    assert_eq!(rng.next_u32(), 0xf3a19994);
    assert_eq!(rng.next_u32(), 0x194b038d);
    assert_eq!(rng.next_u64(), 0xfb25db6d3558f592);
}

#[test]
fn mwc128_seeded() {
    let mut rng = Mwc128XXA32::seed_from_u64(7);
    assert_eq!(rng.next_u32(), 0x2b68bc3d);
    assert_eq!(rng.next_u32(), 0x1192a4db);
    let mut rng = Mwc128XXA32::from_seed(counting_seed());
    assert_eq!(rng.next_u32(), 0xa27adaf5);
    assert_eq!(rng.next_u32(), 0x9c05d536);
}

#[test]
fn mwc128_fill_bytes() {
    let mut rng = Mwc128XXA32::new(1, 2);
    let mut buf = [0u8; 29];
    rng.fill_bytes(&mut buf);
    assert_eq!(
        buf,
        [
            153, 2, 230, 104, 126, 135, 220, 144, 201, 241, 145, 170, 139, 72, 108, 255, 2, 8,
            168, 199, 208, 48, 206, 101, 239, 34, 92, 47, 161
        ]
    );
    assert_eq!(rng.next_u32(), 0x9fdcbce2);
}

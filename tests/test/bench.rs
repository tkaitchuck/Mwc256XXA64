use criterion::*;
use pcg_mwc::{Mwc256XXA64, AesPrng, Mwc128XXA32};
use rand_core::{RngCore, SeedableRng};
use rand_pcg::{Pcg64Mcg, Pcg64};
use rand_xoshiro::Xoshiro256PlusPlus;

fn bench_mwc32_kb(c: &mut Criterion) {
    let mut mwc = Mwc128XXA32::default();
    let mut vec = vec![0; 1024];
    c.bench(
        "Mwc128XA32",
        Benchmark::new("1kb",  move |b| b.iter(|| {
            mwc.fill_bytes(&mut vec[0..1024]);
        })),
    );
}

fn bench_mwc_kb(c: &mut Criterion) {
    let mut mwc = Mwc256XXA64::seed_from_u64(2);
    let mut vec = vec![0; 1024];
    c.bench(
        "Mwc256XXA64",
        Benchmark::new("1kb",  move |b| b.iter(|| {
            mwc.fill_bytes(&mut vec[0..1024]);
        })),
    );
}

fn bench_mwc_64(c: &mut Criterion) {
    let mut mwc = Mwc256XXA64::seed_from_u64(2);
    c.bench(
        "Mwc256XXA64",
        Benchmark::new("64",  move |b| b.iter(|| {
            mwc.next_u64()
        })),
    );
}

fn bench_aes_kb(c: &mut Criterion) {
    let mut aes = AesPrng::new(1);
    let mut vec = vec![0; 1024];
    c.bench(
        "aes_prng",
        Benchmark::new("1kb",  move |b| b.iter(|| {
            aes.fill_bytes(&mut vec[0..1024]);
        })),
    );
}

fn bench_pcg_kb(c: &mut Criterion) {
    let mut mcg = Pcg64::seed_from_u64(2);
    let mut vec = vec![0; 1024];
    c.bench(
        "pcg",
        Benchmark::new("1kb",  move |b| b.iter(|| {
            mcg.fill_bytes(&mut vec[0..1024]);
        })),
    );
}

fn bench_pcg_64(c: &mut Criterion) {
    let mut mcg = Pcg64::seed_from_u64(2);
    c.bench(
        "pcg",
        Benchmark::new("64",  move |b| b.iter(|| {
            mcg.next_u64()
        })),
    );
}

fn bench_pcg_fast_kb(c: &mut Criterion) {
    let mut mcg = Pcg64Mcg::seed_from_u64(2);
    let mut vec = vec![0; 1024];
    c.bench(
        "pcg_fast",
        Benchmark::new("1kb",  move |b| b.iter(|| {
            mcg.fill_bytes(&mut vec[0..1024]);
        })),
    );
}

fn bench_pcg_fast_64(c: &mut Criterion) {
    let mut mcg = Pcg64Mcg::seed_from_u64(2);
    c.bench(
        "pcg_fast",
        Benchmark::new("64",  move |b| b.iter(|| {
            mcg.next_u64()
        })),
    );
}

fn bench_xoshiro_kb(c: &mut Criterion) {
    let mut xoshiro = Xoshiro256PlusPlus::seed_from_u64(2);
    let mut vec = vec![0; 1024];
    c.bench(
        "xorhiro",
        Benchmark::new("1kb",  move |b| b.iter(|| {
            xoshiro.fill_bytes(&mut vec[0..1024]);
        })),
    );
}

fn bench_xoshiro_64(c: &mut Criterion) {
    let mut xoshiro = Xoshiro256PlusPlus::seed_from_u64(2);
    c.bench(
        "xorhiro",
        Benchmark::new("64",  move |b| b.iter(|| {
            xoshiro.next_u64()
        })),
    );
}

criterion_main!(benches);
criterion_group!(
    benches,
    bench_mwc_kb,
    bench_pcg_kb,
    bench_pcg_fast_kb,
    bench_xoshiro_kb,
    bench_aes_kb,
    bench_mwc_64,
    bench_pcg_64,
    bench_pcg_fast_64,
    bench_xoshiro_64,
    bench_mwc32_kb,
);

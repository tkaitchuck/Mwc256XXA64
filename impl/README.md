# Permuted Mwc random number generator

It is a member of the [PCG family](https://www.pcg-random.org/index.html) but uses an [MWC generator](https://en.wikipedia.org/wiki/Multiply-with-carry_pseudorandom_number_generator) instead of an [LCG](https://en.wikipedia.org/wiki/Linear_congruential_generator).
This an MWC is special form of a [MCG](https://en.wikipedia.org/wiki/Lehmer_random_number_generator) generator similar to the [PGC-64-fast](https://docs.rs/pcg_rand/0.13.0/pcg_rand/type.Pcg64Fast.html) variant. 

Compared to PGC-64-fast, Mwc256XXA64 is both faster and produces higher quality rand numbers (due to using the full 256bit state as opposed to only half of it.)

For a detailed description of the design, see: https://tom-kaitchuck.medium.com/designing-a-new-prng-1c4ffd27124d

## Mwc256XXA64
A fast high quality PRNG with 64bits of output, and a 256bit state. This is faster on 64bit architectures. 

## Mwc128XXA32
A fast high quality PRNG with 32bits of output, and a 128bit state. This is faster on 32bit architectures.



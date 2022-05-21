use rand::{ Rng, thread_rng };

// maximum size (in bytes) of an array which can be filled in one go from the RNG
const MAX_ARRAY_SIZE: usize = 32;

/// fill an array of `u8`s with random values
///
/// We use `thread_rng` rom the `rand` crate. 
/// Internally, it uses a ChaCha block cipher with 12 rounds as PRNG, seeded by the OS RNG.
pub fn fill_rand_bytes(a: &mut [u8]) {
    a.chunks_mut(MAX_ARRAY_SIZE)
     .map(|x| x.clone_from_slice(&thread_rng().gen::<[u8; MAX_ARRAY_SIZE]>()[..x.len()]))
     .count();
}

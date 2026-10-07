//! Random numbers for the rules. Native builds seed from the OS; a SpacetimeDB module has no
//! OS randomness, so it calls `reseed` with a value from the reducer context before using a rule.

use rand::{rngs::StdRng, RngCore, SeedableRng};
use std::cell::RefCell;

thread_local! {
    static RNG: RefCell<StdRng> = RefCell::new(initial_rng());
}

#[cfg(not(target_arch = "wasm32"))]
fn initial_rng() -> StdRng {
    StdRng::from_entropy()
}

#[cfg(target_arch = "wasm32")]
fn initial_rng() -> StdRng {
    StdRng::seed_from_u64(0x5eed_5eed)
}

pub fn reseed(seed: u64) {
    RNG.with(|rng| *rng.borrow_mut() = StdRng::seed_from_u64(seed));
}

/// Handle to the shared generator, used where the rules called `rand::thread_rng()`.
pub struct SharedRng;

impl RngCore for SharedRng {
    fn next_u32(&mut self) -> u32 {
        RNG.with(|rng| rng.borrow_mut().next_u32())
    }

    fn next_u64(&mut self) -> u64 {
        RNG.with(|rng| rng.borrow_mut().next_u64())
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        RNG.with(|rng| rng.borrow_mut().fill_bytes(dest))
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand::Error> {
        RNG.with(|rng| rng.borrow_mut().try_fill_bytes(dest))
    }
}

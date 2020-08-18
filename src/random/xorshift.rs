#[doc(hidden)]
pub use crate::random::random_generator::*;

pub struct Xorshift32 {
    seed: u32,
}

impl Xorshift32 {
    pub fn new(seed: u32) -> Self {
        Self { seed }
    }
}

impl RandomGenerator for Xorshift32 {
    type Output = u32;
    const MIN: u32 = u32::MIN;
    const MAX: u32 = u32::MAX;
    fn next(&mut self) -> u32 {
        let mut x: u32 = self.seed;
        x ^= x.wrapping_shl(13);
        x ^= x.wrapping_shr(17);
        x ^= x.wrapping_shl(5);
        self.seed = x;
        x
    }
}

pub struct Xorshift64 {
    seed: u64,
}

impl Xorshift64 {
    pub fn new(seed: u64) -> Self {
        Self { seed }
    }
}

impl RandomGenerator for Xorshift64 {
    type Output = u64;
    const MIN: u64 = u64::MIN;
    const MAX: u64 = u64::MAX;
    fn next(&mut self) -> u64 {
        let mut x: u64 = self.seed;
        x ^= x.wrapping_shl(13);
        x ^= x.wrapping_shr(7);
        x ^= x.wrapping_shl(17);
        self.seed = x;
        x
    }
}

pub struct Xorshift128 {
    seed: [u32; 4],
}

impl Xorshift128 {
    pub fn new(seed: [u32; 4]) -> Self {
        Self { seed }
    }
}

impl RandomGenerator for Xorshift128 {
    type Output = u32;
    const MIN: u32 = u32::MIN;
    const MAX: u32 = u32::MAX;
    fn next(&mut self) -> u32 {
        let mut t: u32 = self.seed[3];
        let s: u32 = self.seed[0];
        self.seed[3] = self.seed[2];
        self.seed[2] = self.seed[1];
        self.seed[1] = s;
        t ^= t.wrapping_shl(11);
        t ^= t.wrapping_shr(8);
        self.seed[0] = t ^ s ^ (s.wrapping_shr(19));
        self.seed[0]
    }
}

use serde::{Deserialize, Serialize};

/// Generador pseudoaleatorio determinista.
///
/// Se usa un LCG simple porque el objetivo de esta fase es reproducibilidad,
/// no seguridad criptográfica.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct DeterministicRng {
    state: u64,
}

impl DeterministicRng {
    pub fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    pub fn next_u32(&mut self) -> u32 {
        self.state = self
            .state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);

        (self.state >> 32) as u32
    }

    pub fn next_range(&mut self, upper_exclusive: u32) -> u32 {
        if upper_exclusive == 0 {
            return 0;
        }

        self.next_u32() % upper_exclusive
    }
}

#[cfg(test)]
mod tests {
    use super::DeterministicRng;

    #[test]
    fn rng_same_seed_repeats_sequence() {
        let mut first = DeterministicRng::new(42);
        let mut second = DeterministicRng::new(42);

        let first_values = (0..8).map(|_| first.next_u32()).collect::<Vec<_>>();
        let second_values = (0..8).map(|_| second.next_u32()).collect::<Vec<_>>();

        assert_eq!(first_values, second_values);
    }

    #[test]
    fn rng_range_respects_upper_bound() {
        let mut rng = DeterministicRng::new(99);

        for _ in 0..100 {
            assert!(rng.next_range(7) < 7);
        }
    }

    #[test]
    fn rng_zero_upper_bound_returns_zero() {
        let mut rng = DeterministicRng::new(99);

        assert_eq!(rng.next_range(0), 0);
    }
}

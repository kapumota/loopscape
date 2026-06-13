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

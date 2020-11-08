//! Super sampling implementation.

use arctk::{geom::Cube, math::Pos3, tools::linear_to_three_dim};
use arctk_attr::input;
use rand::Rng;

/// Super sampling types.
#[input]
#[derive(Clone)]
pub enum SuperSample {
    /// Uniform.
    Uniform([usize; 3]),
    /// Random.
    Random(usize),
}

impl SuperSample {
    /// Determine the nth sampling position.
    #[inline]
    #[must_use]
    pub fn sample<R: Rng>(&self, cube: &Cube, n: usize, rng: &mut R) -> Pos3 {
        match &self {
            Self::Uniform(res) => cube.uniform_pos(&res, &linear_to_three_dim(n, &res)),
            Self::Random(..) => cube.rand_pos(rng),
        }
    }
}

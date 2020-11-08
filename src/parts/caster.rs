//! Ray caster implementation.

use arctk::{
    geom::Ray,
    math::{Dir3, Pos3},
};
use arctk_attr::input;

/// Ray caster order.
#[input]
#[derive(Clone)]
pub enum Caster {
    /// Direction.
    Direction(Dir3),
    /// Target [m].
    Target(Pos3),
    // /// Soft-targeting [m] [rad].
    // Soft(u64, Pos3, f64),
    // /// Radiant.
    // Radiant(u64),
}

impl Caster {
    /// Calculate the total number of samples.
    #[inline]
    #[must_use]
    pub fn gen_ray(&self, pos: Pos3, _n: usize) -> Ray {
        match self {
            Self::Direction(dir) => Ray::new(pos, *dir),
            Self::Target(tar) => Ray::new(pos, Dir3::new_normalize(tar - pos)),
        }
    }

    /// Retrieve the number of potential samples.
    #[inline]
    #[must_use]
    pub fn num_samples(&self) -> usize {
        match self {
            Self::Direction(..) => 1,
            Self::Target(..) => 1,
        }
    }
}

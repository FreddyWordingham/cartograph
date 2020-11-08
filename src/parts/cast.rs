//! Ray caster implementation.

use arctk::math::{Dir3, Pos3};
use arctk_attr::input;

/// Ray caster order.
#[input]
#[derive(Clone)]
pub enum Cast {
    /// Direction.
    Direction(Dir3),
    /// Target [m].
    Target(Pos3),
    // /// Soft-targeting [m] [rad].
    // Soft(u64, Pos3, f64),
    // /// Radiant.
    // Radiant(u64),
}

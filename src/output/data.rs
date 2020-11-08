//! MCRT output structure.

use std::ops::AddAssign;

/// Cartographer output data.
pub struct Data {}

impl Data {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {}
    }
}

impl AddAssign<&Self> for Data {
    #[inline]
    fn add_assign(&mut self, _rhs: &Self) {}
}

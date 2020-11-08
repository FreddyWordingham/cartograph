//! MCRT output structure.

use arctk::{err::Error, file::Save};
use std::ops::AddAssign;
use std::path::Path;

/// Cartographer output data.
pub struct Data {}

impl Data {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {}
    }

    /// Save the maps to the given directory.
    #[inline]
    #[must_use]
    pub fn save_maps(&self, _out_dir: &Path) -> Result<(), Error> {
        Ok(())
    }
}

impl AddAssign<&Self> for Data {
    #[inline]
    fn add_assign(&mut self, _rhs: &Self) {}
}

impl Save for Data {
    #[inline]
    fn save(&self, out_dir: &Path) -> Result<(), Error> {
        self.save_maps(out_dir)
    }
}

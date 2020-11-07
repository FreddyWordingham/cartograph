//! Mapping simulation structure.

use arctk::clone;
use arctk_attr::input;

/// Loadable input settings structure.
#[input]
pub struct Settings {
    /// Bump distance [m].
    bump_dist: f64,
}

impl Settings {
    clone!(bump_dist, f64);
}

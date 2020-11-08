//! Photon-lifetime engine function.

use crate::{parts::Landscape, util::Key};
use arctk::geom::Ray;

/// Determine what a single ray will observe.
#[allow(clippy::expect_used)]
#[inline]
#[must_use]
pub fn observe(_land: &Landscape, _ray: Ray) -> Key {
    use std::str::FromStr;
    Key::from_str("todo").unwrap()
}

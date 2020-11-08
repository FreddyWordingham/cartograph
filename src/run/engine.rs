//! Photon-lifetime engine function.

use crate::{
    output::Data,
    parts::{Interface, Landscape},
    util::Key,
};
use arctk::{
    geom::{Mesh, Ray},
    ord::Set,
};

/// Determine what a single ray will observe.
#[allow(clippy::expect_used)]
#[inline]
#[must_use]
pub fn sample(_land: &Landscape, data: &mut Data, index: [usize; 3]) {
    data.maps.mut_map()["air"][index] + 1.0;
}

// /// Determine what a single ray will observe.
// #[allow(clippy::expect_used)]
// #[inline]
// #[must_use]
// pub fn observe(surfs: &Set<Key, Mesh>, inters: &Set<Key, Interface>, ray: &Ray) -> Key {}

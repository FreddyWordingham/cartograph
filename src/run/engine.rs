//! Photon-lifetime engine function.

use crate::{
    output::Data,
    parts::Landscape,
    // parts::{Interface, Landscape},
    //     util::Key,
};
// use arctk::ord::{X, Y, Z};

/// Determine what a single ray will observe.
#[allow(clippy::expect_used)]
#[inline]
pub fn sample(_land: &Landscape, _data: &mut Data, _index: [usize; 3]) {
    // let _power = land.sett.super_sample_power().unwrap_or(1);
    // let _weight = 1.0 / land.sett.samples_per_cell() as f64;
}

// /// Determine what a single ray will observe.
// #[allow(clippy::expect_used)]
// #[inline]
// #[must_use]
// pub fn observe(surfs: &Set<Key, Mesh>, inters: &Set<Key, Interface>, ray: &Ray) -> Key {}

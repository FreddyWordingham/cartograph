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
pub fn sample(land: &Landscape, _data: &mut Data, _index: [usize; 3]) {
    let _power = land.sett.super_sample_power().unwrap_or(1);
    let _weight = 1.0 / land.sett.samples_per_cell() as f64;
    // let voxel = data.grid.gen_voxel(index);

    // let pos = voxel.uniform_pos([xi,yi,zi]);
    // let pos = voxel.random_pos();

    // for xi in 0..power {
    //     let x = min[X] + (xi * cell_widths[X]);
    //     for yi in 0..power {
    //         let y = min[Y] + (yi * cell_widths[Y]);
    //         for zi in 0..power {
    //             let z = min[Z] + (zi * cell_widths[Z]);

    //             let sub_index = [xi, yi, zi];
    //             let pos = Pos3::new(x + (xi * sub));

    //             let key = observe(surfs, inters, ray);
    //             *data
    //                 .maps
    //                 .mut_map()
    //                 .get_mut("air")
    //                 .unwrap()
    //                 .get_mut(index)
    //                 .unwrap() += weight;
    //         }
    //     }
    // }
}

// /// Determine what a single ray will observe.
// #[allow(clippy::expect_used)]
// #[inline]
// #[must_use]
// pub fn observe(surfs: &Set<Key, Mesh>, inters: &Set<Key, Interface>, ray: &Ray) -> Key {}

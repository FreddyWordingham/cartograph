//! Photon-lifetime engine function.

use crate::{
    output::Data,
    parts::{Interface, Landscape},
    util::Key,
};
use arctk::ord::{X, Y, Z};
use arctk::{
    geom::{Mesh, Ray},
    ord::Set,
};
use rand::Rng;

/// Determine what a single ray will observe.
#[inline]
pub fn sample<R: Rng>(land: &Landscape, data: &mut Data, index: [usize; 3], rng: &mut R) {
    let num_pos_samples = land.sett.super_sampling().num_samples();
    let num_cast_samples = land.sett.caster().num_samples();
    let weight = 1.0 / num_pos_samples as f64;
    let voxel = land.grid.gen_voxel(&index);
    let sampler = land.sett.super_sampling();
    let caster = land.sett.caster();

    for n in 0..num_pos_samples {
        let pos = sampler.sample(&voxel, n, rng);

        for m in 0..num_cast_samples {
            let ray = caster.gen_ray(pos, m);

            if let Some(key) = observe(land.surfs, land.inters, ray) {
                *data
                    .maps
                    .mut_map()
                    .get_mut(&key)
                    .unwrap()
                    .get_mut(index)
                    .unwrap() += weight;
            }

            println!(
                "WARNING! Could not determine key at index: {} : {} : {}",
                index[X], index[Y], index[Z],
            );
        }
    }
}

/// Determine what a single ray will observe.
#[inline]
#[must_use]
pub fn observe(_surfs: &Set<Key, Mesh>, _inters: &Set<Key, Interface>, _ray: Ray) -> Option<Key> {
    None
}

//! Photon-lifetime engine function.

use crate::{output::Data, parts::Landscape, util::Key};
use arctk::geom::{Ray, Trace};
use arctk::ord::{X, Y, Z};
use rand::Rng;

/// Determine what a single ray will observe.
#[inline]
pub fn sample<R: Rng>(land: &Landscape, data: &mut Data, index: [usize; 3], rng: &mut R) {
    let bump_dist = land.sett.bump_dist();
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

            if let Some(key) = scan(land, ray, bump_dist) {
                *data
                    .maps
                    .mut_map()
                    .get_mut(key)
                    .unwrap()
                    .get_mut(index)
                    .unwrap() += weight;

                break;
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
pub fn scan<'a>(land: &'a Landscape, ray: Ray, bump_dist: f64) -> Option<&'a Key> {
    debug_assert!(bump_dist > 0.0);

    let bound_dist = land
        .grid
        .boundary()
        .dist(&ray)
        .expect("Failed to determine distance to grid boundary.");

    if let Some(hit) = land.tree.observe(ray, bump_dist, bound_dist) {
        let interface: &Key = hit.tag();
        let (inside, outside) = land.inters.map().get(interface).expect(&format!(
            "Entry '{}' not found in interface map.",
            interface
        ));

        if hit.side().is_inside() {
            Some(inside)
        } else {
            Some(outside)
        }
    } else {
        None
    }
}

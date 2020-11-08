//! Mapping simulation structure.

use arctk::{access, clone, geom::GridBuilder};
use arctk_attr::input;

/// Loadable input settings structure.
#[input]
pub struct Settings {
    /// Number of samples to simulate in each thread block.
    block_size: u64,
    /// Bump distance [m].
    bump_dist: f64,
    /// Optional super-sampling power.
    super_sample_power: Option<i32>,
    /// Grid settings.
    grid: GridBuilder,
}

impl Settings {
    clone!(block_size, u64);
    clone!(bump_dist, f64);
    clone!(super_sample_power, Option<i32>);
    access!(grid, GridBuilder);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(
        block_size: u64,
        bump_dist: f64,
        super_sample_power: Option<i32>,
        grid: GridBuilder,
    ) -> Self {
        debug_assert!(block_size > 0);
        debug_assert!(bump_dist > 0.0);
        debug_assert!(super_sample_power.is_none() || super_sample_power.unwrap() > 1);

        Self {
            block_size,
            bump_dist,
            super_sample_power,
            grid,
        }
    }

    /// Calculate the total number of sub-samples per cell.
    #[inline]
    #[must_use]
    pub fn samples_per_cell(&self) -> i32 {
        self.super_sample_power
            .map_or(1, |power| power * power * power)
    }

    /// Calculate the total number of sub-samples per cell.
    #[inline]
    #[must_use]
    pub fn total_samples(&self) -> usize {
        self.grid.total_cells() * self.samples_per_cell() as usize
    }
}

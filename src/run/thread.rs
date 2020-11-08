//! Simulation control functions.

use crate::{output::Data, parts::Landscape};
use arctk::{err::Error, tools::ProgressBar};
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

/// Run a multi-threaded cartography simulation.
/// # Errors
/// if the progress bad can not be locked.
#[allow(clippy::module_name_repetitions)]
#[allow(clippy::expect_used)]
#[inline]
pub fn multi_thread(land: &Landscape) -> Result<Data, Error> {
    let pb = ProgressBar::new("Multi-threaded", land.sett.total_samples() as u64);
    let pb = Arc::new(Mutex::new(pb));

    let threads: Vec<_> = (0..num_cpus::get()).collect();
    let mut out: Vec<_> = threads
        .par_iter()
        .map(|_id| thread(&Arc::clone(&pb), land))
        .collect();
    pb.lock()?.finish_with_message("Mapping complete.");

    let mut data = out.pop().expect("No data received.");
    while let Some(o) = out.pop() {
        data += &o;
    }

    Ok(data)
}

/// Run a cartography simulation using a single thread.
#[allow(clippy::module_name_repetitions)]
#[inline]
#[must_use]
pub fn single_thread(land: &Landscape) -> Data {
    let pb = ProgressBar::new("Single-threaded", land.sett.total_samples() as u64);
    let pb = Arc::new(Mutex::new(pb));

    thread(&pb, land)
}

/// Thread control function.
#[allow(clippy::module_name_repetitions)]
#[allow(clippy::expect_used)]
#[inline]
#[must_use]
pub fn thread(pb: &Arc<Mutex<ProgressBar>>, land: &Landscape) -> Data {
    // let res = *land.grid.res();
    let data = Data::new();
    // let mut data = Data::new(land.sett.grid.boundary().clone(), res);

    while let Some((start, end)) = {
        let mut pb = pb.lock().expect("Could not lock progress bar.");
        let b = pb.block(land.sett.block_size());
        std::mem::drop(pb);
        b
    } {
        for _i in start..end {
            // let ray =
            // let sample = super::engine::observe(land, ray);
        }
    }

    data
}

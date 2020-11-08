//! MCRT output structure.

use crate::{parts::Interface, util::Key};
use arctk::{
    err::Error,
    file::Save,
    ord::{Set, X, Y, Z},
};
use ndarray::Array3;
use std::ops::AddAssign;
use std::path::Path;

/// Cartographer output data.
pub struct Data {
    /// Center material sample.
    maps: Set<Key, Array3<f64>>,
}

impl Data {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(inters: &Set<Key, Interface>, res: [usize; 3]) -> Self {
        debug_assert!(res[X] > 0);
        debug_assert!(res[Y] > 0);
        debug_assert!(res[Z] > 0);

        let mut keys = Vec::new();
        for &(ref inside, ref outside) in inters.map().values() {
            keys.push(inside);
            keys.push(outside);
        }
        keys.sort();
        keys.dedup();

        let maps = Set::from_vec(
            keys.into_iter()
                .map(|key| (key.clone(), Array3::zeros(res)))
                .collect(),
        );

        Self { maps }
    }

    /// Save the maps to the given directory.
    /// # Errors
    /// if a map can not be saved.
    #[inline]
    pub fn save_maps(&self, out_dir: &Path) -> Result<(), Error> {
        for (key, map) in self.maps.map() {
            let p = out_dir.join(format!("{}.nc", key));
            println!("Saving: {}", p.display());
            map.save(&p)?;
        }

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

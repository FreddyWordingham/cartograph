//! Mapping simulation input structure.

use crate::{input::Settings, parts::Interface};
use arctk::{
    geom::{Mesh, Tree},
    ord::Set,
};

/// Rendering main input structure.
pub struct Landscape<'a, T: Ord> {
    /// Adaptive tree.
    pub tree: &'a Tree<'a, &'a T>,
    /// Engine settings.
    pub sett: &'a Settings,
    /// Surfaces.
    pub surfs: &'a Set<T, Mesh>,
    /// Interfaces.
    pub inters: &'a Set<T, Interface>,
}

impl<'a, T: Ord> Landscape<'a, T> {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(
        tree: &'a Tree<'a, &T>,
        sett: &'a Settings,
        surfs: &'a Set<T, Mesh>,
        inters: &'a Set<T, Interface>,
    ) -> Self {
        Self {
            tree,
            sett,
            surfs,
            inters,
        }
    }
}

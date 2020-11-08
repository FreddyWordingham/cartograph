//! Mapping simulation input structure.

use crate::{input::Settings, parts::Interface, util::Key};
use arctk::{
    geom::{Mesh, Tree},
    ord::Set,
};

/// Rendering main input structure.
pub struct Landscape<'a> {
    /// Adaptive tree.
    pub tree: &'a Tree<'a, &'a Key>,
    /// Engine settings.
    pub sett: &'a Settings,
    /// Surfaces.
    pub surfs: &'a Set<Key, Mesh>,
    /// Interfaces.
    pub inters: &'a Set<Key, Interface>,
}

impl<'a> Landscape<'a> {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub const fn new(
        tree: &'a Tree<'a, &Key>,
        sett: &'a Settings,
        surfs: &'a Set<Key, Mesh>,
        inters: &'a Set<Key, Interface>,
    ) -> Self {
        Self {
            tree,
            sett,
            surfs,
            inters,
        }
    }
}

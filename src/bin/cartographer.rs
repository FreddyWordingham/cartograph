//! Mapping program.

use arctk::{
    args,
    file::{Build, Load, Redirect, Save},
    geom::{Mesh, MeshBuilder, Tree, TreeBuilder},
    ord::Set,
    util::{banner, dir},
};
use arctk_attr::input;
use cartographer::{
    input::Settings,
    parts::{Interface, Landscape},
    run::multi_thread,
    util::Key,
};
use std::{
    env::current_dir,
    path::{Path, PathBuf},
};

// Input parameters.
#[input]
struct Parameters {
    /// Adaptive mesh settings.
    tree: Redirect<TreeBuilder>,
    /// Render runtime settings.
    sett: Redirect<Settings>,
    /// Surfaces set.
    surfs: Redirect<Set<Key, MeshBuilder>>,
    /// Interfaces map. (Inside material, outside material).
    inters: Redirect<Set<Key, Interface>>,
}

/// Main function.
pub fn main() {
    let term_width = arctk::util::term::width().unwrap_or(80);

    banner::title("CARTOGRAPHER", term_width);
    let (params_path, in_dir, out_dir) = init(term_width);
    let params = input(term_width, &in_dir, &params_path);
    let (tree_sett, map_sett, surfs, inters) = build(term_width, &in_dir, params);
    let tree = grow(term_width, tree_sett, &surfs);
    let input = Landscape::new(&tree, &map_sett, &surfs, &inters);

    banner::section("Mapping", term_width);
    let output = multi_thread(&input).expect("Failed to perform mapping.");
    output
        .save(&out_dir.join("render.png"))
        .expect("Failed to save output data.");

    banner::section("Finished", term_width);
}

/// Initialise the command line arguments and directories.
fn init(term_width: usize) -> (PathBuf, PathBuf, PathBuf) {
    banner::section("Initialisation", term_width);
    banner::sub_section("Command line arguments", term_width);
    args!(bin_path: PathBuf;
        params_path: PathBuf
    );
    println!("{:>32} : {}", "binary path", bin_path.display());
    println!("{:>32} : {}", "parameters path", params_path.display());

    banner::sub_section("Directories", term_width);
    let cwd = current_dir().expect("Failed to determine current working directory.");
    let (in_dir, out_dir) = dir::io_dirs(Some(cwd.join("input")), Some(cwd.join("output")))
        .expect("Failed to initialise directories.");
    println!("{:>32} : {}", "input directory", in_dir.display());
    println!("{:>32} : {}", "output directory", out_dir.display());

    (params_path, in_dir, out_dir)
}

/// Load the input files.
fn input(term_width: usize, in_dir: &Path, params_path: &Path) -> Parameters {
    banner::section("Input", term_width);
    banner::sub_section("Parameters", term_width);
    let path = in_dir.join(params_path);

    Parameters::load(&path).expect("Failed to load parameters file.")
}

/// Build instances.
#[allow(clippy::type_complexity)]
fn build(
    term_width: usize,
    in_dir: &Path,
    params: Parameters,
) -> (TreeBuilder, Settings, Set<Key, Mesh>, Set<Key, Interface>) {
    banner::section("Building", term_width);
    banner::sub_section("Adaptive Tree Settings", term_width);
    let tree_sett = params
        .tree
        .build(in_dir)
        .expect("Failed to redirect adaptive tree settings.");

    banner::sub_section("Map Settings", term_width);
    let map_sett = params
        .sett
        .build(in_dir)
        .expect("Failed to redirect map settings.");

    banner::sub_section("Surfaces", term_width);
    let surfs = params
        .surfs
        .build(in_dir)
        .expect("Failed to redirect surfaces set.")
        .build(in_dir)
        .expect("Failed to build surfaces set.");

    banner::sub_section("Interfaces", term_width);
    let inters = params
        .inters
        .build(in_dir)
        .expect("Failed to redirect interfaces set.");

    (tree_sett, map_sett, surfs, inters)
}

/// Grow domains.
fn grow<'a>(term_width: usize, tree: TreeBuilder, surfs: &'a Set<Key, Mesh>) -> Tree<'a, &Key> {
    banner::section("Growing", term_width);

    banner::sub_section("Adaptive Tree", term_width);
    let tree = tree.build(&surfs);

    tree
}

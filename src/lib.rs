// Chemfiles, a modern library for chemistry file reading and writing
// Copyright (C) 2015-2018 Guillaume Fraux -- BSD licensed

//! Chemfiles is a multi-language library written in modern C++ for reading and
//! writing from and to molecular trajectory files. These files are created by
//! your favorite theoretical chemistry program, and contains information about
//! atomic or residues names and positions. Some format also have additional
//! information, such as velocities, forces, energy, …
//!
//! This crate expose the C API of chemfiles to Rust, and make all the
//! functionalities accessible. For more information on the C++ library,
//! please see its [documentation][cxx_doc]. Specifically, the following pages
//! are worth reading:
//!
//! - The [overview][overview] of the classes organization;
//! - The list of [supported formats][formats];
//! - The documentation for the [selection language][selections];
//!
//! [cxx_doc]: https://chemfiles.org/chemfiles
//! [overview]: https://chemfiles.org/chemfiles/latest/overview.html
//! [formats]: https://chemfiles.org/chemfiles/latest/formats.html
//! [selections]: https://chemfiles.org/chemfiles/latest/selections.html

#![deny(missing_docs)]
#![warn(trivial_casts, unused_import_braces, variant_size_differences)]
#![warn(unused_results)]
// Configuration for clippy lints
#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::needless_return, clippy::module_name_repetitions)]
#![allow(clippy::must_use_candidate, clippy::wildcard_imports)]
// Allow a few more clippy lints in test mode
#![cfg_attr(test, allow(clippy::float_cmp, clippy::unreadable_literal, clippy::shadow_unrelated))]
// deny(warnings) in doc tests
#![doc(test(attr(deny(warnings))))]
#![doc(test(attr(allow(unused_variables))))]

use chemfiles_sys::{chfl_add_configuration, chfl_version};

mod strings;

mod errors;
pub use self::errors::set_warning_callback;
pub use self::errors::{Error, Status};

mod atom;
pub use self::atom::Atom;
pub use self::atom::AtomMut;
pub use self::atom::AtomRef;

mod cell;
pub use self::cell::CellShape;
pub use self::cell::UnitCell;
pub use self::cell::UnitCellMut;
pub use self::cell::UnitCellRef;

mod residue;
pub use self::residue::Residue;
pub use self::residue::ResidueRef;

mod topology;
pub use self::topology::BondOrder;
pub use self::topology::Topology;
pub use self::topology::TopologyRef;

mod frame;
pub use self::frame::Frame;

mod trajectory;
pub use self::trajectory::Trajectory;

mod selection;
pub use self::selection::{Match, Selection};

mod property;
pub use self::property::PropertiesIter;
pub use self::property::Property;

mod misc;
pub use self::misc::{formats_list, guess_format, FormatMetadata};

/// Get the version of the chemfiles library.
///
/// # Example
/// ```
/// let version = chemfiles::version();
/// assert!(version.starts_with("0.10"));
/// ```
pub fn version() -> String {
    unsafe { strings::from_c(chfl_version()) }
}

/// Read configuration data from the file at `path`.
///
/// By default, chemfiles reads configuration from any file named
/// `.chemfiles.toml` in the current directory or any parent directory. This
/// function can be used to add data from another configuration file. Data from
/// the new configuration file will overwrite any existing data.
///
/// # Errors
///
/// This function will fail if there is no file at `path`, or if the file is
/// incorrectly formatted.
///
/// # Example
/// ```no_run
/// chemfiles::add_configuration("local-config.toml").unwrap();
/// // from now on, the data from "local-config.toml" will be used
/// ```
pub fn add_configuration<S>(path: S) -> Result<(), Error>
where
    S: AsRef<str>,
{
    let buffer = strings::to_c(path.as_ref());
    unsafe { errors::check(chfl_add_configuration(buffer.as_ptr())) }
}

#[cfg(test)]
mod tests {
    #[test]
    fn version() {
        assert!(!crate::version().is_empty());
        assert!(crate::version().starts_with("0.10"));
    }
}

#[cfg(test)]
fn assert_vector3d_eq(lhs: &[f64; 3], rhs: &[f64; 3], eps: f64) {
    lhs.iter()
        .zip(rhs)
        .for_each(|(l, r)| approx::assert_ulps_eq!(l, r, epsilon = eps));
}

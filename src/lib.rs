// Chemfiles, a modern library for chemistry file reading and writing
// Copyright (C) 2015-2017 Guillaume Fraux
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Chemfiles is a multi-language library written in modern C++ for reading and
//! writing from and to molecular trajectory files. These files are created by
//! your favorite theoretical chemistry program, and contains informations about
//! atomic or residues names and positions. Some format also have additional
//! informations, such as velocities, forces, energy, …
//!
//! This crate expose the C API of chemfiles to Rust, and make all the
//! functionalities accessibles. For more informations on the C++ library,
//! please see its [documentation][cxx_doc]. Specifically, the following pages
//! are worth reading:
//!
//! - The [overview][overview] of the classes organisation;
//! - The lisf of [supported formats][formats];
//! - The documentation for the [selection language][selections];
//!
//! As all the function call the underlying C library, they all can fail and
//! thus all return a `Result<_, Error>` value.
//!
//! [cxx_doc]: https://chemfiles.org/chemfiles
//! [overview]: https://chemfiles.org/chemfiles/latest/overview.html
//! [formats]: https://chemfiles.org/chemfiles/latest/formats.html
//! [selections]: https://chemfiles.org/chemfiles/latest/selections.html
#![deny(missing_docs)]

#![warn(
    trivial_casts, unused_import_braces, variant_size_differences,
    unused_qualifications, unused_results
)]

#![warn(clippy, clippy_pedantic)]
#![allow(unknown_lints)]
// List of Clippy lints we allow in this code
#![allow(
    needless_return, shadow_reuse, stutter, missing_docs_in_private_items,
    zero_ptr, cast_possible_truncation, or_fun_call
)]

#[cfg(test)]
#[macro_use]
extern crate approx;

extern crate chemfiles_sys;
use chemfiles_sys::chfl_version;

mod strings;

mod errors;
pub use errors::{Error, Status};
pub use errors::set_warning_callback;

/// Custom result type for working with errors in chemfiles
pub type Result<T> = std::result::Result<T, Error>;

mod atom;
pub use atom::Atom;

mod cell;
pub use cell::UnitCell;
pub use cell::CellShape;

mod residue;
pub use residue::Residue;

mod topology;
pub use topology::Topology;

mod frame;
pub use frame::Frame;

mod trajectory;
pub use trajectory::Trajectory;

mod selection;
pub use selection::{Selection, Match};

/// Get the version of the chemfiles library.
pub fn version() -> String {
    unsafe {
        strings::from_c(chfl_version())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn version() {
        assert!(::version().len() > 0);
    }
}

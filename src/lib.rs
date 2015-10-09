/*
 * Chemharp, an efficient IO library for chemistry file formats
 * Copyright (C) 2015 Guillaume Fraux
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/
*/

//! Chemharp is a multi-language library written in modern C++ for reading and
//! writing from and to molecular trajectory files. These files are created by
//! your favorite theoretical chemistry program, and contains informations about
//! atomic or residues names and positions. Some format also have additional
//! informations, such as velocities, forces, energy, …
//!
//! This crate expose the C API of chemharp to Rust, and make all the
//! functionalities accessibles. For more informations on the C++ library,
//! please see its [documentation](http://chemharp.rtfd.org). Specifically, the
//! following pages are worth reading:
//!
//! - The [overview](http://chemharp.rtfd.org/en/latest/overview.html) of the
//!   classes organisation;
//! - The [supported formats](http://chemharp.rtfd.org/en/latest/formats.html);
//!
//!
//! As all the function call the underlying C library, they all can fail and
//! thus all return a `Result<_, Error>` value.
#[macro_use]
mod tests;

mod string;

mod errors;
pub use errors::Error;

pub mod logging;

mod atom;
pub use atom::Atom;
pub use atom::AtomType;

mod cell;
pub use cell::UnitCell;
pub use cell::CellType;

mod topology;
pub use topology::Topology;

mod frame;
pub use frame::Frame;

mod trajectory;
pub use trajectory::Trajectory;

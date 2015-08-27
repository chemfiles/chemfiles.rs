/*
 * Chemharp, an efficient IO library for chemistry file formats
 * Copyright (C) 2015 Guillaume Fraux
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/
*/

#[allow(dead_code, non_camel_case_types)]
mod ffi;

#[macro_use]
mod tests;

mod string;

mod errors;
pub use errors::Error;
pub use errors::LogLevel;

mod atom;
pub use atom::Atom;

mod cell;
pub use cell::UnitCell;

mod topology;
pub use topology::Topology;

mod frame;
pub use frame::Frame;

mod trajectory;
pub use trajectory::Trajectory;

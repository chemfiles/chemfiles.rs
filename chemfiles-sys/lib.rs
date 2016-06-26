// Chemfiles, an efficient IO library for chemistry file formats
// Copyright (C) 2015 Guillaume Fraux
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/
//
// ========================================================================= //
//                       !!!! AUTO-GENERATED FILE !!!!
// Do not edit. See the bindgen repository for the generation code
//                   https://github.com/chemfiles/bindgen
// ========================================================================= //

#![allow(non_camel_case_types)]
extern crate libc;
use libc::{c_float, c_double, c_char, c_int, c_uint, size_t};

pub enum CHFL_TRAJECTORY{}
pub enum CHFL_CELL{}
pub enum CHFL_ATOM{}
pub enum CHFL_FRAME{}
pub enum CHFL_TOPOLOGY{}
pub enum CHFL_SELECTION{}

// C enum CHFL_LOG_LEVEL
pub type CHFL_LOG_LEVEL = c_uint;
pub const CHFL_LOG_ERROR: CHFL_LOG_LEVEL = 0;
pub const CHFL_LOG_WARNING: CHFL_LOG_LEVEL = 1;
pub const CHFL_LOG_INFO: CHFL_LOG_LEVEL = 2;
pub const CHFL_LOG_DEBUG: CHFL_LOG_LEVEL = 3;

// C enum CHFL_CELL_TYPES
pub type CHFL_CELL_TYPES = c_uint;
pub const CHFL_CELL_ORTHORHOMBIC: CHFL_CELL_TYPES = 0;
pub const CHFL_CELL_TRICLINIC: CHFL_CELL_TYPES = 1;
pub const CHFL_CELL_INFINITE: CHFL_CELL_TYPES = 2;

// C enum CHFL_ATOM_TYPES
pub type CHFL_ATOM_TYPES = c_uint;
pub const CHFL_ATOM_ELEMENT: CHFL_ATOM_TYPES = 0;
pub const CHFL_ATOM_COARSE_GRAINED: CHFL_ATOM_TYPES = 1;
pub const CHFL_ATOM_DUMMY: CHFL_ATOM_TYPES = 2;
pub const CHFL_ATOM_UNDEFINED: CHFL_ATOM_TYPES = 3;

pub type CHFL_STATUS = c_int;
pub type chfl_logging_callback_t = extern fn(CHFL_LOG_LEVEL, *const c_char);
pub type c_bool = u8;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct chfl_match_t {
    pub size: c_char,
    pub atoms: [size_t; 4],
}

// TODO: use an enum here
pub const CHFL_SUCCESS: CHFL_STATUS = 0;
pub const CHFL_MEMORY_ERROR: CHFL_STATUS = 1;
pub const CHFL_FILE_ERROR: CHFL_STATUS = 2;
pub const CHFL_FORMAT_ERROR: CHFL_STATUS = 3;
pub const CHFL_SELECTION_ERROR: CHFL_STATUS = 4;
pub const CHFL_GENERIC_ERROR: CHFL_STATUS = 5;
pub const CHFL_CXX_ERROR: CHFL_STATUS = 6;

#[link(name="chemfiles", kind="static")]
extern "C" {
    pub fn chfl_version() -> *const c_char;
    pub fn chfl_strerror(status: c_int) -> *const c_char;
    pub fn chfl_last_error() -> *const c_char;
    pub fn chfl_clear_errors() -> CHFL_STATUS;
    pub fn chfl_loglevel(level: *mut CHFL_LOG_LEVEL) -> CHFL_STATUS;
    pub fn chfl_set_loglevel(level: CHFL_LOG_LEVEL) -> CHFL_STATUS;
    pub fn chfl_logfile(file: *const c_char) -> CHFL_STATUS;
    pub fn chfl_log_stdout() -> CHFL_STATUS;
    pub fn chfl_log_stderr() -> CHFL_STATUS;
    pub fn chfl_log_silent() -> CHFL_STATUS;
    pub fn chfl_log_callback(callback: chfl_logging_callback_t) -> CHFL_STATUS;
    pub fn chfl_trajectory_open(filename: *const c_char, mode: c_char) -> *mut CHFL_TRAJECTORY;
    pub fn chfl_trajectory_with_format(filename: *const c_char, mode: c_char, format: *const c_char) -> *mut CHFL_TRAJECTORY;
    pub fn chfl_trajectory_read(file: *mut CHFL_TRAJECTORY, frame: *mut CHFL_FRAME) -> CHFL_STATUS;
    pub fn chfl_trajectory_read_step(file: *mut CHFL_TRAJECTORY, step: size_t, frame: *mut CHFL_FRAME) -> CHFL_STATUS;
    pub fn chfl_trajectory_write(file: *mut CHFL_TRAJECTORY, frame: *const CHFL_FRAME) -> CHFL_STATUS;
    pub fn chfl_trajectory_set_topology(file: *mut CHFL_TRAJECTORY, topology: *const CHFL_TOPOLOGY) -> CHFL_STATUS;
    pub fn chfl_trajectory_set_topology_file(file: *mut CHFL_TRAJECTORY, filename: *const c_char) -> CHFL_STATUS;
    pub fn chfl_trajectory_set_topology_with_format(file: *mut CHFL_TRAJECTORY, filename: *const c_char, format: *const c_char) -> CHFL_STATUS;
    pub fn chfl_trajectory_set_cell(file: *mut CHFL_TRAJECTORY, cell: *const CHFL_CELL) -> CHFL_STATUS;
    pub fn chfl_trajectory_nsteps(file: *mut CHFL_TRAJECTORY, nsteps: *mut size_t) -> CHFL_STATUS;
    pub fn chfl_trajectory_sync(file: *mut CHFL_TRAJECTORY) -> CHFL_STATUS;
    pub fn chfl_trajectory_close(file: *mut CHFL_TRAJECTORY) -> CHFL_STATUS;
    pub fn chfl_frame(natoms: size_t) -> *mut CHFL_FRAME;
    pub fn chfl_frame_atoms_count(frame: *const CHFL_FRAME, natoms: *mut size_t) -> CHFL_STATUS;
    pub fn chfl_frame_positions(frame: *mut CHFL_FRAME, data: *mut *mut [c_float; 3], size: *mut size_t) -> CHFL_STATUS;
    pub fn chfl_frame_velocities(frame: *mut CHFL_FRAME, data: *mut *mut [c_float; 3], size: *mut size_t) -> CHFL_STATUS;
    pub fn chfl_frame_resize(frame: *mut CHFL_FRAME, natoms: size_t) -> CHFL_STATUS;
    pub fn chfl_frame_add_velocities(frame: *mut CHFL_FRAME) -> CHFL_STATUS;
    pub fn chfl_frame_has_velocities(frame: *const CHFL_FRAME, has_velocities: *mut c_bool) -> CHFL_STATUS;
    pub fn chfl_frame_set_cell(frame: *mut CHFL_FRAME, cell: *const CHFL_CELL) -> CHFL_STATUS;
    pub fn chfl_frame_set_topology(frame: *mut CHFL_FRAME, topology: *const CHFL_TOPOLOGY) -> CHFL_STATUS;
    pub fn chfl_frame_step(frame: *const CHFL_FRAME, step: *mut size_t) -> CHFL_STATUS;
    pub fn chfl_frame_set_step(frame: *mut CHFL_FRAME, step: size_t) -> CHFL_STATUS;
    pub fn chfl_frame_guess_topology(frame: *mut CHFL_FRAME) -> CHFL_STATUS;
    pub fn chfl_frame_free(frame: *mut CHFL_FRAME) -> CHFL_STATUS;
    pub fn chfl_cell(a: c_double, b: c_double, c: c_double) -> *mut CHFL_CELL;
    pub fn chfl_cell_triclinic(a: c_double, b: c_double, c: c_double, alpha: c_double, beta: c_double, gamma: c_double) -> *mut CHFL_CELL;
    pub fn chfl_cell_from_frame(frame: *const CHFL_FRAME) -> *mut CHFL_CELL;
    pub fn chfl_cell_volume(cell: *const CHFL_CELL, V: *mut c_double) -> CHFL_STATUS;
    pub fn chfl_cell_lengths(cell: *const CHFL_CELL, a: *mut c_double, b: *mut c_double, c: *mut c_double) -> CHFL_STATUS;
    pub fn chfl_cell_set_lengths(cell: *mut CHFL_CELL, a: c_double, b: c_double, c: c_double) -> CHFL_STATUS;
    pub fn chfl_cell_angles(cell: *const CHFL_CELL, alpha: *mut c_double, beta: *mut c_double, gamma: *mut c_double) -> CHFL_STATUS;
    pub fn chfl_cell_set_angles(cell: *mut CHFL_CELL, alpha: c_double, beta: c_double, gamma: c_double) -> CHFL_STATUS;
    pub fn chfl_cell_matrix(cell: *const CHFL_CELL, matrix: *mut [c_double; 3]) -> CHFL_STATUS;
    pub fn chfl_cell_type(cell: *const CHFL_CELL, _type: *mut CHFL_CELL_TYPES) -> CHFL_STATUS;
    pub fn chfl_cell_set_type(cell: *mut CHFL_CELL, _type: CHFL_CELL_TYPES) -> CHFL_STATUS;
    pub fn chfl_cell_free(cell: *mut CHFL_CELL) -> CHFL_STATUS;
    pub fn chfl_topology() -> *mut CHFL_TOPOLOGY;
    pub fn chfl_topology_from_frame(frame: *const CHFL_FRAME) -> *mut CHFL_TOPOLOGY;
    pub fn chfl_topology_atoms_count(topology: *const CHFL_TOPOLOGY, natoms: *mut size_t) -> CHFL_STATUS;
    pub fn chfl_topology_append(topology: *mut CHFL_TOPOLOGY, atom: *const CHFL_ATOM) -> CHFL_STATUS;
    pub fn chfl_topology_remove(topology: *mut CHFL_TOPOLOGY, i: size_t) -> CHFL_STATUS;
    pub fn chfl_topology_isbond(topology: *const CHFL_TOPOLOGY, i: size_t, j: size_t, result: *mut c_bool) -> CHFL_STATUS;
    pub fn chfl_topology_isangle(topology: *const CHFL_TOPOLOGY, i: size_t, j: size_t, k: size_t, result: *mut c_bool) -> CHFL_STATUS;
    pub fn chfl_topology_isdihedral(topology: *const CHFL_TOPOLOGY, i: size_t, j: size_t, k: size_t, m: size_t, result: *mut c_bool) -> CHFL_STATUS;
    pub fn chfl_topology_bonds_count(topology: *const CHFL_TOPOLOGY, nbonds: *mut size_t) -> CHFL_STATUS;
    pub fn chfl_topology_angles_count(topology: *const CHFL_TOPOLOGY, nangles: *mut size_t) -> CHFL_STATUS;
    pub fn chfl_topology_dihedrals_count(topology: *const CHFL_TOPOLOGY, ndihedrals: *mut size_t) -> CHFL_STATUS;
    pub fn chfl_topology_bonds(topology: *const CHFL_TOPOLOGY, data: *mut [size_t; 2], nbonds: size_t) -> CHFL_STATUS;
    pub fn chfl_topology_angles(topology: *const CHFL_TOPOLOGY, data: *mut [size_t; 3], nangles: size_t) -> CHFL_STATUS;
    pub fn chfl_topology_dihedrals(topology: *const CHFL_TOPOLOGY, data: *mut [size_t; 4], ndihedrals: size_t) -> CHFL_STATUS;
    pub fn chfl_topology_add_bond(topology: *mut CHFL_TOPOLOGY, i: size_t, j: size_t) -> CHFL_STATUS;
    pub fn chfl_topology_remove_bond(topology: *mut CHFL_TOPOLOGY, i: size_t, j: size_t) -> CHFL_STATUS;
    pub fn chfl_topology_free(topology: *mut CHFL_TOPOLOGY) -> CHFL_STATUS;
    pub fn chfl_atom(name: *const c_char) -> *mut CHFL_ATOM;
    pub fn chfl_atom_from_frame(frame: *const CHFL_FRAME, idx: size_t) -> *mut CHFL_ATOM;
    pub fn chfl_atom_from_topology(topology: *const CHFL_TOPOLOGY, idx: size_t) -> *mut CHFL_ATOM;
    pub fn chfl_atom_mass(atom: *const CHFL_ATOM, mass: *mut c_float) -> CHFL_STATUS;
    pub fn chfl_atom_set_mass(atom: *mut CHFL_ATOM, mass: c_float) -> CHFL_STATUS;
    pub fn chfl_atom_charge(atom: *const CHFL_ATOM, charge: *mut c_float) -> CHFL_STATUS;
    pub fn chfl_atom_set_charge(atom: *mut CHFL_ATOM, charge: c_float) -> CHFL_STATUS;
    pub fn chfl_atom_name(atom: *const CHFL_ATOM, name: *const c_char, buffsize: size_t) -> CHFL_STATUS;
    pub fn chfl_atom_set_name(atom: *mut CHFL_ATOM, name: *const c_char) -> CHFL_STATUS;
    pub fn chfl_atom_full_name(atom: *const CHFL_ATOM, name: *const c_char, buffsize: size_t) -> CHFL_STATUS;
    pub fn chfl_atom_vdw_radius(atom: *const CHFL_ATOM, radius: *mut c_double) -> CHFL_STATUS;
    pub fn chfl_atom_covalent_radius(atom: *const CHFL_ATOM, radius: *mut c_double) -> CHFL_STATUS;
    pub fn chfl_atom_atomic_number(atom: *const CHFL_ATOM, number: *mut c_int) -> CHFL_STATUS;
    pub fn chfl_atom_type(atom: *const CHFL_ATOM, _type: *mut CHFL_ATOM_TYPES) -> CHFL_STATUS;
    pub fn chfl_atom_set_type(atom: *mut CHFL_ATOM, _type: CHFL_ATOM_TYPES) -> CHFL_STATUS;
    pub fn chfl_atom_free(atom: *mut CHFL_ATOM) -> CHFL_STATUS;
    pub fn chfl_selection(selection: *const c_char) -> *mut CHFL_SELECTION;
    pub fn chfl_selection_size(selection: *const CHFL_SELECTION, size: *mut size_t) -> CHFL_STATUS;
    pub fn chfl_selection_evalutate(selection: *mut CHFL_SELECTION, frame: *const CHFL_FRAME, n_matches: *mut size_t) -> CHFL_STATUS;
    pub fn chfl_selection_matches(selection: *const CHFL_SELECTION, matches: *mut chfl_match_t, n_matches: size_t) -> CHFL_STATUS;
    pub fn chfl_selection_free(selection: *mut CHFL_SELECTION) -> CHFL_STATUS;
}

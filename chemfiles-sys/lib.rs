// Chemfiles, a modern library for chemistry file reading and writing
// Copyright (C) 2015-2017 Guillaume Fraux
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
use libc::{c_double, c_char, uint64_t, int64_t};

// Manual definitions. Edit the bindgen code to make sure this matches the
// chemfiles.h header
pub type c_bool = u8;
pub type chfl_warning_callback = extern fn(*const c_char);

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct chfl_match_t {
    pub size: uint64_t,
    pub atoms: [uint64_t; 4],
}
// End manual definitions

pub enum CHFL_TRAJECTORY{}
pub enum CHFL_CELL{}
pub enum CHFL_ATOM{}
pub enum CHFL_FRAME{}
pub enum CHFL_TOPOLOGY{}
pub enum CHFL_SELECTION{}
pub enum CHFL_RESIDUE{}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum chfl_status {
    CHFL_SUCCESS = 0,
    CHFL_MEMORY_ERROR = 1,
    CHFL_FILE_ERROR = 2,
    CHFL_FORMAT_ERROR = 3,
    CHFL_SELECTION_ERROR = 4,
    CHFL_GENERIC_ERROR = 5,
    CHFL_CXX_ERROR = 6,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum chfl_cell_shape_t {
    CHFL_CELL_ORTHORHOMBIC = 0,
    CHFL_CELL_TRICLINIC = 1,
    CHFL_CELL_INFINITE = 2,
}

#[link(name="chemfiles", kind="static")]
extern "C" {
    pub fn chfl_version() -> *const c_char;
    pub fn chfl_last_error() -> *const c_char;
    pub fn chfl_clear_errors() -> chfl_status;
    pub fn chfl_set_warning_callback(callback: chfl_warning_callback) -> chfl_status;
    pub fn chfl_atom(name: *const c_char) -> *mut CHFL_ATOM;
    pub fn chfl_atom_copy(atom: *const CHFL_ATOM) -> *mut CHFL_ATOM;
    pub fn chfl_atom_from_frame(frame: *const CHFL_FRAME, i: uint64_t) -> *mut CHFL_ATOM;
    pub fn chfl_atom_from_topology(topology: *const CHFL_TOPOLOGY, i: uint64_t) -> *mut CHFL_ATOM;
    pub fn chfl_atom_mass(atom: *const CHFL_ATOM, mass: *mut c_double) -> chfl_status;
    pub fn chfl_atom_set_mass(atom: *mut CHFL_ATOM, mass: c_double) -> chfl_status;
    pub fn chfl_atom_charge(atom: *const CHFL_ATOM, charge: *mut c_double) -> chfl_status;
    pub fn chfl_atom_set_charge(atom: *mut CHFL_ATOM, charge: c_double) -> chfl_status;
    pub fn chfl_atom_type(atom: *const CHFL_ATOM, _type: *mut c_char, buffsize: uint64_t) -> chfl_status;
    pub fn chfl_atom_set_type(atom: *mut CHFL_ATOM, _type: *const c_char) -> chfl_status;
    pub fn chfl_atom_name(atom: *const CHFL_ATOM, name: *mut c_char, buffsize: uint64_t) -> chfl_status;
    pub fn chfl_atom_set_name(atom: *mut CHFL_ATOM, name: *const c_char) -> chfl_status;
    pub fn chfl_atom_full_name(atom: *const CHFL_ATOM, name: *mut c_char, buffsize: uint64_t) -> chfl_status;
    pub fn chfl_atom_vdw_radius(atom: *const CHFL_ATOM, radius: *mut c_double) -> chfl_status;
    pub fn chfl_atom_covalent_radius(atom: *const CHFL_ATOM, radius: *mut c_double) -> chfl_status;
    pub fn chfl_atom_atomic_number(atom: *const CHFL_ATOM, number: *mut int64_t) -> chfl_status;
    pub fn chfl_atom_free(atom: *mut CHFL_ATOM) -> chfl_status;
    pub fn chfl_residue(name: *const c_char, resid: uint64_t) -> *mut CHFL_RESIDUE;
    pub fn chfl_residue_from_topology(topology: *const CHFL_TOPOLOGY, i: uint64_t) -> *mut CHFL_RESIDUE;
    pub fn chfl_residue_for_atom(topology: *const CHFL_TOPOLOGY, i: uint64_t) -> *mut CHFL_RESIDUE;
    pub fn chfl_residue_copy(residue: *const CHFL_RESIDUE) -> *mut CHFL_RESIDUE;
    pub fn chfl_residue_atoms_count(residue: *const CHFL_RESIDUE, size: *mut uint64_t) -> chfl_status;
    pub fn chfl_residue_id(residue: *const CHFL_RESIDUE, id: *mut uint64_t) -> chfl_status;
    pub fn chfl_residue_name(residue: *const CHFL_RESIDUE, name: *mut c_char, buffsize: uint64_t) -> chfl_status;
    pub fn chfl_residue_add_atom(residue: *mut CHFL_RESIDUE, i: uint64_t) -> chfl_status;
    pub fn chfl_residue_contains(residue: *const CHFL_RESIDUE, i: uint64_t, result: *mut c_bool) -> chfl_status;
    pub fn chfl_residue_free(residue: *mut CHFL_RESIDUE) -> chfl_status;
    pub fn chfl_topology() -> *mut CHFL_TOPOLOGY;
    pub fn chfl_topology_from_frame(frame: *const CHFL_FRAME) -> *mut CHFL_TOPOLOGY;
    pub fn chfl_topology_copy(topology: *const CHFL_TOPOLOGY) -> *mut CHFL_TOPOLOGY;
    pub fn chfl_topology_atoms_count(topology: *const CHFL_TOPOLOGY, natoms: *mut uint64_t) -> chfl_status;
    pub fn chfl_topology_resize(topology: *mut CHFL_TOPOLOGY, natoms: uint64_t) -> chfl_status;
    pub fn chfl_topology_add_atom(topology: *mut CHFL_TOPOLOGY, atom: *const CHFL_ATOM) -> chfl_status;
    pub fn chfl_topology_remove(topology: *mut CHFL_TOPOLOGY, i: uint64_t) -> chfl_status;
    pub fn chfl_topology_isbond(topology: *const CHFL_TOPOLOGY, i: uint64_t, j: uint64_t, result: *mut c_bool) -> chfl_status;
    pub fn chfl_topology_isangle(topology: *const CHFL_TOPOLOGY, i: uint64_t, j: uint64_t, k: uint64_t, result: *mut c_bool) -> chfl_status;
    pub fn chfl_topology_isdihedral(topology: *const CHFL_TOPOLOGY, i: uint64_t, j: uint64_t, k: uint64_t, m: uint64_t, result: *mut c_bool) -> chfl_status;
    pub fn chfl_topology_bonds_count(topology: *const CHFL_TOPOLOGY, nbonds: *mut uint64_t) -> chfl_status;
    pub fn chfl_topology_angles_count(topology: *const CHFL_TOPOLOGY, nangles: *mut uint64_t) -> chfl_status;
    pub fn chfl_topology_dihedrals_count(topology: *const CHFL_TOPOLOGY, ndihedrals: *mut uint64_t) -> chfl_status;
    pub fn chfl_topology_bonds(topology: *const CHFL_TOPOLOGY, data: *mut [uint64_t; 2], nbonds: uint64_t) -> chfl_status;
    pub fn chfl_topology_angles(topology: *const CHFL_TOPOLOGY, data: *mut [uint64_t; 3], nangles: uint64_t) -> chfl_status;
    pub fn chfl_topology_dihedrals(topology: *const CHFL_TOPOLOGY, data: *mut [uint64_t; 4], ndihedrals: uint64_t) -> chfl_status;
    pub fn chfl_topology_add_bond(topology: *mut CHFL_TOPOLOGY, i: uint64_t, j: uint64_t) -> chfl_status;
    pub fn chfl_topology_remove_bond(topology: *mut CHFL_TOPOLOGY, i: uint64_t, j: uint64_t) -> chfl_status;
    pub fn chfl_topology_residues_count(topology: *const CHFL_TOPOLOGY, nresidues: *mut uint64_t) -> chfl_status;
    pub fn chfl_topology_add_residue(topology: *mut CHFL_TOPOLOGY, residue: *const CHFL_RESIDUE) -> chfl_status;
    pub fn chfl_topology_residues_linked(topology: *const CHFL_TOPOLOGY, first: *const CHFL_RESIDUE, second: *const CHFL_RESIDUE, result: *mut c_bool) -> chfl_status;
    pub fn chfl_topology_free(topology: *mut CHFL_TOPOLOGY) -> chfl_status;
    pub fn chfl_cell(lenghts: *const c_double) -> *mut CHFL_CELL;
    pub fn chfl_cell_triclinic(lenghts: *const c_double, angles: *const c_double) -> *mut CHFL_CELL;
    pub fn chfl_cell_from_frame(frame: *const CHFL_FRAME) -> *mut CHFL_CELL;
    pub fn chfl_cell_copy(cell: *const CHFL_CELL) -> *mut CHFL_CELL;
    pub fn chfl_cell_volume(cell: *const CHFL_CELL, volume: *mut c_double) -> chfl_status;
    pub fn chfl_cell_lengths(cell: *const CHFL_CELL, lengths: *mut c_double) -> chfl_status;
    pub fn chfl_cell_set_lengths(cell: *mut CHFL_CELL, lenghts: *const c_double) -> chfl_status;
    pub fn chfl_cell_angles(cell: *const CHFL_CELL, angles: *mut c_double) -> chfl_status;
    pub fn chfl_cell_set_angles(cell: *mut CHFL_CELL, angles: *const c_double) -> chfl_status;
    pub fn chfl_cell_matrix(cell: *const CHFL_CELL, matrix: *mut [c_double; 3]) -> chfl_status;
    pub fn chfl_cell_shape(cell: *const CHFL_CELL, shape: *mut chfl_cell_shape_t) -> chfl_status;
    pub fn chfl_cell_set_shape(cell: *mut CHFL_CELL, shape: chfl_cell_shape_t) -> chfl_status;
    pub fn chfl_cell_free(cell: *mut CHFL_CELL) -> chfl_status;
    pub fn chfl_frame() -> *mut CHFL_FRAME;
    pub fn chfl_frame_copy(frame: *const CHFL_FRAME) -> *mut CHFL_FRAME;
    pub fn chfl_frame_atoms_count(frame: *const CHFL_FRAME, natoms: *mut uint64_t) -> chfl_status;
    pub fn chfl_frame_positions(frame: *mut CHFL_FRAME, positions: *mut *mut [c_double; 3], size: *mut uint64_t) -> chfl_status;
    pub fn chfl_frame_velocities(frame: *mut CHFL_FRAME, velocities: *mut *mut [c_double; 3], size: *mut uint64_t) -> chfl_status;
    pub fn chfl_frame_add_atom(frame: *mut CHFL_FRAME, atom: *const CHFL_ATOM, position: *const c_double, velocity: *const c_double) -> chfl_status;
    pub fn chfl_frame_remove(frame: *mut CHFL_FRAME, i: uint64_t) -> chfl_status;
    pub fn chfl_frame_resize(frame: *mut CHFL_FRAME, natoms: uint64_t) -> chfl_status;
    pub fn chfl_frame_add_velocities(frame: *mut CHFL_FRAME) -> chfl_status;
    pub fn chfl_frame_has_velocities(frame: *const CHFL_FRAME, has_velocities: *mut c_bool) -> chfl_status;
    pub fn chfl_frame_set_cell(frame: *mut CHFL_FRAME, cell: *const CHFL_CELL) -> chfl_status;
    pub fn chfl_frame_set_topology(frame: *mut CHFL_FRAME, topology: *const CHFL_TOPOLOGY) -> chfl_status;
    pub fn chfl_frame_step(frame: *const CHFL_FRAME, step: *mut uint64_t) -> chfl_status;
    pub fn chfl_frame_set_step(frame: *mut CHFL_FRAME, step: uint64_t) -> chfl_status;
    pub fn chfl_frame_guess_topology(frame: *mut CHFL_FRAME) -> chfl_status;
    pub fn chfl_frame_free(frame: *mut CHFL_FRAME) -> chfl_status;
    pub fn chfl_trajectory_open(path: *const c_char, mode: c_char) -> *mut CHFL_TRAJECTORY;
    pub fn chfl_trajectory_with_format(path: *const c_char, mode: c_char, format: *const c_char) -> *mut CHFL_TRAJECTORY;
    pub fn chfl_trajectory_read(trajectory: *mut CHFL_TRAJECTORY, frame: *mut CHFL_FRAME) -> chfl_status;
    pub fn chfl_trajectory_read_step(trajectory: *mut CHFL_TRAJECTORY, step: uint64_t, frame: *mut CHFL_FRAME) -> chfl_status;
    pub fn chfl_trajectory_write(trajectory: *mut CHFL_TRAJECTORY, frame: *const CHFL_FRAME) -> chfl_status;
    pub fn chfl_trajectory_set_topology(trajectory: *mut CHFL_TRAJECTORY, topology: *const CHFL_TOPOLOGY) -> chfl_status;
    pub fn chfl_trajectory_topology_file(trajectory: *mut CHFL_TRAJECTORY, path: *const c_char, format: *const c_char) -> chfl_status;
    pub fn chfl_trajectory_set_cell(trajectory: *mut CHFL_TRAJECTORY, cell: *const CHFL_CELL) -> chfl_status;
    pub fn chfl_trajectory_nsteps(trajectory: *mut CHFL_TRAJECTORY, nsteps: *mut uint64_t) -> chfl_status;
    pub fn chfl_trajectory_close(trajectory: *mut CHFL_TRAJECTORY) -> chfl_status;
    pub fn chfl_selection(selection: *const c_char) -> *mut CHFL_SELECTION;
    pub fn chfl_selection_copy(selection: *const CHFL_SELECTION) -> *mut CHFL_SELECTION;
    pub fn chfl_selection_size(selection: *const CHFL_SELECTION, size: *mut uint64_t) -> chfl_status;
    pub fn chfl_selection_string(selection: *const CHFL_SELECTION, string: *mut c_char, buffsize: uint64_t) -> chfl_status;
    pub fn chfl_selection_evaluate(selection: *mut CHFL_SELECTION, frame: *const CHFL_FRAME, nmatches: *mut uint64_t) -> chfl_status;
    pub fn chfl_selection_matches(selection: *const CHFL_SELECTION, matches: *mut chfl_match_t, nmatches: uint64_t) -> chfl_status;
    pub fn chfl_selection_free(selection: *mut CHFL_SELECTION) -> chfl_status;
}

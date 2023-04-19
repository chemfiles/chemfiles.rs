// Chemfiles.rs, a modern library for chemistry file reading and writing
// Copyright (C) Guillaume Fraux and contributors -- BSD license
//
// ========================================================================= //
//                       !!!! AUTO-GENERATED FILE !!!!
// Do not edit. See the bindgen repository for the generation code
//                   https://github.com/chemfiles/bindgen
// ========================================================================= //

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(non_camel_case_types)]
use std::os::raw::{c_char, c_double, c_void};

// Manual definitions. Edit the bindgen code to make sure this matches the
// chemfiles.h header
pub type c_bool = u8;
pub type chfl_warning_callback = extern fn(*const c_char);
pub type chfl_vector3d = [c_double; 3];

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct chfl_match {
    pub size: u64,
    pub atoms: [u64; 4],
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct chfl_format_metadata {
    pub name: *const c_char,
    pub extension: *const c_char,
    pub description: *const c_char,
    pub reference: *const c_char,
    pub read: bool,
    pub write: bool,
    pub memory: bool,
    pub positions: bool,
    pub velocities: bool,
    pub unit_cell: bool,
    pub atoms: bool,
    pub bonds: bool,
    pub residues: bool,
}
// End manual definitions

pub enum CHFL_TRAJECTORY{}
pub enum CHFL_CELL{}
pub enum CHFL_ATOM{}
pub enum CHFL_FRAME{}
pub enum CHFL_TOPOLOGY{}
pub enum CHFL_SELECTION{}
pub enum CHFL_RESIDUE{}
pub enum CHFL_PROPERTY{}

#[must_use]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum chfl_status {
    CHFL_SUCCESS = 0,
    CHFL_MEMORY_ERROR = 1,
    CHFL_FILE_ERROR = 2,
    CHFL_FORMAT_ERROR = 3,
    CHFL_SELECTION_ERROR = 4,
    CHFL_CONFIGURATION_ERROR = 5,
    CHFL_OUT_OF_BOUNDS = 6,
    CHFL_PROPERTY_ERROR = 7,
    CHFL_GENERIC_ERROR = 254,
    CHFL_CXX_ERROR = 255,
}


#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum chfl_bond_order {
    CHFL_BOND_UNKNOWN = 0,
    CHFL_BOND_SINGLE = 1,
    CHFL_BOND_DOUBLE = 2,
    CHFL_BOND_TRIPLE = 3,
    CHFL_BOND_QUADRUPLE = 4,
    CHFL_BOND_QUINTUPLET = 5,
    CHFL_BOND_AMIDE = 254,
    CHFL_BOND_AROMATIC = 255,
}


#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum chfl_property_kind {
    CHFL_PROPERTY_BOOL = 0,
    CHFL_PROPERTY_DOUBLE = 1,
    CHFL_PROPERTY_STRING = 2,
    CHFL_PROPERTY_VECTOR3D = 3,
}


#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum chfl_cellshape {
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
    pub fn chfl_add_configuration(path: *const c_char) -> chfl_status;
    pub fn chfl_formats_list(metadata: *mut *mut chfl_format_metadata, count: *mut u64) -> chfl_status;
    pub fn chfl_guess_format(path: *const c_char, format: *mut c_char, buffsize: u64) -> chfl_status;
    pub fn chfl_free(object: *const c_void) -> c_void;
    pub fn chfl_property_bool(value: c_bool) -> *mut CHFL_PROPERTY;
    pub fn chfl_property_double(value: c_double) -> *mut CHFL_PROPERTY;
    pub fn chfl_property_string(value: *const c_char) -> *mut CHFL_PROPERTY;
    pub fn chfl_property_vector3d(value: *const c_double) -> *mut CHFL_PROPERTY;
    pub fn chfl_property_get_kind(property: *const CHFL_PROPERTY, kind: *mut chfl_property_kind) -> chfl_status;
    pub fn chfl_property_get_bool(property: *const CHFL_PROPERTY, value: *mut c_bool) -> chfl_status;
    pub fn chfl_property_get_double(property: *const CHFL_PROPERTY, value: *mut c_double) -> chfl_status;
    pub fn chfl_property_get_string(property: *const CHFL_PROPERTY, buffer: *mut c_char, buffsize: u64) -> chfl_status;
    pub fn chfl_property_get_vector3d(property: *const CHFL_PROPERTY, value: *mut c_double) -> chfl_status;
    pub fn chfl_atom(name: *const c_char) -> *mut CHFL_ATOM;
    pub fn chfl_atom_copy(atom: *const CHFL_ATOM) -> *mut CHFL_ATOM;
    pub fn chfl_atom_from_frame(frame: *mut CHFL_FRAME, index: u64) -> *mut CHFL_ATOM;
    pub fn chfl_atom_from_topology(topology: *mut CHFL_TOPOLOGY, index: u64) -> *mut CHFL_ATOM;
    pub fn chfl_atom_mass(atom: *const CHFL_ATOM, mass: *mut c_double) -> chfl_status;
    pub fn chfl_atom_set_mass(atom: *mut CHFL_ATOM, mass: c_double) -> chfl_status;
    pub fn chfl_atom_charge(atom: *const CHFL_ATOM, charge: *mut c_double) -> chfl_status;
    pub fn chfl_atom_set_charge(atom: *mut CHFL_ATOM, charge: c_double) -> chfl_status;
    pub fn chfl_atom_type(atom: *const CHFL_ATOM, _type: *mut c_char, buffsize: u64) -> chfl_status;
    pub fn chfl_atom_set_type(atom: *mut CHFL_ATOM, _type: *const c_char) -> chfl_status;
    pub fn chfl_atom_name(atom: *const CHFL_ATOM, name: *mut c_char, buffsize: u64) -> chfl_status;
    pub fn chfl_atom_set_name(atom: *mut CHFL_ATOM, name: *const c_char) -> chfl_status;
    pub fn chfl_atom_full_name(atom: *const CHFL_ATOM, name: *mut c_char, buffsize: u64) -> chfl_status;
    pub fn chfl_atom_vdw_radius(atom: *const CHFL_ATOM, radius: *mut c_double) -> chfl_status;
    pub fn chfl_atom_covalent_radius(atom: *const CHFL_ATOM, radius: *mut c_double) -> chfl_status;
    pub fn chfl_atom_atomic_number(atom: *const CHFL_ATOM, number: *mut u64) -> chfl_status;
    pub fn chfl_atom_properties_count(atom: *const CHFL_ATOM, count: *mut u64) -> chfl_status;
    pub fn chfl_atom_list_properties(atom: *const CHFL_ATOM, names: *mut *mut c_char, count: u64) -> chfl_status;
    pub fn chfl_atom_set_property(atom: *mut CHFL_ATOM, name: *const c_char, property: *const CHFL_PROPERTY) -> chfl_status;
    pub fn chfl_atom_get_property(atom: *const CHFL_ATOM, name: *const c_char) -> *mut CHFL_PROPERTY;
    pub fn chfl_residue(name: *const c_char) -> *mut CHFL_RESIDUE;
    pub fn chfl_residue_with_id(name: *const c_char, resid: i64) -> *mut CHFL_RESIDUE;
    pub fn chfl_residue_from_topology(topology: *const CHFL_TOPOLOGY, i: u64) -> *const CHFL_RESIDUE;
    pub fn chfl_residue_for_atom(topology: *const CHFL_TOPOLOGY, i: u64) -> *const CHFL_RESIDUE;
    pub fn chfl_residue_copy(residue: *const CHFL_RESIDUE) -> *mut CHFL_RESIDUE;
    pub fn chfl_residue_atoms_count(residue: *const CHFL_RESIDUE, count: *mut u64) -> chfl_status;
    pub fn chfl_residue_atoms(residue: *const CHFL_RESIDUE, atoms: *mut u64, count: u64) -> chfl_status;
    pub fn chfl_residue_id(residue: *const CHFL_RESIDUE, id: *mut i64) -> chfl_status;
    pub fn chfl_residue_name(residue: *const CHFL_RESIDUE, name: *mut c_char, buffsize: u64) -> chfl_status;
    pub fn chfl_residue_add_atom(residue: *mut CHFL_RESIDUE, i: u64) -> chfl_status;
    pub fn chfl_residue_contains(residue: *const CHFL_RESIDUE, i: u64, result: *mut c_bool) -> chfl_status;
    pub fn chfl_residue_properties_count(residue: *const CHFL_RESIDUE, count: *mut u64) -> chfl_status;
    pub fn chfl_residue_list_properties(residue: *const CHFL_RESIDUE, names: *mut *mut c_char, count: u64) -> chfl_status;
    pub fn chfl_residue_set_property(residue: *mut CHFL_RESIDUE, name: *const c_char, property: *const CHFL_PROPERTY) -> chfl_status;
    pub fn chfl_residue_get_property(residue: *const CHFL_RESIDUE, name: *const c_char) -> *mut CHFL_PROPERTY;
    pub fn chfl_topology() -> *mut CHFL_TOPOLOGY;
    pub fn chfl_topology_from_frame(frame: *const CHFL_FRAME) -> *const CHFL_TOPOLOGY;
    pub fn chfl_topology_copy(topology: *const CHFL_TOPOLOGY) -> *mut CHFL_TOPOLOGY;
    pub fn chfl_topology_atoms_count(topology: *const CHFL_TOPOLOGY, count: *mut u64) -> chfl_status;
    pub fn chfl_topology_resize(topology: *mut CHFL_TOPOLOGY, natoms: u64) -> chfl_status;
    pub fn chfl_topology_add_atom(topology: *mut CHFL_TOPOLOGY, atom: *const CHFL_ATOM) -> chfl_status;
    pub fn chfl_topology_remove(topology: *mut CHFL_TOPOLOGY, i: u64) -> chfl_status;
    pub fn chfl_topology_bonds_count(topology: *const CHFL_TOPOLOGY, count: *mut u64) -> chfl_status;
    pub fn chfl_topology_angles_count(topology: *const CHFL_TOPOLOGY, count: *mut u64) -> chfl_status;
    pub fn chfl_topology_dihedrals_count(topology: *const CHFL_TOPOLOGY, count: *mut u64) -> chfl_status;
    pub fn chfl_topology_impropers_count(topology: *const CHFL_TOPOLOGY, count: *mut u64) -> chfl_status;
    pub fn chfl_topology_bonds(topology: *const CHFL_TOPOLOGY, data: *mut [u64; 2], count: u64) -> chfl_status;
    pub fn chfl_topology_angles(topology: *const CHFL_TOPOLOGY, data: *mut [u64; 3], count: u64) -> chfl_status;
    pub fn chfl_topology_dihedrals(topology: *const CHFL_TOPOLOGY, data: *mut [u64; 4], count: u64) -> chfl_status;
    pub fn chfl_topology_impropers(topology: *const CHFL_TOPOLOGY, data: *mut [u64; 4], count: u64) -> chfl_status;
    pub fn chfl_topology_add_bond(topology: *mut CHFL_TOPOLOGY, i: u64, j: u64) -> chfl_status;
    pub fn chfl_topology_remove_bond(topology: *mut CHFL_TOPOLOGY, i: u64, j: u64) -> chfl_status;
    pub fn chfl_topology_clear_bonds(topology: *mut CHFL_TOPOLOGY) -> chfl_status;
    pub fn chfl_topology_residues_count(topology: *const CHFL_TOPOLOGY, count: *mut u64) -> chfl_status;
    pub fn chfl_topology_add_residue(topology: *mut CHFL_TOPOLOGY, residue: *const CHFL_RESIDUE) -> chfl_status;
    pub fn chfl_topology_residues_linked(topology: *const CHFL_TOPOLOGY, first: *const CHFL_RESIDUE, second: *const CHFL_RESIDUE, result: *mut c_bool) -> chfl_status;
    pub fn chfl_topology_bond_with_order(topology: *mut CHFL_TOPOLOGY, i: u64, j: u64, bond_order: chfl_bond_order) -> chfl_status;
    pub fn chfl_topology_bond_orders(topology: *const CHFL_TOPOLOGY, orders: *mut chfl_bond_order, nbonds: u64) -> chfl_status;
    pub fn chfl_topology_bond_order(topology: *const CHFL_TOPOLOGY, i: u64, j: u64, order: *mut chfl_bond_order) -> chfl_status;
    pub fn chfl_cell(lengths: *const c_double, angles: *const c_double) -> *mut CHFL_CELL;
    pub fn chfl_cell_from_matrix(matrix: *mut [c_double; 3]) -> *mut CHFL_CELL;
    pub fn chfl_cell_from_frame(frame: *mut CHFL_FRAME) -> *mut CHFL_CELL;
    pub fn chfl_cell_copy(cell: *const CHFL_CELL) -> *mut CHFL_CELL;
    pub fn chfl_cell_volume(cell: *const CHFL_CELL, volume: *mut c_double) -> chfl_status;
    pub fn chfl_cell_lengths(cell: *const CHFL_CELL, lengths: *mut c_double) -> chfl_status;
    pub fn chfl_cell_set_lengths(cell: *mut CHFL_CELL, lengths: *const c_double) -> chfl_status;
    pub fn chfl_cell_angles(cell: *const CHFL_CELL, angles: *mut c_double) -> chfl_status;
    pub fn chfl_cell_set_angles(cell: *mut CHFL_CELL, angles: *const c_double) -> chfl_status;
    pub fn chfl_cell_matrix(cell: *const CHFL_CELL, matrix: *mut [c_double; 3]) -> chfl_status;
    pub fn chfl_cell_shape(cell: *const CHFL_CELL, shape: *mut chfl_cellshape) -> chfl_status;
    pub fn chfl_cell_set_shape(cell: *mut CHFL_CELL, shape: chfl_cellshape) -> chfl_status;
    pub fn chfl_cell_wrap(cell: *const CHFL_CELL, vector: *mut c_double) -> chfl_status;
    pub fn chfl_frame() -> *mut CHFL_FRAME;
    pub fn chfl_frame_copy(frame: *const CHFL_FRAME) -> *mut CHFL_FRAME;
    pub fn chfl_frame_atoms_count(frame: *const CHFL_FRAME, count: *mut u64) -> chfl_status;
    pub fn chfl_frame_positions(frame: *mut CHFL_FRAME, positions: *mut *mut [c_double; 3], size: *mut u64) -> chfl_status;
    pub fn chfl_frame_velocities(frame: *mut CHFL_FRAME, velocities: *mut *mut [c_double; 3], size: *mut u64) -> chfl_status;
    pub fn chfl_frame_add_atom(frame: *mut CHFL_FRAME, atom: *const CHFL_ATOM, position: *const c_double, velocity: *const c_double) -> chfl_status;
    pub fn chfl_frame_remove(frame: *mut CHFL_FRAME, i: u64) -> chfl_status;
    pub fn chfl_frame_resize(frame: *mut CHFL_FRAME, size: u64) -> chfl_status;
    pub fn chfl_frame_add_velocities(frame: *mut CHFL_FRAME) -> chfl_status;
    pub fn chfl_frame_has_velocities(frame: *const CHFL_FRAME, has_velocities: *mut c_bool) -> chfl_status;
    pub fn chfl_frame_set_cell(frame: *mut CHFL_FRAME, cell: *const CHFL_CELL) -> chfl_status;
    pub fn chfl_frame_set_topology(frame: *mut CHFL_FRAME, topology: *const CHFL_TOPOLOGY) -> chfl_status;
    pub fn chfl_frame_step(frame: *const CHFL_FRAME, step: *mut u64) -> chfl_status;
    pub fn chfl_frame_set_step(frame: *mut CHFL_FRAME, step: u64) -> chfl_status;
    pub fn chfl_frame_guess_bonds(frame: *mut CHFL_FRAME) -> chfl_status;
    pub fn chfl_frame_distance(frame: *const CHFL_FRAME, i: u64, j: u64, distance: *mut c_double) -> chfl_status;
    pub fn chfl_frame_angle(frame: *const CHFL_FRAME, i: u64, j: u64, k: u64, angle: *mut c_double) -> chfl_status;
    pub fn chfl_frame_dihedral(frame: *const CHFL_FRAME, i: u64, j: u64, k: u64, m: u64, dihedral: *mut c_double) -> chfl_status;
    pub fn chfl_frame_out_of_plane(frame: *const CHFL_FRAME, i: u64, j: u64, k: u64, m: u64, distance: *mut c_double) -> chfl_status;
    pub fn chfl_frame_properties_count(frame: *const CHFL_FRAME, count: *mut u64) -> chfl_status;
    pub fn chfl_frame_list_properties(frame: *const CHFL_FRAME, names: *mut *mut c_char, count: u64) -> chfl_status;
    pub fn chfl_frame_set_property(frame: *mut CHFL_FRAME, name: *const c_char, property: *const CHFL_PROPERTY) -> chfl_status;
    pub fn chfl_frame_get_property(frame: *const CHFL_FRAME, name: *const c_char) -> *mut CHFL_PROPERTY;
    pub fn chfl_frame_add_bond(frame: *mut CHFL_FRAME, i: u64, j: u64) -> chfl_status;
    pub fn chfl_frame_bond_with_order(frame: *mut CHFL_FRAME, i: u64, j: u64, bond_order: chfl_bond_order) -> chfl_status;
    pub fn chfl_frame_remove_bond(frame: *mut CHFL_FRAME, i: u64, j: u64) -> chfl_status;
    pub fn chfl_frame_clear_bonds(frame: *mut CHFL_FRAME) -> chfl_status;
    pub fn chfl_frame_add_residue(frame: *mut CHFL_FRAME, residue: *const CHFL_RESIDUE) -> chfl_status;
    pub fn chfl_trajectory_open(path: *const c_char, mode: c_char) -> *mut CHFL_TRAJECTORY;
    pub fn chfl_trajectory_with_format(path: *const c_char, mode: c_char, format: *const c_char) -> *mut CHFL_TRAJECTORY;
    pub fn chfl_trajectory_memory_reader(memory: *const c_char, size: u64, format: *const c_char) -> *mut CHFL_TRAJECTORY;
    pub fn chfl_trajectory_memory_writer(format: *const c_char) -> *mut CHFL_TRAJECTORY;
    pub fn chfl_trajectory_path(trajectory: *const CHFL_TRAJECTORY, path: *mut c_char, buffsize: u64) -> chfl_status;
    pub fn chfl_trajectory_read(trajectory: *mut CHFL_TRAJECTORY, frame: *mut CHFL_FRAME) -> chfl_status;
    pub fn chfl_trajectory_read_step(trajectory: *mut CHFL_TRAJECTORY, step: u64, frame: *mut CHFL_FRAME) -> chfl_status;
    pub fn chfl_trajectory_write(trajectory: *mut CHFL_TRAJECTORY, frame: *const CHFL_FRAME) -> chfl_status;
    pub fn chfl_trajectory_set_topology(trajectory: *mut CHFL_TRAJECTORY, topology: *const CHFL_TOPOLOGY) -> chfl_status;
    pub fn chfl_trajectory_topology_file(trajectory: *mut CHFL_TRAJECTORY, path: *const c_char, format: *const c_char) -> chfl_status;
    pub fn chfl_trajectory_set_cell(trajectory: *mut CHFL_TRAJECTORY, cell: *const CHFL_CELL) -> chfl_status;
    pub fn chfl_trajectory_nsteps(trajectory: *mut CHFL_TRAJECTORY, nsteps: *mut u64) -> chfl_status;
    pub fn chfl_trajectory_memory_buffer(trajectory: *const CHFL_TRAJECTORY, data: *mut *const c_char, size: *mut u64) -> chfl_status;
    pub fn chfl_trajectory_close(trajectory: *const CHFL_TRAJECTORY) -> c_void;
    pub fn chfl_selection(selection: *const c_char) -> *mut CHFL_SELECTION;
    pub fn chfl_selection_copy(selection: *const CHFL_SELECTION) -> *mut CHFL_SELECTION;
    pub fn chfl_selection_size(selection: *const CHFL_SELECTION, size: *mut u64) -> chfl_status;
    pub fn chfl_selection_string(selection: *const CHFL_SELECTION, string: *mut c_char, buffsize: u64) -> chfl_status;
    pub fn chfl_selection_evaluate(selection: *mut CHFL_SELECTION, frame: *const CHFL_FRAME, n_matches: *mut u64) -> chfl_status;
    pub fn chfl_selection_matches(selection: *const CHFL_SELECTION, matches: *mut chfl_match, n_matches: u64) -> chfl_status;
}

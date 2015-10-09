#![allow(dead_code, non_camel_case_types)]

extern crate libc;

pub enum CHRP_TRAJECTORY{}
pub enum CHRP_FRAME{}
pub enum CHRP_ATOM{}
pub enum CHRP_CELL{}
pub enum CHRP_TOPOLOGY{}

pub type CHRP_LOG_LEVEL = libc::c_uint;
pub const NONE: libc::c_uint = 0;
pub const ERROR: libc::c_uint = 1;
pub const WARNING: libc::c_uint = 2;
pub const INFO: libc::c_uint = 3;
pub const DEBUG: libc::c_uint = 4;

pub type CHRP_CELL_TYPE = libc::c_uint;
pub const ORTHOROMBIC: libc::c_uint = 0;
pub const TRICLINIC: libc::c_uint = 1;
pub const INFINITE: libc::c_uint = 2;

pub type CHRP_ATOM_TYPE = libc::c_uint;
pub const ELEMENT: libc::c_uint = 0;
pub const CORSE_GRAIN: libc::c_uint = 1;
pub const DUMMY: libc::c_uint = 2;
pub const UNDEFINED: libc::c_uint = 3;

pub type CHRP_STATUS = libc::c_int;

#[link(name="chemharp", kind="static")]
extern "C" {
    pub fn chrp_strerror(status: libc::c_int) -> *const libc::c_char;
    pub fn chrp_last_error() -> *const libc::c_char;
    pub fn chrp_loglevel(level: *mut CHRP_LOG_LEVEL) -> CHRP_STATUS;
    pub fn chrp_set_loglevel(level: CHRP_LOG_LEVEL) -> CHRP_STATUS;
    pub fn chrp_logfile(file: *const libc::c_char) -> CHRP_STATUS;
    pub fn chrp_log_stderr() -> CHRP_STATUS;

    pub fn chrp_trajectory_open(filename: *const libc::c_char, mode: *const libc::c_char) -> *mut CHRP_TRAJECTORY;
    pub fn chrp_trajectory_with_format(filename: *const libc::c_char, mode: *const libc::c_char, format: *const libc::c_char) -> *mut CHRP_TRAJECTORY;
    pub fn chrp_trajectory_read(file: *mut CHRP_TRAJECTORY, frame: *mut CHRP_FRAME) -> CHRP_STATUS;
    pub fn chrp_trajectory_read_step(file: *mut CHRP_TRAJECTORY, step: libc::size_t, frame: *mut CHRP_FRAME) -> CHRP_STATUS;
    pub fn chrp_trajectory_write(file: *mut CHRP_TRAJECTORY, frame: *const CHRP_FRAME) -> CHRP_STATUS;
    pub fn chrp_trajectory_set_topology(file: *mut CHRP_TRAJECTORY, topology: *const CHRP_TOPOLOGY) -> CHRP_STATUS;
    pub fn chrp_trajectory_set_topology_file(file: *mut CHRP_TRAJECTORY, filename: *const libc::c_char) -> CHRP_STATUS;
    pub fn chrp_trajectory_set_cell(file: *mut CHRP_TRAJECTORY, cell: *const CHRP_CELL) -> CHRP_STATUS;
    pub fn chrp_trajectory_nsteps(file: *mut CHRP_TRAJECTORY, nsteps: *mut libc::size_t) -> CHRP_STATUS;
    pub fn chrp_trajectory_close(file: *mut CHRP_TRAJECTORY) -> CHRP_STATUS;

    pub fn chrp_frame(natoms: libc::size_t) -> *mut CHRP_FRAME;
    pub fn chrp_frame_atoms_count(frame: *const CHRP_FRAME, natoms: *mut libc::size_t) -> CHRP_STATUS;
    pub fn chrp_frame_positions(frame: *const CHRP_FRAME, data: *mut libc::c_float, size: libc::size_t) -> CHRP_STATUS;
    pub fn chrp_frame_set_positions(frame: *mut CHRP_FRAME, data: *const libc::c_float, size: libc::size_t) -> CHRP_STATUS;
    pub fn chrp_frame_velocities(frame: *const CHRP_FRAME, data: *mut libc::c_float, size: libc::size_t) -> CHRP_STATUS;
    pub fn chrp_frame_set_velocities(frame: *mut CHRP_FRAME, data: *const libc::c_float, size: libc::size_t) -> CHRP_STATUS;
    pub fn chrp_frame_has_velocities(frame: *const CHRP_FRAME, has_vel: *mut u8) -> CHRP_STATUS;
    pub fn chrp_frame_set_cell(frame: *mut CHRP_FRAME, cell: *const CHRP_CELL) -> CHRP_STATUS;
    pub fn chrp_frame_set_topology(frame: *mut CHRP_FRAME, topology: *const CHRP_TOPOLOGY) -> CHRP_STATUS;
    pub fn chrp_frame_step(frame: *const CHRP_FRAME, step: *mut libc::size_t) -> CHRP_STATUS;
    pub fn chrp_frame_set_step(frame: *mut CHRP_FRAME, step: libc::size_t) -> CHRP_STATUS;
    pub fn chrp_frame_guess_topology(frame: *mut CHRP_FRAME, bonds: u8) -> CHRP_STATUS;
    pub fn chrp_frame_free(frame: *mut CHRP_FRAME) -> CHRP_STATUS;

    pub fn chrp_cell(a: libc::c_double, b: libc::c_double, c: libc::c_double) -> *mut CHRP_CELL;
    pub fn chrp_cell_triclinic(a: libc::c_double, b: libc::c_double, c: libc::c_double, alpha: libc::c_double, beta: libc::c_double, gamma: libc::c_double) -> *mut CHRP_CELL;
    pub fn chrp_cell_from_frame(frame: *mut CHRP_FRAME) -> *mut CHRP_CELL;
    pub fn chrp_cell_lengths(cell: *const CHRP_CELL, a: *mut libc::c_double, b: *mut libc::c_double, c: *mut libc::c_double) -> CHRP_STATUS;
    pub fn chrp_cell_set_lengths(cell: *mut CHRP_CELL, a: libc::c_double, b: libc::c_double, c: libc::c_double) -> CHRP_STATUS;
    pub fn chrp_cell_angles(cell: *const CHRP_CELL, alpha: *mut libc::c_double, beta: *mut libc::c_double, gamma: *mut libc::c_double) -> CHRP_STATUS;
    pub fn chrp_cell_set_angles(cell: *mut CHRP_CELL, alpha: libc::c_double, beta: libc::c_double, gamma: libc::c_double) -> CHRP_STATUS;
    pub fn chrp_cell_matrix(cell: *const CHRP_CELL, mat: *mut [libc::c_double; 3usize]) -> CHRP_STATUS;
    pub fn chrp_cell_type(cell: *const CHRP_CELL, _type: *mut CHRP_CELL_TYPE) -> CHRP_STATUS;
    pub fn chrp_cell_set_type(cell: *mut CHRP_CELL, _type: CHRP_CELL_TYPE) -> CHRP_STATUS;
    pub fn chrp_cell_periodicity(cell: *const CHRP_CELL, x: *mut u8, y: *mut u8, z: *mut u8) -> CHRP_STATUS;
    pub fn chrp_cell_set_periodicity(cell: *mut CHRP_CELL, x: u8, y: u8, z: u8) -> CHRP_STATUS;
    pub fn chrp_cell_volume(cell: *const CHRP_CELL, V: *mut libc::c_double) -> CHRP_STATUS;
    pub fn chrp_cell_free(cell: *mut CHRP_CELL) -> CHRP_STATUS;

    pub fn chrp_topology() -> *mut CHRP_TOPOLOGY;
    pub fn chrp_topology_from_frame(frame: *mut CHRP_FRAME) -> *mut CHRP_TOPOLOGY;
    pub fn chrp_topology_atoms_count(topology: *const CHRP_TOPOLOGY, natoms: *mut libc::size_t) -> CHRP_STATUS;
    pub fn chrp_topology_append(topology: *mut CHRP_TOPOLOGY, atom: *const CHRP_ATOM) -> CHRP_STATUS;
    pub fn chrp_topology_remove(topology: *mut CHRP_TOPOLOGY, i: libc::size_t) -> CHRP_STATUS;
    pub fn chrp_topology_isbond(topology: *const CHRP_TOPOLOGY, i: libc::size_t, j: libc::size_t, result: *mut u8) -> CHRP_STATUS;
    pub fn chrp_topology_isangle(topology: *const CHRP_TOPOLOGY, i: libc::size_t, j: libc::size_t, k: libc::size_t, result: *mut u8) -> CHRP_STATUS;
    pub fn chrp_topology_isdihedral(topology: *const CHRP_TOPOLOGY, i: libc::size_t, j: libc::size_t, k: libc::size_t, m: libc::size_t, result: *mut u8) -> CHRP_STATUS;
    pub fn chrp_topology_bonds_count(topology: *const CHRP_TOPOLOGY, nbonds: *mut libc::size_t) -> CHRP_STATUS;
    pub fn chrp_topology_angles_count(topology: *const CHRP_TOPOLOGY, nangles: *mut libc::size_t) -> CHRP_STATUS;
    pub fn chrp_topology_dihedrals_count(topology: *const CHRP_TOPOLOGY, ndihedrals: *mut libc::size_t) -> CHRP_STATUS;
    pub fn chrp_topology_bonds(topology: *const CHRP_TOPOLOGY, data: *mut libc::size_t, nbonds: libc::size_t) -> CHRP_STATUS;
    pub fn chrp_topology_angles(topology: *const CHRP_TOPOLOGY, data: *mut libc::size_t, nangles: libc::size_t) -> CHRP_STATUS;
    pub fn chrp_topology_dihedrals(topology: *const CHRP_TOPOLOGY, data: *mut libc::size_t, ndihedrals: libc::size_t) -> CHRP_STATUS;
    pub fn chrp_topology_add_bond(topology: *mut CHRP_TOPOLOGY, i: libc::size_t, j: libc::size_t) -> CHRP_STATUS;
    pub fn chrp_topology_remove_bond(topology: *mut CHRP_TOPOLOGY, i: libc::size_t, j: libc::size_t) -> CHRP_STATUS;
    pub fn chrp_topology_free(topology: *mut CHRP_TOPOLOGY) -> CHRP_STATUS;

    pub fn chrp_atom(name: *const libc::c_char) -> *mut CHRP_ATOM;
    pub fn chrp_atom_from_frame(frame: *const CHRP_FRAME, idx: libc::size_t) -> *mut CHRP_ATOM;
    pub fn chrp_atom_from_topology(topology: *const CHRP_TOPOLOGY, idx: libc::size_t) -> *mut CHRP_ATOM;
    pub fn chrp_atom_mass(atom: *const CHRP_ATOM, mass: *mut libc::c_float) -> CHRP_STATUS;
    pub fn chrp_atom_set_mass(atom: *mut CHRP_ATOM, mass: libc::c_float) -> CHRP_STATUS;
    pub fn chrp_atom_charge(atom: *const CHRP_ATOM, charge: *mut libc::c_float) -> CHRP_STATUS;
    pub fn chrp_atom_set_charge(atom: *mut CHRP_ATOM, charge: libc::c_float) -> CHRP_STATUS;
    pub fn chrp_atom_name(atom: *const CHRP_ATOM, name: *mut libc::c_char, buffsize: libc::size_t) -> CHRP_STATUS;
    pub fn chrp_atom_set_name(atom: *mut CHRP_ATOM, name: *const libc::c_char) -> CHRP_STATUS;
    pub fn chrp_atom_full_name(atom: *const CHRP_ATOM, name: *mut libc::c_char, buffsize: libc::size_t) -> CHRP_STATUS;
    pub fn chrp_atom_vdw_radius(atom: *const CHRP_ATOM, radius: *mut libc::c_double) -> CHRP_STATUS;
    pub fn chrp_atom_covalent_radius(atom: *const CHRP_ATOM, radius: *mut libc::c_double) -> CHRP_STATUS;
    pub fn chrp_atom_atomic_number(atom: *const CHRP_ATOM, number: *mut libc::c_int) -> CHRP_STATUS;
    pub fn chrp_atom_type(cell: *const CHRP_ATOM, _type: *mut CHRP_ATOM_TYPE) -> CHRP_STATUS;
    pub fn chrp_atom_set_type(cell: *mut CHRP_ATOM, _type: CHRP_ATOM_TYPE) -> CHRP_STATUS;
    pub fn chrp_atom_free(atom: *mut CHRP_ATOM) -> CHRP_STATUS;
}

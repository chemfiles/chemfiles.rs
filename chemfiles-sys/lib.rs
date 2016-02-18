#![allow(non_camel_case_types)]
extern crate libc;

pub enum CHFL_TRAJECTORY{}
pub enum CHFL_FRAME{}
pub enum CHFL_ATOM{}
pub enum CHFL_CELL{}
pub enum CHFL_TOPOLOGY{}

pub const CHFL_SUCCESS: CHFL_STATUS = 0;
pub const CHFL_MEMORY_ERROR: CHFL_STATUS = 1;
pub const CHFL_FILE_ERROR: CHFL_STATUS = 2;
pub const CHFL_FORMAT_ERROR: CHFL_STATUS = 3;
pub const CHFL_SELECTION_ERROR: CHFL_STATUS = 4;
pub const CHFL_GENERIC_ERROR: CHFL_STATUS = 5;
pub const CHFL_CXX_ERROR: CHFL_STATUS = 6;

pub type CHFL_LOG_LEVEL = libc::c_uint;
pub const ERROR: CHFL_LOG_LEVEL = 0;
pub const WARNING: CHFL_LOG_LEVEL = 1;
pub const INFO: CHFL_LOG_LEVEL = 2;
pub const DEBUG: CHFL_LOG_LEVEL = 3;

pub type CHFL_CELL_TYPE = libc::c_uint;
pub const ORTHOROMBIC: CHFL_CELL_TYPE = 0;
pub const TRICLINIC: CHFL_CELL_TYPE = 1;
pub const INFINITE: CHFL_CELL_TYPE = 2;

pub type CHFL_ATOM_TYPE = libc::c_uint;
pub const ELEMENT: CHFL_ATOM_TYPE = 0;
pub const COARSE_GRAINED: CHFL_ATOM_TYPE = 1;
pub const DUMMY: CHFL_ATOM_TYPE = 2;
pub const UNDEFINED: CHFL_ATOM_TYPE = 3;

pub type CHFL_STATUS = libc::c_int;
pub type chfl_logging_callback_t = extern fn(CHFL_LOG_LEVEL, *const libc::c_char);
pub type c_bool = u8;

#[link(name="chemfiles", kind="static")]
extern "C" {
    pub fn chfl_version() -> *const libc::c_char;

    pub fn chfl_strerror(status: libc::c_int) -> *const libc::c_char;
    pub fn chfl_last_error() -> *const libc::c_char;

    pub fn chfl_loglevel(level: *mut CHFL_LOG_LEVEL) -> CHFL_STATUS;
    pub fn chfl_set_loglevel(level: CHFL_LOG_LEVEL) -> CHFL_STATUS;
    pub fn chfl_log_callback(callback: chfl_logging_callback_t) -> CHFL_STATUS;
    pub fn chfl_logfile(file: *const libc::c_char) -> CHFL_STATUS;
    pub fn chfl_log_stderr() -> CHFL_STATUS;
    pub fn chfl_log_stdout() -> CHFL_STATUS;
    pub fn chfl_log_silent() -> CHFL_STATUS;

    pub fn chfl_trajectory_open(filename: *const libc::c_char, mode: *const libc::c_char) -> *mut CHFL_TRAJECTORY;
    pub fn chfl_trajectory_with_format(filename: *const libc::c_char, mode: *const libc::c_char, format: *const libc::c_char) -> *mut CHFL_TRAJECTORY;
    pub fn chfl_trajectory_read(file: *mut CHFL_TRAJECTORY, frame: *mut CHFL_FRAME) -> CHFL_STATUS;
    pub fn chfl_trajectory_read_step(file: *mut CHFL_TRAJECTORY, step: libc::size_t, frame: *mut CHFL_FRAME) -> CHFL_STATUS;
    pub fn chfl_trajectory_write(file: *mut CHFL_TRAJECTORY, frame: *const CHFL_FRAME) -> CHFL_STATUS;
    pub fn chfl_trajectory_set_topology(file: *mut CHFL_TRAJECTORY, topology: *const CHFL_TOPOLOGY) -> CHFL_STATUS;
    pub fn chfl_trajectory_set_topology_file(file: *mut CHFL_TRAJECTORY, filename: *const libc::c_char) -> CHFL_STATUS;
    pub fn chfl_trajectory_set_cell(file: *mut CHFL_TRAJECTORY, cell: *const CHFL_CELL) -> CHFL_STATUS;
    pub fn chfl_trajectory_nsteps(file: *mut CHFL_TRAJECTORY, nsteps: *mut libc::size_t) -> CHFL_STATUS;
    pub fn chfl_trajectory_sync(file: *mut CHFL_TRAJECTORY) -> CHFL_STATUS;
    pub fn chfl_trajectory_close(file: *mut CHFL_TRAJECTORY) -> CHFL_STATUS;

    pub fn chfl_frame(natoms: libc::size_t) -> *mut CHFL_FRAME;
    pub fn chfl_frame_atoms_count(frame: *const CHFL_FRAME, natoms: *mut libc::size_t) -> CHFL_STATUS;
    pub fn chfl_frame_resize(frame: *mut CHFL_FRAME, natoms: libc::size_t) -> CHFL_STATUS;
    pub fn chfl_frame_positions(frame: *mut CHFL_FRAME, data: *mut *mut [libc::c_float; 3], size: *mut libc::size_t) -> CHFL_STATUS;
    pub fn chfl_frame_velocities(frame: *mut CHFL_FRAME, data: *mut *mut [libc::c_float; 3], size: *mut libc::size_t) -> CHFL_STATUS;
    pub fn chfl_frame_has_velocities(frame: *const CHFL_FRAME, has_velocities: *mut c_bool) -> CHFL_STATUS;
    pub fn chfl_frame_add_velocities(frame: *mut CHFL_FRAME) -> CHFL_STATUS;
    pub fn chfl_frame_set_cell(frame: *mut CHFL_FRAME, cell: *const CHFL_CELL) -> CHFL_STATUS;
    pub fn chfl_frame_set_topology(frame: *mut CHFL_FRAME, topology: *const CHFL_TOPOLOGY) -> CHFL_STATUS;
    pub fn chfl_frame_step(frame: *const CHFL_FRAME, step: *mut libc::size_t) -> CHFL_STATUS;
    pub fn chfl_frame_set_step(frame: *mut CHFL_FRAME, step: libc::size_t) -> CHFL_STATUS;
    pub fn chfl_frame_guess_topology(frame: *mut CHFL_FRAME, bonds: c_bool) -> CHFL_STATUS;
    pub fn chfl_frame_selection(frame: *const CHFL_FRAME, selection: *const libc::c_char, matched: *mut c_bool, natoms: libc::size_t) -> CHFL_STATUS;
    pub fn chfl_frame_free(frame: *mut CHFL_FRAME) -> CHFL_STATUS;

    pub fn chfl_cell(a: libc::c_double, b: libc::c_double, c: libc::c_double) -> *mut CHFL_CELL;
    pub fn chfl_cell_triclinic(a: libc::c_double, b: libc::c_double, c: libc::c_double, alpha: libc::c_double, beta: libc::c_double, gamma: libc::c_double) -> *mut CHFL_CELL;
    pub fn chfl_cell_from_frame(frame: *const CHFL_FRAME) -> *mut CHFL_CELL;
    pub fn chfl_cell_lengths(cell: *const CHFL_CELL, a: *mut libc::c_double, b: *mut libc::c_double, c: *mut libc::c_double) -> CHFL_STATUS;
    pub fn chfl_cell_set_lengths(cell: *mut CHFL_CELL, a: libc::c_double, b: libc::c_double, c: libc::c_double) -> CHFL_STATUS;
    pub fn chfl_cell_angles(cell: *const CHFL_CELL, alpha: *mut libc::c_double, beta: *mut libc::c_double, gamma: *mut libc::c_double) -> CHFL_STATUS;
    pub fn chfl_cell_set_angles(cell: *mut CHFL_CELL, alpha: libc::c_double, beta: libc::c_double, gamma: libc::c_double) -> CHFL_STATUS;
    pub fn chfl_cell_matrix(cell: *const CHFL_CELL, matrix: *mut [libc::c_double; 3usize]) -> CHFL_STATUS;
    pub fn chfl_cell_type(cell: *const CHFL_CELL, _type: *mut CHFL_CELL_TYPE) -> CHFL_STATUS;
    pub fn chfl_cell_set_type(cell: *mut CHFL_CELL, _type: CHFL_CELL_TYPE) -> CHFL_STATUS;
    pub fn chfl_cell_volume(cell: *const CHFL_CELL, V: *mut libc::c_double) -> CHFL_STATUS;
    pub fn chfl_cell_free(cell: *mut CHFL_CELL) -> CHFL_STATUS;

    pub fn chfl_topology() -> *mut CHFL_TOPOLOGY;
    pub fn chfl_topology_from_frame(frame: *const CHFL_FRAME) -> *mut CHFL_TOPOLOGY;
    pub fn chfl_topology_atoms_count(topology: *const CHFL_TOPOLOGY, natoms: *mut libc::size_t) -> CHFL_STATUS;
    pub fn chfl_topology_append(topology: *mut CHFL_TOPOLOGY, atom: *const CHFL_ATOM) -> CHFL_STATUS;
    pub fn chfl_topology_remove(topology: *mut CHFL_TOPOLOGY, i: libc::size_t) -> CHFL_STATUS;
    pub fn chfl_topology_isbond(topology: *const CHFL_TOPOLOGY, i: libc::size_t, j: libc::size_t, result: *mut c_bool) -> CHFL_STATUS;
    pub fn chfl_topology_isangle(topology: *const CHFL_TOPOLOGY, i: libc::size_t, j: libc::size_t, k: libc::size_t, result: *mut c_bool) -> CHFL_STATUS;
    pub fn chfl_topology_isdihedral(topology: *const CHFL_TOPOLOGY, i: libc::size_t, j: libc::size_t, k: libc::size_t, m: libc::size_t, result: *mut c_bool) -> CHFL_STATUS;
    pub fn chfl_topology_bonds_count(topology: *const CHFL_TOPOLOGY, nbonds: *mut libc::size_t) -> CHFL_STATUS;
    pub fn chfl_topology_angles_count(topology: *const CHFL_TOPOLOGY, nangles: *mut libc::size_t) -> CHFL_STATUS;
    pub fn chfl_topology_dihedrals_count(topology: *const CHFL_TOPOLOGY, ndihedrals: *mut libc::size_t) -> CHFL_STATUS;
    pub fn chfl_topology_bonds(topology: *const CHFL_TOPOLOGY, data: *mut libc::size_t, nbonds: libc::size_t) -> CHFL_STATUS;
    pub fn chfl_topology_angles(topology: *const CHFL_TOPOLOGY, data: *mut libc::size_t, nangles: libc::size_t) -> CHFL_STATUS;
    pub fn chfl_topology_dihedrals(topology: *const CHFL_TOPOLOGY, data: *mut libc::size_t, ndihedrals: libc::size_t) -> CHFL_STATUS;
    pub fn chfl_topology_add_bond(topology: *mut CHFL_TOPOLOGY, i: libc::size_t, j: libc::size_t) -> CHFL_STATUS;
    pub fn chfl_topology_remove_bond(topology: *mut CHFL_TOPOLOGY, i: libc::size_t, j: libc::size_t) -> CHFL_STATUS;
    pub fn chfl_topology_free(topology: *mut CHFL_TOPOLOGY) -> CHFL_STATUS;

    pub fn chfl_atom(name: *const libc::c_char) -> *mut CHFL_ATOM;
    pub fn chfl_atom_from_frame(frame: *const CHFL_FRAME, idx: libc::size_t) -> *mut CHFL_ATOM;
    pub fn chfl_atom_from_topology(topology: *const CHFL_TOPOLOGY, idx: libc::size_t) -> *mut CHFL_ATOM;
    pub fn chfl_atom_mass(atom: *const CHFL_ATOM, mass: *mut libc::c_float) -> CHFL_STATUS;
    pub fn chfl_atom_set_mass(atom: *mut CHFL_ATOM, mass: libc::c_float) -> CHFL_STATUS;
    pub fn chfl_atom_charge(atom: *const CHFL_ATOM, charge: *mut libc::c_float) -> CHFL_STATUS;
    pub fn chfl_atom_set_charge(atom: *mut CHFL_ATOM, charge: libc::c_float) -> CHFL_STATUS;
    pub fn chfl_atom_name(atom: *const CHFL_ATOM, name: *mut libc::c_char, buffsize: libc::size_t) -> CHFL_STATUS;
    pub fn chfl_atom_set_name(atom: *mut CHFL_ATOM, name: *const libc::c_char) -> CHFL_STATUS;
    pub fn chfl_atom_full_name(atom: *const CHFL_ATOM, name: *mut libc::c_char, buffsize: libc::size_t) -> CHFL_STATUS;
    pub fn chfl_atom_vdw_radius(atom: *const CHFL_ATOM, radius: *mut libc::c_double) -> CHFL_STATUS;
    pub fn chfl_atom_covalent_radius(atom: *const CHFL_ATOM, radius: *mut libc::c_double) -> CHFL_STATUS;
    pub fn chfl_atom_atomic_number(atom: *const CHFL_ATOM, number: *mut libc::c_int) -> CHFL_STATUS;
    pub fn chfl_atom_type(cell: *const CHFL_ATOM, _type: *mut CHFL_ATOM_TYPE) -> CHFL_STATUS;
    pub fn chfl_atom_set_type(cell: *mut CHFL_ATOM, _type: CHFL_ATOM_TYPE) -> CHFL_STATUS;
    pub fn chfl_atom_free(atom: *mut CHFL_ATOM) -> CHFL_STATUS;
}

// Chemfiles, a modern library for chemistry file reading and writing
// Copyright (C) 2015-2018 Guillaume Fraux -- BSD licensed
use chemfiles_sys::*;

use crate::{Atom, AtomMut, AtomRef};
use crate::{BondOrder, Residue, Topology, TopologyRef};
use crate::{UnitCell, UnitCellMut, UnitCellRef};

use crate::errors::{check, check_not_null, check_success, Error};
use crate::property::{PropertiesIter, Property, RawProperty};
use crate::strings;

/// A `Frame` contains data from one simulation step: the current unit
/// cell, the topology, the positions, and the velocities of the particles in
/// the system. If some information is missing (topology or velocity or unit
/// cell), the corresponding data is filled with a default value.
pub struct Frame {
    handle: *mut CHFL_FRAME,
}

impl Clone for Frame {
    fn clone(&self) -> Frame {
        unsafe {
            let new_handle = chfl_frame_copy(self.as_ptr());
            Frame::from_ptr(new_handle)
        }
    }
}

pub struct AtomIter<'a> {
    frame: &'a Frame,
    index: usize,
    size: usize,
}

impl Frame {
    /// Create a `Frame` from a C pointer.
    ///
    /// This function is unsafe because no validity check is made on the pointer,
    /// except for it being non-null.
    #[inline]
    pub(crate) unsafe fn from_ptr(ptr: *mut CHFL_FRAME) -> Frame {
        check_not_null(ptr);
        Frame { handle: ptr }
    }

    /// Get the underlying C pointer as a const pointer.
    #[inline]
    pub(crate) fn as_ptr(&self) -> *const CHFL_FRAME {
        self.handle
    }

    /// Get the underlying C pointer as a mutable pointer.
    #[inline]
    pub(crate) fn as_mut_ptr(&mut self) -> *mut CHFL_FRAME {
        self.handle
    }

    /// Get the underlying C pointer as a mutable pointer FROM A SHARED REFERENCE.
    ///
    /// For uses with functions of the C API using mut pointers for both read
    /// and write access. Users should check that this does not lead to multiple
    /// mutable borrows
    #[inline]
    #[allow(non_snake_case)]
    pub(crate) fn as_mut_ptr_MANUALLY_CHECKING_BORROW(&self) -> *mut CHFL_FRAME {
        self.handle
    }

    /// Create an empty frame. It will be resized by the library as needed.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::Frame;
    /// let frame = Frame::new();
    ///
    /// assert_eq!(frame.size(), 0);
    /// ```
    pub fn new() -> Frame {
        unsafe { Frame::from_ptr(chfl_frame()) }
    }

    /// Get a reference to the atom at the given `index` in this frame.
    ///
    /// # Panics
    ///
    /// If `index` is out of bounds.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::{Frame, Atom};
    /// let mut frame = Frame::new();
    /// frame.add_atom(&Atom::new("Zn"), [0.0; 3], None);
    ///
    /// let atom = frame.atom(0);
    /// assert_eq!(atom.name(), "Zn");
    /// ```
    pub fn atom(&self, index: usize) -> AtomRef {
        unsafe {
            let handle = chfl_atom_from_frame(self.as_mut_ptr_MANUALLY_CHECKING_BORROW(), index as u64);
            Atom::ref_from_ptr(handle)
        }
    }

    /// Get a mutable reference to the atom at the given `index` in this frame.
    ///
    /// # Panics
    ///
    /// If `index` is out of bounds.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::{Frame, Atom};
    /// let mut frame = Frame::new();
    /// frame.add_atom(&Atom::new("Zn"), [0.0; 3], None);
    ///
    /// assert_eq!(frame.atom(0).name(), "Zn");
    ///
    /// frame.atom_mut(0).set_name("Fe");
    /// assert_eq!(frame.atom(0).name(), "Fe");
    /// ```
    pub fn atom_mut(&mut self, index: usize) -> AtomMut {
        unsafe {
            let handle = chfl_atom_from_frame(self.as_mut_ptr(), index as u64);
            Atom::ref_mut_from_ptr(handle)
        }
    }

    /// Get the current number of atoms in this frame.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::Frame;
    /// let mut frame = Frame::new();
    /// assert_eq!(frame.size(), 0);
    ///
    /// frame.resize(67);
    /// assert_eq!(frame.size(), 67);
    /// ```
    pub fn size(&self) -> usize {
        let mut size = 0;
        unsafe {
            check_success(chfl_frame_atoms_count(self.as_ptr(), &mut size));
        }
        #[allow(clippy::cast_possible_truncation)]
        return size as usize;
    }

    /// Resize the positions and the velocities in this frame, to make space for
    /// `natoms` atoms. Previous data is conserved, as well as the presence of
    /// absence of velocities.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::Frame;
    /// let mut frame = Frame::new();
    /// frame.resize(67);
    /// assert_eq!(frame.size(), 67);
    /// ```
    pub fn resize(&mut self, natoms: usize) {
        unsafe {
            check_success(chfl_frame_resize(self.as_mut_ptr(), natoms as u64));
        }
    }

    /// Add an `Atom` and the corresponding position and optionally velocity
    /// data to this frame.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::{Frame, Atom};
    /// let mut frame = Frame::new();
    /// frame.add_atom(&Atom::new("Zn"), [1.0, 1.0, 2.0], None);
    ///
    /// frame.add_velocities();
    /// frame.add_atom(&Atom::new("Zn"), [-1.0, 1.0, 2.0], [0.2, 0.1, 0.0]);
    /// ```
    pub fn add_atom(&mut self, atom: &Atom, position: [f64; 3], velocity: impl Into<Option<[f64; 3]>>) {
        let velocity = velocity.into();
        let velocity_ptr = match velocity {
            Some(ref data) => data.as_ptr(),
            None => std::ptr::null(),
        };

        unsafe {
            check_success(chfl_frame_add_atom(
                self.as_mut_ptr(),
                atom.as_ptr(),
                position.as_ptr(),
                velocity_ptr,
            ));
        }
    }

    /// Remove the atom at index `i` in this frame.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::{Frame, Atom};
    /// let mut frame = Frame::new();
    /// frame.add_atom(&Atom::new("Zn"), [0.0; 3], None);
    /// frame.add_atom(&Atom::new("Fe"), [0.0; 3], None);
    /// frame.add_atom(&Atom::new("Sn"), [0.0; 3], None);
    /// assert_eq!(frame.size(), 3);
    ///
    /// frame.remove(1);
    /// assert_eq!(frame.size(), 2);
    /// assert_eq!(frame.atom(1).name(), "Sn");
    /// ```
    pub fn remove(&mut self, i: usize) {
        unsafe {
            check_success(chfl_frame_remove(self.as_mut_ptr(), i as u64));
        }
    }

    /// Add a bond between the atoms at indexes `i` and `j` in the frame.
    ///
    /// The bond order is set to `BondOrder::Unknown`.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::{Frame, BondOrder};
    /// let mut frame = Frame::new();
    /// assert_eq!(frame.topology().bonds_count(), 0);
    /// frame.resize(5);
    ///
    /// frame.add_bond(0, 1);
    /// frame.add_bond(3, 1);
    /// frame.add_bond(2, 4);
    /// assert_eq!(frame.topology().bonds_count(), 3);
    ///
    /// assert_eq!(frame.topology().bond_order(0, 1), BondOrder::Unknown);
    /// assert_eq!(frame.topology().bonds(), vec![[0, 1], [1, 3], [2, 4]]);
    /// ```
    pub fn add_bond(&mut self, i: usize, j: usize) {
        unsafe {
            check_success(chfl_frame_add_bond(self.as_mut_ptr(), i as u64, j as u64));
        }
    }

    /// Add a bond between the atoms at indexes `i` and `j` in the frame
    /// with the given bond `order`.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::{Frame, BondOrder};
    /// let mut frame = Frame::new();
    /// assert_eq!(frame.topology().bonds_count(), 0);
    /// frame.resize(2);
    ///
    /// frame.add_bond_with_order(0, 1, BondOrder::Double);
    /// assert_eq!(frame.topology().bond_order(0, 1), BondOrder::Double);
    /// ```
    pub fn add_bond_with_order(&mut self, i: usize, j: usize, order: BondOrder) {
        unsafe {
            check_success(chfl_frame_bond_with_order(
                self.as_mut_ptr(),
                i as u64,
                j as u64,
                order.as_raw(),
            ));
        }
    }

    /// Remove any existing bond between the atoms at indexes `i` and `j` in
    /// the frame.
    ///
    /// This function does nothing if there is no bond between `i` and `j`.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::Frame;
    /// let mut frame = Frame::new();
    /// frame.resize(5);
    ///
    /// frame.add_bond(0, 1);
    /// frame.add_bond(3, 1);
    /// frame.add_bond(2, 4);
    ///
    /// let bonds = frame.topology().bonds();
    /// assert_eq!(bonds, vec![[0, 1], [1, 3], [2, 4]]);
    ///
    /// frame.remove_bond(2, 4);
    /// let bonds = frame.topology().bonds();
    /// assert_eq!(bonds, vec![[0, 1], [1, 3]]);
    /// ```
    pub fn remove_bond(&mut self, i: usize, j: usize) {
        unsafe {
            check_success(chfl_frame_remove_bond(self.as_mut_ptr(), i as u64, j as u64));
        }
    }

    /// Add a copy of `residue` to this frame.
    ///
    /// # Errors
    ///
    /// This function fails is the residue id is already in this frame's
    /// topology, or if the residue contain atoms that are already in another
    /// residue.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::{Frame, Residue};
    /// let mut frame = Frame::new();
    ///
    /// let residue = Residue::new("foo");
    /// frame.add_residue(&residue).unwrap();
    ///
    /// let topology = frame.topology();
    /// assert_eq!(topology.residues_count(), 1);
    /// assert_eq!(topology.residue(0).unwrap().name(), "foo");
    /// ```
    pub fn add_residue(&mut self, residue: &Residue) -> Result<(), Error> {
        unsafe { check(chfl_frame_add_residue(self.as_mut_ptr(), residue.as_ptr())) }
    }

    /// Get the distance between the atoms at indexes `i` and `j` in this frame,
    /// accounting for periodic boundary conditions. The result is expressed in
    /// Angstroms.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::{Frame, Atom};
    /// let mut frame = Frame::new();
    /// frame.add_atom(&Atom::new("A"), [0.0, 0.0, 0.0], None);
    /// frame.add_atom(&Atom::new("B"), [1.0, 2.0, 3.0], None);
    ///
    /// assert_eq!(frame.distance(0, 1), f64::sqrt(14.0));
    /// ```
    pub fn distance(&self, i: usize, j: usize) -> f64 {
        let mut distance = 0.0;
        unsafe {
            check_success(chfl_frame_distance(self.as_ptr(), i as u64, j as u64, &mut distance));
        }
        return distance;
    }

    /// Get the angle formed by the atoms at indexes `i`, `j` and `k` in this
    /// frame, accounting for periodic boundary conditions. The result is
    /// expressed in radians.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::{Frame, Atom};
    /// # use std::f64;
    /// let mut frame = Frame::new();
    /// frame.add_atom(&Atom::new("A"), [1.0, 0.0, 0.0], None);
    /// frame.add_atom(&Atom::new("B"), [0.0, 0.0, 0.0], None);
    /// frame.add_atom(&Atom::new("C"), [0.0, 1.0, 0.0], None);
    ///
    /// assert_eq!(frame.angle(0, 1, 2), f64::consts::PI / 2.0);
    /// ```
    pub fn angle(&self, i: usize, j: usize, k: usize) -> f64 {
        let mut angle = 0.0;
        unsafe {
            check_success(chfl_frame_angle(
                self.as_ptr(),
                i as u64,
                j as u64,
                k as u64,
                &mut angle,
            ));
        }
        return angle;
    }

    /// Get the dihedral angle formed by the atoms at indexes `i`, `j`, `k` and
    /// `m` in this frame, accounting for periodic boundary conditions. The
    /// result is expressed in radians.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::{Frame, Atom};
    /// # use std::f64;
    /// let mut frame = Frame::new();
    /// frame.add_atom(&Atom::new("A"), [1.0, 0.0, 0.0], None);
    /// frame.add_atom(&Atom::new("B"), [0.0, 0.0, 0.0], None);
    /// frame.add_atom(&Atom::new("C"), [0.0, 1.0, 0.0], None);
    /// frame.add_atom(&Atom::new("D"), [0.0, 1.0, 1.0], None);
    ///
    /// assert_eq!(frame.dihedral(0, 1, 2, 3), f64::consts::PI / 2.0);
    /// ```
    pub fn dihedral(&self, i: usize, j: usize, k: usize, m: usize) -> f64 {
        let mut dihedral = 0.0;
        unsafe {
            check_success(chfl_frame_dihedral(
                self.as_ptr(),
                i as u64,
                j as u64,
                k as u64,
                m as u64,
                &mut dihedral,
            ));
        }
        return dihedral;
    }

    /// Get the out of plane distance formed by the atoms at indexes `i`, `j`,
    /// `k` and `m` in this frame, accounting for periodic boundary conditions.
    /// The result is expressed in angstroms.
    ///
    /// This is the distance between the atom j and the ikm plane. The j atom
    /// is the center of the improper dihedral angle formed by i, j, k and m.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::{Frame, Atom};
    /// let mut frame = Frame::new();
    /// frame.add_atom(&Atom::new("A"), [0.0, 0.0, 0.0], None);
    /// frame.add_atom(&Atom::new("B"), [0.0, 0.0, 2.0], None);
    /// frame.add_atom(&Atom::new("C"), [1.0, 0.0, 0.0], None);
    /// frame.add_atom(&Atom::new("D"), [0.0, 1.0, 0.0], None);
    ///
    /// assert_eq!(frame.out_of_plane(0, 1, 2, 3), 2.0);
    /// ```
    pub fn out_of_plane(&self, i: usize, j: usize, k: usize, m: usize) -> f64 {
        let mut distance = 0.0;
        unsafe {
            check_success(chfl_frame_out_of_plane(
                self.as_ptr(),
                i as u64,
                j as u64,
                k as u64,
                m as u64,
                &mut distance,
            ));
        }
        return distance;
    }

    /// Get a view into the positions of this frame.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::Frame;
    /// let mut frame = Frame::new();
    /// frame.resize(67);
    ///
    /// let positions = frame.positions();
    /// assert_eq!(positions.len(), 67);
    /// assert_eq!(positions[0], [0.0, 0.0, 0.0]);
    /// ```
    pub fn positions(&self) -> &[[f64; 3]] {
        let mut ptr = std::ptr::null_mut();
        let mut natoms = 0;
        unsafe {
            check_success(chfl_frame_positions(
                self.as_mut_ptr_MANUALLY_CHECKING_BORROW(),
                &mut ptr,
                &mut natoms,
            ));
        }

        #[allow(clippy::cast_possible_truncation)]
        let size = natoms as usize;
        unsafe {
            return std::slice::from_raw_parts(ptr, size);
        }
    }

    /// Get a mutable view into the positions of this frame.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::Frame;
    /// let mut frame = Frame::new();
    /// frame.resize(67);
    /// {
    ///     let positions = frame.positions_mut();
    ///     assert_eq!(positions[0], [0.0, 0.0, 0.0]);
    ///     positions[0] = [1.0, 2.0, 3.0];
    /// }
    ///
    /// let positions = frame.positions();
    /// assert_eq!(positions[0], [1.0, 2.0, 3.0]);
    /// ```
    pub fn positions_mut(&mut self) -> &mut [[f64; 3]] {
        let mut ptr = std::ptr::null_mut();
        let mut natoms = 0;
        unsafe {
            check_success(chfl_frame_positions(self.as_mut_ptr(), &mut ptr, &mut natoms));
        }
        #[allow(clippy::cast_possible_truncation)]
        let size = natoms as usize;
        unsafe {
            return std::slice::from_raw_parts_mut(ptr, size);
        }
    }

    /// Get a view into the velocities of this frame.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::Frame;
    /// let mut frame = Frame::new();
    /// frame.resize(67);
    /// frame.add_velocities();
    ///
    /// let velocities = frame.velocities().expect("missing velocities");
    /// assert_eq!(velocities.len(), 67);
    /// assert_eq!(velocities[0], [0.0, 0.0, 0.0]);
    /// ```
    pub fn velocities(&self) -> Option<&[[f64; 3]]> {
        if !self.has_velocities() {
            return None;
        }

        let mut ptr = std::ptr::null_mut();
        let mut natoms = 0;
        unsafe {
            check_success(chfl_frame_velocities(
                self.as_mut_ptr_MANUALLY_CHECKING_BORROW(),
                &mut ptr,
                &mut natoms,
            ));
        }
        #[allow(clippy::cast_possible_truncation)]
        let size = natoms as usize;
        unsafe {
            return Some(std::slice::from_raw_parts(ptr, size));
        }
    }

    /// Get a mutable view into the velocities of this frame.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::Frame;
    /// let mut frame = Frame::new();
    /// frame.resize(67);
    /// frame.add_velocities();
    /// {
    ///     let velocities = frame.velocities_mut().expect("missing velocities");
    ///     assert_eq!(velocities[0], [0.0, 0.0, 0.0]);
    ///     velocities[0] = [1.0, 2.0, 3.0];
    /// }
    ///
    /// let velocities = frame.velocities().expect("missing velocities");
    /// assert_eq!(velocities[0], [1.0, 2.0, 3.0]);
    /// ```
    pub fn velocities_mut(&mut self) -> Option<&mut [[f64; 3]]> {
        if !self.has_velocities() {
            return None;
        }

        let mut ptr = std::ptr::null_mut();
        let mut natoms = 0;
        unsafe {
            check_success(chfl_frame_velocities(self.as_mut_ptr(), &mut ptr, &mut natoms));
        }
        #[allow(clippy::cast_possible_truncation)]
        let size = natoms as usize;
        unsafe {
            return Some(std::slice::from_raw_parts_mut(ptr, size));
        }
    }

    /// Check if this frame contains velocity data.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::Frame;
    /// let mut frame = Frame::new();
    /// assert_eq!(frame.has_velocities(), false);
    ///
    /// frame.add_velocities();
    /// assert_eq!(frame.has_velocities(), true);
    /// ```
    pub fn has_velocities(&self) -> bool {
        let mut res = 0;
        unsafe {
            check_success(chfl_frame_has_velocities(self.as_ptr(), &mut res));
        }
        return res != 0;
    }

    /// Add velocity data to this frame. If the frame already have velocities,
    /// this does nothing.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::Frame;
    /// let mut frame = Frame::new();
    /// assert_eq!(frame.has_velocities(), false);
    ///
    /// frame.add_velocities();
    /// assert_eq!(frame.has_velocities(), true);
    /// ```
    pub fn add_velocities(&mut self) {
        unsafe {
            check_success(chfl_frame_add_velocities(self.as_mut_ptr()));
        }
    }

    /// Get a reference to the `UnitCell` from this frame.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::{Frame, CellShape};
    /// let frame = Frame::new();
    ///
    /// let cell = frame.cell();
    /// assert_eq!(cell.shape(), CellShape::Infinite);
    /// ```
    pub fn cell(&self) -> UnitCellRef {
        unsafe {
            let handle = chfl_cell_from_frame(self.as_mut_ptr_MANUALLY_CHECKING_BORROW());
            UnitCell::ref_from_ptr(handle)
        }
    }

    /// Get a mutable reference to the `UnitCell` from this frame.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::{Frame, CellShape};
    /// let mut frame = Frame::new();
    ///
    /// assert_eq!(frame.cell().shape(), CellShape::Infinite);
    ///
    /// frame.cell_mut().set_shape(CellShape::Triclinic).unwrap();
    /// assert_eq!(frame.cell().shape(), CellShape::Triclinic);
    /// ```
    pub fn cell_mut(&mut self) -> UnitCellMut {
        unsafe {
            let handle = chfl_cell_from_frame(self.as_mut_ptr());
            UnitCell::ref_mut_from_ptr(handle)
        }
    }

    /// Set the `UnitCell` of this frame to `cell`.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::{Frame, UnitCell, CellShape};
    /// let mut frame = Frame::new();
    ///
    /// frame.set_cell(&UnitCell::new([10.0, 10.0, 10.0]));
    ///
    /// let cell = frame.cell();
    /// assert_eq!(cell.shape(), CellShape::Orthorhombic);
    /// assert_eq!(cell.lengths(), [10.0, 10.0, 10.0]);
    /// ```
    pub fn set_cell(&mut self, cell: &UnitCell) {
        unsafe {
            check_success(chfl_frame_set_cell(self.as_mut_ptr(), cell.as_ptr()));
        }
    }

    /// Get a reference to the `Topology` of this frame.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::Frame;
    /// let mut frame = Frame::new();
    /// frame.resize(42);
    ///
    /// let topology = frame.topology();
    /// assert_eq!(topology.size(), 42);
    /// ```
    pub fn topology(&self) -> TopologyRef {
        unsafe {
            let handle = chfl_topology_from_frame(self.as_ptr());
            Topology::ref_from_ptr(handle)
        }
    }

    /// Set the `Topology` of this frame to `topology`.
    ///
    /// # Errors
    ///
    /// This function fails if the topology contains a different number of atoms
    /// than this frame.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::{Frame, Topology, Atom};
    /// let mut frame = Frame::new();
    /// frame.resize(2);
    ///
    /// let mut topology = Topology::new();
    /// topology.add_atom(&Atom::new("Cl"));
    /// topology.add_atom(&Atom::new("Cl"));
    /// topology.add_bond(0, 1);
    ///
    /// frame.set_topology(&topology).unwrap();
    /// assert_eq!(frame.atom(0).name(), "Cl");
    /// ```
    pub fn set_topology(&mut self, topology: &Topology) -> Result<(), Error> {
        unsafe { check(chfl_frame_set_topology(self.as_mut_ptr(), topology.as_ptr())) }
    }

    /// Get this frame step, i.e. the frame number in the trajectory
    ///
    /// # Example
    /// ```
    /// # use chemfiles::Frame;
    /// let frame = Frame::new();
    /// assert_eq!(frame.step(), 0);
    /// ```
    pub fn step(&self) -> usize {
        let mut step = 0;
        unsafe {
            check_success(chfl_frame_step(self.as_ptr(), &mut step));
        }
        #[allow(clippy::cast_possible_truncation)]
        return step as usize;
    }

    /// Set this frame step to `step`.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::Frame;
    /// let mut frame = Frame::new();
    /// assert_eq!(frame.step(), 0);
    ///
    /// frame.set_step(10);
    /// assert_eq!(frame.step(), 10);
    /// ```
    pub fn set_step(&mut self, step: usize) {
        unsafe {
            check_success(chfl_frame_set_step(self.as_mut_ptr(), step as u64));
        }
    }

    /// Guess the bonds, angles and dihedrals in this `frame`.
    ///
    /// The bonds are guessed using a distance-based algorithm, and then angles
    /// and dihedrals are guessed from the bonds.
    ///
    /// # Errors
    ////
    /// This function can fail if the covalent radius is unknown for some atoms
    /// in the frame.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::{Frame, Atom};
    /// let mut frame = Frame::new();
    ///
    /// frame.add_atom(&Atom::new("Cl"), [0.0, 0.0, 0.0], None);
    /// frame.add_atom(&Atom::new("Cl"), [1.5, 0.0, 0.0], None);
    /// assert_eq!(frame.topology().bonds_count(), 0);
    ///
    /// frame.guess_bonds().unwrap();
    /// assert_eq!(frame.topology().bonds_count(), 1);
    /// ```
    pub fn guess_bonds(&mut self) -> Result<(), Error> {
        unsafe { check(chfl_frame_guess_bonds(self.as_mut_ptr())) }
    }

    /// Remove all existing bonds, angles, dihedral angles and improper
    /// dihedral angles in the topology of the frame.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::{Atom, Frame};
    /// let mut frame = Frame::new();
    /// frame.add_atom(&Atom::new("H"), [1.0, 0.0, 0.0], None);
    /// frame.add_atom(&Atom::new("O"), [0.0, 0.0, 0.0], None);
    /// frame.add_atom(&Atom::new("H"), [0.0, 1.0, 0.0], None);
    ///
    /// frame.add_bond(0, 1);
    /// frame.add_bond(1, 2);
    ///
    /// assert_eq!(frame.topology().bonds().len(), 2);
    /// assert_eq!(frame.topology().angles().len(), 1);
    ///
    /// frame.clear_bonds();
    /// assert!(frame.topology().bonds().is_empty());
    /// assert!(frame.topology().angles().is_empty());
    /// ```
    pub fn clear_bonds(&mut self) {
        unsafe {
            check_success(chfl_frame_clear_bonds(self.as_mut_ptr()));
        }
    }

    /// Add a new `property` with the given `name` to this frame.
    ///
    /// If a property with the same name already exists, this function override
    /// the existing property with the new one.
    ///
    /// # Examples
    /// ```
    /// # use chemfiles::{Frame, Property};
    /// let mut frame = Frame::new();
    /// frame.set("a string", "hello");
    /// frame.set("a double", 4.3);
    ///
    /// assert_eq!(frame.get("a string"), Some(Property::String("hello".into())));
    /// assert_eq!(frame.get("a double"), Some(Property::Double(4.3)));
    /// ```
    pub fn set(&mut self, name: &str, property: impl Into<Property>) {
        let buffer = strings::to_c(name);
        let property = property.into().as_raw();
        unsafe {
            check_success(chfl_frame_set_property(
                self.as_mut_ptr(),
                buffer.as_ptr(),
                property.as_ptr(),
            ));
        }
    }

    /// Get a property with the given `name` in this frame, if it exist.
    ///
    /// # Examples
    /// ```
    /// # use chemfiles::{Frame, Property};
    /// let mut frame = Frame::new();
    /// frame.set("foo", Property::Double(22.2));
    ///
    /// assert_eq!(frame.get("foo"), Some(Property::Double(22.2)));
    /// assert_eq!(frame.get("Bar"), None);
    /// ```
    pub fn get(&self, name: &str) -> Option<Property> {
        let buffer = strings::to_c(name);
        unsafe {
            let handle = chfl_frame_get_property(self.as_ptr(), buffer.as_ptr());
            if handle.is_null() {
                None
            } else {
                let raw = RawProperty::from_ptr(handle);
                Some(Property::from_raw(raw))
            }
        }
    }

    /// Get an iterator over all (name, property) pairs for this frame
    ///
    /// # Examples
    /// ```
    /// # use chemfiles::{Frame, Property};
    /// let mut frame = Frame::new();
    /// frame.set("foo", Property::Double(22.2));
    /// frame.set("bar", Property::Bool(false));
    ///
    /// for (name, property) in frame.properties() {
    ///     if name == "foo" {
    ///         assert_eq!(property, Property::Double(22.2));
    ///     } else if name == "bar" {
    ///         assert_eq!(property, Property::Bool(false));
    ///     }
    /// }
    /// ```
    pub fn properties(&self) -> PropertiesIter {
        let mut count = 0;
        unsafe {
            check_success(chfl_frame_properties_count(self.as_ptr(), &mut count));
        }

        #[allow(clippy::cast_possible_truncation)]
        let size = count as usize;
        let mut c_names = vec![std::ptr::null_mut(); size];
        unsafe {
            check_success(chfl_frame_list_properties(self.as_ptr(), c_names.as_mut_ptr(), count));
        }

        let mut names = Vec::new();
        for ptr in c_names {
            names.push(strings::from_c(ptr));
        }

        PropertiesIter {
            names: names.into_iter(),
            getter: Box::new(move |name| self.get(name).expect("failed to get property")),
        }
    }

    /// Gets an iterator over atoms
    ///
    /// # Example
    /// ```
    /// # use chemfiles::{Atom, AtomRef, Frame};
    /// let mut frame = Frame::new();
    ///
    /// frame.add_atom(&Atom::new("O"), [0.0, 0.0, 0.0], None);
    /// frame.add_atom(&Atom::new("H"), [1.0, 0.0, 0.0], None);
    ///
    /// let mut atoms: Vec<AtomRef> = Vec::new();
    ///
    /// for atom in frame.iter_atoms() {
    ///     atoms.push(atom);
    /// }
    ///
    /// assert_eq!(atoms.len(), 2);
    /// ```
    pub fn iter_atoms(&self) -> AtomIter<'_> {
        AtomIter {
            frame: self,
            index: 0,
            size: self.size(),
        }
    }
}

impl Drop for Frame {
    fn drop(&mut self) {
        unsafe {
            let _ = chfl_free(self.as_ptr().cast());
        }
    }
}

impl<'a> Iterator for AtomIter<'a> {
    type Item = AtomRef<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.size <= self.index {
            return None;
        }
        let atom = self.frame.atom(self.index);
        self.index += 1;
        Some(atom)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn clone() {
        let mut frame = Frame::new();
        assert_eq!(frame.size(), 0);
        let copy = frame.clone();
        assert_eq!(copy.size(), 0);

        frame.resize(42);
        assert_eq!(frame.size(), 42);
        assert_eq!(copy.size(), 0);
    }

    #[test]
    fn size() {
        let mut frame = Frame::new();
        assert_eq!(frame.size(), 0);

        frame.resize(12);
        assert_eq!(frame.size(), 12);
    }

    #[test]
    fn add_atom() {
        let mut frame = Frame::new();

        frame.add_atom(&Atom::new("U"), [1.0, 1.0, 2.0], None);
        assert_eq!(frame.size(), 1);
        assert_eq!(frame.atom(0).name(), "U");

        let positions = &[[1.0, 1.0, 2.0]];
        assert_eq!(frame.positions(), positions);

        frame.add_velocities();
        frame.add_atom(&Atom::new("F"), [1.0, 1.0, 2.0], [4.0, 3.0, 2.0]);
        assert_eq!(frame.size(), 2);
        assert_eq!(frame.atom(0).name(), "U");
        assert_eq!(frame.atom(1).name(), "F");

        let positions = &[[1.0, 1.0, 2.0], [1.0, 1.0, 2.0]];
        assert_eq!(frame.positions(), positions);

        let velocities = &[[0.0, 0.0, 0.0], [4.0, 3.0, 2.0]];
        assert_eq!(frame.velocities().unwrap(), velocities);
    }

    #[test]
    #[should_panic]
    fn out_of_bounds_atom() {
        let mut frame = Frame::new();
        frame.resize(22);
        let _atom = frame.atom(23);
    }

    #[test]
    fn remove_atom() {
        let mut frame = Frame::new();
        frame.add_atom(&Atom::new("U"), [1.0, 1.0, 2.0], None);
        frame.add_atom(&Atom::new("F"), [1.0, 1.0, 2.0], None);

        assert_eq!(frame.size(), 2);
        assert_eq!(frame.atom(0).name(), "U");

        frame.remove(0);
        assert_eq!(frame.size(), 1);
        assert_eq!(frame.atom(0).name(), "F");
    }

    #[test]
    #[should_panic]
    fn remove_out_of_bounds() {
        let mut frame = Frame::new();
        frame.resize(32);

        frame.remove(100);
    }

    #[test]
    fn positions() {
        let mut frame = Frame::new();
        frame.resize(4);
        let expected = &[[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0], [10.0, 11.0, 12.0]];

        frame.positions_mut().clone_from_slice(expected);
        assert_eq!(frame.positions(), expected);
    }

    #[test]
    fn velocities() {
        let mut frame = Frame::new();
        frame.resize(4);
        assert!(!frame.has_velocities());
        frame.add_velocities();
        assert!(frame.has_velocities());

        let expected = &[[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0], [10.0, 11.0, 12.0]];

        frame.velocities_mut().unwrap().clone_from_slice(expected);
        assert_eq!(frame.velocities().unwrap(), expected);
    }

    #[test]
    fn cell() {
        let mut frame = Frame::new();
        frame.set_cell(&UnitCell::new([3.0, 4.0, 5.0]));
        let cell = frame.cell();
        assert_eq!(cell.lengths(), [3.0, 4.0, 5.0]);
    }

    #[test]
    fn topology() {
        let mut frame = Frame::new();
        frame.resize(2);
        let mut topology = Topology::new();

        topology.add_atom(&Atom::new("Zn"));
        topology.add_atom(&Atom::new("Ar"));

        assert!(frame.set_topology(&topology).is_ok());

        let topology = frame.topology();

        assert_eq!(topology.atom(0).name(), "Zn");
        assert_eq!(topology.atom(1).name(), "Ar");

        assert_eq!(frame.atom(0).name(), "Zn");
        assert_eq!(frame.atom(1).name(), "Ar");
    }

    #[test]
    fn bonds() {
        let mut frame = Frame::new();
        frame.resize(12);
        assert_eq!(frame.topology().bonds_count(), 0);

        frame.add_bond(0, 1);
        frame.add_bond(9, 2);
        frame.add_bond_with_order(3, 7, BondOrder::Aromatic);
        assert_eq!(frame.topology().bonds_count(), 3);

        assert_eq!(frame.topology().bonds(), vec![[0, 1], [2, 9], [3, 7]]);
        let expected = vec![BondOrder::Unknown, BondOrder::Unknown, BondOrder::Aromatic];
        assert_eq!(frame.topology().bond_orders(), expected);

        assert_eq!(frame.topology().bond_order(0, 1), BondOrder::Unknown);
        assert_eq!(frame.topology().bond_order(3, 7), BondOrder::Aromatic);

        frame.remove_bond(3, 7);
        // Removing unexisting bond is OK if both indexes are in bounds
        frame.remove_bond(8, 7);
        assert_eq!(frame.topology().bonds_count(), 2);

        frame.clear_bonds();
        assert_eq!(frame.topology().bonds_count(), 0);
    }

    #[test]
    #[should_panic]
    fn out_of_bounds_bonds() {
        let mut frame = Frame::new();
        frame.resize(12);
        frame.add_bond(300, 7);
    }

    #[test]
    #[should_panic]
    fn out_of_bounds_remove_bond() {
        let mut frame = Frame::new();
        frame.resize(12);
        frame.remove_bond(300, 7);
    }

    #[test]
    #[should_panic]
    fn out_of_bounds_bonds_with_order() {
        let mut frame = Frame::new();
        frame.resize(12);
        frame.add_bond_with_order(300, 7, BondOrder::Unknown);
    }

    #[test]
    fn residues() {
        let mut frame = Frame::new();
        assert_eq!(frame.topology().residues_count(), 0);

        let residue = &Residue::new("foobar");
        frame.add_residue(residue).unwrap();
        frame.add_residue(residue).unwrap();
        frame.add_residue(residue).unwrap();

        assert_eq!(frame.topology().residues_count(), 3);
        assert_eq!(frame.topology().residue(0).unwrap().name(), "foobar");
    }

    #[test]
    fn step() {
        let mut frame = Frame::new();
        assert_eq!(frame.step(), 0);
        frame.set_step(42);
        assert_eq!(frame.step(), 42);
    }

    #[test]
    fn property() {
        let mut frame = Frame::new();
        frame.set("foo", -22.0);
        assert_eq!(frame.get("foo"), Some(Property::Double(-22.0)));
        assert_eq!(frame.get("bar"), None);

        frame.set("bar", Property::String("here".into()));
        for (name, property) in frame.properties() {
            if name == "foo" {
                assert_eq!(property, Property::Double(-22.0));
            } else if name == "bar" {
                assert_eq!(property, Property::String("here".into()));
            }
        }
    }

    #[test]
    fn pbc_geometry() {
        use std::f64::consts::PI;

        let mut frame = Frame::new();
        let atom = &Atom::new("");

        frame.add_atom(atom, [1.0, 0.0, 0.0], None);
        frame.add_atom(atom, [0.0, 0.0, 0.0], None);
        frame.add_atom(atom, [0.0, 1.0, 0.0], None);
        frame.add_atom(atom, [0.0, 1.0, 1.0], None);
        frame.add_atom(atom, [0.0, 0.0, 2.0], None);

        assert_eq!(frame.distance(0, 2), f64::sqrt(2.0));
        assert_eq!(frame.angle(0, 1, 2), PI / 2.0);
        assert_eq!(frame.dihedral(0, 1, 2, 3), PI / 2.0);
        assert_eq!(frame.out_of_plane(1, 4, 0, 2), 2.0);
    }

    #[test]
    fn atom_iterator() {
        let mut frame = Frame::new();

        frame.add_atom(&Atom::new("H1"), [1.0, 0.0, 0.0], None);
        frame.add_atom(&Atom::new("H2"), [0.0, 1.0, 0.0], None);
        frame.add_atom(&Atom::new("H3"), [0.0, 0.0, 1.0], None);
        frame.add_atom(&Atom::new("H4"), [1.0, 1.0, 1.0], None);

        let mut items: Vec<(AtomRef, &[f64; 3])> = Vec::new();

        for item in frame.iter_atoms().zip(frame.positions()) {
            items.push(item);
        }

        assert_eq!(items[0].0.name(), "H1");
        assert_eq!(items[2].0.name(), "H3");

        assert_eq!(items[1].1, &[0.0_f64, 1.0_f64, 0.0_f64]);
        assert_eq!(items[3].1, &[1.0_f64, 1.0_f64, 1.0_f64]);
    }
}

// Chemfiles, a modern library for chemistry file reading and writing
// Copyright (C) 2015-2018 Guillaume Fraux -- BSD licensed
use std::marker::PhantomData;
use std::ops::{Deref, Drop};

use super::{Atom, AtomMut, AtomRef};
use super::{Residue, ResidueRef};
use chemfiles_sys::*;
use errors::{check, check_not_null, check_success, Error};

/// Possible bond order associated with bonds
#[repr(C)]
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BondOrder {
    /// Unknown or unspecified bond order
    Unknown = chfl_bond_order::CHFL_BOND_UNKNOWN as isize,
    /// Single bond
    Single = chfl_bond_order::CHFL_BOND_SINGLE as isize,
    /// Double bond
    Double = chfl_bond_order::CHFL_BOND_DOUBLE as isize,
    /// Triple bond
    Triple = chfl_bond_order::CHFL_BOND_TRIPLE as isize,
    /// Quadruple bond (present in some metals)
    Quadruple = chfl_bond_order::CHFL_BOND_QUADRUPLE as isize,
    /// Quintuplet bond (present in some metals)
    Quintuplet = chfl_bond_order::CHFL_BOND_QUINTUPLET as isize,
    /// Amide bond (required by some file formats)
    Amide = chfl_bond_order::CHFL_BOND_AMIDE as isize,
    /// Aromatic bond (required by some file formats)
    Aromatic = chfl_bond_order::CHFL_BOND_AROMATIC as isize,
}

impl BondOrder {
    pub(crate) fn as_raw(self) -> chfl_bond_order {
        match self {
            BondOrder::Unknown => chfl_bond_order::CHFL_BOND_UNKNOWN,
            BondOrder::Single => chfl_bond_order::CHFL_BOND_SINGLE,
            BondOrder::Double => chfl_bond_order::CHFL_BOND_DOUBLE,
            BondOrder::Triple => chfl_bond_order::CHFL_BOND_TRIPLE,
            BondOrder::Quadruple => chfl_bond_order::CHFL_BOND_QUADRUPLE,
            BondOrder::Quintuplet => chfl_bond_order::CHFL_BOND_QUINTUPLET,
            BondOrder::Amide => chfl_bond_order::CHFL_BOND_AMIDE,
            BondOrder::Aromatic => chfl_bond_order::CHFL_BOND_AROMATIC,
        }
    }
}

impl From<chfl_bond_order> for BondOrder {
    fn from(order: chfl_bond_order) -> BondOrder {
        match order {
            chfl_bond_order::CHFL_BOND_UNKNOWN => BondOrder::Unknown,
            chfl_bond_order::CHFL_BOND_SINGLE => BondOrder::Single,
            chfl_bond_order::CHFL_BOND_DOUBLE => BondOrder::Double,
            chfl_bond_order::CHFL_BOND_TRIPLE => BondOrder::Triple,
            chfl_bond_order::CHFL_BOND_QUADRUPLE => BondOrder::Quadruple,
            chfl_bond_order::CHFL_BOND_QUINTUPLET => BondOrder::Quintuplet,
            chfl_bond_order::CHFL_BOND_AMIDE => BondOrder::Amide,
            chfl_bond_order::CHFL_BOND_AROMATIC => BondOrder::Aromatic,
        }
    }
}

/// A `Topology` contains the definition of all the atoms in the system, and
/// the liaisons between the atoms (bonds, angles, dihedrals, ...). It will
/// also contain all the residues information if it is available.
pub struct Topology {
    handle: *mut CHFL_TOPOLOGY,
}

/// An analog to a reference to a topology (`&Topology`)
pub struct TopologyRef<'a> {
    inner: Topology,
    marker: PhantomData<&'a Topology>,
}

impl<'a> Deref for TopologyRef<'a> {
    type Target = Topology;
    fn deref(&self) -> &Topology {
        &self.inner
    }
}

impl Clone for Topology {
    fn clone(&self) -> Topology {
        unsafe {
            let new_handle = chfl_topology_copy(self.as_ptr());
            Topology::from_ptr(new_handle)
        }
    }
}

impl Topology {
    /// Create a `Topology` from a C pointer.
    ///
    /// This function is unsafe because no validity check is made on the pointer,
    /// except for it being non-null.
    #[inline]
    pub(crate) unsafe fn from_ptr(ptr: *mut CHFL_TOPOLOGY) -> Topology {
        check_not_null(ptr);
        Topology { handle: ptr }
    }

    /// Create a borrowed `Topology` from a C pointer.
    ///
    /// This function is unsafe because no validity check is made on the
    /// pointer, except for it being non-null, and the caller is responsible
    /// for setting the right lifetime
    #[inline]
    pub(crate) unsafe fn ref_from_ptr<'a>(ptr: *const CHFL_TOPOLOGY) -> TopologyRef<'a> {
        TopologyRef {
            inner: Topology::from_ptr(ptr as *mut CHFL_TOPOLOGY),
            marker: PhantomData,
        }
    }

    /// Get the underlying C pointer as a const pointer.
    #[inline]
    pub(crate) fn as_ptr(&self) -> *const CHFL_TOPOLOGY {
        self.handle
    }

    /// Get the underlying C pointer as a mutable pointer.
    #[inline]
    pub(crate) fn as_mut_ptr(&mut self) -> *mut CHFL_TOPOLOGY {
        self.handle
    }

    /// Get the underlying C pointer as a mutable pointer FROM A SHARED REFERENCE.
    ///
    /// For uses with functions of the C API using mut pointers for both read
    /// and write access. Users should check that this does not lead to multiple
    /// mutable borrows
    #[inline]
    #[allow(non_snake_case)]
    pub(crate) fn as_mut_ptr_MANUALLY_CHECKING_BORROW(&self) -> *mut CHFL_TOPOLOGY {
        self.handle
    }

    /// Create a new empty topology.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::Topology;
    /// let topology = Topology::new();
    /// assert_eq!(topology.size(), 0);
    /// ```
    pub fn new() -> Topology {
        unsafe { Topology::from_ptr(chfl_topology()) }
    }

    /// Get a reference of the atom at the given `index` in this topology.
    ///
    /// # Panics
    ///
    /// If `index` is out of bounds.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::Topology;
    /// let mut topology = Topology::new();
    /// topology.resize(6);
    ///
    /// let atom = topology.atom(4);
    /// assert_eq!(atom.name(), "");
    /// ```
    pub fn atom(&self, index: usize) -> AtomRef {
        unsafe {
            let handle = chfl_atom_from_topology(self.as_mut_ptr_MANUALLY_CHECKING_BORROW(), index as u64);
            Atom::ref_from_ptr(handle)
        }
    }

    /// Get a mutable reference to the atom at the given `index` in this topology.
    ///
    /// # Panics
    ///
    /// If `index` is out of bounds.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::Topology;
    /// let mut topology = Topology::new();
    /// topology.resize(6);
    ///
    /// assert_eq!(topology.atom(4).name(), "");
    ///
    /// topology.atom_mut(4).set_name("Fe");
    /// assert_eq!(topology.atom(4).name(), "Fe");
    /// ```
    pub fn atom_mut(&mut self, index: usize) -> AtomMut {
        unsafe {
            let handle = chfl_atom_from_topology(self.as_mut_ptr(), index as u64);
            Atom::ref_mut_from_ptr(handle)
        }
    }

    /// Get the current number of atoms in this topology.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::Topology;
    /// let mut topology = Topology::new();
    /// assert_eq!(topology.size(), 0);
    ///
    /// topology.resize(6);
    /// assert_eq!(topology.size(), 6);
    /// ```
    pub fn size(&self) -> usize {
        let mut size = 0;
        unsafe {
            check_success(chfl_topology_atoms_count(self.as_ptr(), &mut size));
        }
        #[allow(clippy::cast_possible_truncation)]
        return size as usize;
    }

    /// Resize this topology to hold `natoms` atoms, inserting dummy atoms if
    /// the new size if bigger than the old one.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::Topology;
    /// let mut topology = Topology::new();
    /// assert_eq!(topology.size(), 0);
    ///
    /// topology.resize(6);
    /// assert_eq!(topology.size(), 6);
    /// ```
    pub fn resize(&mut self, natoms: usize) {
        unsafe {
            check_success(chfl_topology_resize(self.as_mut_ptr(), natoms as u64));
        }
    }

    /// Add an `Atom` at the end of this topology
    ///
    /// # Example
    /// ```
    /// # use chemfiles::{Topology, Atom};
    /// let mut topology = Topology::new();
    /// topology.add_atom(&Atom::new("Mg"));
    ///
    /// let atom = topology.atom(0);
    /// assert_eq!(atom.name(), "Mg");
    /// ```
    pub fn add_atom(&mut self, atom: &Atom) {
        unsafe {
            check_success(chfl_topology_add_atom(self.as_mut_ptr(), atom.as_ptr()));
        }
    }

    /// Remove an `Atom` from this topology by `index`. This modify all the
    /// other atoms indexes.
    ///
    /// # Panics
    ///
    /// If the `index` is out of bounds
    ///
    /// # Example
    /// ```
    /// # use chemfiles::Topology;
    /// let mut topology = Topology::new();
    /// topology.resize(9);
    /// assert_eq!(topology.size(), 9);
    ///
    /// topology.remove(7);
    /// assert_eq!(topology.size(), 8);
    /// ```
    pub fn remove(&mut self, index: usize) {
        unsafe {
            check_success(chfl_topology_remove(self.as_mut_ptr(), index as u64));
        }
    }

    /// Get the number of bonds in the topology.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::Topology;
    /// let mut topology = Topology::new();
    /// assert_eq!(topology.bonds_count(), 0);
    /// topology.resize(4);
    ///
    /// topology.add_bond(0, 1);
    /// topology.add_bond(2, 1);
    /// topology.add_bond(2, 3);
    /// assert_eq!(topology.bonds_count(), 3);
    /// ```
    pub fn bonds_count(&self) -> usize {
        let mut count = 0;
        unsafe {
            check_success(chfl_topology_bonds_count(self.as_ptr(), &mut count));
        }
        #[allow(clippy::cast_possible_truncation)]
        return count as usize;
    }

    /// Get the number of angles in the topology.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::Topology;
    /// let mut topology = Topology::new();
    /// assert_eq!(topology.angles_count(), 0);
    /// topology.resize(4);
    ///
    /// topology.add_bond(0, 1);
    /// topology.add_bond(2, 1);
    /// topology.add_bond(2, 3);
    /// assert_eq!(topology.angles_count(), 2);
    /// ```
    pub fn angles_count(&self) -> usize {
        let mut count = 0;
        unsafe {
            check_success(chfl_topology_angles_count(self.as_ptr(), &mut count));
        }
        #[allow(clippy::cast_possible_truncation)]
        return count as usize;
    }

    /// Get the number of dihedral angles in the topology.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::Topology;
    /// let mut topology = Topology::new();
    /// assert_eq!(topology.dihedrals_count(), 0);
    /// topology.resize(4);
    ///
    /// topology.add_bond(0, 1);
    /// topology.add_bond(2, 1);
    /// topology.add_bond(2, 3);
    /// assert_eq!(topology.dihedrals_count(), 1);
    /// ```
    pub fn dihedrals_count(&self) -> usize {
        let mut count = 0;
        unsafe {
            check_success(chfl_topology_dihedrals_count(self.as_ptr(), &mut count));
        }
        #[allow(clippy::cast_possible_truncation)]
        return count as usize;
    }

    /// Get the number of improper dihedral angles in the topology.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::Topology;
    /// let mut topology = Topology::new();
    /// assert_eq!(topology.dihedrals_count(), 0);
    /// topology.resize(4);
    ///
    /// topology.add_bond(0, 1);
    /// topology.add_bond(0, 2);
    /// topology.add_bond(0, 3);
    /// assert_eq!(topology.impropers_count(), 1);
    /// ```
    pub fn impropers_count(&self) -> usize {
        let mut count = 0;
        unsafe {
            check_success(chfl_topology_impropers_count(self.as_ptr(), &mut count));
        }
        #[allow(clippy::cast_possible_truncation)]
        return count as usize;
    }

    /// Get the list of bonds in the topology.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::Topology;
    /// let mut topology = Topology::new();
    /// topology.resize(4);
    ///
    /// topology.add_bond(0, 1);
    /// topology.add_bond(2, 1);
    /// topology.add_bond(2, 3);
    /// assert_eq!(topology.bonds(), vec![[0, 1], [1, 2], [2, 3]]);
    /// ```
    pub fn bonds(&self) -> Vec<[usize; 2]> {
        let size = self.bonds_count();
        let count = size as u64;
        let mut bonds = vec![[u64::max_value(); 2]; size];
        unsafe {
            check_success(chfl_topology_bonds(self.as_ptr(), bonds.as_mut_ptr(), count));
        }
        #[allow(clippy::cast_possible_truncation)]
        return bonds
            .into_iter()
            .map(|bond| [bond[0] as usize, bond[1] as usize])
            .collect();
    }

    /// Get the list of angles in the topology.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::Topology;
    /// let mut topology = Topology::new();
    /// topology.resize(4);
    ///
    /// topology.add_bond(0, 1);
    /// topology.add_bond(2, 1);
    /// topology.add_bond(2, 3);
    /// assert_eq!(topology.angles(), vec![[0, 1, 2], [1, 2, 3]]);
    /// ```
    pub fn angles(&self) -> Vec<[usize; 3]> {
        let size = self.angles_count();
        let count = size as u64;
        let mut angles = vec![[u64::max_value(); 3]; size];
        unsafe {
            check_success(chfl_topology_angles(self.as_ptr(), angles.as_mut_ptr(), count));
        }
        #[allow(clippy::cast_possible_truncation)]
        return angles
            .into_iter()
            .map(|angle| [angle[0] as usize, angle[1] as usize, angle[2] as usize])
            .collect();
    }

    /// Get the list of dihedral angles in the topology.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::Topology;
    /// let mut topology = Topology::new();
    /// topology.resize(4);
    ///
    /// topology.add_bond(0, 1);
    /// topology.add_bond(2, 1);
    /// topology.add_bond(2, 3);
    ///
    /// assert_eq!(topology.dihedrals(), vec![[0, 1, 2, 3]]);
    /// ```
    pub fn dihedrals(&self) -> Vec<[usize; 4]> {
        let size = self.dihedrals_count();
        let count = size as u64;
        let mut dihedrals = vec![[u64::max_value(); 4]; size];
        unsafe {
            check_success(chfl_topology_dihedrals(self.as_ptr(), dihedrals.as_mut_ptr(), count));
        }
        #[allow(clippy::cast_possible_truncation)]
        return dihedrals
            .into_iter()
            .map(|dihedral| {
                [
                    dihedral[0] as usize,
                    dihedral[1] as usize,
                    dihedral[2] as usize,
                    dihedral[3] as usize,
                ]
            })
            .collect();
    }

    /// Get the list of improper dihedral angles in the topology.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::Topology;
    /// let mut topology = Topology::new();
    /// topology.resize(4);
    ///
    /// topology.add_bond(0, 1);
    /// topology.add_bond(0, 2);
    /// topology.add_bond(0, 3);
    ///
    /// assert_eq!(topology.impropers(), vec![[1, 0, 2, 3]]);
    /// ```
    pub fn impropers(&self) -> Vec<[usize; 4]> {
        let size = self.impropers_count();
        let count = size as u64;
        let mut impropers = vec![[u64::max_value(); 4]; size];
        unsafe {
            check_success(chfl_topology_impropers(self.as_ptr(), impropers.as_mut_ptr(), count));
        }
        #[allow(clippy::cast_possible_truncation)]
        return impropers
            .into_iter()
            .map(|improper| {
                [
                    improper[0] as usize,
                    improper[1] as usize,
                    improper[2] as usize,
                    improper[3] as usize,
                ]
            })
            .collect();
    }

    /// Remove all existing bonds, angles, dihedral angles and improper
    /// dihedral angles in the topology.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::Topology;
    /// let mut topology = Topology::new();
    /// assert_eq!(topology.bonds_count(), 0);
    /// topology.resize(4);
    /// topology.add_bond(0, 1);
    /// topology.add_bond(0, 2);
    /// assert_eq!(topology.bonds_count(), 2);
    /// assert_eq!(topology.angles().len(), 1);
    ///
    /// topology.clear_bonds();
    /// assert!(topology.bonds().is_empty());
    /// assert!(topology.angles().is_empty());
    /// ```
    pub fn clear_bonds(&mut self) {
        unsafe {
            check_success(chfl_topology_clear_bonds(self.as_mut_ptr()));
        }
    }

    /// Add a bond between the atoms at indexes `i` and `j` in the topology.
    ///
    /// The bond order is set to `BondOrder::Unknown`.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::{Topology, BondOrder};
    /// let mut topology = Topology::new();
    /// assert_eq!(topology.bonds_count(), 0);
    /// topology.resize(4);
    ///
    /// topology.add_bond(0, 1);
    /// topology.add_bond(0, 2);
    /// assert_eq!(topology.bonds_count(), 2);
    ///
    /// assert_eq!(topology.bond_order(0, 1), BondOrder::Unknown);
    /// ```
    pub fn add_bond(&mut self, i: usize, j: usize) {
        unsafe {
            check_success(chfl_topology_add_bond(self.as_mut_ptr(), i as u64, j as u64));
        }
    }

    /// Add a bond between the atoms at indexes `i` and `j` in the topology
    /// with the given bond `order`.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::{Topology, BondOrder};
    /// let mut topology = Topology::new();
    /// assert_eq!(topology.bonds_count(), 0);
    /// topology.resize(2);
    ///
    /// topology.add_bond_with_order(0, 1, BondOrder::Double);
    /// assert_eq!(topology.bond_order(0, 1), BondOrder::Double);
    /// ```
    pub fn add_bond_with_order(&mut self, i: usize, j: usize, order: BondOrder) {
        unsafe {
            check_success(chfl_topology_bond_with_order(
                self.as_mut_ptr(),
                i as u64,
                j as u64,
                order.as_raw(),
            ));
        }
    }

    /// Get the bond order for the bond between the atoms at indexes `i` and
    /// `j`.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::{Topology, BondOrder};
    /// let mut topology = Topology::new();
    /// assert_eq!(topology.bonds_count(), 0);
    /// topology.resize(2);
    ///
    /// topology.add_bond_with_order(0, 1, BondOrder::Double);
    /// assert_eq!(topology.bond_order(0, 1), BondOrder::Double);
    /// ```
    pub fn bond_order(&self, i: usize, j: usize) -> BondOrder {
        let mut order = chfl_bond_order::CHFL_BOND_UNKNOWN;
        unsafe {
            check_success(chfl_topology_bond_order(self.as_ptr(), i as u64, j as u64, &mut order));
        }
        return order.into();
    }

    /// Get the bond order for all the bonds in the topology
    ///
    /// # Example
    /// ```
    /// # use chemfiles::{Topology, BondOrder};
    /// let mut topology = Topology::new();
    /// assert_eq!(topology.bonds_count(), 0);
    /// topology.resize(3);
    ///
    /// topology.add_bond_with_order(0, 1, BondOrder::Double);
    /// topology.add_bond_with_order(0, 2, BondOrder::Single);
    ///
    /// assert_eq!(topology.bond_orders(), &[BondOrder::Double, BondOrder::Single]);
    /// ```
    pub fn bond_orders(&self) -> Vec<BondOrder> {
        let size = self.bonds_count();
        let count = size as u64;
        let mut bonds = vec![BondOrder::Unknown; size];
        unsafe {
            check_success(chfl_topology_bond_orders(
                self.as_ptr(),
                // Casting BondOrder to chfl_bond_order is safe, as they are
                // both `repr(C)` enums with the same values.
                bonds.as_mut_ptr().cast(),
                count,
            ));
        }
        return bonds;
    }

    /// Remove any existing bond between the atoms at indexes `i` and `j` in
    /// this topology.
    ///
    /// This function does nothing if there is no bond between `i` and `j`.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::Topology;
    /// let mut topology = Topology::new();
    /// assert_eq!(topology.bonds_count(), 0);
    /// topology.resize(4);
    ///
    /// topology.add_bond(0, 1);
    /// topology.add_bond(1, 2);
    /// assert_eq!(topology.bonds_count(), 2);
    ///
    /// topology.remove_bond(0, 1);
    /// assert_eq!(topology.bonds_count(), 1);
    ///
    /// // Removing a bond that does not exists is fine
    /// topology.remove_bond(0, 2);
    /// assert_eq!(topology.bonds_count(), 1);
    /// ```
    pub fn remove_bond(&mut self, i: usize, j: usize) {
        unsafe {
            check_success(chfl_topology_remove_bond(self.as_mut_ptr(), i as u64, j as u64));
        }
    }

    /// Get a reference to the residue at index `index` from this topology.
    ///
    /// The residue index in the topology is not always the same as the residue
    /// `id`.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::{Topology, Residue};
    /// let mut topology = Topology::new();
    /// topology.add_residue(&Residue::new("water")).unwrap();
    ///
    /// let residue = topology.residue(0).unwrap();
    /// assert_eq!(residue.name(), "water");
    /// ```
    pub fn residue(&self, index: u64) -> Option<ResidueRef> {
        unsafe {
            let handle = chfl_residue_from_topology(self.as_ptr(), index as u64);
            if handle.is_null() {
                None
            } else {
                Some(Residue::ref_from_ptr(handle))
            }
        }
    }

    /// Get a copy of the residue containing the atom at index `index` in this
    /// topology, if any.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::{Topology, Residue};
    /// let mut topology = Topology::new();
    /// topology.resize(8);
    ///
    /// let mut residue = Residue::new("water");
    /// residue.add_atom(0);
    /// residue.add_atom(1);
    /// residue.add_atom(2);
    /// topology.add_residue(&residue).unwrap();
    ///
    /// let residue = topology.residue_for_atom(0).unwrap();
    /// assert_eq!(residue.name(), "water");
    ///
    /// assert!(topology.residue_for_atom(6).is_none());
    /// ```
    pub fn residue_for_atom(&self, index: usize) -> Option<ResidueRef> {
        let handle = unsafe { chfl_residue_for_atom(self.as_ptr(), index as u64) };
        if handle.is_null() {
            None
        } else {
            unsafe { Some(Residue::ref_from_ptr(handle)) }
        }
    }

    /// Get the number of residues in this topology.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::{Topology, Residue};
    /// let mut topology = Topology::new();
    /// assert_eq!(topology.residues_count(), 0);
    ///
    /// topology.add_residue(&Residue::with_id("water", 0)).unwrap();
    /// topology.add_residue(&Residue::with_id("protein", 1)).unwrap();
    /// assert_eq!(topology.residues_count(), 2);
    /// ```
    pub fn residues_count(&self) -> u64 {
        let mut count = 0;
        unsafe {
            check_success(chfl_topology_residues_count(self.as_ptr(), &mut count));
        }
        return count;
    }

    /// Add a residue to this topology.
    ///
    /// # Errors
    ///
    /// This function fails is the residue `id` is not already in the topology,
    /// or if the residue contains atoms that are already in another residue.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::{Topology, Residue};
    /// let mut topology = Topology::new();
    /// topology.add_residue(&Residue::new("water")).unwrap();
    ///
    /// let residue = topology.residue(0).unwrap();
    /// assert_eq!(residue.name(), "water");
    /// ```
    pub fn add_residue(&mut self, residue: &Residue) -> Result<(), Error> {
        unsafe { check(chfl_topology_add_residue(self.as_mut_ptr(), residue.as_ptr())) }
    }

    /// Check if the two residues `first` and `second` from the `topology` are
    /// linked together, *i.e.* if there is a bond between one atom in the
    /// first residue and one atom in the second one.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::{Topology, Residue};
    /// let mut topology = Topology::new();
    ///
    /// topology.add_residue(&Residue::with_id("water", 0)).unwrap();
    /// topology.add_residue(&Residue::with_id("protein", 1)).unwrap();
    ///
    /// let first = topology.residue(0).unwrap();
    /// let second = topology.residue(1).unwrap();
    /// assert_eq!(topology.are_linked(&first, &second), false);
    /// ```
    pub fn are_linked(&self, first: &Residue, second: &Residue) -> bool {
        let mut linked = 0;
        unsafe {
            check_success(chfl_topology_residues_linked(
                self.as_ptr(),
                first.as_ptr(),
                second.as_ptr(),
                &mut linked,
            ));
        }
        return linked != 0;
    }
}

impl Drop for Topology {
    fn drop(&mut self) {
        unsafe {
            let _ = chfl_free(self.as_ptr().cast());
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use {Atom, Residue};

    #[test]
    fn clone() {
        let mut topology = Topology::new();
        assert_eq!(topology.size(), 0);

        let copy = topology.clone();
        assert_eq!(copy.size(), 0);

        topology.resize(10);
        assert_eq!(topology.size(), 10);
        assert_eq!(copy.size(), 0);
    }

    #[test]
    fn size() {
        let mut topology = Topology::new();
        assert_eq!(topology.size(), 0);

        topology.resize(10);
        assert_eq!(topology.size(), 10);

        topology.remove(7);
        assert_eq!(topology.size(), 9);

        topology.add_atom(&Atom::new("Hg"));
        assert_eq!(topology.size(), 10);
    }

    #[test]
    fn atoms() {
        let mut topology = Topology::new();

        topology.add_atom(&Atom::new("Hg"));
        topology.add_atom(&Atom::new("Mn"));
        topology.add_atom(&Atom::new("W"));
        topology.add_atom(&Atom::new("Fe"));

        assert_eq!(topology.atom(0).name(), "Hg");
        assert_eq!(topology.atom(3).name(), "Fe");
    }

    #[test]
    fn remove() {
        let mut topology = Topology::new();
        topology.add_atom(&Atom::new("Hg"));
        topology.add_atom(&Atom::new("Mn"));
        topology.add_atom(&Atom::new("W"));
        topology.add_atom(&Atom::new("Fe"));

        assert_eq!(topology.atom(0).name(), "Hg");
        assert_eq!(topology.atom(2).name(), "W");

        topology.remove(1);
        assert_eq!(topology.atom(0).name(), "Hg");
        assert_eq!(topology.atom(2).name(), "Fe");
    }

    #[test]
    #[should_panic]
    fn out_of_bounds_remove() {
        let mut topology = Topology::new();
        topology.resize(18);
        topology.remove(33);
    }

    #[test]
    fn bonds() {
        let mut topology = Topology::new();
        topology.resize(12);
        assert_eq!(topology.bonds_count(), 0);

        topology.add_bond(0, 1);
        topology.add_bond(9, 2);
        topology.add_bond_with_order(3, 7, BondOrder::Aromatic);
        assert_eq!(topology.bonds_count(), 3);

        assert_eq!(topology.bonds(), vec![[0, 1], [2, 9], [3, 7]]);
        let expected = vec![BondOrder::Unknown, BondOrder::Unknown, BondOrder::Aromatic];
        assert_eq!(topology.bond_orders(), expected);

        assert_eq!(topology.bond_order(0, 1), BondOrder::Unknown);
        assert_eq!(topology.bond_order(3, 7), BondOrder::Aromatic);

        topology.remove_bond(3, 7);
        // Removing unexisting bond is OK if both indexes are in bounds
        topology.remove_bond(8, 7);
        assert_eq!(topology.bonds_count(), 2);

        topology.clear_bonds();
        assert_eq!(topology.bonds_count(), 0);
    }

    #[test]
    #[should_panic]
    fn out_of_bounds_bonds() {
        let mut topology = Topology::new();
        topology.resize(12);
        topology.add_bond(300, 7);
    }

    #[test]
    #[should_panic]
    fn out_of_bounds_remove_bond() {
        let mut topology = Topology::new();
        topology.resize(12);
        topology.remove_bond(300, 7);
    }

    #[test]
    #[should_panic]
    fn out_of_bounds_bonds_with_order() {
        let mut topology = Topology::new();
        topology.resize(12);
        topology.add_bond_with_order(300, 7, BondOrder::Unknown);
    }

    #[test]
    fn angles() {
        let mut topology = Topology::new();
        topology.resize(12);
        assert_eq!(topology.angles_count(), 0);

        topology.add_bond(0, 1);
        topology.add_bond(1, 2);
        topology.add_bond(3, 7);
        topology.add_bond(3, 5);
        assert_eq!(topology.angles_count(), 2);

        assert_eq!(topology.angles(), vec![[0, 1, 2], [5, 3, 7]]);

        topology.clear_bonds();
        assert_eq!(topology.angles_count(), 0);
    }

    #[test]
    fn dihedrals() {
        let mut topology = Topology::new();
        topology.resize(12);
        assert_eq!(topology.dihedrals_count(), 0);

        topology.add_bond(0, 1);
        topology.add_bond(1, 2);
        topology.add_bond(3, 2);
        topology.add_bond(4, 7);
        topology.add_bond(4, 5);
        topology.add_bond(7, 10);
        assert_eq!(topology.dihedrals_count(), 2);

        assert_eq!(topology.dihedrals(), vec![[0, 1, 2, 3], [5, 4, 7, 10]]);

        topology.clear_bonds();
        assert_eq!(topology.dihedrals_count(), 0);
    }

    #[test]
    fn impropers() {
        let mut topology = Topology::new();
        topology.resize(12);
        assert_eq!(topology.impropers_count(), 0);

        topology.add_bond(0, 1);
        topology.add_bond(0, 2);
        topology.add_bond(0, 3);
        topology.add_bond(4, 7);
        topology.add_bond(4, 5);
        topology.add_bond(4, 8);
        assert_eq!(topology.impropers_count(), 2);

        assert_eq!(topology.impropers(), vec![[1, 0, 2, 3], [5, 4, 7, 8]]);

        topology.clear_bonds();
        assert_eq!(topology.impropers_count(), 0);
    }

    #[test]
    fn residues() {
        let mut topology = Topology::new();
        topology.resize(4);
        assert_eq!(topology.residues_count(), 0);

        let mut residue = Residue::new("Foo");
        residue.add_atom(0);
        residue.add_atom(2);

        topology.add_residue(&residue).unwrap();
        assert_eq!(topology.residues_count(), 1);

        assert_eq!(topology.residue(0).unwrap().name(), "Foo");
        {
            let residue = topology.residue_for_atom(2).unwrap();
            assert_eq!(residue.name(), "Foo");
        }

        let mut residue = Residue::new("Bar");
        residue.add_atom(3);
        topology.add_residue(&residue).unwrap();
        assert_eq!(topology.residues_count(), 2);

        let first = topology.residue(0).unwrap();
        let second = topology.residue(0).unwrap();
        assert!(topology.are_linked(&first, &second));

        // missing residue
        assert!(topology.residue_for_atom(1).is_none());
        // out of bounds
        assert!(topology.residue_for_atom(67).is_none());
    }
}

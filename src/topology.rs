// Chemfiles, a modern library for chemistry file reading and writing
// Copyright (C) 2015-2018 Guillaume Fraux -- BSD licensed
use std::ops::{Drop, Deref};
use std::marker::PhantomData;
use std::u64;

use chemfiles_sys::*;
use errors::{check, check_not_null, check_success, Error};
use super::{Atom, AtomRef, AtomMut};
use super::{Residue, ResidueRef};

/// A `Topology` contains the definition of all the atoms in the system, and
/// the liaisons between the atoms (bonds, angles, dihedrals, ...). It will
/// also contain all the residues information if it is available.
pub struct Topology {
    handle: *mut CHFL_TOPOLOGY,
}

/// An analog to a reference to a topology (`&Topology`)
pub struct TopologyRef<'a> {
    inner: Topology,
    marker: PhantomData<&'a Topology>
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
        Topology {
            handle: ptr
        }
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
        unsafe {
            Topology::from_ptr(chfl_topology())
        }
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
    pub fn atom(&self, index: u64) -> AtomRef {
        unsafe {
            let handle = chfl_atom_from_topology(
                self.as_mut_ptr_MANUALLY_CHECKING_BORROW(), index
            );
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
    pub fn atom_mut(&mut self, index: u64) -> AtomMut {
        unsafe {
            let handle = chfl_atom_from_topology(
                self.as_mut_ptr(), index
            );
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
    pub fn size(&self) -> u64 {
        let mut size = 0;
        unsafe {
            check_success(chfl_topology_atoms_count(self.as_ptr(), &mut size));
        }
        return size;
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
    pub fn resize(&mut self, natoms: u64) {
        unsafe {
            check_success(chfl_topology_resize(self.as_mut_ptr(), natoms));
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
    pub fn remove(&mut self, index: u64) {
        unsafe {
            check_success(chfl_topology_remove(self.as_mut_ptr(), index));
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
    pub fn bonds_count(&self) -> u64 {
        let mut count = 0;
        unsafe {
            check_success(chfl_topology_bonds_count(self.as_ptr(), &mut count));
        }
        return count;
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
    pub fn angles_count(&self) -> u64 {
        let mut count = 0;
        unsafe {
            check_success(chfl_topology_angles_count(self.as_ptr(), &mut count));
        }
        return count;
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
    pub fn dihedrals_count(&self) -> u64 {
        let mut count = 0;
        unsafe {
            check_success(chfl_topology_dihedrals_count(self.as_ptr(), &mut count));
        }
        return count;
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
    pub fn impropers_count(&self) -> u64 {
        let mut count = 0;
        unsafe {
            check_success(chfl_topology_impropers_count(self.as_ptr(), &mut count));
        }
        return count;
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
    pub fn bonds(&self) -> Vec<[u64; 2]> {
        let count = self.bonds_count();
        #[allow(cast_possible_truncation)]
        let size = count as usize;
        let mut bonds = vec![[u64::max_value(); 2]; size];
        unsafe {
            check_success(chfl_topology_bonds(self.handle, bonds.as_mut_ptr(), count));
        }
        return bonds;
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
    pub fn angles(&self) -> Vec<[u64; 3]> {
        let count = self.angles_count();
        #[allow(cast_possible_truncation)]
        let size = count as usize;
        let mut angles = vec![[u64::max_value(); 3]; size];
        unsafe {
            check_success(chfl_topology_angles(self.as_ptr(), angles.as_mut_ptr(), count));
        }
        return angles;
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
    pub fn dihedrals(&self) -> Vec<[u64; 4]> {
        let count = self.dihedrals_count();
        #[allow(cast_possible_truncation)]
        let size = count as usize;
        let mut dihedrals = vec![[u64::max_value(); 4]; size];
        unsafe {
            check_success(chfl_topology_dihedrals(
                self.as_ptr(), dihedrals.as_mut_ptr(), count
            ));
        }
        return dihedrals;
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
    pub fn impropers(&self) -> Vec<[u64; 4]> {
        let count = self.impropers_count();
        #[allow(cast_possible_truncation)]
        let size = count as usize;
        let mut impropers = vec![[u64::max_value(); 4]; size];
        unsafe {
            check_success(chfl_topology_impropers(
                self.as_ptr(), impropers.as_mut_ptr(), count
            ));
        }
        return impropers;
    }

    /// Add a bond between the atoms at indexes `i` and `j` in the topology.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::Topology;
    /// let mut topology = Topology::new();
    /// assert_eq!(topology.bonds_count(), 0);
    /// topology.resize(4);
    ///
    /// topology.add_bond(0, 1);
    /// topology.add_bond(0, 2);
    /// assert_eq!(topology.bonds_count(), 2);
    /// ```
    pub fn add_bond(&mut self, i: u64, j: u64) {
        unsafe {
            check_success(chfl_topology_add_bond(self.as_mut_ptr(), i, j));
        }
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
    pub fn remove_bond(&mut self, i: u64, j: u64) {
        unsafe {
            check_success(chfl_topology_remove_bond(self.as_mut_ptr(), i, j));
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
            let handle = chfl_residue_from_topology(self.as_ptr(), index);
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
    pub fn residue_for_atom(&self, index: u64) -> Option<ResidueRef> {
        let handle = unsafe {
            chfl_residue_for_atom(self.as_ptr(), index)
        };
        if handle.is_null() {
            None
        } else {
            unsafe {
                Some(Residue::ref_from_ptr(handle))
            }
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
    /// The residue `id` must not already be in the topology, and the residue
    /// must contain only atoms that are not already in another residue.
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
        unsafe {
            check(chfl_topology_add_residue(self.as_mut_ptr(), residue.as_ptr()))
        }
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
                &mut linked
            ));
        }
        return linked != 0;
    }
}

impl Drop for Topology {
    fn drop(&mut self) {
        unsafe {
            let status = chfl_topology_free(self.as_mut_ptr());
            debug_assert_eq!(status, chfl_status::CHFL_SUCCESS);
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
        topology.add_bond(3, 7);
        assert_eq!(topology.bonds_count(), 3);

        assert_eq!(topology.bonds(), vec![[0, 1], [2, 9], [3, 7]]);

        topology.remove_bond(3, 7);
        // Removing unexisting bond is OK
        topology.remove_bond(8, 7);
        assert_eq!(topology.bonds_count(), 2);
    }

    #[test]
    #[should_panic]
    fn out_of_bounds_bonds() {
        let mut topology = Topology::new();
        topology.resize(12);
        // Adding a bond between non-existing atoms is Ok
        topology.add_bond(300, 7);
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
        assert_eq!(topology.are_linked(&first, &second), true);

        // missing residue
        assert!(topology.residue_for_atom(1).is_none());
        // out of bounds
        assert!(topology.residue_for_atom(67).is_none());
    }
}

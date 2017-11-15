// Chemfiles, a modern library for chemistry file reading and writing
// Copyright (C) 2015-2017 Guillaume Fraux
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/
use std::ops::Drop;
use std::u64;

use chemfiles_sys::*;
use errors::{check, Error};
use {Atom, Residue};
use Result;

/// A `Topology` contains the definition of all the atoms in the system, and
/// the liaisons between the atoms (bonds, angles, dihedrals, ...). It will
/// also contain all the residues information if it is available.
pub struct Topology {
    handle: *const CHFL_TOPOLOGY,
}

impl Clone for Topology {
    fn clone(&self) -> Topology {
        unsafe {
            let new_handle = chfl_topology_copy(self.as_ptr());
            Topology::from_ptr(new_handle).expect("Out of memory when copying a Topology")
        }
    }
}

impl Topology {
    /// Create a `Topology` from a C pointer.
    ///
    /// This function is unsafe because no validity check is made on the pointer,
    /// except for it being non-null.
    #[inline]
    #[doc(hidden)]
    pub unsafe fn from_ptr(ptr: *const CHFL_TOPOLOGY) -> Result<Topology> {
        if ptr.is_null() {
            Err(Error::null_ptr())
        } else {
            Ok(Topology { handle: ptr })
        }
    }

    /// Get the underlying C pointer as a const pointer.
    #[inline]
    #[doc(hidden)]
    pub fn as_ptr(&self) -> *const CHFL_TOPOLOGY {
        self.handle
    }

    /// Get the underlying C pointer as a mutable pointer.
    #[inline]
    #[doc(hidden)]
    pub fn as_mut_ptr(&mut self) -> *mut CHFL_TOPOLOGY {
        self.handle as *mut CHFL_TOPOLOGY
    }

    /// Create a new empty topology.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::Topology;
    /// let topology = Topology::new().unwrap();
    /// assert_eq!(topology.size(), Ok(0));
    /// ```
    pub fn new() -> Result<Topology> {
        unsafe {
            let handle = chfl_topology();
            Topology::from_ptr(handle)
        }
    }

    /// Get a copy of the atom at index `index` from this topology.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::Topology;
    /// let mut topology = Topology::new().unwrap();
    /// topology.resize(6).unwrap();
    ///
    /// let atom = topology.atom(4).unwrap();
    /// assert_eq!(atom.name(), Ok(String::new()));
    /// ```
    pub fn atom(&self, index: u64) -> Result<Atom> {
        unsafe {
            let handle = chfl_atom_from_topology(self.as_ptr(), index);
            Atom::from_ptr(handle)
        }
    }

    /// Get the current number of atoms in this topology.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::Topology;
    /// let mut topology = Topology::new().unwrap();
    /// assert_eq!(topology.size(), Ok(0));
    ///
    /// topology.resize(6).unwrap();
    /// assert_eq!(topology.size(), Ok(6));
    /// ```
    pub fn size(&self) -> Result<u64> {
        let mut natoms = 0;
        unsafe {
            try!(check(chfl_topology_atoms_count(self.as_ptr(), &mut natoms)));
        }
        return Ok(natoms);
    }

    /// Resize this topology to hold `natoms` atoms, inserting dummy atoms if
    /// the new size if bigger than the old one.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::Topology;
    /// let mut topology = Topology::new().unwrap();
    /// assert_eq!(topology.size(), Ok(0));
    ///
    /// topology.resize(6).unwrap();
    /// assert_eq!(topology.size(), Ok(6));
    /// ```
    pub fn resize(&mut self, natoms: u64) -> Result<()> {
        unsafe {
            try!(check(chfl_topology_resize(self.as_mut_ptr(), natoms)));
        }
        return Ok(());
    }

    /// Add an `Atom` at the end of this topology
    ///
    /// # Example
    /// ```
    /// # use chemfiles::{Topology, Atom};
    /// let mut topology = Topology::new().unwrap();
    /// topology.add_atom(&Atom::new("Mg").unwrap()).unwrap();
    ///
    /// let atom = topology.atom(0).unwrap();
    /// assert_eq!(atom.name(), Ok(String::from("Mg")));
    /// ```
    pub fn add_atom(&mut self, atom: &Atom) -> Result<()> {
        unsafe {
            try!(check(chfl_topology_add_atom(self.as_mut_ptr(), atom.as_ptr())));
        }
        return Ok(());
    }

    /// Remove an `Atom` from this topology by index. This modify all the other
    /// atoms indexes.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::Topology;
    /// let mut topology = Topology::new().unwrap();
    /// topology.resize(9).unwrap();
    /// assert_eq!(topology.size(), Ok(9));
    ///
    /// topology.remove(7).unwrap();
    /// assert_eq!(topology.size(), Ok(8));
    /// ```
    pub fn remove(&mut self, index: u64) -> Result<()> {
        unsafe {
            try!(check(chfl_topology_remove(self.as_mut_ptr(), index)));
        }
        return Ok(());
    }

    /// Get the number of bonds in the topology.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::{Topology, Atom};
    /// let mut topology = Topology::new().unwrap();
    /// assert_eq!(topology.bonds_count(), Ok(0));
    ///
    /// topology.add_atom(&Atom::new("F").unwrap()).unwrap();
    /// topology.add_atom(&Atom::new("F").unwrap()).unwrap();
    /// topology.add_atom(&Atom::new("F").unwrap()).unwrap();
    /// topology.add_atom(&Atom::new("F").unwrap()).unwrap();
    ///
    /// topology.add_bond(0, 1).unwrap();
    /// topology.add_bond(2, 1).unwrap();
    /// topology.add_bond(2, 3).unwrap();
    /// assert_eq!(topology.bonds_count(), Ok(3));
    /// ```
    pub fn bonds_count(&self) -> Result<u64> {
        let mut res = 0;
        unsafe {
            try!(check(chfl_topology_bonds_count(self.as_ptr(), &mut res)));
        }
        return Ok(res);
    }

    /// Get the number of angles in the topology.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::{Topology, Atom};
    /// let mut topology = Topology::new().unwrap();
    /// assert_eq!(topology.angles_count(), Ok(0));
    ///
    /// topology.add_atom(&Atom::new("F").unwrap()).unwrap();
    /// topology.add_atom(&Atom::new("F").unwrap()).unwrap();
    /// topology.add_atom(&Atom::new("F").unwrap()).unwrap();
    /// topology.add_atom(&Atom::new("F").unwrap()).unwrap();
    ///
    /// topology.add_bond(0, 1).unwrap();
    /// topology.add_bond(2, 1).unwrap();
    /// topology.add_bond(2, 3).unwrap();
    /// assert_eq!(topology.angles_count(), Ok(2));
    /// ```
    pub fn angles_count(&self) -> Result<u64> {
        let mut res = 0;
        unsafe {
            try!(check(chfl_topology_angles_count(self.as_ptr(), &mut res)));
        }
        return Ok(res);
    }

    /// Get the number of dihedral angles in the topology.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::{Topology, Atom};
    /// let mut topology = Topology::new().unwrap();
    /// assert_eq!(topology.dihedrals_count(), Ok(0));
    ///
    /// topology.add_atom(&Atom::new("F").unwrap()).unwrap();
    /// topology.add_atom(&Atom::new("F").unwrap()).unwrap();
    /// topology.add_atom(&Atom::new("F").unwrap()).unwrap();
    /// topology.add_atom(&Atom::new("F").unwrap()).unwrap();
    ///
    /// topology.add_bond(0, 1).unwrap();
    /// topology.add_bond(2, 1).unwrap();
    /// topology.add_bond(2, 3).unwrap();
    /// assert_eq!(topology.dihedrals_count(), Ok(1));
    /// ```
    pub fn dihedrals_count(&self) -> Result<u64> {
        let mut res = 0;
        unsafe {
            try!(check(chfl_topology_dihedrals_count(self.as_ptr(), &mut res)));
        }
        return Ok(res);
    }

    /// Get the number of improper dihedral angles in the topology.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::{Topology, Atom};
    /// let mut topology = Topology::new().unwrap();
    /// assert_eq!(topology.dihedrals_count(), Ok(0));
    ///
    /// topology.add_atom(&Atom::new("F").unwrap()).unwrap();
    /// topology.add_atom(&Atom::new("F").unwrap()).unwrap();
    /// topology.add_atom(&Atom::new("F").unwrap()).unwrap();
    /// topology.add_atom(&Atom::new("F").unwrap()).unwrap();
    ///
    /// topology.add_bond(0, 1).unwrap();
    /// topology.add_bond(0, 2).unwrap();
    /// topology.add_bond(0, 3).unwrap();
    /// assert_eq!(topology.impropers_count(), Ok(1));
    /// ```
    pub fn impropers_count(&self) -> Result<u64> {
        let mut res = 0;
        unsafe {
            try!(check(chfl_topology_impropers_count(self.as_ptr(), &mut res)));
        }
        return Ok(res);
    }

    /// Get the list of bonds in the topology.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::{Topology, Atom};
    /// let mut topology = Topology::new().unwrap();
    /// topology.add_atom(&Atom::new("F").unwrap()).unwrap();
    /// topology.add_atom(&Atom::new("F").unwrap()).unwrap();
    /// topology.add_atom(&Atom::new("F").unwrap()).unwrap();
    /// topology.add_atom(&Atom::new("F").unwrap()).unwrap();
    ///
    /// topology.add_bond(0, 1).unwrap();
    /// topology.add_bond(2, 1).unwrap();
    /// topology.add_bond(2, 3).unwrap();
    /// assert_eq!(topology.bonds(), Ok(vec![[0, 1], [1, 2], [2, 3]]));
    /// ```
    pub fn bonds(&self) -> Result<Vec<[u64; 2]>> {
        let nbonds = try!(self.bonds_count());
        let mut res = vec![[u64::MAX; 2]; nbonds as usize];
        unsafe {
            try!(check(chfl_topology_bonds(self.handle, res.as_mut_ptr(), nbonds)));
        }
        return Ok(res);
    }

    /// Get the list of angles in the topology.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::{Topology, Atom};
    /// let mut topology = Topology::new().unwrap();
    /// topology.add_atom(&Atom::new("F").unwrap()).unwrap();
    /// topology.add_atom(&Atom::new("F").unwrap()).unwrap();
    /// topology.add_atom(&Atom::new("F").unwrap()).unwrap();
    /// topology.add_atom(&Atom::new("F").unwrap()).unwrap();
    ///
    /// topology.add_bond(0, 1).unwrap();
    /// topology.add_bond(2, 1).unwrap();
    /// topology.add_bond(2, 3).unwrap();
    /// assert_eq!(topology.angles(), Ok(vec![[0, 1, 2], [1, 2, 3]]));
    /// ```
    pub fn angles(&self) -> Result<Vec<[u64; 3]>> {
        let nangles = try!(self.angles_count());
        let mut res = vec![[u64::MAX; 3]; nangles as usize];
        unsafe {
            try!(check(chfl_topology_angles(self.as_ptr(), res.as_mut_ptr(), nangles)));
        }
        return Ok(res);
    }

    /// Get the list of dihedral angles in the topology.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::{Topology, Atom};
    /// let mut topology = Topology::new().unwrap();
    /// topology.add_atom(&Atom::new("F").unwrap()).unwrap();
    /// topology.add_atom(&Atom::new("F").unwrap()).unwrap();
    /// topology.add_atom(&Atom::new("F").unwrap()).unwrap();
    /// topology.add_atom(&Atom::new("F").unwrap()).unwrap();
    ///
    /// topology.add_bond(0, 1).unwrap();
    /// topology.add_bond(2, 1).unwrap();
    /// topology.add_bond(2, 3).unwrap();
    ///
    /// assert_eq!(topology.dihedrals(), Ok(vec![[0, 1, 2, 3]]));
    /// ```
    pub fn dihedrals(&self) -> Result<Vec<[u64; 4]>> {
        let ndihedrals = try!(self.dihedrals_count());
        let mut res = vec![[u64::MAX; 4]; ndihedrals as usize];
        unsafe {
            try!(check(chfl_topology_dihedrals(self.as_ptr(), res.as_mut_ptr(), ndihedrals)));
        }
        return Ok(res);
    }

    /// Get the list of improper dihedral angles in the topology.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::{Topology, Atom};
    /// let mut topology = Topology::new().unwrap();
    /// topology.add_atom(&Atom::new("F").unwrap()).unwrap();
    /// topology.add_atom(&Atom::new("F").unwrap()).unwrap();
    /// topology.add_atom(&Atom::new("F").unwrap()).unwrap();
    /// topology.add_atom(&Atom::new("F").unwrap()).unwrap();
    ///
    /// topology.add_bond(0, 1).unwrap();
    /// topology.add_bond(0, 2).unwrap();
    /// topology.add_bond(0, 3).unwrap();
    ///
    /// assert_eq!(topology.impropers(), Ok(vec![[1, 0, 2, 3]]));
    /// ```
    pub fn impropers(&self) -> Result<Vec<[u64; 4]>> {
        let nimpropers = try!(self.impropers_count());
        let mut res = vec![[u64::MAX; 4]; nimpropers as usize];
        unsafe {
            try!(check(chfl_topology_impropers(self.as_ptr(), res.as_mut_ptr(), nimpropers)));
        }
        return Ok(res);
    }

    /// Add a bond between the atoms at indexes `i` and `j` in the topology.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::{Topology, Atom};
    /// let mut topology = Topology::new().unwrap();
    /// assert_eq!(topology.bonds_count(), Ok(0));
    ///
    /// topology.add_atom(&Atom::new("F").unwrap()).unwrap();
    /// topology.add_atom(&Atom::new("F").unwrap()).unwrap();
    /// topology.add_atom(&Atom::new("F").unwrap()).unwrap();
    ///
    /// topology.add_bond(0, 1).unwrap();
    /// topology.add_bond(0, 2).unwrap();
    /// assert_eq!(topology.bonds_count(), Ok(2));
    /// ```
    pub fn add_bond(&mut self, i: u64, j: u64) -> Result<()> {
        unsafe {
            try!(check(chfl_topology_add_bond(self.as_mut_ptr(), i, j)));
        }
        Ok(())
    }

    /// Remove any existing bond between the atoms at indexes `i` and `j` in
    /// this topology.
    ///
    /// This function does nothing if there is no bond between `i` and `j`.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::{Topology, Atom};
    /// let mut topology = Topology::new().unwrap();
    /// assert_eq!(topology.bonds_count(), Ok(0));
    ///
    /// topology.add_atom(&Atom::new("F").unwrap()).unwrap();
    /// topology.add_atom(&Atom::new("F").unwrap()).unwrap();
    /// topology.add_atom(&Atom::new("F").unwrap()).unwrap();
    ///
    /// topology.add_bond(0, 1).unwrap();
    /// topology.add_bond(1, 2).unwrap();
    /// assert_eq!(topology.bonds_count(), Ok(2));
    ///
    /// topology.remove_bond(0, 1).unwrap();
    /// assert_eq!(topology.bonds_count(), Ok(1));
    ///
    /// // Removing a bond that does not exists
    /// topology.remove_bond(0, 2).unwrap();
    /// assert_eq!(topology.bonds_count(), Ok(1));
    /// ```
    pub fn remove_bond(&mut self, i: u64, j: u64) -> Result<()> {
        unsafe {
            try!(check(chfl_topology_remove_bond(self.as_mut_ptr(), i, j)));
        }
        Ok(())
    }

    /// Get a copy of the residue at index `index` from this topology.
    ///
    /// The residue index in the topology is not always the same as the residue
    /// `id`.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::{Topology, Residue};
    /// let mut topology = Topology::new().unwrap();
    /// topology.add_residue(&Residue::new("water").unwrap()).unwrap();
    ///
    /// let residue = topology.residue(0).unwrap();
    /// assert_eq!(residue.name(), Ok(String::from("water")));
    /// ```
    pub fn residue(&self, index: u64) -> Result<Residue> {
        unsafe {
            let handle = chfl_residue_from_topology(self.as_ptr(), index);
            Residue::from_ptr(handle)
        }
    }

    /// Get a copy of the residue containing the atom at index `index` in this
    /// topology, if any.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::{Topology, Residue};
    /// let mut topology = Topology::new().unwrap();
    /// topology.resize(8).unwrap();
    ///
    /// let mut residue = Residue::new("water").unwrap();
    /// residue.add_atom(0).unwrap();
    /// residue.add_atom(1).unwrap();
    /// residue.add_atom(2).unwrap();
    /// topology.add_residue(&residue).unwrap();
    ///
    /// let residue = topology.residue_for_atom(0).unwrap().unwrap();
    /// assert_eq!(residue.name(), Ok(String::from("water")));
    ///
    /// let residue = topology.residue_for_atom(6).unwrap();
    /// assert!(residue.is_none());
    /// ```
    pub fn residue_for_atom(&self, index: u64) -> Result<Option<Residue>> {
        let handle = unsafe { chfl_residue_for_atom(self.as_ptr(), index) };
        if handle.is_null() {
            let natoms = try!(self.size());
            if index >= natoms {
                let result = unsafe { Residue::from_ptr(handle).map(Some) };
                assert!(result.is_err());
                result
            } else {
                // Not out of bounds, there is no residue for this atom
                Ok(None)
            }
        } else {
            let residue = unsafe { try!(Residue::from_ptr(handle)) };
            Ok(Some(residue))
        }
    }

    /// Get the number of residues in this topology.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::{Topology, Residue};
    /// let mut topology = Topology::new().unwrap();
    /// assert_eq!(topology.residues_count(), Ok(0));
    ///
    /// topology.add_residue(&Residue::with_id("water", 0).unwrap()).unwrap();
    /// topology.add_residue(&Residue::with_id("protein", 1).unwrap()).unwrap();
    /// assert_eq!(topology.residues_count(), Ok(2));
    /// ```
    pub fn residues_count(&self) -> Result<u64> {
        let mut res = 0;
        unsafe {
            try!(check(chfl_topology_residues_count(self.as_ptr(), &mut res)));
        }
        Ok(res)
    }

    /// Add a residue to this topology.
    ///
    /// The residue `id` must not already be in the topology, and the residue
    /// must contain only atoms that are not already in another residue.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::{Topology, Residue};
    /// let mut topology = Topology::new().unwrap();
    /// topology.add_residue(&Residue::new("water").unwrap()).unwrap();
    ///
    /// let residue = topology.residue(0).unwrap();
    /// assert_eq!(residue.name(), Ok(String::from("water")));
    /// ```
    pub fn add_residue(&mut self, residue: &Residue) -> Result<()> {
        unsafe {
            try!(check(chfl_topology_add_residue(self.as_mut_ptr(), residue.as_ptr())));
        }
        Ok(())
    }

    /// Check if the two residues `first` and `second` from the `topology` are
    /// linked together, *i.e.* if there is a bond between one atom in the
    /// first residue and one atom in the second one.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::{Topology, Residue};
    /// let mut topology = Topology::new().unwrap();
    ///
    /// topology.add_residue(&Residue::with_id("water", 0).unwrap()).unwrap();
    /// topology.add_residue(&Residue::with_id("protein", 1).unwrap()).unwrap();
    ///
    /// let first = topology.residue(0).unwrap();
    /// let second = topology.residue(1).unwrap();
    /// assert_eq!(topology.are_linked(&first, &second), Ok(false));
    /// ```
    pub fn are_linked(&self, first: &Residue, second: &Residue) -> Result<bool> {
        let mut res = 0;
        unsafe {
            try!(check(chfl_topology_residues_linked(
                self.as_ptr(),
                first.as_ptr(),
                second.as_ptr(),
                &mut res
            )));
        }
        Ok(res != 0)
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
        let mut topology = Topology::new().unwrap();
        assert_eq!(topology.size(), Ok(0));

        let copy = topology.clone();
        assert_eq!(copy.size(), Ok(0));

        topology.resize(10).unwrap();
        assert_eq!(topology.size(), Ok(10));
        assert_eq!(copy.size(), Ok(0));
    }

    #[test]
    fn size() {
        let mut topology = Topology::new().unwrap();
        assert_eq!(topology.size(), Ok(0));

        topology.resize(10).unwrap();
        assert_eq!(topology.size(), Ok(10));

        topology.remove(7).unwrap();
        assert_eq!(topology.size(), Ok(9));

        topology.add_atom(&Atom::new("Hg").unwrap()).unwrap();
        assert_eq!(topology.size(), Ok(10));
    }

    #[test]
    fn atoms() {
        let mut topology = Topology::new().unwrap();

        topology.add_atom(&Atom::new("Hg").unwrap()).unwrap();
        topology.add_atom(&Atom::new("Mn").unwrap()).unwrap();
        topology.add_atom(&Atom::new("W").unwrap()).unwrap();
        topology.add_atom(&Atom::new("Fe").unwrap()).unwrap();

        assert_eq!(topology.atom(0).unwrap().name(), Ok(String::from("Hg")));
        assert_eq!(topology.atom(3).unwrap().name(), Ok(String::from("Fe")));
    }

    #[test]
    fn bonds() {
        let mut topology = Topology::new().unwrap();
        for _ in 0..12 {
            topology.add_atom(&Atom::new("S").unwrap()).unwrap();
        }

        assert_eq!(topology.bonds_count(), Ok(0));

        topology.add_bond(0, 1).unwrap();
        topology.add_bond(9, 2).unwrap();
        topology.add_bond(3, 7).unwrap();
        assert_eq!(topology.bonds_count(), Ok(3));

        assert_eq!(topology.bonds(), Ok(vec![[0, 1], [2, 9], [3, 7]]));

        topology.remove_bond(3, 7).unwrap();
        // Removing unexisting bond is OK
        topology.remove_bond(8, 7).unwrap();
        assert_eq!(topology.bonds_count(), Ok(2));
    }

    #[test]
    fn angles() {
        let mut topology = Topology::new().unwrap();
        for _ in 0..12 {
            topology.add_atom(&Atom::new("S").unwrap()).unwrap();
        }

        assert_eq!(topology.angles_count(), Ok(0));

        topology.add_bond(0, 1).unwrap();
        topology.add_bond(1, 2).unwrap();
        topology.add_bond(3, 7).unwrap();
        topology.add_bond(3, 5).unwrap();
        assert_eq!(topology.angles_count(), Ok(2));

        assert_eq!(topology.angles(), Ok(vec![[0, 1, 2], [5, 3, 7]]));
    }

    #[test]
    fn dihedrals() {
        let mut topology = Topology::new().unwrap();
        for _ in 0..12 {
            topology.add_atom(&Atom::new("S").unwrap()).unwrap();
        }

        assert_eq!(topology.dihedrals_count(), Ok(0));

        topology.add_bond(0, 1).unwrap();
        topology.add_bond(1, 2).unwrap();
        topology.add_bond(3, 2).unwrap();
        topology.add_bond(4, 7).unwrap();
        topology.add_bond(4, 5).unwrap();
        topology.add_bond(7, 10).unwrap();
        assert_eq!(topology.dihedrals_count(), Ok(2));

        assert_eq!(topology.dihedrals(), Ok(vec![[0, 1, 2, 3], [5, 4, 7, 10]]));
    }

    #[test]
    fn impropers() {
        let mut topology = Topology::new().unwrap();
        for _ in 0..12 {
            topology.add_atom(&Atom::new("S").unwrap()).unwrap();
        }

        assert_eq!(topology.dihedrals_count(), Ok(0));

        topology.add_bond(0, 1).unwrap();
        topology.add_bond(0, 2).unwrap();
        topology.add_bond(0, 3).unwrap();
        topology.add_bond(4, 7).unwrap();
        topology.add_bond(4, 5).unwrap();
        topology.add_bond(4, 8).unwrap();
        assert_eq!(topology.impropers_count(), Ok(2));

        assert_eq!(topology.impropers(), Ok(vec![[1, 0, 2, 3], [5, 4, 7, 8]]));
    }

    #[test]
    fn residues() {
        let mut topology = Topology::new().unwrap();
        topology.resize(4).unwrap();
        assert_eq!(topology.residues_count(), Ok(0));

        let mut residue = Residue::new("Foo").unwrap();
        residue.add_atom(0).unwrap();
        residue.add_atom(2).unwrap();

        topology.add_residue(&residue).unwrap();
        assert_eq!(topology.residues_count(), Ok(1));

        assert_eq!(topology.residue(0).unwrap().name(), Ok("Foo".into()));
        let residue = topology.residue_for_atom(2).unwrap().unwrap();
        assert_eq!(residue.name(), Ok("Foo".into()));

        let mut residue = Residue::new("Bar").unwrap();
        residue.add_atom(3).unwrap();
        topology.add_residue(&residue).unwrap();
        assert_eq!(topology.residues_count(), Ok(2));

        let first = topology.residue(0).unwrap();
        let second = topology.residue(0).unwrap();
        assert_eq!(topology.are_linked(&first, &second), Ok(true));

        let missing = topology.residue_for_atom(1).unwrap();
        assert!(missing.is_none());
        assert!(topology.residue_for_atom(67).is_err());
    }
}

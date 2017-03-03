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
    handle: *const CHFL_TOPOLOGY
}

impl Clone for Topology {
    fn clone(&self) -> Topology {
        unsafe {
            let new_handle = chfl_topology_copy(self.as_ptr());
            Topology::from_ptr(new_handle).expect(
                "Out of memory when copying a Topology"
            )
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
            Ok(Topology{handle: ptr})
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
    pub fn new() -> Result<Topology> {
        unsafe {
            let handle = chfl_topology();
            Topology::from_ptr(handle)
        }
    }

    /// Get a copy of the atom at index `index` from this topology.
    pub fn atom(&self, index: u64) -> Result<Atom> {
        unsafe {
            let handle = chfl_atom_from_topology(self.as_ptr(), index);
            Atom::from_ptr(handle)
        }
    }

    /// Get the current number of atoms in this topology.
    pub fn natoms(&self) -> Result<u64> {
        let mut natoms = 0;
        unsafe {
            try!(check(chfl_topology_atoms_count(self.as_ptr(), &mut natoms)));
        }
        return Ok(natoms);
    }

    /// Resize this topology to hold `natoms` atoms, inserting dummy atoms if
    /// the new size if bigger than the old one.
    pub fn resize(&mut self, natoms: u64) -> Result<()> {
        unsafe {
            try!(check(chfl_topology_resize(self.as_mut_ptr(), natoms)));
        }
        return Ok(());
    }

    /// Add an `Atom` at the end of this topology
    pub fn push(&mut self, atom: &Atom) -> Result<()> {
        unsafe {
            try!(check(chfl_topology_add_atom(
                self.as_mut_ptr(),
                atom.as_ptr()
            )));
        }
        return Ok(());
    }

    /// Remove an `Atom` from this topology by index. This modify all the other
    /// atoms indexes.
    pub fn remove(&mut self, index: u64) -> Result<()> {
        unsafe {
            try!(check(chfl_topology_remove(self.as_mut_ptr(), index)));
        }
        return Ok(());
    }

    /// Tell if the atoms at indexes `i` and `j` are bonded together.
    pub fn is_bond(&self, i: u64, j: u64) -> Result<bool> {
        let mut res = 0;
        unsafe {
            try!(check(chfl_topology_isbond(self.as_ptr(), i, j, &mut res)));
        }
        return Ok(res != 0);
    }

    /// Tell if the atoms at indexes `i`, `j` and `k` constitues an angle.
    pub fn is_angle(&self, i: u64, j: u64, k: u64) -> Result<bool> {
        let mut res = 0;
        unsafe {
            try!(check(chfl_topology_isangle(self.as_ptr(), i, j, k, &mut res)));
        }
        return Ok(res != 0);
    }

    /// Tell if the atoms at indexes `i`, `j`, `k` and `m` constitues a
    /// dihedral angle.
    pub fn is_dihedral(&self, i: u64, j: u64, k: u64, m: u64) -> Result<bool> {
        let mut res = 0;
        unsafe {
            try!(check(chfl_topology_isdihedral(self.as_ptr(), i, j, k, m, &mut res)));
        }
        return Ok(res != 0);
    }

    /// Get the number of bonds in the system.
    pub fn bonds_count(&self) -> Result<u64> {
        let mut res = 0;
        unsafe {
            try!(check(chfl_topology_bonds_count(self.as_ptr(), &mut res)));
        }
        return Ok(res);
    }

    /// Get the number of angles in the system.
    pub fn angles_count(&self) -> Result<u64> {
        let mut res = 0;
        unsafe {
            try!(check(chfl_topology_angles_count(self.as_ptr(), &mut res)));
        }
        return Ok(res);
    }

    /// Get the number of dihedral angles in the system.
    pub fn dihedrals_count(&self) -> Result<u64> {
        let mut res = 0;
        unsafe {
            try!(check(chfl_topology_dihedrals_count(self.as_ptr(), &mut res)));
        }
        return Ok(res);
    }

    /// Get the list of bonds in the system.
    pub fn bonds(&self) -> Result<Vec<[u64; 2]>> {
        let nbonds = try!(self.bonds_count());
        let mut res = vec![[u64::MAX; 2]; nbonds as usize];
        unsafe {
            try!(check(chfl_topology_bonds(
                self.handle,
                res.as_mut_ptr(),
                nbonds
            )));
        }
        return Ok(res);
    }

    /// Get the list of angles in the system.
    pub fn angles(&self) -> Result<Vec<[u64; 3]>> {
        let nangles = try!(self.angles_count());
        let mut res = vec![[u64::MAX; 3]; nangles as usize];
        unsafe {
            try!(check(chfl_topology_angles(self.as_ptr(), res.as_mut_ptr(), nangles)));
        }
        return Ok(res);
    }

    /// Get the list of dihedral angles in the system.
    pub fn dihedrals(&self) -> Result<Vec<[u64; 4]>> {
        let ndihedrals = try!(self.dihedrals_count());
        let mut res = vec![[u64::MAX; 4]; ndihedrals as usize];
        unsafe {
            try!(check(chfl_topology_dihedrals(self.as_ptr(), res.as_mut_ptr(), ndihedrals)));
        }
        return Ok(res);
    }

    /// Add a bond between the atoms at indexes `i` and `j` in the system.
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
    pub fn residue(&self, index: u64) -> Result<Residue> {
        unsafe {
            let handle = chfl_residue_from_topology(self.as_ptr(), index);
            Residue::from_ptr(handle)
        }
    }

    /// Get a copy of the residue containing the atom at index `index` in this
    /// topology, if any.
    pub fn residue_for_atom(&self, index: u64) -> Result<Option<Residue>> {
        let handle = unsafe {
            chfl_residue_for_atom(self.as_ptr(), index)
        };
        // TODO: make the difference between errors, out-of-bounds and missing
        // residue.
        if handle.is_null() {
            Ok(None)
        } else {
            let residue = unsafe {
                try!(Residue::from_ptr(handle))
            };
            Ok(Some(residue))
        }
    }

    /// Get the number of residues in this topology.
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
    pub fn add_residue(&mut self, residue: Residue) -> Result<()> {
        unsafe {
            try!(check(chfl_topology_add_residue(self.as_mut_ptr(), residue.as_ptr())));
        }
        Ok(())
    }

    /// Check if the two residues `first` and `second` from the `topology` are
    /// linked together, *i.e.* if there is a bond between one atom in the
    /// first residue and one atom in the second one.
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
        assert_eq!(topology.natoms(), Ok(0));

        let copy = topology.clone();
        assert_eq!(copy.natoms(), Ok(0));

        topology.push(&Atom::new("H").unwrap()).unwrap();
        assert_eq!(topology.natoms(), Ok(1));
        assert_eq!(copy.natoms(), Ok(0));
    }

    #[test]
    fn topology() {
        let mut topology = Topology::new().unwrap();

        assert_eq!(topology.natoms(), Ok(0));

        let h = Atom::new("H").unwrap();
        let o = Atom::new("O").unwrap();

        assert!(topology.push(&h).is_ok());
        assert!(topology.push(&o).is_ok());
        assert!(topology.push(&o).is_ok());
        assert!(topology.push(&h).is_ok());

        assert_eq!(topology.natoms(), Ok(4));

        assert_eq!(topology.bonds_count(), Ok(0));
        assert_eq!(topology.angles_count(), Ok(0));
        assert_eq!(topology.dihedrals_count(), Ok(0));

        assert!(topology.add_bond(0, 1).is_ok());
        assert!(topology.add_bond(1, 2).is_ok());
        assert!(topology.add_bond(2, 3).is_ok());

        assert_eq!(topology.bonds_count().unwrap(), 3);
        assert_eq!(topology.angles_count().unwrap(), 2);
        assert_eq!(topology.dihedrals_count().unwrap(), 1);

        assert_eq!(topology.is_bond(0, 1), Ok(true));
        assert_eq!(topology.is_bond(0, 3), Ok(false));

        assert_eq!(topology.is_angle(0, 1, 2), Ok(true));
        assert_eq!(topology.is_angle(0, 1, 3), Ok(false));

        assert_eq!(topology.is_dihedral(0, 1, 2, 3), Ok(true));
        assert_eq!(topology.is_dihedral(0, 1, 3, 2), Ok(false));

        assert_eq!(topology.bonds(), Ok(vec![[0, 1], [1, 2], [2, 3]]));
        assert_eq!(topology.angles(), Ok(vec![[0, 1, 2], [1, 2, 3]]));
        assert_eq!(topology.dihedrals(), Ok(vec![[0, 1, 2, 3]]));

        assert!(topology.remove_bond(2, 3).is_ok());

        assert_eq!(topology.bonds_count().unwrap(), 2);
        assert_eq!(topology.angles_count().unwrap(), 1);
        assert_eq!(topology.dihedrals_count().unwrap(), 0);

        assert!(topology.remove(3).is_ok());
        assert_eq!(topology.natoms(), Ok(3));
    }

    #[test]
    fn residues() {
        let mut topology = Topology::new().unwrap();
        assert_eq!(topology.residues_count(), Ok(0));
        let h = Atom::new("H").unwrap();
        let o = Atom::new("O").unwrap();

        topology.push(&h).unwrap();
        topology.push(&o).unwrap();
        topology.push(&o).unwrap();
        topology.push(&h).unwrap();

        let mut residue = Residue::new("Foo").unwrap();
        residue.add_atom(0).unwrap();
        residue.add_atom(2).unwrap();

        topology.add_residue(residue).unwrap();
        assert_eq!(topology.residues_count(), Ok(1));

        assert_eq!(topology.residue(0).unwrap().name(), Ok("Foo".into()));
        let residue = topology.residue_for_atom(2).unwrap().unwrap();
        assert_eq!(residue.name(), Ok("Foo".into()));

        let mut residue = Residue::new("Bar").unwrap();
        residue.add_atom(3).unwrap();
        topology.add_residue(residue).unwrap();
        assert_eq!(topology.residues_count(), Ok(2));

        let first = topology.residue(0).unwrap();
        let second = topology.residue(0).unwrap();
        assert_eq!(topology.are_linked(&first, &second), Ok(true));
    }
}

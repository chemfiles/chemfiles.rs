/* Chemfiles, an efficient IO library for chemistry file formats
 * Copyright (C) 2015 Guillaume Fraux
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/
*/
use std::ops::Drop;
use std::u64;

use chemfiles_sys::*;
use errors::{check, Error};
use {Atom, Residue};
use Result;

/// A `Topology` contains the definition of all the particles in the system, and
/// the liaisons between the particles (bonds, angles, dihedrals, ...).
///
/// Only the atoms and the bonds are stored, the angles and the dihedrals are
/// computed automaticaly.
pub struct Topology {
    handle: *const CHFL_TOPOLOGY
}

impl Topology {
    /// Create a `Topology` from a C pointer.
    ///
    /// This function is unsafe because no validity check is made on the pointer,
    /// except for it being non-null.
    #[inline]
    pub unsafe fn from_ptr(ptr: *const CHFL_TOPOLOGY) -> Result<Topology> {
        if ptr.is_null() {
            Err(Error::null_ptr())
        } else {
            Ok(Topology{handle: ptr})
        }
    }

    /// Get the underlying C pointer as a const pointer.
    #[inline]
    pub fn as_ptr(&self) -> *const CHFL_TOPOLOGY {
        self.handle
    }

    /// Get the underlying C pointer as a mutable pointer.
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut CHFL_TOPOLOGY {
        self.handle as *mut CHFL_TOPOLOGY
    }

    /// Create a new empty topology
    pub fn new() -> Result<Topology> {
        unsafe {
            let handle = chfl_topology();
            Topology::from_ptr(handle)
        }
    }

    /// Get a specific `Atom` from a topology, given its `index` in the topology
    pub fn atom(&self, index: u64) -> Result<Atom> {
        unsafe {
            let handle = chfl_atom_from_topology(self.as_ptr(), index);
            Atom::from_ptr(handle)
        }
    }

    /// Get the current number of atoms in the topology.
    pub fn natoms(&self) -> Result<u64> {
        let mut natoms = 0;
        unsafe {
            try!(check(chfl_topology_atoms_count(self.as_ptr(), &mut natoms)));
        }
        return Ok(natoms);
    }

    /// Resize the topology to hold `natoms` atoms, inserting dummy atoms if
    /// the new size if bigger than the old one.
    pub fn resize(&mut self, size: u64) -> Result<()> {
        unsafe {
            try!(check(chfl_topology_resize(self.as_mut_ptr(), size)));
        }
        return Ok(());
    }

    /// Add an `Atom` at the end of a topology
    pub fn push(&mut self, atom: &Atom) -> Result<()> {
        unsafe {
            try!(check(chfl_topology_add_atom(
                self.as_mut_ptr(),
                atom.as_ptr()
            )));
        }
        return Ok(());
    }

    /// Remove an `Atom` from a topology by index. This modify all the other
    /// atoms indexes.
    pub fn remove(&mut self, index: u64) -> Result<()> {
        unsafe {
            try!(check(chfl_topology_remove(self.as_mut_ptr(), index)));
        }
        return Ok(());
    }

    /// Tell if the atoms at indexes `i` and `j` are bonded together
    pub fn is_bond(&self, i: u64, j: u64) -> Result<bool> {
        let mut res = 0;
        unsafe {
            try!(check(chfl_topology_isbond(self.as_ptr(), i, j, &mut res)));
        }
        return Ok(res != 0);
    }

    /// Tell if the atoms at indexes `i`, `j` and `k` constitues an angle
    pub fn is_angle(&self, i: u64, j: u64, k: u64) -> Result<bool> {
        let mut res = 0;
        unsafe {
            try!(check(chfl_topology_isangle(self.as_ptr(), i, j, k, &mut res)));
        }
        return Ok(res != 0);
    }

    /// Tell if the atoms at indexes `i`, `j`, `k` and `m` constitues a dihedral
    /// angle
    pub fn is_dihedral(&self, i: u64, j: u64, k: u64, m: u64) -> Result<bool> {
        let mut res = 0;
        unsafe {
            try!(check(chfl_topology_isdihedral(self.as_ptr(), i, j, k, m, &mut res)));
        }
        return Ok(res != 0);
    }

    /// Get the number of bonds in the system
    pub fn bonds_count(&self) -> Result<u64> {
        let mut res = 0;
        unsafe {
            try!(check(chfl_topology_bonds_count(self.as_ptr(), &mut res)));
        }
        return Ok(res);
    }

    /// Get the number of angles in the system
    pub fn angles_count(&self) -> Result<u64> {
        let mut res = 0;
        unsafe {
            try!(check(chfl_topology_angles_count(self.as_ptr(), &mut res)));
        }
        return Ok(res);
    }

    /// Get the number of dihedral angles in the system
    pub fn dihedrals_count(&self) -> Result<u64> {
        let mut res = 0;
        unsafe {
            try!(check(chfl_topology_dihedrals_count(self.as_ptr(), &mut res)));
        }
        return Ok(res);
    }

    /// Get the list of bonds in the system
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

    /// Get the list of angles in the system
    pub fn angles(&self) -> Result<Vec<[u64; 3]>> {
        let nangles = try!(self.angles_count());
        let mut res = vec![[u64::MAX; 3]; nangles as usize];
        unsafe {
            try!(check(chfl_topology_angles(self.as_ptr(), res.as_mut_ptr(), nangles)));
        }
        return Ok(res);
    }

    /// Get the list of dihedral angles in the system
    pub fn dihedrals(&self) -> Result<Vec<[u64; 4]>> {
        let ndihedrals = try!(self.dihedrals_count());
        let mut res = vec![[u64::MAX; 4]; ndihedrals as usize];
        unsafe {
            try!(check(chfl_topology_dihedrals(self.as_ptr(), res.as_mut_ptr(), ndihedrals)));
        }
        return Ok(res);
    }

    /// Add a bond between the atoms at indexes `i` and `j` in the system
    pub fn add_bond(&mut self, i: u64, j: u64) -> Result<()> {
        unsafe {
            try!(check(chfl_topology_add_bond(self.as_mut_ptr(), i, j)));
        }
        Ok(())
    }

    /// Remove any existing bond between the atoms at indexes `i` and `j` in
    /// the system
    pub fn remove_bond(&mut self, i: u64, j: u64) -> Result<()> {
        unsafe {
            try!(check(chfl_topology_remove_bond(self.as_mut_ptr(), i, j)));
        }
        Ok(())
    }

    /// Get a specific `Residue` from a topology, given its `index` in the
    /// topology
    pub fn residue(&self, index: u64) -> Result<Residue> {
        unsafe {
            let handle = chfl_residue_from_topology(self.as_ptr(), index);
            Residue::from_ptr(handle)
        }
    }

    /// Get the `Residue` containing the atom at the given index, if there is
    /// one.
    pub fn residue_for_atom(&self, index: u64) -> Result<Option<Residue>> {
        let handle = unsafe {
            chfl_residue_for_atom(self.as_ptr(), index)
        };
        if handle.is_null() {
            Ok(None)
        } else {
            let residue = unsafe {
                try!(Residue::from_ptr(handle))
            };
            Ok(Some(residue))
        }
    }

    /// Get the number of residues in the system
    pub fn residues_count(&self) -> Result<u64> {
        let mut res = 0;
        unsafe {
            try!(check(chfl_topology_residues_count(self.as_ptr(), &mut res)));
        }
        Ok(res)
    }

    /// Add a residue to this topology
    pub fn add_residue(&mut self, residue: Residue) -> Result<()> {
        unsafe {
            try!(check(chfl_topology_add_residue(self.as_mut_ptr(), residue.as_ptr())));
        }
        Ok(())
    }

    /// Add a residue to this topology
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
            check(
                chfl_topology_free(self.as_mut_ptr())
            ).expect("Error while freeing memory!");
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use {Atom, Residue};

    #[test]
    fn topology() {
        let mut top = Topology::new().unwrap();

        assert_eq!(top.natoms(), Ok(0));

        let h = Atom::new("H").unwrap();
        let o = Atom::new("O").unwrap();

        assert!(top.push(&h).is_ok());
        assert!(top.push(&o).is_ok());
        assert!(top.push(&o).is_ok());
        assert!(top.push(&h).is_ok());

        assert_eq!(top.natoms(), Ok(4));

        assert_eq!(top.bonds_count(), Ok(0));
        assert_eq!(top.angles_count(), Ok(0));
        assert_eq!(top.dihedrals_count(), Ok(0));

        assert!(top.add_bond(0, 1).is_ok());
        assert!(top.add_bond(1, 2).is_ok());
        assert!(top.add_bond(2, 3).is_ok());

        assert_eq!(top.bonds_count().unwrap(), 3);
        assert_eq!(top.angles_count().unwrap(), 2);
        assert_eq!(top.dihedrals_count().unwrap(), 1);

        assert_eq!(top.is_bond(0, 1), Ok(true));
        assert_eq!(top.is_bond(0, 3), Ok(false));

        assert_eq!(top.is_angle(0, 1, 2), Ok(true));
        assert_eq!(top.is_angle(0, 1, 3), Ok(false));

        assert_eq!(top.is_dihedral(0, 1, 2, 3), Ok(true));
        assert_eq!(top.is_dihedral(0, 1, 3, 2), Ok(false));

        assert_eq!(top.bonds(), Ok(vec![[0, 1], [1, 2], [2, 3]]));
        assert_eq!(top.angles(), Ok(vec![[0, 1, 2], [1, 2, 3]]));
        assert_eq!(top.dihedrals(), Ok(vec![[0, 1, 2, 3]]));

        assert!(top.remove_bond(2, 3).is_ok());

        assert_eq!(top.bonds_count().unwrap(), 2);
        assert_eq!(top.angles_count().unwrap(), 1);
        assert_eq!(top.dihedrals_count().unwrap(), 0);

        assert!(top.remove(3).is_ok());
        assert_eq!(top.natoms(), Ok(3));
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

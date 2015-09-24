/*
 * Chemharp, an efficient IO library for chemistry file formats
 * Copyright (C) 2015 Guillaume Fraux
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/
*/

extern crate libc;

use std::ops::Drop;
use std::u64;

use ::ffi::*;
use ::errors::{check, Error};

use super::Atom;

/// A `Topology` contains the definition of all the particles in the system, and
/// the liaisons between the particles (bonds, angles, dihedrals, ...).
///
/// Only the atoms and the bonds are stored, the angles and the dihedrals are
/// computed automaticaly.
pub struct Topology {
    handle: *const CHRP_TOPOLOGY
}

impl Topology {
    /// Create a new empty topology
    pub fn new() -> Result<Topology, Error> {
        let handle : *const CHRP_TOPOLOGY;
        unsafe {
            handle = chrp_topology();
        }
        if handle.is_null() {
            return Err(Error::ChemharpCppError{message: Error::last_error()})
        }
        Ok(Topology{handle: handle})
    }

    /// Get a specific `Atom` from a topology, given its `index` in the topology
    pub fn atom(&self, index: u64) -> Result<Atom, Error> {
        let handle : *const CHRP_ATOM;
        unsafe {
            handle = chrp_atom_from_topology(self.handle, index);
        }
        if handle.is_null() {
            return Err(Error::ChemharpCppError{message: Error::last_error()})
        }
        unsafe {
            Ok(Atom::from_ptr(handle))
        }
    }

    /// Get the current number of atoms in the topology.
    pub fn natoms(&self) -> Result<usize, Error> {
        let mut natoms = 0;
        unsafe {
            try!(check(chrp_topology_atoms_count(self.handle, &mut natoms)));
        }
        return Ok(natoms as usize);
    }

    /// Add an `Atom` at the end of a topology
    pub fn push(&mut self, atom: &Atom) -> Result<(), Error> {
        unsafe {
            try!(check(chrp_topology_append(
                self.handle as *mut CHRP_TOPOLOGY,
                atom.as_ptr()
            )));
        }
        return Ok(());
    }

    /// Remove an `Atom` from a topology by index. This modify all the other
    /// atoms indexes.
    pub fn remove(&mut self, index: u64) -> Result<(), Error> {
        unsafe {
            try!(check(chrp_topology_remove(self.handle as *mut CHRP_TOPOLOGY, index)));
        }
        return Ok(());
    }

    /// Tell if the atoms at indexes `i` and `j` are bonded together
    pub fn is_bond(&self, i: u64, j: u64) -> Result<bool, Error> {
        let mut res = 0;
        unsafe {
            try!(check(chrp_topology_isbond(self.handle, i, j, &mut res)));
        }
        return Ok(res != 0);
    }

    /// Tell if the atoms at indexes `i`, `j` and `k` constitues an angle
    pub fn is_angle(&self, i: u64, j: u64, k: u64) -> Result<bool, Error> {
        let mut res = 0;
        unsafe {
            try!(check(chrp_topology_isangle(self.handle, i, j, k, &mut res)));
        }
        return Ok(res != 0);
    }

    /// Tell if the atoms at indexes `i`, `j`, `k` and `m` constitues a dihedral
    /// angle
    pub fn is_dihedral(&self, i: u64, j: u64, k: u64, m: u64) -> Result<bool, Error> {
        let mut res = 0;
        unsafe {
            try!(check(chrp_topology_isdihedral(self.handle, i, j, k, m, &mut res)));
        }
        return Ok(res != 0);
    }

    /// Get the number of bonds in the system
    pub fn bonds_count(&self) -> Result<usize, Error> {
        let mut res = 0;
        unsafe {
            try!(check(chrp_topology_bonds_count(self.handle, &mut res)));
        }
        return Ok(res as usize);
    }

    /// Get the number of angles in the system
    pub fn angles_count(&self) -> Result<usize, Error> {
        let mut res = 0;
        unsafe {
            try!(check(chrp_topology_angles_count(self.handle, &mut res)));
        }
        return Ok(res as usize);
    }

    /// Get the number of dihedral angles in the system
    pub fn dihedrals_count(&self) -> Result<usize, Error> {
        let mut res = 0;
        unsafe {
            try!(check(chrp_topology_dihedrals_count(self.handle, &mut res)));
        }
        return Ok(res as usize);
    }

    /// Get the list of bonds in the system
    pub fn bonds(&self) -> Result<Vec<[u64; 2]>, Error> {
        let nbonds = try!(self.bonds_count());
        let mut res = vec![[u64::MAX; 2]; nbonds];
        unsafe {
            try!(check(chrp_topology_bonds(
                self.handle,
                (*res.as_mut_ptr()).as_mut_ptr(),
                nbonds as u64
            )));
        }
        return Ok(res);
    }

    /// Get the list of angles in the system
    pub fn angles(&self) -> Result<Vec<[u64; 3]>, Error> {
        let nangles = try!(self.angles_count());
        let mut res = vec![[u64::MAX; 3]; nangles];
        unsafe {
            try!(check(chrp_topology_angles(
                self.handle,
                (*res.as_mut_ptr()).as_mut_ptr(),
                nangles as u64
            )));
        }
        return Ok(res);
    }

    /// Get the list of dihedral angles in the system
    pub fn dihedrals(&self) -> Result<Vec<[u64; 4]>, Error> {
        let ndihedrals = try!(self.dihedrals_count());
        let mut res = vec![[u64::MAX; 4]; ndihedrals];
        unsafe {
            try!(check(chrp_topology_dihedrals(
                self.handle,
                (*res.as_mut_ptr()).as_mut_ptr(),
                ndihedrals as u64
            )));
        }
        return Ok(res);
    }

    /// Add a bond between the atoms at indexes `i` and `j` in the system
    pub fn add_bond(&mut self, i: u64, j: u64) -> Result<(), Error> {
        unsafe {
            try!(check(chrp_topology_add_bond(self.handle as *mut CHRP_TOPOLOGY, i, j)));
        }
        Ok(())
    }

    /// Remove any existing bond between the atoms at indexes `i` and `j` in
    /// the system
    pub fn remove_bond(&mut self, i: u64, j: u64) -> Result<(), Error> {
        unsafe {
            try!(check(chrp_topology_remove_bond(self.handle as *mut CHRP_TOPOLOGY, i, j)));
        }
        Ok(())
    }

    /// Create a `Topology` from a C pointer. This function is unsafe because no
    /// validity check is made on the pointer.
    pub unsafe fn from_ptr(ptr: *const CHRP_TOPOLOGY) -> Topology {
        Topology{handle: ptr}
    }

    /// Get the underlying C pointer. This function is unsafe because no
    /// lifetime guarantee is made on the pointer.
    pub unsafe fn as_ptr(&self) -> *const CHRP_TOPOLOGY {
        self.handle
    }
}

impl Drop for Topology {
    fn drop(&mut self) {
        unsafe {
            check(
                chrp_topology_free(self.handle as *mut CHRP_TOPOLOGY)
            ).ok().expect("Error while freeing memory!");
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use ::atom::Atom;

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

        assert_eq!(top.bonds(), Ok(vec![[2, 3], [1, 2], [0, 1]]));
        assert_eq!(top.angles(), Ok(vec![[0, 1, 2], [1, 2, 3]]));
        assert_eq!(top.dihedrals(), Ok(vec![[0, 1, 2, 3]]));

        assert!(top.remove_bond(2, 3).is_ok());

        assert_eq!(top.bonds_count().unwrap(), 2);
        assert_eq!(top.angles_count().unwrap(), 1);
        assert_eq!(top.dihedrals_count().unwrap(), 0);

        assert!(top.remove(3).is_ok());
        assert_eq!(top.natoms(), Ok(3));
    }
}

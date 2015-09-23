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

use ::ffi::*;
use ::errors::{check, Error};

use super::{Atom, Topology, UnitCell};

/// A `Frame` holds data from one step of a simulation: the current `Topology`,
/// the positions, and maybe the velocities of the particles in the system.
pub struct Frame {
    handle: *const CHRP_FRAME
}

impl Frame {
    /// Create an empty frame with initial capacity of `natoms`. It will be
    /// resized by the library as needed.
    pub fn new(natoms: u64) -> Result<Frame, Error> {
        let handle : *const CHRP_FRAME;
        unsafe {
            handle = chrp_frame(natoms);
        }
        if handle.is_null() {
            return Err(Error::ChemharpCppError{message: Error::last_error()})
        }
        Ok(Frame{handle: handle})
    }

    /// Get a specific `Atom` from a frame, given its `index` in the frame
    pub fn atom(&self, index: u64) -> Result<Atom, Error> {
        let handle : *const CHRP_ATOM;
        unsafe {
            handle = chrp_atom_from_frame(self.handle as *mut CHRP_FRAME, index);
        }
        if handle.is_null() {
            return Err(Error::ChemharpCppError{message: Error::last_error()})
        }
        unsafe {
            Ok(Atom::from_ptr(handle))
        }
    }

    /// Get the current number of atoms in the `Frame`.
    pub fn natoms(&self) -> Result<usize, Error> {
        let mut natoms = 0;
        unsafe {
            try!(check(chrp_frame_atoms_count(self.handle, &mut natoms)));
        }
        return Ok(natoms as usize);
    }

    /// Get the positions from the `Frame`.
    pub fn positions(&self) -> Result<Vec<[f32; 3]>, Error> {
        let natoms = try!(self.natoms());
        let mut res = vec![[0.0; 3]; natoms];
        unsafe {
            try!(check(chrp_frame_positions(
                self.handle,
                res.as_mut_ptr() as *mut libc::c_void,
                natoms as u64
            )));
        }
        return Ok(res);
    }

    /// Set the positions in the `Frame`.
    pub fn set_positions(&mut self, positions: Vec<[f32; 3]>) -> Result<(), Error> {
        let mut positions = positions;
        unsafe {
            try!(check(chrp_frame_set_positions(
                self.handle as *mut CHRP_FRAME,
                positions.as_mut_ptr() as *mut libc::c_void,
                positions.len() as u64)));
        }
        Ok(())
    }

    /// Get the velocities from the `Frame`.
    pub fn velocities(&self) -> Result<Vec<[f32; 3]>, Error> {
        let natoms = try!(self.natoms());
        let mut res = vec![[0.0; 3]; natoms];
        unsafe {
            try!(check(chrp_frame_velocities(
                self.handle,
                res.as_mut_ptr() as *mut libc::c_void,
                natoms as u64
            )));
        }
        return Ok(res);
    }

    /// Set the velocities in the `Frame`.
    pub fn set_velocities(&mut self, velocities: Vec<[f32; 3]>) -> Result<(), Error> {
        let mut velocities = velocities;
        unsafe {
            try!(check(chrp_frame_set_velocities(
                self.handle as *mut CHRP_FRAME,
                velocities.as_mut_ptr() as *mut libc::c_void,
                velocities.len() as u64)));
        }
        Ok(())
    }

    /// Check if the `Frame` has velocity information.
    pub fn has_velocities(&self) -> Result<bool, Error> {
        let mut res = 0;
        unsafe {
            try!(check(chrp_frame_has_velocities(self.handle, &mut res)));
        }
        return Ok(res != 0);
    }

    /// Get the `UnitCell` from the `Frame`
    pub fn cell(&self) -> Result<UnitCell, Error> {
        let handle : *const CHRP_CELL;
        unsafe {
            handle = chrp_cell_from_frame(self.handle as *mut CHRP_FRAME);
        }
        if handle.is_null() {
            return Err(Error::ChemharpCppError{message: Error::last_error()})
        }
        unsafe {
            Ok(UnitCell::from_ptr(handle))
        }
    }

    /// Set the `UnitCell` of the `Frame`
    pub fn set_cell(&mut self, cell: &UnitCell) -> Result<(), Error> {
        unsafe {
            try!(check(chrp_frame_set_cell(
                self.handle as *mut CHRP_FRAME,
                cell.as_ptr()
            )));
        }
        return Ok(());
    }

    /// Get the `Topology` from the `Frame`
    pub fn topology(&self) -> Result<Topology, Error> {
        let handle : *const CHRP_TOPOLOGY;
        unsafe {
            handle = chrp_topology_from_frame(self.handle as *mut CHRP_FRAME);
        }
        if handle.is_null() {
            return Err(Error::ChemharpCppError{message: Error::last_error()})
        }
        unsafe {
            Ok(Topology::from_ptr(handle))
        }
    }

    /// Set the `Topology` of the `Frame`
    pub fn set_topology(&mut self, topology: &Topology) -> Result<(), Error> {
        unsafe {
            try!(check(chrp_frame_set_topology(
                self.handle as *mut CHRP_FRAME,
                topology.as_ptr()
            )));
        }
        return Ok(());
    }

    /// Get the `Frame` step, i.e. the frame number in the trajectory
    pub fn step(&self) -> Result<u64, Error> {
        let mut res = 0;
        unsafe {
            try!(check(chrp_frame_step(self.handle, &mut res)));
        }
        return Ok(res);
    }

    /// Set the `Frame` step
    pub fn set_step(&mut self, step: u64) -> Result<(), Error> {
        unsafe {
            try!(check(chrp_frame_set_step(self.handle as *mut CHRP_FRAME, step)));
        }
        return Ok(());
    }

    /// Try to guess the bonds, angles and dihedrals in the system. If `bonds`
    /// is true, guess everything; else only guess the angles and dihedrals from
    /// the topology bond list.
    pub fn guess_topology(&self, bonds: bool) -> Result<(), Error> {
        unsafe {
            try!(check(chrp_frame_guess_topology(self.handle as *mut CHRP_FRAME, bool_to_u8(bonds))));
        }
        return Ok(());
    }

    /// Create a `Frame` from a C pointer. This function is unsafe because
    /// no validity check is made on the pointer.
    pub unsafe fn from_ptr(ptr: *const CHRP_FRAME) -> Frame {
        Frame{handle: ptr}
    }

    /// Get the underlying C pointer. This function is unsafe because no
    /// lifetime guarantee is made on the pointer.
    pub unsafe fn as_ptr(&self) -> *const CHRP_FRAME {
        self.handle
    }
}

fn bool_to_u8(val: bool) -> u8 {
    match val {
        true => 1,
        false => 0
    }
}

impl Drop for Frame {
    fn drop(&mut self) {
        unsafe {
            check(
                chrp_frame_free(self.handle as *mut CHRP_FRAME)
            ).ok().expect("Error while freeing memory!");
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use ::{Atom, Topology, UnitCell};

    #[test]
    fn size() {
        let frame = Frame::new(0).unwrap();
        assert_eq!(frame.natoms(), Ok(0));

        let frame = Frame::new(4).unwrap();
        assert_eq!(frame.natoms(), Ok(4));
    }

    #[test]
    fn positions() {
        let mut frame = Frame::new(4).unwrap();
        let positions = vec![[1.0, 2.0, 3.0],
                             [4.0, 5.0, 6.0],
                             [7.0, 8.0, 9.0],
                             [10.0, 11.0, 12.0],];

        assert!(frame.set_positions(positions.clone()).is_ok());
        assert_eq!(frame.positions(), Ok(positions));
    }

    #[test]
    fn velocities() {
        let mut frame = Frame::new(4).unwrap();

        assert_eq!(frame.has_velocities(), Ok(false));
        let velocities = vec![[1.0, 2.0, 3.0],
                             [4.0, 5.0, 6.0],
                             [7.0, 8.0, 9.0],
                             [10.0, 11.0, 12.0],];

        assert!(frame.set_velocities(velocities.clone()).is_ok());
        assert_eq!(frame.velocities(), Ok(velocities));
        assert_eq!(frame.has_velocities(), Ok(true));
    }

    #[test]
    fn cell() {
        let mut frame = Frame::new(0).unwrap();
        let cell = UnitCell::new(3.0, 4.0, 5.0).unwrap();

        assert!(frame.set_cell(&cell).is_ok());
        let cell = frame.cell().unwrap();
        assert_eq!(cell.lengths(), Ok((3.0, 4.0, 5.0)));
    }


    #[test]
    fn topology() {
        let mut frame = Frame::new(0).unwrap();
        let mut topology = Topology::new().unwrap();

        topology.push(&Atom::new("Zn").unwrap()).unwrap();
        topology.push(&Atom::new("Ar").unwrap()).unwrap();

        assert!(frame.set_topology(&topology).is_ok());

        let topology = frame.topology().unwrap();

        assert_eq!(topology.atom(0).unwrap().name(), Ok(String::from("Zn")));
        assert_eq!(topology.atom(1).unwrap().name(), Ok(String::from("Ar")));

        assert_eq!(frame.atom(0).unwrap().name(), Ok(String::from("Zn")));
        assert_eq!(frame.atom(1).unwrap().name(), Ok(String::from("Ar")));
    }

    #[test]
    fn step() {
        let mut frame = Frame::new(0).unwrap();
        assert_eq!(frame.step(), Ok(0));

        assert!(frame.set_step(42).is_ok());
        assert_eq!(frame.step(), Ok(42));
    }
}

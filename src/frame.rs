/* Chemfiles, an efficient IO library for chemistry file formats
 * Copyright (C) 2015 Guillaume Fraux
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/
*/
use std::ops::Drop;
use std::ptr;
use std::slice;

use chemfiles_sys::*;
use errors::{check, Error, ErrorKind};
use {Atom, Topology, UnitCell};

/// A `Frame` holds data from one step of a simulation: the current `Topology`,
/// the positions, and maybe the velocities of the particles in the system.
pub struct Frame {
    handle: *const CHFL_FRAME
}

impl Frame {
    /// Create an empty frame with initial capacity of `natoms`. It will be
    /// resized by the library as needed.
    pub fn new(natoms: usize) -> Result<Frame, Error> {
        let handle : *const CHFL_FRAME;
        unsafe {
            handle = chfl_frame(natoms);
        }
        if handle.is_null() {
            return Err(Error::new(ErrorKind::ChemfilesCppError));
        }
        Ok(Frame{handle: handle})
    }

    /// Get a specific `Atom` from a frame, given its `index` in the frame
    pub fn atom(&self, index: usize) -> Result<Atom, Error> {
        let handle : *const CHFL_ATOM;
        unsafe {
            handle = chfl_atom_from_frame(self.handle, index);
        }
        if handle.is_null() {
            return Err(Error::new(ErrorKind::ChemfilesCppError));
        }
        unsafe {
            Ok(Atom::from_ptr(handle))
        }
    }

    /// Get the current number of atoms in the `Frame`.
    pub fn natoms(&self) -> Result<usize, Error> {
        let mut natoms = 0;
        unsafe {
            try!(check(chfl_frame_atoms_count(self.handle, &mut natoms)));
        }
        return Ok(natoms as usize);
    }

    /// Resize the positions and the velocities in frame, to make space for
    /// `natoms` atoms. Previous data is conserved, as well as the presence of
    /// absence of velocities.
    pub fn resize(&mut self, natoms: usize) -> Result<(), Error> {
        unsafe {
            try!(check(chfl_frame_resize(self.handle as *mut CHFL_FRAME, natoms)));
        }
        return Ok(());
    }

    /// Get a view into the positions of the `Frame`.
    pub fn positions(&self) -> Result<&[[f32; 3]], Error> {
        let mut ptr = ptr::null_mut();
        let mut natoms = 0;
        unsafe {
            try!(check(chfl_frame_positions(
                self.handle as *mut CHFL_FRAME,
                &mut ptr,
                &mut natoms
            )));
        }
        let res = unsafe {
            slice::from_raw_parts(ptr, natoms)
        };
        return Ok(res);
    }

    /// Get a mutable view into the positions of the `Frame`.
    pub fn positions_mut(&mut self) -> Result<&mut [[f32; 3]], Error> {
        let mut ptr = ptr::null_mut();
        let mut natoms = 0;
        unsafe {
            try!(check(chfl_frame_positions(
                self.handle as *mut CHFL_FRAME,
                &mut ptr,
                &mut natoms
            )));
        }
        let res = unsafe {
            slice::from_raw_parts_mut(ptr, natoms)
        };
        return Ok(res);
    }

    /// Get a view into the velocities of the `Frame`.
    pub fn velocities(&self) -> Result<&[[f32; 3]], Error> {
        let mut ptr = ptr::null_mut();
        let mut natoms = 0;
        unsafe {
            try!(check(chfl_frame_velocities(
                self.handle as *mut CHFL_FRAME,
                &mut ptr,
                &mut natoms
            )));
        }
        let res = unsafe {
            slice::from_raw_parts(ptr, natoms)
        };
        return Ok(res);
    }

    /// Get a mutable view into the velocities of the `Frame`.
    pub fn velocities_mut(&mut self) -> Result<&mut [[f32; 3]], Error> {
        let mut ptr = ptr::null_mut();
        let mut natoms = 0;
        unsafe {
            try!(check(chfl_frame_velocities(
                self.handle as *mut CHFL_FRAME,
                &mut ptr,
                &mut natoms
            )));
        }
        let res = unsafe {
            slice::from_raw_parts_mut(ptr, natoms)
        };
        return Ok(res);
    }

    /// Check if the `Frame` has velocity information.
    pub fn has_velocities(&self) -> Result<bool, Error> {
        let mut res = 0;
        unsafe {
            try!(check(chfl_frame_has_velocities(self.handle, &mut res)));
        }
        return Ok(res != 0);
    }

    /// Add velocity storage to this frame for `Frame::natoms` atoms. If the
    /// frame already have velocities, this does nothing.
    pub fn add_velocities(&mut self) -> Result<(), Error> {
        unsafe {
            try!(check(chfl_frame_add_velocities(self.handle as *mut CHFL_FRAME)));
        }
        return Ok(());
    }

    /// Get the `UnitCell` from the `Frame`
    pub fn cell(&self) -> Result<UnitCell, Error> {
        let handle : *const CHFL_CELL;
        unsafe {
            handle = chfl_cell_from_frame(self.handle);
        }
        if handle.is_null() {
            return Err(Error::new(ErrorKind::ChemfilesCppError));
        }
        unsafe {
            Ok(UnitCell::from_ptr(handle))
        }
    }

    /// Set the `UnitCell` of the `Frame`
    pub fn set_cell(&mut self, cell: &UnitCell) -> Result<(), Error> {
        unsafe {
            try!(check(chfl_frame_set_cell(
                self.handle as *mut CHFL_FRAME,
                cell.as_ptr()
            )));
        }
        return Ok(());
    }

    /// Get the `Topology` from the `Frame`
    pub fn topology(&self) -> Result<Topology, Error> {
        let handle : *const CHFL_TOPOLOGY;
        unsafe {
            handle = chfl_topology_from_frame(self.handle);
        }
        if handle.is_null() {
            return Err(Error::new(ErrorKind::ChemfilesCppError));
        }
        unsafe {
            Ok(Topology::from_ptr(handle))
        }
    }

    /// Set the `Topology` of the `Frame`
    pub fn set_topology(&mut self, topology: &Topology) -> Result<(), Error> {
        unsafe {
            try!(check(chfl_frame_set_topology(
                self.handle as *mut CHFL_FRAME,
                topology.as_ptr()
            )));
        }
        return Ok(());
    }

    /// Get the `Frame` step, i.e. the frame number in the trajectory
    pub fn step(&self) -> Result<usize, Error> {
        let mut res = 0;
        unsafe {
            try!(check(chfl_frame_step(self.handle, &mut res)));
        }
        return Ok(res);
    }

    /// Set the `Frame` step
    pub fn set_step(&mut self, step: usize) -> Result<(), Error> {
        unsafe {
            try!(check(chfl_frame_set_step(self.handle as *mut CHFL_FRAME, step)));
        }
        return Ok(());
    }

    /// Guess the bonds, angles and dihedrals in the system using an atomic
    /// distance criteria
    pub fn guess_topology(&mut self) -> Result<(), Error> {
        unsafe {
            try!(check(chfl_frame_guess_topology(self.handle as *mut CHFL_FRAME)));
        }
        return Ok(());
    }

    /// Create a `Frame` from a C pointer. This function is unsafe because
    /// no validity check is made on the pointer.
    pub unsafe fn from_ptr(ptr: *const CHFL_FRAME) -> Frame {
        Frame{handle: ptr}
    }

    /// Get the underlying C pointer. This function is unsafe because no
    /// lifetime guarantee is made on the pointer.
    pub unsafe fn as_ptr(&self) -> *const CHFL_FRAME {
        self.handle
    }
}

impl Drop for Frame {
    fn drop(&mut self) {
        unsafe {
            check(
                chfl_frame_free(self.handle as *mut CHFL_FRAME)
            ).ok().expect("Error while freeing memory!");
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use ::{Atom, Topology, UnitCell};

    // TODO: remove this when 1.7.0 hit stable. This is slice::clone_from_slice
    fn clone_from_slice<T: Clone>(dst: &mut [T], src: &[T]) {
        assert!(dst.len() == src.len());
        for (d, s) in dst.iter_mut().zip(src) {
            *d = s.clone();
        }
    }

    #[test]
    fn size() {
        let mut frame = Frame::new(0).unwrap();
        assert_eq!(frame.natoms(), Ok(0));

        frame.resize(12).unwrap();
        assert_eq!(frame.natoms(), Ok(12));

        let frame = Frame::new(4).unwrap();
        assert_eq!(frame.natoms(), Ok(4));
    }

    #[test]
    fn positions() {
        let mut frame = Frame::new(4).unwrap();
        let mut expected = [[1.0, 2.0, 3.0],
                            [4.0, 5.0, 6.0],
                            [7.0, 8.0, 9.0],
                            [10.0, 11.0, 12.0]];
        {
            let positions = frame.positions_mut().unwrap();
            clone_from_slice(positions, &mut expected);
        }

        assert_eq!(frame.positions(), Ok(expected.as_ref()));
    }

    #[test]
    fn velocities() {
        let mut frame = Frame::new(4).unwrap();
        assert_eq!(frame.has_velocities(), Ok(false));
        frame.add_velocities().unwrap();
        assert_eq!(frame.has_velocities(), Ok(true));

        let mut expected = [[1.0, 2.0, 3.0],
                            [4.0, 5.0, 6.0],
                            [7.0, 8.0, 9.0],
                            [10.0, 11.0, 12.0]];

        {
            let velocities = frame.velocities_mut().unwrap();
            clone_from_slice(velocities, &mut expected);
        }

        assert_eq!(frame.velocities(), Ok(expected.as_ref()));
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
        let mut frame = Frame::new(2).unwrap();
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

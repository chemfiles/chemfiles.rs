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
use errors::{check, Error};
use {Atom, Topology, UnitCell};
use Result;

/// A `Frame` holds data from one step of a simulation: the current `Topology`,
/// the positions, and maybe the velocities of the particles in the system.
pub struct Frame {
    handle: *const CHFL_FRAME
}

impl Frame {
    /// Create a `Frame` from a C pointer.
    ///
    /// This function is unsafe because no validity check is made on the pointer,
    /// except for it being non-null.
    #[inline]
    pub unsafe fn from_ptr(ptr: *const CHFL_FRAME) -> Result<Frame> {
        if ptr.is_null() {
            Err(Error::null_ptr())
        } else {
            Ok(Frame{handle: ptr})
        }
    }

    /// Get the underlying C pointer as a const pointer.
    #[inline]
    pub fn as_ptr(&self) -> *const CHFL_FRAME {
        self.handle
    }

    /// Get the underlying C pointer as a mutable pointer.
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut CHFL_FRAME {
        self.handle as *mut CHFL_FRAME
    }

    /// Create an empty frame. It will be resized by the library as needed.
    pub fn new() -> Result<Frame> {
        let handle: *const CHFL_FRAME;
        unsafe {
            handle = chfl_frame();
        }

        if handle.is_null() {
            Err(Error::null_ptr())
        } else {
            Ok(Frame{handle: handle})
        }
    }

    /// Get a specific `Atom` from a frame, given its `index` in the frame
    pub fn atom(&self, index: u64) -> Result<Atom> {
        unsafe {
            let handle = chfl_atom_from_frame(self.as_ptr(), index);
            Atom::from_ptr(handle)
        }
    }

    /// Get the current number of atoms in the `Frame`.
    pub fn natoms(&self) -> Result<u64> {
        let mut natoms = 0;
        unsafe {
            try!(check(chfl_frame_atoms_count(self.as_ptr(), &mut natoms)));
        }
        return Ok(natoms);
    }

    /// Resize the positions and the velocities in frame, to make space for
    /// `natoms` atoms. Previous data is conserved, as well as the presence of
    /// absence of velocities.
    pub fn resize(&mut self, natoms: u64) -> Result<()> {
        unsafe {
            try!(check(chfl_frame_resize(self.as_mut_ptr(), natoms)));
        }
        return Ok(());
    }

    /// Add an `Atom` and the corresponding position and optionally velocity data to a `Frame`.
    pub fn add_atom<V>(&mut self, atom: Atom, position: (f64, f64, f64), velocity: V) -> Result<()>
        where V: Into<Option<(f64, f64, f64)>> {
        let position = [position.0, position.1, position.2];
        let velocity_data;
        let velocity_ptr = match velocity.into() {
            Some((x, y, z)) => {
                velocity_data = [x, y, z];
                velocity_data.as_ptr()
            }
            None => 0 as *const _
        };

        unsafe {
            try!(check(chfl_frame_add_atom(
                self.as_mut_ptr(),
                atom.as_ptr(),
                position.as_ptr(),
                velocity_ptr
            )));
        }

        return Ok(());
    }

    /// Get a view into the positions of the `Frame`.
    pub fn positions(&self) -> Result<&[[f64; 3]]> {
        let mut ptr = ptr::null_mut();
        let mut natoms = 0;
        unsafe {
            try!(check(chfl_frame_positions(
                // not using .as_ptr() because the C function uses a *mut pointer
                // and we are re-creating the shared/mut by ourselve here
                self.handle as *mut CHFL_FRAME,
                &mut ptr,
                &mut natoms
            )));
        }
        let res = unsafe {
            slice::from_raw_parts(ptr, natoms as usize)
        };
        return Ok(res);
    }

    /// Get a mutable view into the positions of the `Frame`.
    pub fn positions_mut(&mut self) -> Result<&mut [[f64; 3]]> {
        let mut ptr = ptr::null_mut();
        let mut natoms = 0;
        unsafe {
            try!(check(chfl_frame_positions(
                self.as_mut_ptr(),
                &mut ptr,
                &mut natoms
            )));
        }
        let res = unsafe {
            slice::from_raw_parts_mut(ptr, natoms as usize)
        };
        return Ok(res);
    }

    /// Get a view into the velocities of the `Frame`.
    pub fn velocities(&self) -> Result<&[[f64; 3]]> {
        let mut ptr = ptr::null_mut();
        let mut natoms = 0;
        unsafe {
            try!(check(chfl_frame_velocities(
                // not using .as_ptr() because the C function uses a *mut pointer
                // and we are re-creating the shared/mut by ourselve here
                self.handle as *mut CHFL_FRAME,
                &mut ptr,
                &mut natoms
            )));
        }
        let res = unsafe {
            slice::from_raw_parts(ptr, natoms as usize)
        };
        return Ok(res);
    }

    /// Get a mutable view into the velocities of the `Frame`.
    pub fn velocities_mut(&mut self) -> Result<&mut [[f64; 3]]> {
        let mut ptr = ptr::null_mut();
        let mut natoms = 0;
        unsafe {
            try!(check(chfl_frame_velocities(
                self.as_mut_ptr(),
                &mut ptr,
                &mut natoms
            )));
        }
        let res = unsafe {
            slice::from_raw_parts_mut(ptr, natoms as usize)
        };
        return Ok(res);
    }

    /// Check if the `Frame` has velocity information.
    pub fn has_velocities(&self) -> Result<bool> {
        let mut res = 0;
        unsafe {
            try!(check(chfl_frame_has_velocities(self.as_ptr(), &mut res)));
        }
        return Ok(res != 0);
    }

    /// Add velocity storage to this frame for `Frame::natoms` atoms. If the
    /// frame already have velocities, this does nothing.
    pub fn add_velocities(&mut self) -> Result<()> {
        unsafe {
            try!(check(chfl_frame_add_velocities(self.as_mut_ptr())));
        }
        return Ok(());
    }

    /// Get the `UnitCell` from the `Frame`
    pub fn cell(&self) -> Result<UnitCell> {
        unsafe {
            let handle = chfl_cell_from_frame(self.as_ptr());
            UnitCell::from_ptr(handle)
        }
    }

    /// Set the `UnitCell` of the `Frame`
    pub fn set_cell(&mut self, cell: &UnitCell) -> Result<()> {
        unsafe {
            try!(check(chfl_frame_set_cell(
                self.as_mut_ptr(),
                cell.as_ptr()
            )));
        }
        return Ok(());
    }

    /// Get the `Topology` from the `Frame`
    pub fn topology(&self) -> Result<Topology> {
        unsafe {
            let handle = chfl_topology_from_frame(self.as_ptr());
            Topology::from_ptr(handle)
        }
    }

    /// Set the `Topology` of the `Frame`
    pub fn set_topology(&mut self, topology: &Topology) -> Result<()> {
        unsafe {
            try!(check(chfl_frame_set_topology(
                self.as_mut_ptr(),
                topology.as_ptr()
            )));
        }
        return Ok(());
    }

    /// Get the `Frame` step, i.e. the frame number in the trajectory
    pub fn step(&self) -> Result<u64> {
        let mut res = 0;
        unsafe {
            try!(check(chfl_frame_step(self.as_ptr(), &mut res)));
        }
        return Ok(res);
    }

    /// Set the `Frame` step
    pub fn set_step(&mut self, step: u64) -> Result<()> {
        unsafe {
            try!(check(chfl_frame_set_step(self.as_mut_ptr(), step)));
        }
        return Ok(());
    }

    /// Guess the bonds, angles and dihedrals in the system using an atomic
    /// distance criteria
    pub fn guess_topology(&mut self) -> Result<()> {
        unsafe {
            try!(check(chfl_frame_guess_topology(self.as_mut_ptr())));
        }
        return Ok(());
    }
}

impl Drop for Frame {
    fn drop(&mut self) {
        unsafe {
            check(
                chfl_frame_free(self.as_mut_ptr())
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
        let mut frame = Frame::new().unwrap();
        assert_eq!(frame.natoms(), Ok(0));

        frame.resize(12).unwrap();
        assert_eq!(frame.natoms(), Ok(12));
    }

    #[test]
    fn add_atom() {
        let atom = Atom::new("U").unwrap();
        let mut frame = Frame::new().unwrap();

        frame.add_atom(atom, (1.0, 1.0, 2.0), None).unwrap();
        assert_eq!(frame.natoms(), Ok(1));
        assert_eq!(frame.atom(0).unwrap().name(), Ok("U".into()));

        let positions: &[[f64; 3]] = &[[1.0, 1.0, 2.0]];
        assert_eq!(frame.positions(), Ok(positions));

        frame.add_velocities().unwrap();

        let atom = Atom::new("F").unwrap();
        frame.add_atom(atom, (1.0, 1.0, 2.0), (4.0, 3.0, 2.0)).unwrap();
        assert_eq!(frame.natoms(), Ok(2));
        assert_eq!(frame.atom(0).unwrap().name(), Ok("U".into()));
        assert_eq!(frame.atom(1).unwrap().name(), Ok("F".into()));

        let positions: &[[f64; 3]] = &[[1.0, 1.0, 2.0], [1.0, 1.0, 2.0]];
        assert_eq!(frame.positions(), Ok(positions));

        let velocities: &[[f64; 3]] = &[[0.0, 0.0, 0.0], [4.0, 3.0, 2.0]];
        assert_eq!(frame.velocities(), Ok(velocities));
    }

    #[test]
    fn positions() {
        let mut frame = Frame::new().unwrap();
        frame.resize(4).unwrap();
        let expected = [[1.0, 2.0, 3.0],
                        [4.0, 5.0, 6.0],
                        [7.0, 8.0, 9.0],
                        [10.0, 11.0, 12.0]];
        {
            let positions = frame.positions_mut().unwrap();
            positions.clone_from_slice(expected.as_ref());
        }

        assert_eq!(frame.positions(), Ok(expected.as_ref()));
    }

    #[test]
    fn velocities() {
        let mut frame = Frame::new().unwrap();
        frame.resize(4).unwrap();
        assert_eq!(frame.has_velocities(), Ok(false));
        frame.add_velocities().unwrap();
        assert_eq!(frame.has_velocities(), Ok(true));

        let expected = [[1.0, 2.0, 3.0],
                        [4.0, 5.0, 6.0],
                        [7.0, 8.0, 9.0],
                        [10.0, 11.0, 12.0]];

        {
            let velocities = frame.velocities_mut().unwrap();
            velocities.clone_from_slice(expected.as_ref());
        }

        assert_eq!(frame.velocities(), Ok(expected.as_ref()));
    }

    #[test]
    fn cell() {
        let mut frame = Frame::new().unwrap();
        let cell = UnitCell::new(3.0, 4.0, 5.0).unwrap();

        assert!(frame.set_cell(&cell).is_ok());
        let cell = frame.cell().unwrap();
        assert_eq!(cell.lengths(), Ok((3.0, 4.0, 5.0)));
    }

    #[test]
    fn topology() {
        let mut frame = Frame::new().unwrap();
        frame.resize(2).unwrap();
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
        let mut frame = Frame::new().unwrap();
        assert_eq!(frame.step(), Ok(0));

        assert!(frame.set_step(42).is_ok());
        assert_eq!(frame.step(), Ok(42));
    }
}

// Chemfiles, a modern library for chemistry file reading and writing
// Copyright (C) 2015-2018 Guillaume Fraux -- BSD licensed
use std::ops::Drop;
use std::ptr;
use std::slice;

use chemfiles_sys::*;
use strings;
use errors::{check, Error};
use {Atom, Residue, Topology, UnitCell};
use property::{Property, RawProperty};
use Result;

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
            Frame::from_ptr(new_handle).expect("Out of memory when copying a Frame")
        }
    }
}

impl Frame {
    /// Create a `Frame` from a C pointer.
    ///
    /// This function is unsafe because no validity check is made on the pointer,
    /// except for it being non-null.
    #[inline]
    pub(crate) unsafe fn from_ptr(ptr: *mut CHFL_FRAME) -> Result<Frame> {
        if ptr.is_null() {
            Err(Error::null_ptr())
        } else {
            Ok(Frame { handle: ptr })
        }
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

    /// Create an empty frame. It will be resized by the library as needed.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::Frame;
    /// let frame = Frame::new().unwrap();
    /// ```
    pub fn new() -> Result<Frame> {
        let handle = unsafe { chfl_frame() };
        if handle.is_null() {
            Err(Error::null_ptr())
        } else {
            Ok(Frame { handle: handle })
        }
    }

    /// Get a copy of the atom at index `index` in this frame.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::{Frame, Atom};
    /// let mut frame = Frame::new().unwrap();
    /// frame.add_atom(&Atom::new("Zn").unwrap(), [0.0; 3], None).unwrap();
    ///
    /// let atom = frame.atom(0).unwrap();
    /// assert_eq!(atom.name(), Ok(String::from("Zn")));
    /// ```
    pub fn atom(&self, index: u64) -> Result<Atom> {
        unsafe {
            let handle = chfl_atom_from_frame(self.as_ptr(), index);
            Atom::from_ptr(handle)
        }
    }

    /// Get the current number of atoms in this frame.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::{Frame, Atom};
    /// let mut frame = Frame::new().unwrap();
    /// assert_eq!(frame.size(), Ok(0));
    ///
    /// frame.resize(67).unwrap();
    /// assert_eq!(frame.size(), Ok(67));
    /// ```
    pub fn size(&self) -> Result<u64> {
        let mut natoms = 0;
        unsafe {
            try!(check(chfl_frame_atoms_count(self.as_ptr(), &mut natoms)));
        }
        return Ok(natoms);
    }

    /// Resize the positions and the velocities in this frame, to make space for
    /// `natoms` atoms. Previous data is conserved, as well as the presence of
    /// absence of velocities.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::{Frame, Atom};
    /// let mut frame = Frame::new().unwrap();
    /// frame.resize(67).unwrap();
    /// assert_eq!(frame.size(), Ok(67));
    /// ```
    pub fn resize(&mut self, natoms: u64) -> Result<()> {
        unsafe {
            try!(check(chfl_frame_resize(self.as_mut_ptr(), natoms)));
        }
        return Ok(());
    }

    /// Add an `Atom` and the corresponding position and optionally velocity
    /// data to this frame.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::{Frame, Atom};
    /// let mut frame = Frame::new().unwrap();
    /// frame.add_atom(&Atom::new("Zn").unwrap(), [1.0, 1.0, 2.0], None).unwrap();
    ///
    /// frame.add_velocities().unwrap();
    /// frame.add_atom(&Atom::new("Zn").unwrap(), [-1.0, 1.0, 2.0], [0.2, 0.1, 0.0]).unwrap();
    /// ```
    pub fn add_atom<V>(&mut self, atom: &Atom, position: [f64; 3], velocity: V) -> Result<()>
    where
        V: Into<Option<[f64; 3]>>,
    {
        let velocity = velocity.into();
        let velocity_ptr = match velocity {
            Some(ref data) => data.as_ptr(),
            None => ptr::null(),
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

    /// Remove the atom at index `i` in this frame.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::{Frame, Atom};
    /// let mut frame = Frame::new().unwrap();
    /// frame.add_atom(&Atom::new("Zn").unwrap(), [0.0; 3], None).unwrap();
    /// frame.add_atom(&Atom::new("Fe").unwrap(), [0.0; 3], None).unwrap();
    /// frame.add_atom(&Atom::new("Sn").unwrap(), [0.0; 3], None).unwrap();
    /// assert_eq!(frame.size(), Ok(3));
    ///
    /// frame.remove(1).unwrap();
    /// assert_eq!(frame.size(), Ok(2));
    /// assert_eq!(frame.atom(1).unwrap().name().unwrap(), "Sn");
    /// ```
    pub fn remove(&mut self, i: usize) -> Result<()> {
        unsafe {
            try!(check(chfl_frame_remove(self.as_mut_ptr(), i as u64)));
        }
        return Ok(());
    }

    /// Add a bond between the atoms at indexes `i` and `j` in the frame.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::{Frame, Atom};
    /// let mut frame = Frame::new().unwrap();
    /// for i in 0..5 {
    ///    frame.add_atom(&Atom::new("C").unwrap(), [0.0; 3], None).unwrap();
    /// }
    ///
    /// frame.add_bond(0, 1).unwrap();
    /// frame.add_bond(3, 1).unwrap();
    /// frame.add_bond(2, 4).unwrap();
    ///
    /// let bonds = frame.topology().unwrap().bonds().unwrap();
    /// assert_eq!(bonds, vec![[0, 1], [1, 3], [2, 4]]);
    /// ```
    pub fn add_bond(&mut self, i: usize, j: usize) -> Result<()> {
        unsafe {
            try!(check(chfl_frame_add_bond(self.as_mut_ptr(), i as u64, j as u64)));
        }
        return Ok(());
    }

    /// Remove any existing bond between the atoms at indexes `i` and `j` in
    /// the frame.
    ///
    /// This function does nothing if there is no bond between `i` and `j`.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::{Frame, Atom};
    /// let mut frame = Frame::new().unwrap();
    /// for i in 0..5 {
    ///    frame.add_atom(&Atom::new("C").unwrap(), [0.0; 3], None).unwrap();
    /// }
    ///
    /// frame.add_bond(0, 1).unwrap();
    /// frame.add_bond(3, 1).unwrap();
    /// frame.add_bond(2, 4).unwrap();
    ///
    /// let bonds = frame.topology().unwrap().bonds().unwrap();
    /// assert_eq!(bonds, vec![[0, 1], [1, 3], [2, 4]]);
    ///
    /// frame.remove_bond(2, 4).unwrap();
    /// let bonds = frame.topology().unwrap().bonds().unwrap();
    /// assert_eq!(bonds, vec![[0, 1], [1, 3]]);
    /// ```
    pub fn remove_bond(&mut self, i: usize, j: usize) -> Result<()> {
        unsafe {
            try!(check(chfl_frame_remove_bond(self.as_mut_ptr(), i as u64, j as u64)));
        }
        return Ok(());
    }

    /// Remove any existing bond between the atoms at indexes `i` and `j` in
    /// the frame.
    ///
    /// This function does nothing if there is no bond between `i` and `j`.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::{Frame, Residue};
    /// let mut frame = Frame::new().unwrap();
    ///
    /// let residue = Residue::new("foo").unwrap();
    /// frame.add_residue(&residue).unwrap();
    ///
    /// let topology = frame.topology().unwrap();
    /// assert_eq!(topology.residues_count(), Ok(1));
    /// assert_eq!(topology.residue(0).unwrap().name().unwrap(), "foo");
    /// ```
    pub fn add_residue(&mut self, residue: &Residue) -> Result<()> {
        unsafe {
            try!(check(chfl_frame_add_residue(self.as_mut_ptr(), residue.as_ptr())));
        }
        return Ok(());
    }

    /// Get the distance between the atoms at indexes `i` and `j` in this frame,
    /// accounting for periodic boundary conditions. The result is expressed in
    /// Angstroms.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::{Frame, Atom};
    /// let mut frame = Frame::new().unwrap();
    /// frame.add_atom(&Atom::new("A").unwrap(), [0.0, 0.0, 0.0], None).unwrap();
    /// frame.add_atom(&Atom::new("B").unwrap(), [1.0, 2.0, 3.0], None).unwrap();
    ///
    /// assert_eq!(frame.distance(0, 1), Ok(f64::sqrt(14.0)));
    /// ```
    pub fn distance(&self, i: usize, j: usize) -> Result<f64> {
        let mut distance = 0.0;
        unsafe {
            try!(check(chfl_frame_distance(self.as_ptr(), i as u64, j as u64, &mut distance)));
        }
        return Ok(distance);
    }

    /// Get the angle formed by the atoms at indexes `i`, `j` and `k` in this
    /// frame, accounting for periodic boundary conditions. The result is
    /// expressed in radians.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::{Frame, Atom};
    /// # use std::f64;
    /// let mut frame = Frame::new().unwrap();
    /// frame.add_atom(&Atom::new("A").unwrap(), [1.0, 0.0, 0.0], None).unwrap();
    /// frame.add_atom(&Atom::new("B").unwrap(), [0.0, 0.0, 0.0], None).unwrap();
    /// frame.add_atom(&Atom::new("B").unwrap(), [0.0, 1.0, 0.0], None).unwrap();
    ///
    /// assert_eq!(frame.angle(0, 1, 2), Ok(f64::consts::PI / 2.0));
    /// ```
    pub fn angle(&self, i: usize, j: usize, k: usize) -> Result<f64> {
        let mut angle = 0.0;
        unsafe {
            try!(check(chfl_frame_angle(self.as_ptr(), i as u64, j as u64, k as u64, &mut angle)));
        }
        return Ok(angle);
    }

    /// Get the dihedral angle formed by the atoms at indexes `i`, `j`, `k` and
    /// `m` in this frame, accounting for periodic boundary conditions. The
    /// result is expressed in radians.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::{Frame, Atom};
    /// # use std::f64;
    /// let mut frame = Frame::new().unwrap();
    /// frame.add_atom(&Atom::new("A").unwrap(), [1.0, 0.0, 0.0], None).unwrap();
    /// frame.add_atom(&Atom::new("B").unwrap(), [0.0, 0.0, 0.0], None).unwrap();
    /// frame.add_atom(&Atom::new("B").unwrap(), [0.0, 1.0, 0.0], None).unwrap();
    /// frame.add_atom(&Atom::new("B").unwrap(), [0.0, 1.0, 1.0], None).unwrap();
    ///
    /// assert_eq!(frame.dihedral(0, 1, 2, 3), Ok(f64::consts::PI / 2.0));
    /// ```
    pub fn dihedral(&self, i: usize, j: usize, k: usize, m: usize) -> Result<f64> {
        let mut dihedral = 0.0;
        unsafe {
            try!(check(chfl_frame_dihedral(
                self.as_ptr(),
                i as u64,
                j as u64,
                k as u64,
                m as u64,
                &mut dihedral
            )));
        }
        return Ok(dihedral);
    }

    /// Get the out of plane distance formed by the atoms at indexes `i`, `j`,
    /// `k` and `m` in this frame, accounting for periodic boundary conditions.
    /// The result is expressed in angstroms.
    ///
    /// This is the distance betweent the atom j and the ikm plane. The j atom
    /// is the center of the improper dihedral angle formed by i, j, k and m.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::{Frame, Atom};
    /// let mut frame = Frame::new().unwrap();
    /// frame.add_atom(&Atom::new("A").unwrap(), [0.0, 0.0, 0.0], None).unwrap();
    /// frame.add_atom(&Atom::new("B").unwrap(), [0.0, 0.0, 2.0], None).unwrap();
    /// frame.add_atom(&Atom::new("B").unwrap(), [1.0, 0.0, 0.0], None).unwrap();
    /// frame.add_atom(&Atom::new("B").unwrap(), [0.0, 1.0, 0.0], None).unwrap();
    ///
    /// assert_eq!(frame.out_of_plane(0, 1, 2, 3), Ok(2.0));
    /// ```
    pub fn out_of_plane(&self, i: usize, j: usize, k: usize, m: usize) -> Result<f64> {
        let mut distance = 0.0;
        unsafe {
            try!(check(chfl_frame_out_of_plane(
                self.as_ptr(),
                i as u64,
                j as u64,
                k as u64,
                m as u64,
                &mut distance
            )));
        }
        return Ok(distance);
    }

    /// Get a view into the positions of this frame.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::Frame;
    /// let mut frame = Frame::new().unwrap();
    /// frame.resize(67).unwrap();
    ///
    /// let positions = frame.positions().unwrap();
    /// assert_eq!(positions.len(), 67);
    /// assert_eq!(positions[0], [0.0, 0.0, 0.0]);
    /// ```
    pub fn positions(&self) -> Result<&[[f64; 3]]> {
        let mut ptr = ptr::null_mut();
        let mut natoms = 0;
        unsafe {
            try!(check(chfl_frame_positions(
                // not using .as_ptr() because the C function uses a *mut
                // pointer and we are re-creating the shared/mut by ourselve
                self.handle,
                &mut ptr,
                &mut natoms
            )));
        }
        let res = unsafe {
            #[allow(cast_possible_truncation)]
            slice::from_raw_parts(ptr, natoms as usize)
        };
        return Ok(res);
    }

    /// Get a mutable view into the positions of this frame.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::Frame;
    /// let mut frame = Frame::new().unwrap();
    /// frame.resize(67).unwrap();
    /// {
    ///     let positions = frame.positions_mut().unwrap();
    ///     assert_eq!(positions[0], [0.0, 0.0, 0.0]);
    ///     positions[0] = [1.0, 2.0, 3.0];
    /// }
    ///
    /// let positions = frame.positions().unwrap();
    /// assert_eq!(positions[0], [1.0, 2.0, 3.0]);
    /// ```
    pub fn positions_mut(&mut self) -> Result<&mut [[f64; 3]]> {
        let mut ptr = ptr::null_mut();
        let mut natoms = 0;
        unsafe {
            try!(check(chfl_frame_positions(self.as_mut_ptr(), &mut ptr, &mut natoms)));
        }
        let res = unsafe {
            #[allow(cast_possible_truncation)]
            slice::from_raw_parts_mut(ptr, natoms as usize)
        };
        return Ok(res);
    }

    /// Get a view into the velocities of this frame.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::Frame;
    /// let mut frame = Frame::new().unwrap();
    /// frame.resize(67).unwrap();
    /// frame.add_velocities().unwrap();
    ///
    /// let velocities = frame.velocities().unwrap();
    /// assert_eq!(velocities.len(), 67);
    /// assert_eq!(velocities[0], [0.0, 0.0, 0.0]);
    /// ```
    pub fn velocities(&self) -> Result<&[[f64; 3]]> {
        let mut ptr = ptr::null_mut();
        let mut natoms = 0;
        unsafe {
            try!(check(chfl_frame_velocities(
                // not using .as_ptr() because the C function uses a *mut
                // pointer and we are re-creating the shared/mut by ourselve
                self.handle,
                &mut ptr,
                &mut natoms
            )));
        }
        let res = unsafe {
            #[allow(cast_possible_truncation)]
            slice::from_raw_parts(ptr, natoms as usize)
        };
        return Ok(res);
    }

    /// Get a mutable view into the velocities of this frame.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::Frame;
    /// let mut frame = Frame::new().unwrap();
    /// frame.resize(67).unwrap();
    /// frame.add_velocities().unwrap();
    /// {
    ///     let velocities = frame.velocities_mut().unwrap();
    ///     assert_eq!(velocities[0], [0.0, 0.0, 0.0]);
    ///     velocities[0] = [1.0, 2.0, 3.0];
    /// }
    ///
    /// let velocities = frame.velocities().unwrap();
    /// assert_eq!(velocities[0], [1.0, 2.0, 3.0]);
    /// ```
    pub fn velocities_mut(&mut self) -> Result<&mut [[f64; 3]]> {
        let mut ptr = ptr::null_mut();
        let mut natoms = 0;
        unsafe {
            try!(check(chfl_frame_velocities(self.as_mut_ptr(), &mut ptr, &mut natoms)));
        }
        let res = unsafe {
            #[allow(cast_possible_truncation)]
            slice::from_raw_parts_mut(ptr, natoms as usize)
        };
        return Ok(res);
    }

    /// Check if this frame contains velocity data.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::Frame;
    /// let mut frame = Frame::new().unwrap();
    /// assert_eq!(frame.has_velocities(), Ok(false));
    ///
    /// frame.add_velocities().unwrap();
    /// assert_eq!(frame.has_velocities(), Ok(true));
    /// ```
    pub fn has_velocities(&self) -> Result<bool> {
        let mut res = 0;
        unsafe {
            try!(check(chfl_frame_has_velocities(self.as_ptr(), &mut res)));
        }
        return Ok(res != 0);
    }

    /// Add velocity data to this frame. If the frame already have velocities,
    /// this does nothing.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::Frame;
    /// let mut frame = Frame::new().unwrap();
    /// assert_eq!(frame.has_velocities(), Ok(false));
    ///
    /// frame.add_velocities().unwrap();
    /// assert_eq!(frame.has_velocities(), Ok(true));
    /// ```
    pub fn add_velocities(&mut self) -> Result<()> {
        unsafe {
            try!(check(chfl_frame_add_velocities(self.as_mut_ptr())));
        }
        return Ok(());
    }

    /// Get a copy of the `UnitCell` from this frame.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::{Frame, CellShape};
    /// let frame = Frame::new().unwrap();
    ///
    /// let cell = frame.cell().unwrap();
    /// assert_eq!(cell.shape(), Ok(CellShape::Infinite));
    /// ```
    pub fn cell(&self) -> Result<UnitCell> {
        unsafe {
            let handle = chfl_cell_from_frame(self.as_ptr());
            UnitCell::from_ptr(handle)
        }
    }

    /// Set the `UnitCell` of this frame to `cell`.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::{Frame, UnitCell, CellShape};
    /// let mut frame = Frame::new().unwrap();
    ///
    /// frame.set_cell(&UnitCell::new([10.0, 10.0, 10.0]).unwrap()).unwrap();
    ///
    /// let cell = frame.cell().unwrap();
    /// assert_eq!(cell.shape(), Ok(CellShape::Orthorhombic));
    /// assert_eq!(cell.lengths(), Ok([10.0, 10.0, 10.0]));
    /// ```
    pub fn set_cell(&mut self, cell: &UnitCell) -> Result<()> {
        unsafe {
            try!(check(chfl_frame_set_cell(self.as_mut_ptr(), cell.as_ptr())));
        }
        return Ok(());
    }

    /// Get a copy of the `Topology` from this frame.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::Frame;
    /// let mut frame = Frame::new().unwrap();
    /// frame.resize(42).unwrap();
    ///
    /// let topology = frame.topology().unwrap();
    /// assert_eq!(topology.size(), Ok(42));
    /// ```
    pub fn topology(&self) -> Result<Topology> {
        unsafe {
            let handle = chfl_topology_from_frame(self.as_ptr());
            Topology::from_ptr(handle)
        }
    }

    /// Set the `Topology` of this frame to `topology`. The topology must
    /// contain the same number of atoms that this frame.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::{Frame, Topology, Atom};
    /// let mut frame = Frame::new().unwrap();
    /// frame.resize(2).unwrap();
    ///
    /// let mut topology = Topology::new().unwrap();
    /// topology.add_atom(&Atom::new("Cl").unwrap()).unwrap();
    /// topology.add_atom(&Atom::new("Cl").unwrap()).unwrap();
    /// topology.add_bond(0, 1);
    ///
    /// frame.set_topology(&topology);
    /// assert_eq!(frame.atom(0).unwrap().name(), Ok(String::from("Cl")));
    /// ```
    pub fn set_topology(&mut self, topology: &Topology) -> Result<()> {
        unsafe {
            try!(check(chfl_frame_set_topology(self.as_mut_ptr(), topology.as_ptr())));
        }
        return Ok(());
    }

    /// Get this frame step, i.e. the frame number in the trajectory
    ///
    /// # Example
    /// ```
    /// # use chemfiles::Frame;
    /// let frame = Frame::new().unwrap();
    /// assert_eq!(frame.step(), Ok(0));
    /// ```
    pub fn step(&self) -> Result<u64> {
        let mut res = 0;
        unsafe {
            try!(check(chfl_frame_step(self.as_ptr(), &mut res)));
        }
        return Ok(res);
    }

    /// Set this frame step to `step`.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::Frame;
    /// let mut frame = Frame::new().unwrap();
    /// assert_eq!(frame.step(), Ok(0));
    ///
    /// frame.set_step(10).unwrap();
    /// assert_eq!(frame.step(), Ok(10));
    /// ```
    pub fn set_step(&mut self, step: u64) -> Result<()> {
        unsafe {
            try!(check(chfl_frame_set_step(self.as_mut_ptr(), step)));
        }
        return Ok(());
    }

    /// Guess the bonds, angles and dihedrals in this `frame`.
    ///
    /// The bonds are guessed using a distance-based algorithm, and then angles
    /// and dihedrals are guessed from the bonds.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::{Frame, Atom};
    /// let mut frame = Frame::new().unwrap();
    ///
    /// frame.add_atom(&Atom::new("Cl").unwrap(), [0.0, 0.0, 0.0], None).unwrap();
    /// frame.add_atom(&Atom::new("Cl").unwrap(), [1.5, 0.0, 0.0], None).unwrap();
    /// assert_eq!(frame.topology().unwrap().bonds_count(), Ok(0));
    ///
    /// frame.guess_topology().unwrap();
    /// assert_eq!(frame.topology().unwrap().bonds_count(), Ok(1));
    /// ```
    pub fn guess_topology(&mut self) -> Result<()> {
        unsafe {
            try!(check(chfl_frame_guess_topology(self.as_mut_ptr())));
        }
        return Ok(());
    }


    /// Add a new `property` with the given `name` to this frame.
    ///
    /// If a property with the same name already exists, this function override
    /// the existing property with the new one.
    ///
    /// # Examples
    /// ```
    /// # use chemfiles::{Frame, Property};
    /// let mut frame = Frame::new().unwrap();
    /// frame.set("a string", Property::String("hello".into()));
    ///
    /// assert_eq!(frame.get("a string").unwrap(), Some(Property::String("hello".into())));
    /// ```
    #[allow(needless_pass_by_value)]  // property
    pub fn set(&mut self, name: &str, property: Property) -> Result<()> {
        let buffer = strings::to_c(name);
        let property = try!(property.as_raw());
        unsafe {
            try!(check(
                chfl_frame_set_property(self.as_mut_ptr(), buffer.as_ptr(), property.as_ptr())
            ));
        }
        return Ok(());
    }

    /// Get a property with the given `name` in this frame, if it exist.
    ///
    /// # Examples
    /// ```
    /// # use chemfiles::{Frame, Property};
    /// let mut frame = Frame::new().unwrap();
    /// frame.set("foo", Property::Double(22.2));
    ///
    /// assert_eq!(frame.get("foo").unwrap(), Some(Property::Double(22.2)));
    /// assert_eq!(frame.get("Bar").unwrap(), None);
    /// ```
    pub fn get(&mut self, name: &str) -> Result<Option<Property>> {
        let buffer = strings::to_c(name);
        unsafe {
            let handle = chfl_frame_get_property(self.as_ptr(), buffer.as_ptr());
            if handle.is_null() {
                Ok(None)
            } else {
                let raw = try!(RawProperty::from_ptr(handle));
                let property = try!(Property::from_raw(raw));
                Ok(Some(property))
            }
        }
    }
}

impl Drop for Frame {
    fn drop(&mut self) {
        unsafe {
            let status = chfl_frame_free(self.as_mut_ptr());
            debug_assert_eq!(status, chfl_status::CHFL_SUCCESS);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use {Atom, Topology, UnitCell};

    #[test]
    fn clone() {
        let mut frame = Frame::new().unwrap();
        assert_eq!(frame.size(), Ok(0));
        let copy = frame.clone();
        assert_eq!(copy.size(), Ok(0));

        frame.resize(42).unwrap();
        assert_eq!(frame.size(), Ok(42));
        assert_eq!(copy.size(), Ok(0));
    }

    #[test]
    fn size() {
        let mut frame = Frame::new().unwrap();
        assert_eq!(frame.size(), Ok(0));

        frame.resize(12).unwrap();
        assert_eq!(frame.size(), Ok(12));
    }

    #[test]
    fn add_atom() {
        let atom = Atom::new("U").unwrap();
        let mut frame = Frame::new().unwrap();

        frame.add_atom(&atom, [1.0, 1.0, 2.0], None).unwrap();
        assert_eq!(frame.size(), Ok(1));
        assert_eq!(frame.atom(0).unwrap().name(), Ok("U".into()));

        let positions: &[[f64; 3]] = &[[1.0, 1.0, 2.0]];
        assert_eq!(frame.positions(), Ok(positions));

        frame.add_velocities().unwrap();

        let atom = Atom::new("F").unwrap();
        frame.add_atom(&atom, [1.0, 1.0, 2.0], [4.0, 3.0, 2.0]).unwrap();
        assert_eq!(frame.size(), Ok(2));
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
        let expected = [
            [1.0, 2.0, 3.0],
            [4.0, 5.0, 6.0],
            [7.0, 8.0, 9.0],
            [10.0, 11.0, 12.0],
        ];

        frame.positions_mut().unwrap().clone_from_slice(expected.as_ref());
        assert_eq!(frame.positions(), Ok(expected.as_ref()));
    }

    #[test]
    fn velocities() {
        let mut frame = Frame::new().unwrap();
        frame.resize(4).unwrap();
        assert_eq!(frame.has_velocities(), Ok(false));
        frame.add_velocities().unwrap();
        assert_eq!(frame.has_velocities(), Ok(true));

        let expected = [
            [1.0, 2.0, 3.0],
            [4.0, 5.0, 6.0],
            [7.0, 8.0, 9.0],
            [10.0, 11.0, 12.0],
        ];

        frame.velocities_mut().unwrap().clone_from_slice(expected.as_ref());
        assert_eq!(frame.velocities(), Ok(expected.as_ref()));
    }

    #[test]
    fn cell() {
        let mut frame = Frame::new().unwrap();
        let cell = UnitCell::new([3.0, 4.0, 5.0]).unwrap();

        assert!(frame.set_cell(&cell).is_ok());
        let cell = frame.cell().unwrap();
        assert_eq!(cell.lengths(), Ok([3.0, 4.0, 5.0]));
    }

    #[test]
    fn topology() {
        let mut frame = Frame::new().unwrap();
        frame.resize(2).unwrap();
        let mut topology = Topology::new().unwrap();

        topology.add_atom(&Atom::new("Zn").unwrap()).unwrap();
        topology.add_atom(&Atom::new("Ar").unwrap()).unwrap();

        assert!(frame.set_topology(&topology).is_ok());

        let topology = frame.topology().unwrap();

        assert_eq!(topology.atom(0).unwrap().name(), Ok(String::from("Zn")));
        assert_eq!(topology.atom(1).unwrap().name(), Ok(String::from("Ar")));

        assert_eq!(frame.atom(0).unwrap().name(), Ok(String::from("Zn")));
        assert_eq!(frame.atom(1).unwrap().name(), Ok(String::from("Ar")));
    }

    #[test]
    fn bonds() {
        let mut frame = Frame::new().unwrap();
        let atom = &Atom::new("").unwrap();
        frame.add_atom(atom, [0.0, 0.0, 0.0], None).unwrap();
        frame.add_atom(atom, [0.0, 0.0, 0.0], None).unwrap();
        frame.add_atom(atom, [0.0, 0.0, 0.0], None).unwrap();

        frame.add_bond(0, 1).unwrap();
        frame.add_bond(2, 1).unwrap();

        assert_eq!(frame.topology().unwrap().bonds(), Ok(vec![[0, 1], [1, 2]]));

        frame.remove_bond(2, 1).unwrap();
        // Various useless operations to make sure they don't crash
        frame.remove_bond(2, 1).unwrap();
        frame.remove_bond(2, 0).unwrap();

        assert_eq!(frame.topology().unwrap().bonds(), Ok(vec![[0, 1]]));
    }

    #[test]
    fn residues() {
        let mut frame = Frame::new().unwrap();
        assert_eq!(frame.topology().unwrap().residues_count(), Ok(0));

        let residue = &Residue::new("foobar").unwrap();
        frame.add_residue(residue).unwrap();
        frame.add_residue(residue).unwrap();
        frame.add_residue(residue).unwrap();

        assert_eq!(frame.topology().unwrap().residues_count(), Ok(3));
        assert_eq!(frame.topology().unwrap().residue(0).unwrap().name().unwrap(), "foobar");
    }

    #[test]
    fn step() {
        let mut frame = Frame::new().unwrap();
        assert_eq!(frame.step(), Ok(0));

        assert!(frame.set_step(42).is_ok());
        assert_eq!(frame.step(), Ok(42));
    }

    #[test]
    fn property() {
        let mut frame = Frame::new().unwrap();
        assert_eq!(frame.set("foo", Property::Double(-22.0)), Ok(()));
        assert_eq!(frame.get("foo"), Ok(Some(Property::Double(-22.0))));
    }

    #[test]
    fn pbc_geometry() {
        use std::f64::consts::PI;

        let mut frame = Frame::new().unwrap();
        let atom = &Atom::new("").unwrap();

        frame.add_atom(atom, [1.0, 0.0, 0.0], None).unwrap();
        frame.add_atom(atom, [0.0, 0.0, 0.0], None).unwrap();
        frame.add_atom(atom, [0.0, 1.0, 0.0], None).unwrap();
        frame.add_atom(atom, [0.0, 1.0, 1.0], None).unwrap();
        frame.add_atom(atom, [0.0, 0.0, 2.0], None).unwrap();

        assert_eq!(frame.distance(0, 2), Ok(f64::sqrt(2.0)));
        assert_eq!(frame.angle(0, 1, 2), Ok(PI / 2.0));
        assert_eq!(frame.dihedral(0, 1, 2, 3), Ok(PI / 2.0));
        assert_eq!(frame.out_of_plane(1, 4, 0, 2), Ok(2.0));
    }
}

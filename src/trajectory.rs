// Chemfiles, a modern library for chemistry file reading and writing
// Copyright (C) 2015-2017 Guillaume Fraux
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/
use std::ops::Drop;
use std::path::Path;
use std::ptr;

use chemfiles_sys::*;
use errors::{check, Error};
use strings;
use Result;

use {UnitCell, Topology, Frame};

/// The `Trajectory` type is the main entry point when using chemfiles. A
/// `Trajectory` behave a bit like a file, allowing to read and/or write
/// `Frame`.
pub struct Trajectory {
    handle: *const CHFL_TRAJECTORY
}

impl Trajectory {
    /// Create a `Trajectory` from a C pointer.
    ///
    /// This function is unsafe because no validity check is made on the pointer,
    /// except for it being non-null.
    #[inline]
    #[doc(hidden)]
    pub unsafe fn from_ptr(ptr: *const CHFL_TRAJECTORY) -> Result<Trajectory> {
        if ptr.is_null() {
            Err(Error::null_ptr())
        } else {
            Ok(Trajectory{handle: ptr})
        }
    }

    /// Get the underlying C pointer as a const pointer.
    #[inline]
    #[doc(hidden)]
    pub fn as_ptr(&self) -> *const CHFL_TRAJECTORY {
        self.handle
    }

    /// Get the underlying C pointer as a mutable pointer.
    #[inline]
    #[doc(hidden)]
    pub fn as_mut_ptr(&mut self) -> *mut CHFL_TRAJECTORY {
        self.handle as *mut CHFL_TRAJECTORY
    }

    /// Open the file at the given `path` in the given `mode`.
    ///
    /// Valid modes are `'r'` for read, `'w'` for write and `'a'` for append.
    ///
    /// # Example
    /// ```no_run
    /// # use chemfiles::Trajectory;
    /// let trajectory = Trajectory::open("water.xyz", 'r').unwrap();
    /// ```
    pub fn open<P>(path: P, mode: char) -> Result<Trajectory> where P: AsRef<Path> {
        let path = try!(path.as_ref().to_str().ok_or(
            Error::utf8_path_error(path.as_ref())
        ));

        let path = strings::to_c(path);
        unsafe {
            #[allow(cast_possible_wrap)]
            let handle = chfl_trajectory_open(path.as_ptr(), mode as i8);
            Trajectory::from_ptr(handle)
        }
    }

    /// Open the file at the given `path` using a specific file `format` and
    /// the given `mode`.
    ///
    /// Valid modes are `'r'` for read, `'w'` for write and `'a'` for append.
    ///
    /// Specifying a format is needed when the file format does not match the
    /// extension, or when there is not standard extension for this format. If
    /// `format` is an empty string, the format will be guessed from the
    /// extension.
    ///
    /// # Example
    /// ```no_run
    /// # use chemfiles::Trajectory;
    /// let trajectory = Trajectory::open_with_format("water.zeo", 'r', "XYZ").unwrap();
    /// ```
    pub fn open_with_format<'a, P, S>(filename: P, mode: char, format: S) -> Result<Trajectory> where P: AsRef<Path>, S: Into<&'a str> {
        let filename = try!(filename.as_ref().to_str().ok_or(
            Error::utf8_path_error(filename.as_ref())
        ));

        let filename = strings::to_c(filename);
        let format = strings::to_c(format.into());
        unsafe {
            #[allow(cast_possible_wrap)]
            let handle = chfl_trajectory_with_format(
                filename.as_ptr(),
                mode as i8,
                format.as_ptr()
            );
            Trajectory::from_ptr(handle)
        }
    }

    /// Read the next step of this trajectory into a `frame`.
    ///
    /// If the number of atoms in frame does not correspond to the number of atom
    /// in the next step, the frame is resized.
    ///
    /// # Example
    /// ```no_run
    /// # use chemfiles::{Trajectory, Frame};
    /// let mut trajectory = Trajectory::open("water.xyz", 'r').unwrap();
    /// let mut frame = Frame::new().unwrap();
    ///
    /// trajectory.read(&mut frame).unwrap();
    /// ```
    pub fn read(&mut self, frame: &mut Frame) -> Result<()> {
        unsafe {
            try!(check(chfl_trajectory_read(
                self.as_mut_ptr(), frame.as_mut_ptr()
            )))
        }
        Ok(())
    }

    /// Read a specific `step` of this trajectory into a `frame`.
    ///
    /// If the number of atoms in frame does not correspond to the number of
    /// atom at this step, the frame is resized.
    ///
    /// # Example
    /// ```no_run
    /// # use chemfiles::{Trajectory, Frame};
    /// let mut trajectory = Trajectory::open("water.xyz", 'r').unwrap();
    /// let mut frame = Frame::new().unwrap();
    ///
    /// trajectory.read_step(10, &mut frame).unwrap();
    /// ```
    pub fn read_step(&mut self, step: u64, frame: &mut Frame) -> Result<()> {
        unsafe {
            try!(check(chfl_trajectory_read_step(
                self.as_mut_ptr(), step, frame.as_mut_ptr()
            )))
        }
        Ok(())
    }

    /// Write a `frame` to this trajectory.
    ///
    /// # Example
    /// ```no_run
    /// # use chemfiles::{Trajectory, Frame};
    /// let mut trajectory = Trajectory::open("water.pdb", 'w').unwrap();
    /// let mut frame = Frame::new().unwrap();
    ///
    /// trajectory.write(&mut frame).unwrap();
    /// ```
    pub fn write(&mut self, frame: &Frame) -> Result<()> {
        unsafe {
            try!(check(chfl_trajectory_write(self.as_mut_ptr(), frame.as_ptr())))
        }
        Ok(())
    }

    /// Set the `topology` associated with this trajectory. This topology will
    /// be used when reading and writing the files, replacing any topology in
    /// the frames or files.
    ///
    /// # Example
    /// ```no_run
    /// # use chemfiles::{Trajectory, Atom, Topology};
    /// let mut topology = Topology::new().unwrap();
    /// topology.add_atom(&Atom::new("H").unwrap()).unwrap();
    /// topology.add_atom(&Atom::new("O").unwrap()).unwrap();
    /// topology.add_atom(&Atom::new("H").unwrap()).unwrap();
    /// topology.add_bond(0, 1).unwrap();
    /// topology.add_bond(1, 2).unwrap();
    ///
    /// let mut trajectory = Trajectory::open("water.xyz", 'r').unwrap();
    /// trajectory.set_topology(&topology).unwrap();
    /// ```
    pub fn set_topology(&mut self, topology: &Topology) -> Result<()> {
        unsafe {
            try!(check(chfl_trajectory_set_topology(self.as_mut_ptr(), topology.as_ptr())))
        }
        Ok(())
    }

    /// Set the topology associated with this trajectory by reading the first
    /// frame of the file at the given `path` using the file format in
    /// `format`; and extracting the topology of this frame.
    ///
    /// # Example
    /// ```no_run
    /// # use chemfiles::Trajectory;
    /// let mut trajectory = Trajectory::open("water.nc", 'r').unwrap();
    /// trajectory.set_topology_file("topology.pdb").unwrap();
    /// ```
    pub fn set_topology_file<P>(&mut self, path: P) -> Result<()> where P: AsRef<Path> {
        let path = try!(path.as_ref().to_str().ok_or(
            Error::utf8_path_error(path.as_ref())
        ));

        let path = strings::to_c(path);
        unsafe {
            try!(check(chfl_trajectory_topology_file(
                self.as_mut_ptr(),
                path.as_ptr(),
                ptr::null()
            )))
        }
        Ok(())
    }

    /// Set the topology associated with this trajectory by reading the first
    /// frame of the file at the given `path` using the file format in
    /// `format`; and extracting the topology of this frame.
    ///
    /// If `format` is an empty string or `NULL`, the format will be guessed
    /// from the path extension.
    ///
    /// # Example
    /// ```no_run
    /// # use chemfiles::Trajectory;
    /// let mut trajectory = Trajectory::open("water.nc", 'r').unwrap();
    /// trajectory.set_topology_with_format("topology.mol", "PDB").unwrap();
    /// ```
    pub fn set_topology_with_format<'a, P, S>(&mut self, path: P, format: S) -> Result<()>
        where P: AsRef<Path>, S: Into<&'a str> {
        let path = try!(path.as_ref().to_str().ok_or(
            Error::utf8_path_error(path.as_ref())
        ));

        let format = strings::to_c(format.into());
        let path = strings::to_c(path);
        unsafe {
            try!(check(chfl_trajectory_topology_file(
                self.as_mut_ptr(),
                path.as_ptr(),
                format.as_ptr()
            )))
        }
        Ok(())
    }

    /// Set the unit `cell` associated with a trajectory. This cell will be
    /// used when reading and writing the files, replacing any unit cell in the
    /// frames or files.
    ///
    /// # Example
    /// ```no_run
    /// # use chemfiles::{Trajectory, UnitCell};
    /// let mut trajectory = Trajectory::open("water.xyz", 'r').unwrap();
    /// trajectory.set_cell(&UnitCell::new(10.0, 11.0, 12.5).unwrap()).unwrap();
    /// ```
    pub fn set_cell(&mut self, cell: &UnitCell) -> Result<()> {
        unsafe {
            try!(check(chfl_trajectory_set_cell(
                self.as_mut_ptr(), cell.as_ptr()
            )))
        }
        Ok(())
    }

    /// Get the number of steps (the number of frames) in a trajectory.
    ///
    /// # Example
    /// ```no_run
    /// # use chemfiles::Trajectory;
    /// let mut trajectory = Trajectory::open("water.xyz", 'r').unwrap();
    /// let steps = trajectory.nsteps().unwrap();
    ///
    /// println!("This trajectory contains {} steps", steps);
    /// ```
    // FIXME should this take &self instead? The file can be modified by this
    // function, but the format should reset the state.
    pub fn nsteps(&mut self) -> Result<u64> {
        let mut res = 0;
        unsafe {
            try!(check(chfl_trajectory_nsteps(self.as_mut_ptr(), &mut res)));
        }
        Ok(res)
    }
}

impl Drop for Trajectory {
    fn drop(&mut self) {
        unsafe {
            let status = chfl_trajectory_close(self.as_mut_ptr());
            debug_assert_eq!(status, chfl_status::CHFL_SUCCESS);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use std::fs;
    use std::path::Path;
    use std::io::Read;

    use ::{Frame, Topology, UnitCell, Atom};

    #[test]
    fn read() {
        let root = Path::new(file!()).parent().unwrap().join("..");
        let filename = root.join("data").join("water.xyz");
        let mut file = Trajectory::open(filename.to_str().unwrap(), 'r').unwrap();

        assert_eq!(file.nsteps(), Ok(100));

        let mut frame = Frame::new().unwrap();
        assert!(file.read(&mut frame).is_ok());

        assert_eq!(frame.natoms(), Ok(297));

        {
            let positions = frame.positions().unwrap();
            assert_eq!(positions[0], [0.417219, 8.303366, 11.737172]);
            assert_eq!(positions[124], [5.099554, -0.045104, 14.153846]);
        }

        assert_eq!(frame.atom(0).unwrap().name(), Ok(String::from("O")));

        assert!(file.set_cell(&UnitCell::new(30.0, 30.0, 30.0).unwrap()).is_ok());

        assert!(file.read_step(41, &mut frame).is_ok());
        let cell = frame.cell().unwrap();
        assert_eq!(cell.lengths(), Ok((30.0, 30.0, 30.0)));

        {
            let positions = frame.positions().unwrap();
            assert_eq!(positions[0], [0.761277, 8.106125, 10.622949]);
            assert_eq!(positions[124], [5.13242, 0.079862, 14.194161]);
        }

        let topology = frame.topology().unwrap();
        assert_eq!(topology.natoms(), Ok(297));
        assert_eq!(topology.bonds_count(), Ok(0));

        assert!(frame.guess_topology().is_ok());
        let topology = frame.topology().unwrap();
        assert_eq!(topology.natoms(), Ok(297));
        assert_eq!(topology.bonds_count(), Ok(181));
        assert_eq!(topology.angles_count(), Ok(87));

        let mut topology = Topology::new().unwrap();
        let atom = Atom::new("Cs").unwrap();
        for _ in 0..297 {
            topology.add_atom(&atom).unwrap();
        }

        assert!(file.set_topology(&topology).is_ok());
        assert!(file.read_step(10, &mut frame).is_ok());
        assert_eq!(frame.atom(42).unwrap().name(), Ok(String::from("Cs")));

        let filename = root.join("data").join("topology.xyz");
        assert!(file.set_topology_file(filename.to_str().unwrap()).is_ok());
        assert!(file.read(&mut frame).is_ok());
        assert_eq!(frame.atom(100).unwrap().name(), Ok(String::from("Rd")));

        let filename = root.join("data").join("helium.xyz.but.not.really");
        let mut file = Trajectory::open_with_format(
            filename.to_str().unwrap(), 'r', "XYZ"
        ).unwrap();
        assert!(file.read(&mut frame).is_ok());
        assert_eq!(frame.natoms(), Ok(125));
    }

    fn write_file(path: &str) {
        let mut file = Trajectory::open(path, 'w').unwrap();
        let mut frame = Frame::new().unwrap();
        frame.resize(4).unwrap();

        {
            let positions = frame.positions_mut().unwrap();
            for i in 0..positions.len() {
                positions[i] = [1.0, 2.0, 3.0];
            }
        }

        let mut topology = Topology::new().unwrap();
        let atom = Atom::new("X").unwrap();
        for _ in 0..4 {
            topology.add_atom(&atom).unwrap();
        }
        frame.set_topology(&topology).unwrap();
        assert!(file.write(&frame).is_ok());
    }

    #[test]
    fn write() {
        let filename = "test-tmp.xyz";
        write_file(filename);

        let expected_content = "4
Written by the chemfiles library
X 1 2 3
X 1 2 3
X 1 2 3
X 1 2 3".lines().collect::<Vec<_>>();

        let mut file = fs::File::open(filename).unwrap();
        let mut content = String::new();
        let _ = file.read_to_string(&mut content).unwrap();

        assert_eq!(expected_content, content.lines().collect::<Vec<_>>());
        fs::remove_file(filename).unwrap();
    }
}

/* Chemfiles, an efficient IO library for chemistry file formats
 * Copyright (C) 2015 Guillaume Fraux
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/
*/
use std::ops::Drop;
use std::path::Path;
use std::ptr;

use chemfiles_sys::*;
use errors::{check, Error};
use string;
use Result;

use {UnitCell, Topology, Frame};

/// A Trajectory is a chemistry file on the hard drive. It is the main entry
/// point of chemfiles.
pub struct Trajectory {
    handle: *mut CHFL_TRAJECTORY
}

impl Trajectory {
    /// Open a trajectory file in read mode.
    pub fn open<P>(filename: P) -> Result<Trajectory> where P: AsRef<Path>{
        let filename = match filename.as_ref().to_str() {
            Some(val) => val,
            None => {
                return Err(Error::utf8_path_error(filename.as_ref()))
            }
        };

        let handle: *mut CHFL_TRAJECTORY;
        let filename = string::to_c(filename);
        unsafe {
            handle = chfl_trajectory_open(filename.as_ptr(), b'r' as i8);
        }

        if handle.is_null() {
            Err(Error::null_ptr())
        } else {
            Ok(Trajectory{handle: handle})
        }
    }

    /// Open a trajectory file in write mode.
    pub fn create<P>(filename: P) -> Result<Trajectory> where P: AsRef<Path> {
        let filename = match filename.as_ref().to_str() {
            Some(val) => val,
            None => {
                return Err(Error::utf8_path_error(filename.as_ref()))
            }
        };

        let handle: *mut CHFL_TRAJECTORY;
        let filename = string::to_c(filename);
        unsafe {
            handle = chfl_trajectory_open(filename.as_ptr(), b'w' as i8);
        }

        if handle.is_null() {
            Err(Error::null_ptr())
        } else {
            Ok(Trajectory{handle: handle})
        }
    }

    /// Open a trajectory file in read mode using a specific `format`.
    pub fn open_with_format<'a, P, S>(filename: P, format: S) -> Result<Trajectory> where P: AsRef<Path>, S: Into<&'a str> {
        let filename = match filename.as_ref().to_str() {
            Some(val) => val,
            None => {
                return Err(Error::utf8_path_error(filename.as_ref()))
            }
        };

        let handle: *mut CHFL_TRAJECTORY;
        let filename = string::to_c(filename);
        let format = string::to_c(format.into());
        unsafe {
            handle = chfl_trajectory_with_format(filename.as_ptr(), b'r' as i8, format.as_ptr());
        }

        if handle.is_null() {
            Err(Error::null_ptr())
        } else {
            Ok(Trajectory{handle: handle})
        }
    }

    /// Open a trajectory file in write mode.
    pub fn create_with_format<'a, P, S>(filename: P, format: S) -> Result<Trajectory> where P: AsRef<Path>, S: Into<&'a str> {
        let filename = match filename.as_ref().to_str() {
            Some(val) => val,
            None => {
                return Err(Error::utf8_path_error(filename.as_ref()))
            }
        };

        let handle: *mut CHFL_TRAJECTORY;
        let filename = string::to_c(filename);
        let format = string::to_c(format.into());
        unsafe {
            handle = chfl_trajectory_with_format(filename.as_ptr(), b'w' as i8, format.as_ptr());
        }

        if handle.is_null() {
            Err(Error::null_ptr())
        } else {
            Ok(Trajectory{handle: handle})
        }
    }

    /// Read the next step of the trajectory into a frame
    pub fn read(&mut self, frame: &mut Frame) -> Result<()> {
        unsafe {
            try!(check(chfl_trajectory_read(
                self.handle,
                frame.as_ptr() as *mut CHFL_FRAME)))
        }
        Ok(())
    }

    /// Read a specific step of the trajectory in a frame
    pub fn read_step(&mut self, step: u64, frame: &mut Frame) -> Result<()> {
        unsafe {
            try!(check(chfl_trajectory_read_step(
                self.handle,
                step,
                frame.as_ptr() as *mut CHFL_FRAME)))
        }
        Ok(())
    }

    /// Write a frame to the trajectory.
    pub fn write(&mut self, frame: &Frame) -> Result<()> {
        unsafe {
            try!(check(chfl_trajectory_write(self.handle, frame.as_ptr())))
        }
        Ok(())
    }

    /// Set the topology associated with a trajectory. This topology will be
    /// used when reading and writing the files, replacing any topology in the
    /// frames or files.
    pub fn set_topology(&mut self, topology: Topology) -> Result<()> {
        unsafe {
            try!(check(chfl_trajectory_set_topology(self.handle, topology.as_ptr())))
        }
        Ok(())
    }

    /// Set the topology associated with a trajectory by reading the first frame
    /// of `filename`; and extracting the topology of this frame.
    pub fn set_topology_file<P>(&mut self, filename: P) -> Result<()> where P: AsRef<Path> {
        let filename = match filename.as_ref().to_str() {
            Some(val) => val,
            None => {
                return Err(Error::utf8_path_error(filename.as_ref()))
            }
        };

        let filename = string::to_c(filename);
        unsafe {
            try!(check(chfl_trajectory_topology_file(
                self.handle,
                filename.as_ptr(),
                ptr::null()
            )))
        }
        Ok(())
    }

    /// Set the unit cell associated with a trajectory. This cell will be used
    /// when reading and writing the files, replacing any unit cell in the
    /// frames or files.
    pub fn set_cell(&mut self, cell: UnitCell) -> Result<()> {
        unsafe {
            try!(check(chfl_trajectory_set_cell(self.handle, cell.as_ptr())))
        }
        Ok(())
    }

    /// Get the number of steps (the number of frames) in a trajectory.
    pub fn nsteps(&mut self) -> Result<u64> {
        let mut res = 0;
        unsafe {
            try!(check(chfl_trajectory_nsteps(self.handle, &mut res)));
        }
        Ok(res)
    }

    /// Create a `Trajectory` from a C pointer. This function is unsafe because
    /// no validity check is made on the pointer.
    pub unsafe fn from_ptr(ptr: *mut CHFL_TRAJECTORY) -> Trajectory {
        Trajectory{handle: ptr}
    }

    /// Get the underlying C pointer. This function is unsafe because no
    /// lifetime guarantee is made on the pointer.
    pub unsafe fn as_ptr(&self) -> *const CHFL_TRAJECTORY {
        self.handle
    }
}

impl Drop for Trajectory {
    fn drop(&mut self) {
        unsafe {
            check(
                chfl_trajectory_close(self.handle as *mut CHFL_TRAJECTORY)
            ).ok().expect("Error while freeing memory!");
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
        let mut file = Trajectory::open(filename.to_str().unwrap()).unwrap();

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

        assert!(file.set_cell(UnitCell::new(30.0, 30.0, 30.0).unwrap()).is_ok());

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
            topology.push(&atom).unwrap();
        }

        assert!(file.set_topology(topology).is_ok());
        assert!(file.read_step(10, &mut frame).is_ok());
        assert_eq!(frame.atom(42).unwrap().name(), Ok(String::from("Cs")));

        let filename = root.join("data").join("topology.xyz");
        assert!(file.set_topology_file(filename.to_str().unwrap()).is_ok());
        assert!(file.read(&mut frame).is_ok());
        assert_eq!(frame.atom(100).unwrap().name(), Ok(String::from("Rd")));

        let filename = root.join("data").join("helium.xyz.but.not.really");
        let mut file = Trajectory::open_with_format(filename.to_str().unwrap(), "XYZ").unwrap();
        assert!(file.read(&mut frame).is_ok());
        assert_eq!(frame.natoms(), Ok(125));
    }

    fn write_file(path: &str) {
        let mut file = Trajectory::create(path).unwrap();
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
            topology.push(&atom).unwrap();
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
X 1 2 3";

        let mut file = fs::File::open(filename).unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();

        assert_eq!(expected_content, content.trim());
        fs::remove_file(filename).unwrap();
    }
}

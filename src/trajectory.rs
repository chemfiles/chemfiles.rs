// Chemfiles, a modern library for chemistry file reading and writing
// Copyright (C) 2015-2018 Guillaume Fraux -- BSD licensed
use std::ops::Drop;
use std::path::Path;
use std::ptr;

use chemfiles_sys::*;
use errors::{check, check_success, Error, Status};
use strings;

use {Frame, Topology, UnitCell};

/// The `Trajectory` type is the main entry point when using chemfiles. A
/// `Trajectory` behave a bit like a file, allowing to read and/or write
/// `Frame`.
pub struct Trajectory {
    handle: *mut CHFL_TRAJECTORY,
}

impl Trajectory {
    /// Create a `Trajectory` from a C pointer.
    ///
    /// This function is unsafe because no validity check is made on the pointer.
    #[inline]
    pub(crate) unsafe fn from_ptr(ptr: *mut CHFL_TRAJECTORY) -> Result<Trajectory, Error> {
        if ptr.is_null() {
            Err(Error {
                status: Status::FileError,
                message: Error::last_error()
            })
        } else {
            Ok(Trajectory {
                handle: ptr
            })
        }
    }

    /// Get the underlying C pointer as a pointer.
    #[inline]
    pub(crate) fn as_ptr(&self) -> *const CHFL_TRAJECTORY {
        self.handle
    }

    /// Get the underlying C pointer as a mutable pointer.
    #[inline]
    pub(crate) fn as_mut_ptr(&mut self) -> *mut CHFL_TRAJECTORY {
        self.handle
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
    pub fn open<P>(path: P, mode: char) -> Result<Trajectory, Error>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref().to_str().ok_or(Error::utf8_path_error(path.as_ref()))?;

        let path = strings::to_c(path);
        unsafe {
            #[allow(clippy::cast_possible_wrap)]
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
    pub fn open_with_format<'a, P, S>(filename: P, mode: char, format: S) -> Result<Trajectory, Error>
    where
        P: AsRef<Path>,
        S: Into<&'a str>,
    {
        let filename =
            filename.as_ref().to_str().ok_or(Error::utf8_path_error(filename.as_ref()))?;

        let filename = strings::to_c(filename);
        let format = strings::to_c(format.into());
        unsafe {
            #[allow(clippy::cast_possible_wrap)]
            let handle = chfl_trajectory_with_format(
                filename.as_ptr(), mode as i8, format.as_ptr()
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
    /// let mut frame = Frame::new();
    ///
    /// trajectory.read(&mut frame).unwrap();
    /// ```
    pub fn read(&mut self, frame: &mut Frame) -> Result<(), Error> {
        unsafe {
            check(chfl_trajectory_read(self.as_mut_ptr(), frame.as_mut_ptr()))
        }
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
    /// let mut frame = Frame::new();
    ///
    /// trajectory.read_step(10, &mut frame).unwrap();
    /// ```
    pub fn read_step(&mut self, step: usize, frame: &mut Frame) -> Result<(), Error> {
        unsafe {
            check(chfl_trajectory_read_step(self.as_mut_ptr(), step as u64, frame.as_mut_ptr()))
        }
    }

    /// Write a `frame` to this trajectory.
    ///
    /// # Example
    /// ```no_run
    /// # use chemfiles::{Trajectory, Frame};
    /// let mut trajectory = Trajectory::open("water.pdb", 'w').unwrap();
    /// let mut frame = Frame::new();
    ///
    /// trajectory.write(&mut frame).unwrap();
    /// ```
    pub fn write(&mut self, frame: &Frame) -> Result<(), Error> {
        unsafe {
            check(chfl_trajectory_write(self.as_mut_ptr(), frame.as_ptr()))
        }
    }

    /// Set the `topology` associated with this trajectory. This topology will
    /// be used when reading and writing the files, replacing any topology in
    /// the frames or files.
    ///
    /// # Example
    /// ```no_run
    /// # use chemfiles::{Trajectory, Atom, Topology};
    /// let mut topology = Topology::new();
    /// topology.add_atom(&Atom::new("H"));
    /// topology.add_atom(&Atom::new("O"));
    /// topology.add_atom(&Atom::new("H"));
    /// topology.add_bond(0, 1);
    /// topology.add_bond(1, 2);
    ///
    /// let mut trajectory = Trajectory::open("water.xyz", 'r').unwrap();
    /// trajectory.set_topology(&topology);
    /// ```
    pub fn set_topology(&mut self, topology: &Topology) {
        unsafe {
            check_success(chfl_trajectory_set_topology(self.as_mut_ptr(), topology.as_ptr()));
        }
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
    pub fn set_topology_file<P>(&mut self, path: P) -> Result<(), Error>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref().to_str().ok_or(Error::utf8_path_error(path.as_ref()))?;

        let path = strings::to_c(path);
        unsafe {
            check(chfl_trajectory_topology_file(self.as_mut_ptr(), path.as_ptr(), ptr::null()))
        }
    }

    /// Set the topology associated with this trajectory by reading the first
    /// frame of the file at the given `path` using the file format in
    /// `format`; and extracting the topology of this frame.
    ///
    /// If `format` is an empty string, the format will be guessed from the
    /// `path` extension.
    ///
    /// # Example
    /// ```no_run
    /// # use chemfiles::Trajectory;
    /// let mut trajectory = Trajectory::open("water.nc", 'r').unwrap();
    /// trajectory.set_topology_with_format("topology.mol", "PDB").unwrap();
    /// ```
    pub fn set_topology_with_format<'a, P, S>(&mut self, path: P, format: S) -> Result<(), Error>
    where
        P: AsRef<Path>,
        S: Into<&'a str>,
    {
        let path = path.as_ref().to_str().ok_or(Error::utf8_path_error(path.as_ref()))?;

        let format = strings::to_c(format.into());
        let path = strings::to_c(path);
        unsafe {
            check(chfl_trajectory_topology_file(self.as_mut_ptr(), path.as_ptr(), format.as_ptr()))
        }
    }

    /// Set the unit `cell` associated with a trajectory. This cell will be
    /// used when reading and writing the files, replacing any unit cell in the
    /// frames or files.
    ///
    /// # Example
    /// ```no_run
    /// # use chemfiles::{Trajectory, UnitCell};
    /// let mut trajectory = Trajectory::open("water.xyz", 'r').unwrap();
    /// trajectory.set_cell(&UnitCell::new([10.0, 11.0, 12.5]));
    /// ```
    pub fn set_cell(&mut self, cell: &UnitCell) {
        unsafe {
            check_success(chfl_trajectory_set_cell(self.as_mut_ptr(), cell.as_ptr()));
        }
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
    pub fn nsteps(&mut self) -> Result<usize, Error> {
        let mut res = 0;
        unsafe {
            check(chfl_trajectory_nsteps(self.as_mut_ptr(), &mut res))?;
        }
        #[allow(clippy::cast_possible_truncation)]
        Ok(res as usize)
    }

    /// Get file path for this trajectory.
    ///
    /// # Example
    /// ```no_run
    /// # use chemfiles::Trajectory;
    /// let trajectory = Trajectory::open("water.xyz", 'r').unwrap();
    ///
    /// assert_eq!(trajectory.path(), "water.xyz");
    /// ```
    pub fn path(&self) -> String {
        let mut path = ::std::ptr::null_mut();
        unsafe {
            check_success(chfl_trajectory_path(self.as_ptr(), &mut path));
        }
        return strings::from_c(path);
    }
}

impl Drop for Trajectory {
    fn drop(&mut self) {
        unsafe {
            let _ = chfl_trajectory_close(self.as_ptr());
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use std::fs;
    use std::path::Path;
    use std::io::Read;

    use {Atom, Frame, Topology, UnitCell};

    #[test]
    fn read() {
        let root = Path::new(file!()).parent().unwrap().join("..");
        let filename = root.join("data").join("water.xyz");
        let mut file = Trajectory::open(filename.to_str().unwrap(), 'r').unwrap();

        if cfg!(target_family = "unix") {
            assert_eq!(file.path(), "src/../data/water.xyz");
        } else if cfg!(target_family = "windows") {
            assert_eq!(file.path(), "src\\..\\data\\water.xyz");
        } else {
            panic!("please add test for this OS!");
        }

        assert_eq!(file.nsteps(), Ok(100));

        let mut frame = Frame::new();
        assert!(file.read(&mut frame).is_ok());

        assert_eq!(frame.size(), 297);

        {
            let positions = frame.positions();
            assert_eq!(positions[0], [0.417219, 8.303366, 11.737172]);
            assert_eq!(positions[124], [5.099554, -0.045104, 14.153846]);
        }

        assert_eq!(frame.atom(0).name(), "O");

        file.set_cell(&UnitCell::new([30.0, 30.0, 30.0]));
        assert!(file.read_step(41, &mut frame).is_ok());
        let cell = frame.cell().clone();
        assert_eq!(cell.lengths(), [30.0, 30.0, 30.0]);

        {
            let positions = frame.positions();
            assert_eq!(positions[0], [0.761277, 8.106125, 10.622949]);
            assert_eq!(positions[124], [5.13242, 0.079862, 14.194161]);
        }

        {
            let topology = frame.topology();
            assert_eq!(topology.size(), 297);
            assert_eq!(topology.bonds_count(), 0);
        }

        assert!(frame.guess_bonds().is_ok());
        {
            let topology = frame.topology();
            assert_eq!(topology.size(), 297);
            assert_eq!(topology.bonds_count(), 181);
            assert_eq!(topology.angles_count(), 87);
        }

        let mut topology = Topology::new();
        let atom = Atom::new("Cs");
        for _ in 0..297 {
            topology.add_atom(&atom);
        }

        file.set_topology(&topology);
        assert!(file.read_step(10, &mut frame).is_ok());
        assert_eq!(frame.atom(42).name(), "Cs");

        let filename = root.join("data").join("topology.xyz");
        assert!(file.set_topology_file(filename.to_str().unwrap()).is_ok());
        assert!(file.read(&mut frame).is_ok());
        assert_eq!(frame.atom(100).name(), "Rd");

        let filename = root.join("data").join("helium.xyz.but.not.really");
        let filename = filename.to_str().unwrap();
        let mut file = Trajectory::open_with_format(filename, 'r', "XYZ").unwrap();
        assert!(file.read(&mut frame).is_ok());
        assert_eq!(frame.size(), 125);
    }

    fn write_file(path: &str) {
        let mut file = Trajectory::open(path, 'w').unwrap();
        let mut frame = Frame::new();
        frame.resize(4);

        {
            let positions = frame.positions_mut();
            for i in 0..positions.len() {
                positions[i] = [1.0, 2.0, 3.0];
            }
        }

        let mut topology = Topology::new();
        let atom = Atom::new("X");
        for _ in 0..4 {
            topology.add_atom(&atom);
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

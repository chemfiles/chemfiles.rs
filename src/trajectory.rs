// Chemfiles, a modern library for chemistry file reading and writing
// Copyright (C) 2015-2018 Guillaume Fraux -- BSD licensed
use std::convert::TryInto;
use std::os::raw::c_char;
use std::path::Path;

use chemfiles_sys as ffi;

use crate::errors::{check, check_success, Error, Status};
use crate::strings;
use crate::{Frame, Topology, UnitCell};

/// The `Trajectory` type is the main entry point when using chemfiles. A
/// `Trajectory` behave a bit like a file, allowing to read and/or write
/// `Frame`.
#[derive(Debug)]
pub struct Trajectory {
    handle: *mut ffi::CHFL_TRAJECTORY,
}

impl Drop for Trajectory {
    fn drop(&mut self) {
        unsafe {
            let _ = ffi::chfl_trajectory_close(self.as_ptr());
        }
    }
}

impl Trajectory {
    /// Create a `Trajectory` from a C pointer.
    ///
    /// This function is unsafe because no validity check is made on the pointer.
    #[inline]
    pub(crate) unsafe fn from_ptr(ptr: *mut ffi::CHFL_TRAJECTORY) -> Result<Trajectory, Error> {
        if ptr.is_null() {
            Err(Error {
                status: Status::FileError,
                message: Error::last_error(),
            })
        } else {
            Ok(Trajectory { handle: ptr })
        }
    }

    /// Get the underlying C pointer as a pointer.
    #[inline]
    pub(crate) fn as_ptr(&self) -> *const ffi::CHFL_TRAJECTORY {
        self.handle
    }

    /// Get the underlying C pointer as a mutable pointer.
    #[inline]
    pub(crate) fn as_mut_ptr(&mut self) -> *mut ffi::CHFL_TRAJECTORY {
        self.handle
    }

    /// Open the file at the given `path` in the given `mode`.
    ///
    /// Valid modes are `'r'` for read, `'w'` for write and `'a'` for append.
    ///
    /// # Errors
    ///
    /// This function fails if the file is not accessible for the given mode, if
    /// it is incorrectly formatted for the corresponding format, or in case of
    /// I/O errors from the OS.
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
        let path = path
            .as_ref()
            .to_str()
            .ok_or_else(|| Error::utf8_path_error(path.as_ref()))?;

        let path = strings::to_c(path);
        unsafe {
            #[allow(clippy::cast_possible_wrap)]
            let handle = ffi::chfl_trajectory_open(path.as_ptr(), mode as c_char);
            Trajectory::from_ptr(handle)
        }
    }

    /// Open the file at the given `path` using a specific file `format` and the
    /// given `mode`.
    ///
    /// Valid modes are `'r'` for read, `'w'` for write and `'a'` for append.
    ///
    /// Specifying a format is needed when the file format does not match the
    /// extension, or when there is not standard extension for this format. If
    /// `format` is an empty string, the format will be guessed from the
    /// extension.
    ///
    /// # Errors
    ///
    /// This function fails if the file is not accessible for the given mode, if
    /// it is incorrectly formatted for the corresponding format, or in case of
    /// I/O errors from the OS.
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
        let filename = filename
            .as_ref()
            .to_str()
            .ok_or_else(|| Error::utf8_path_error(filename.as_ref()))?;

        let filename = strings::to_c(filename);
        let format = strings::to_c(format.into());
        unsafe {
            #[allow(clippy::cast_possible_wrap)]
            let handle = ffi::chfl_trajectory_with_format(filename.as_ptr(), mode as c_char, format.as_ptr());
            Trajectory::from_ptr(handle)
        }
    }

    /// Write to a memory buffer as though it was a formatted file.
    ///
    /// The `format` parameter should follow the same rules as in the main
    /// `Trajectory` constructor, except that compression specification
    /// is not supported.
    ///
    /// The `memory_buffer` function can be used to retrieve the data written
    /// to memory of the `Trajectory`.
    ///
    /// # Errors
    ///
    /// This function fails if the format do not support in-memory writers.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::Trajectory;
    /// let trajectory_memory = Trajectory::memory_writer("SMI");
    ///
    /// // Binary formats typically do not support this feature
    /// assert!(Trajectory::memory_writer("XTC").is_err());
    /// ```
    pub fn memory_writer<'a, S>(format: S) -> Result<Trajectory, Error>
    where
        S: Into<&'a str>,
    {
        let format = strings::to_c(format.into());
        unsafe {
            let handle = ffi::chfl_trajectory_memory_writer(format.as_ptr());
            Trajectory::from_ptr(handle)
        }
    }

    /// Read the next step of this trajectory into a `frame`.
    ///
    /// If the number of atoms in frame does not correspond to the number of atom
    /// in the next step, the frame is resized.
    ///
    /// # Errors
    ///
    /// This function fails if the data is incorrectly formatted for the
    /// corresponding format, or in case of I/O errors from the OS.
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
        unsafe { check(ffi::chfl_trajectory_read(self.as_mut_ptr(), frame.as_mut_ptr())) }
    }

    /// Read a specific `step` of this trajectory into a `frame`.
    ///
    /// If the number of atoms in frame does not correspond to the number of
    /// atom at this step, the frame is resized.
    ///
    /// # Errors
    ///
    /// This function fails if the data is incorrectly formatted for the
    /// corresponding format.
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
            check(ffi::chfl_trajectory_read_step(
                self.as_mut_ptr(),
                step as u64,
                frame.as_mut_ptr(),
            ))
        }
    }

    /// Write a `frame` to this trajectory.
    ///
    /// # Errors
    ///
    /// This function fails if the data is incorrectly formatted for the
    /// corresponding format.
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
        unsafe { check(ffi::chfl_trajectory_write(self.as_mut_ptr(), frame.as_ptr())) }
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
            check_success(ffi::chfl_trajectory_set_topology(self.as_mut_ptr(), topology.as_ptr()));
        }
    }

    /// Set the topology associated with this trajectory by reading the first
    /// frame of the file at the given `path` using the file format in `format`;
    /// and extracting the topology of this frame.
    ///
    /// # Errors
    ///
    /// This function fails if the topology file is incorrectly formatted for
    /// the corresponding format, or in case of I/O errors from the OS.
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
        let path = path
            .as_ref()
            .to_str()
            .ok_or_else(|| Error::utf8_path_error(path.as_ref()))?;

        let path = strings::to_c(path);
        unsafe {
            check(ffi::chfl_trajectory_topology_file(
                self.as_mut_ptr(),
                path.as_ptr(),
                std::ptr::null(),
            ))
        }
    }

    /// Set the topology associated with this trajectory by reading the first
    /// frame of the file at the given `path` using the file format in
    /// `format`; and extracting the topology of this frame.
    ///
    /// If `format` is an empty string, the format will be guessed from the
    /// `path` extension.
    ///
    /// # Errors
    ///
    /// This function fails if the topology file is incorrectly formatted for
    /// the corresponding format, or in case of I/O errors from the OS.
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
        let path = path
            .as_ref()
            .to_str()
            .ok_or_else(|| Error::utf8_path_error(path.as_ref()))?;

        let format = strings::to_c(format.into());
        let path = strings::to_c(path);
        unsafe {
            check(ffi::chfl_trajectory_topology_file(
                self.as_mut_ptr(),
                path.as_ptr(),
                format.as_ptr(),
            ))
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
            check_success(ffi::chfl_trajectory_set_cell(self.as_mut_ptr(), cell.as_ptr()));
        }
    }

    /// Get the number of steps (the number of frames) in a trajectory.
    ///
    /// # Example
    /// ```no_run
    /// # use chemfiles::Trajectory;
    /// let mut trajectory = Trajectory::open("water.xyz", 'r').unwrap();
    ///
    /// println!("This trajectory contains {} steps", trajectory.nsteps());
    /// ```
    // FIXME should this take &self instead? The file can be modified by this
    // function, but the format should reset the state.
    pub fn nsteps(&mut self) -> usize {
        let mut res = 0;
        unsafe {
            check(ffi::chfl_trajectory_nsteps(self.as_mut_ptr(), &mut res))
                .expect("failed to get the number of steps in this trajectory");
        }
        #[allow(clippy::cast_possible_truncation)]
        return res as usize;
    }

    /// Obtain the memory buffer written to by the trajectory.
    ///
    /// # Errors
    ///
    /// This fails if the trajectory was not opened with
    /// `Trajectory::memory_writer`.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::{Atom, BondOrder, Frame, Trajectory};
    /// let mut trajectory_memory = Trajectory::memory_writer("SMI").unwrap();
    ///
    /// let mut frame = Frame::new();
    /// frame.add_atom(&Atom::new("C"), [0.0, 0.0, 0.0], None);
    /// frame.add_atom(&Atom::new("C"), [0.0, 0.0, 0.0], None);
    /// frame.add_bond_with_order(0, 1, BondOrder::Single);
    ///
    /// trajectory_memory.write(&frame).unwrap();
    ///
    /// let result = trajectory_memory.memory_buffer();
    /// assert_eq!(result.unwrap(), "CC\n");
    /// ```
    #[allow(clippy::cast_possible_truncation)]
    pub fn memory_buffer(&self) -> Result<&str, Error> {
        let mut ptr: *const c_char = std::ptr::null();
        let mut count: u64 = 0;
        let buffer = unsafe {
            check(ffi::chfl_trajectory_memory_buffer(self.as_ptr(), &mut ptr, &mut count))?;
            std::slice::from_raw_parts(ptr.cast(), count.try_into().expect("failed to convert u64 to usize"))
        };

        let string = std::str::from_utf8(buffer)?;
        Ok(string)
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
        let get_string = |ptr, len| unsafe { ffi::chfl_trajectory_path(self.as_ptr(), ptr, len) };
        let path = strings::call_autogrow_buffer(1024, get_string).expect("failed to get path string");
        return strings::from_c(path.as_ptr());
    }
}

/// `MemoryTrajectoryReader` is a handle for a `Trajectory` in memory.
pub struct MemoryTrajectoryReader<'data> {
    inner: Trajectory,
    phantom: std::marker::PhantomData<&'data [u8]>,
}

impl<'data> MemoryTrajectoryReader<'data> {
    /// Read a memory buffer as though it was a formatted file.
    ///
    /// The memory buffer used to store the file is given using the `data`
    /// argument. The `format` parameter is required and should follow the same
    /// rules as in the main `Trajectory` constructor.
    ///
    /// # Errors
    ///
    /// This function fails if the data is incorrectly formatted for the
    /// corresponding format, or if the format do not support in-memory readers.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::{MemoryTrajectoryReader, Frame};
    /// let aromatics = "c1ccccc1\nc1ccco1\nc1ccccn1\n";
    /// let mut trajectory = MemoryTrajectoryReader::new(aromatics.as_bytes(), "SMI").unwrap();
    /// let mut frame = Frame::new();
    /// trajectory.read(&mut frame).unwrap();
    /// assert_eq!(frame.size(), 6);
    /// ```
    pub fn new<Data, Format>(data: Data, format: Format) -> Result<MemoryTrajectoryReader<'data>, Error>
    where
        Data: Into<&'data [u8]>,
        Format: AsRef<str>,
    {
        let data = data.into();
        let format = strings::to_c(format.as_ref());
        let trajectory = unsafe {
            let handle = ffi::chfl_trajectory_memory_reader(data.as_ptr().cast(), data.len() as u64, format.as_ptr());
            Trajectory::from_ptr(handle)
        };
        Ok(MemoryTrajectoryReader {
            inner: trajectory?,
            phantom: std::marker::PhantomData,
        })
    }
}

impl<'a> std::ops::Deref for MemoryTrajectoryReader<'a> {
    type Target = Trajectory;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'a> std::ops::DerefMut for MemoryTrajectoryReader<'a> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use std::io::Read;
    use std::path::Path;

    use approx::assert_ulps_eq;

    use crate::{Atom, CellShape, Frame, Topology, UnitCell};

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

        assert_eq!(file.nsteps(), 100);

        let mut frame = Frame::new();
        assert!(file.read(&mut frame).is_ok());

        assert_eq!(frame.size(), 297);
        assert_ulps_eq!(frame.positions()[0][0], 0.417219);
        assert_ulps_eq!(frame.positions()[0][1], 8.303366);
        assert_ulps_eq!(frame.positions()[0][2], 11.737172);
        assert_ulps_eq!(frame.positions()[124][0], 5.099554);
        assert_ulps_eq!(frame.positions()[124][1], -0.045104);
        assert_ulps_eq!(frame.positions()[124][2], 14.153846);

        assert_eq!(frame.atom(0).name(), "O");

        file.set_cell(&UnitCell::new([30.0, 30.0, 30.0]));
        assert!(file.read_step(41, &mut frame).is_ok());
        let cell = frame.cell().clone();
        assert_eq!(cell.lengths(), [30.0, 30.0, 30.0]);

        assert_ulps_eq!(frame.positions()[0][0], 0.761277);
        assert_ulps_eq!(frame.positions()[0][1], 8.106125);
        assert_ulps_eq!(frame.positions()[0][2], 10.622949);
        assert_ulps_eq!(frame.positions()[124][0], 5.13242);
        assert_ulps_eq!(frame.positions()[124][1], 0.079862);
        assert_ulps_eq!(frame.positions()[124][2], 14.194161);

        {
            let topology = frame.topology();
            assert_eq!(topology.size(), 297);
            assert_eq!(topology.bonds_count(), 0);
        }

        assert!(frame.guess_bonds().is_ok());
        {
            let topology = frame.topology();
            assert_eq!(topology.size(), 297);
            assert_eq!(topology.bonds_count(), 180);
            assert_eq!(topology.angles_count(), 84);
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

    fn write_file<P>(path: P)
    where
        P: AsRef<Path>,
    {
        let mut file = Trajectory::open(path, 'w').unwrap();
        let mut frame = Frame::new();
        frame.resize(4);

        for position in frame.positions_mut() {
            *position = [1.0, 2.0, 3.0];
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
Properties=species:S:1:pos:R:3
X 1 2 3
X 1 2 3
X 1 2 3
X 1 2 3"
            .lines()
            .collect::<Vec<_>>();

        let mut file = std::fs::File::open(filename).unwrap();
        let mut content = String::new();
        let _ = file.read_to_string(&mut content).unwrap();

        assert_eq!(expected_content, content.lines().collect::<Vec<_>>());
        std::fs::remove_file(filename).unwrap();
    }

    #[test]
    fn memory() {
        // formats in decreasing order of their memory buffer length to check null termination
        for format in &["CSSR", "GRO", "XYZ"] {
            let mut frame_write = Frame::new();
            frame_write.add_atom(&Atom::new("H"), [1.5, 3.0, -10.0], None);
            frame_write.add_atom(&Atom::new("O"), [2.3, -1.4, 50.0], None);
            frame_write.add_atom(&Atom::new("H"), [-1.5, 10.0, 0.0], None);
            let cell = UnitCell::new([10.0, 11.0, 12.5]);

            let mut trajectory_write = Trajectory::memory_writer(*format).unwrap();
            trajectory_write.set_cell(&cell);
            trajectory_write.write(&frame_write).unwrap();

            let buffer = trajectory_write.memory_buffer().unwrap();
            let mut trajectory_read = MemoryTrajectoryReader::new(buffer.as_bytes(), *format).unwrap();
            let mut frame_read = Frame::new();
            trajectory_read.read(&mut frame_read).unwrap();

            assert_eq!(trajectory_read.nsteps(), 1);
            assert_eq!(frame_read.cell().shape(), CellShape::Orthorhombic);
            assert_eq!(frame_read.size(), 3);
            assert_eq!(frame_read.atom(1).name(), "O");
            crate::assert_vector3d_eq(&frame_read.positions()[2], &[-1.5, 10.0, 0.0], 1e-4);
        }
    }
}

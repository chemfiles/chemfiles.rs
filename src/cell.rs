/* Chemfiles, an efficient IO library for chemistry file formats
 * Copyright (C) 2015 Guillaume Fraux
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/
*/
use std::ops::Drop;

use chemfiles_sys::*;
use errors::{check, Error};
use Result;

/// Available unit cell shapes
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CellShape {
    /// Orthorombic cell, with the three angles equals to 90°
    Orthorhombic,
    /// Triclinic cell, with any values for the angles.
    Triclinic,
    /// Infinite cell, to use when there is no cell.
    Infinite,
}

impl From<chfl_cell_shape_t> for CellShape {
    fn from(celltype: chfl_cell_shape_t) -> CellShape {
        match celltype {
            chfl_cell_shape_t::CHFL_CELL_ORTHORHOMBIC => CellShape::Orthorhombic,
            chfl_cell_shape_t::CHFL_CELL_TRICLINIC => CellShape::Triclinic,
            chfl_cell_shape_t::CHFL_CELL_INFINITE => CellShape::Infinite,
        }
    }
}

impl From<CellShape> for chfl_cell_shape_t {
    fn from(celltype: CellShape) -> chfl_cell_shape_t {
        match celltype {
            CellShape::Orthorhombic => chfl_cell_shape_t::CHFL_CELL_ORTHORHOMBIC,
            CellShape::Triclinic => chfl_cell_shape_t::CHFL_CELL_TRICLINIC,
            CellShape::Infinite => chfl_cell_shape_t::CHFL_CELL_INFINITE,
        }
    }
}

/// An `UnitCell` represent the box containing the atoms in the system, and its
/// periodicity.
///
/// A unit cell is fully represented by three lenghts (a, b, c); and three
/// angles (alpha, beta, gamma). The angles are stored in degrees, and the
/// lenghts in Angstroms. A cell also has a matricial representation, by
/// projecting the three base vector into an orthonormal base. We choose to
/// represent such matrix as an upper triangular matrix:
///
///             | a_x   b_x   c_x |
///             |  0    b_y   c_y |
///             |  0     0    c_z |
///
/// An unit cell also have a cell type, represented by the `CellType` enum.
pub struct UnitCell {
    handle: *const CHFL_CELL
}

impl UnitCell {
    /// Create an `Orthorombic` `UnitCell` from the three lenghts, in Angstroms.
    pub fn new(a: f64, b: f64, c: f64) -> Result<UnitCell> {
        let handle: *const CHFL_CELL;
        let lenghts = [a, b, c];
        unsafe {
            handle = chfl_cell(lenghts.as_ptr());
        }

        if handle.is_null() {
            Err(Error::null_ptr())
        } else {
            Ok(UnitCell{handle: handle})
        }
    }

    /// Create an `Infinite` `UnitCell`
    pub fn infinite() -> Result<UnitCell> {
        let mut cell = try!(UnitCell::new(0.0, 0.0, 0.0));
        try!(cell.set_shape(CellShape::Infinite));
        Ok(cell)
    }

    /// Create an `Triclinic` `UnitCell` from the three lenghts (in Angstroms)
    /// and three angles (in degree). `alpha` is the angle between the vectors
    /// `b` and `c`; `beta` is the between the vectors `a` and `c` and `gamma`
    /// is the angle between the vectors `a` and `b`.
    pub fn triclinic(a: f64, b: f64, c: f64, alpha: f64, beta: f64, gamma: f64) -> Result<UnitCell> {
        let handle: *const CHFL_CELL;
        let lenghts = [a, b, c];
        let angles = [alpha, beta, gamma];
        unsafe {
            handle = chfl_cell_triclinic(lenghts.as_ptr(), angles.as_ptr());
        }

        if handle.is_null() {
            Err(Error::null_ptr())
        } else {
            Ok(UnitCell{handle: handle})
        }
    }

    /// Get the three lenghts of an `UnitCell`, in Angstroms.
    pub fn lengths(&self) -> Result<(f64, f64, f64)> {
        let mut lengths = [0.0f64; 3];
        unsafe {
            try!(check(chfl_cell_lengths(self.as_ptr(), lengths.as_mut_ptr())));
        }
        Ok((lengths[0], lengths[1], lengths[2]))
    }

    /// Set the three lenghts of an `UnitCell`, in Angstroms.
    pub fn set_lengths(&mut self, a:f64, b:f64, c:f64) -> Result<()> {
        let lengths = [a, b, c];
        unsafe {
            try!(check(chfl_cell_set_lengths(self.as_mut_ptr(), lengths.as_ptr())));
        }
        Ok(())
    }

    /// Get the three angles of an `UnitCell`, in degrees.
    pub fn angles(&self) -> Result<(f64, f64, f64)> {
        let mut angles = [0.0f64; 3];
        unsafe {
            try!(check(chfl_cell_angles(self.as_ptr(), angles.as_mut_ptr())));
        }
        Ok((angles[0], angles[1], angles[2]))
    }

    /// Set the three angles of an `UnitCell`, in degrees. This is only possible
    /// with `Triclinic` cells.
    pub fn set_angles(&mut self, alpha:f64, beta:f64, gamma:f64) -> Result<()> {
        let angles = [alpha, beta, gamma];
        unsafe {
            try!(check(chfl_cell_set_angles(self.as_mut_ptr(), angles.as_ptr())));
        }
        Ok(())
    }

    /// Get the unit cell matricial representation.
    pub fn matrix(&self) -> Result<[[f64; 3]; 3]> {
        let mut res = [[0.0; 3]; 3];
        unsafe {
            try!(check(chfl_cell_matrix(self.as_ptr(), res.as_mut_ptr())));
        }
        Ok(res)
    }

    /// Get the shape of the unit cell
    pub fn shape(&self) -> Result<CellShape> {
        let mut shape = chfl_cell_shape_t::CHFL_CELL_INFINITE;
        unsafe {
            try!(check(chfl_cell_shape(self.as_ptr(), &mut shape)));
        }
        Ok(CellShape::from(shape))
    }

    /// Set the shape of the unit cell
    pub fn set_shape(&mut self, shape: CellShape) -> Result<()> {
        unsafe {
            try!(check(chfl_cell_set_shape(self.as_mut_ptr(), shape.into())));
        }
        Ok(())
    }

    /// Get the volume of the unit cell
    pub fn volume(&self) -> Result<f64> {
        let mut res = 0.0;
        unsafe {
            try!(check(chfl_cell_volume(self.as_ptr(), &mut res)));
        }
        Ok(res)
    }

    /// Create an `UnitCell` from a C pointer. This function is unsafe because
    /// no validity check is made on the pointer.
    pub unsafe fn from_ptr(ptr: *const CHFL_CELL) -> UnitCell {
        UnitCell{handle: ptr}
    }

    /// Get the underlying C pointer as a const pointer.
    pub fn as_ptr(&self) -> *const CHFL_CELL {
        self.handle
    }

    /// Get the underlying C pointer as a mutable pointer.
    pub fn as_mut_ptr(&mut self) -> *mut CHFL_CELL {
        self.handle as *mut CHFL_CELL
    }
}

impl Drop for UnitCell {
    fn drop(&mut self) {
        unsafe {
            check(
                chfl_cell_free(self.as_mut_ptr())
            ).ok().expect("Error while freeing memory!");
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn lengths() {
        let mut cell = UnitCell::new(2.0, 3.0, 4.0).unwrap();

        assert_eq!(cell.lengths(), Ok((2.0, 3.0, 4.0)));

        assert!(cell.set_lengths(10.0, 12.0, 11.0).is_ok());
        assert_eq!(cell.lengths(), Ok((10.0, 12.0, 11.0)));
    }

    #[test]
    fn angles() {
        let mut cell = UnitCell::new(2.0, 3.0, 4.0).unwrap();

        assert_eq!(cell.angles(), Ok((90.0, 90.0, 90.0)));

        assert!(cell.set_shape(CellShape::Triclinic).is_ok());
        assert!(cell.set_angles(80.0, 89.0, 100.0).is_ok());

        assert_eq!(cell.angles(), Ok((80.0, 89.0, 100.0)));

        let cell = UnitCell::triclinic(1., 2., 3., 80., 90., 100.).unwrap();
        assert_eq!(cell.angles(), Ok((80.0, 90.0, 100.0)));
    }

    #[test]
    fn volume() {
        let cell = UnitCell::new(2.0, 3.0, 4.0).unwrap();

        assert_eq!(cell.volume(), Ok(2.0 * 3.0 * 4.0));
    }

    #[test]
    fn matrix() {
        let cell = UnitCell::new(2.0, 3.0, 4.0).unwrap();

        let matrix = cell.matrix().unwrap();
        let result = [[2.0, 0.0, 0.0], [0.0, 3.0, 0.0], [0.0, 0.0, 4.0]];

        for i in 0..3 {
            for j in 0..3 {
                assert_approx_eq!(matrix[i][j], result[i][j], 1e-9);
            }
        }
    }

    #[test]
    fn shape() {
        let mut cell = UnitCell::new(2.0, 3.0, 4.0).unwrap();
        assert_eq!(cell.shape(), Ok(CellShape::Orthorhombic));

        assert!(cell.set_shape(CellShape::Infinite).is_ok());
        assert_eq!(cell.shape(), Ok(CellShape::Infinite));

        let cell = UnitCell::infinite().unwrap();
        assert_eq!(cell.shape(), Ok(CellShape::Infinite));

        let cell = UnitCell::triclinic(1., 2., 3., 80., 90., 100.).unwrap();
        assert_eq!(cell.shape(), Ok(CellShape::Triclinic));
    }
}

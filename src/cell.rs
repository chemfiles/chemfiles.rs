/* Chemfiles, an efficient IO library for chemistry file formats
 * Copyright (C) 2015 Guillaume Fraux
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/
*/
use std::ops::Drop;

use chemfiles_sys::*;
use errors::{check, Error, ErrorKind};
use Result;

/// Available unit cell types
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CellType {
    /// Orthorombic cell, with the three angles equals to 90°
    Orthorhombic = CHFL_CELL_ORTHORHOMBIC as isize,
    /// Triclinic cell, with any values for the angles.
    Triclinic = CHFL_CELL_TRICLINIC as isize,
    /// Infinite cell, to use when there is no cell.
    Infinite = CHFL_CELL_INFINITE as isize,
}

impl From<CHFL_CELL_TYPES> for CellType {
    fn from(celltype: CHFL_CELL_TYPES) -> CellType {
        match celltype {
            CHFL_CELL_ORTHORHOMBIC => CellType::Orthorhombic,
            CHFL_CELL_TRICLINIC => CellType::Triclinic,
            CHFL_CELL_INFINITE => CellType::Infinite,
            _ => unreachable!()
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
        let handle : *const CHFL_CELL;
        unsafe {
            handle = chfl_cell(a, b, c);
        }
        if handle.is_null() {
            return Err(Error::new(ErrorKind::ChemfilesCppError));
        }
        Ok(UnitCell{handle: handle})
    }

    /// Create an `Infinite` `UnitCell`
    pub fn infinite() -> Result<UnitCell> {
        let handle : *const CHFL_CELL;
        unsafe {
            handle = chfl_cell(0.0, 0.0, 0.0);
        }
        if handle.is_null() {
            return Err(Error::new(ErrorKind::ChemfilesCppError));
        }
        let mut cell = UnitCell{handle: handle};
        try!(cell.set_cell_type(CellType::Infinite));
        Ok(cell)
    }

    /// Create an `Triclinic` `UnitCell` from the three lenghts (in Angstroms)
    /// and three angles (in degree). `alpha` is the angle between the vectors
    /// `b` and `c`; `beta` is the between the vectors `a` and `c` and `gamma`
    /// is the angle between the vectors `a` and `b`.
    pub fn triclinic(a: f64, b: f64, c: f64, alpha: f64, beta: f64, gamma: f64) -> Result<UnitCell> {
        let handle : *const CHFL_CELL;
        unsafe {
            handle = chfl_cell_triclinic(a, b, c, alpha, beta, gamma);
        }
        if handle.is_null() {
            return Err(Error::new(ErrorKind::ChemfilesCppError));
        }
        Ok(UnitCell{handle: handle})
    }

    /// Get the three lenghts of an `UnitCell`, in Angstroms.
    pub fn lengths(&self) -> Result<(f64, f64, f64)> {
        let (mut a, mut b, mut c) = (0.0, 0.0, 0.0);
        unsafe {
            try!(check(chfl_cell_lengths(self.handle, &mut a, &mut b, &mut c)));
        }
        Ok((a, b, c))
    }

    /// Set the three lenghts of an `UnitCell`, in Angstroms.
    pub fn set_lengths(&mut self, a:f64, b:f64, c:f64) -> Result<()> {
        unsafe {
            try!(check(chfl_cell_set_lengths(self.handle as *mut CHFL_CELL, a, b, c)));
        }
        Ok(())
    }

    /// Get the three angles of an `UnitCell`, in degrees.
    pub fn angles(&self) -> Result<(f64, f64, f64)> {
        let (mut alpha, mut beta, mut gamma) = (0.0, 0.0, 0.0);
        unsafe {
            try!(check(chfl_cell_angles(self.handle, &mut alpha, &mut beta, &mut gamma)));
        }
        Ok((alpha, beta, gamma))
    }

    /// Set the three angles of an `UnitCell`, in degrees. This is only possible
    /// with `Triclinic` cells.
    pub fn set_angles(&mut self, alpha:f64, beta:f64, gamma:f64) -> Result<()> {
        unsafe {
            try!(check(chfl_cell_set_angles(self.handle as *mut CHFL_CELL, alpha, beta, gamma)));
        }
        Ok(())
    }

    /// Get the unit cell matricial representation.
    pub fn matrix(&self) -> Result<[[f64; 3]; 3]> {
        let mut res = [[0.0; 3]; 3];
        unsafe {
            try!(check(chfl_cell_matrix(self.handle, res.as_mut_ptr())));
        }
        Ok(res)
    }

    /// Get the type of the unit cell
    pub fn cell_type(&self) -> Result<CellType> {
        let mut res = 0;
        unsafe {
            try!(check(chfl_cell_type(self.handle, &mut res)));
        }
        Ok(CellType::from(res))
    }

    /// Set the type of the unit cell
    pub fn set_cell_type(&mut self, cell_type: CellType) -> Result<()> {
        unsafe {
            try!(check(chfl_cell_set_type(self.handle as *mut CHFL_CELL, cell_type as CHFL_CELL_TYPES)));
        }
        Ok(())
    }

    /// Get the volume of the unit cell
    pub fn volume(&self) -> Result<f64> {
        let mut res = 0.0;
        unsafe {
            try!(check(chfl_cell_volume(self.handle, &mut res)));
        }
        Ok(res)
    }

    /// Create an `UnitCell` from a C pointer. This function is unsafe because
    /// no validity check is made on the pointer.
    pub unsafe fn from_ptr(ptr: *const CHFL_CELL) -> UnitCell {
        UnitCell{handle: ptr}
    }

    /// Get the underlying C pointer. This function is unsafe because no
    /// lifetime guarantee is made on the pointer.
    pub unsafe fn as_ptr(&self) -> *const CHFL_CELL {
        self.handle
    }
}

impl Drop for UnitCell {
    fn drop(&mut self) {
        unsafe {
            check(
                chfl_cell_free(self.handle as *mut CHFL_CELL)
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

        assert!(cell.set_cell_type(CellType::Triclinic).is_ok());
        assert!(cell.set_angles(80.0, 89.0, 100.0).is_ok());

        assert_eq!(cell.angles(), Ok((80.0, 89.0, 100.0)));
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
    fn cell_type() {
        let mut cell = UnitCell::new(2.0, 3.0, 4.0).unwrap();

        assert_eq!(cell.cell_type(), Ok(CellType::Orthorhombic));

        assert!(cell.set_cell_type(CellType::Infinite).is_ok());
        assert_eq!(cell.cell_type(), Ok(CellType::Infinite));

        let cell = UnitCell::infinite().unwrap();
        assert_eq!(cell.cell_type(), Ok(CellType::Infinite));
    }
}

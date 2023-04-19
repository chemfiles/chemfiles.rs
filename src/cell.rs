// Chemfiles, a modern library for chemistry file reading and writing
// Copyright (C) 2015-2018 Guillaume Fraux -- BSD licensed
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut, Drop};
use std::ptr;

use chemfiles_sys::*;
use errors::{check, check_not_null, check_success, Error};

/// Available unit cell shapes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CellShape {
    /// Orthorhombic cell, with the three angles equals to 90°.
    Orthorhombic,
    /// Triclinic cell, with any values for the angles.
    Triclinic,
    /// Infinite cell, to use when there is no cell.
    Infinite,
}

impl From<chfl_cellshape> for CellShape {
    fn from(celltype: chfl_cellshape) -> CellShape {
        match celltype {
            chfl_cellshape::CHFL_CELL_ORTHORHOMBIC => CellShape::Orthorhombic,
            chfl_cellshape::CHFL_CELL_TRICLINIC => CellShape::Triclinic,
            chfl_cellshape::CHFL_CELL_INFINITE => CellShape::Infinite,
        }
    }
}

impl From<CellShape> for chfl_cellshape {
    fn from(celltype: CellShape) -> chfl_cellshape {
        match celltype {
            CellShape::Orthorhombic => chfl_cellshape::CHFL_CELL_ORTHORHOMBIC,
            CellShape::Triclinic => chfl_cellshape::CHFL_CELL_TRICLINIC,
            CellShape::Infinite => chfl_cellshape::CHFL_CELL_INFINITE,
        }
    }
}

/// An `UnitCell` represent the box containing the atoms, and its periodicity.
///
/// An unit cell is fully represented by three lengths (a, b, c); and three
/// angles (alpha, beta, gamma). The angles are stored in degrees, and the
/// lengths in Angstroms.
///
/// A cell also has a matricial representation, by projecting the three base
/// vector into an orthonormal base. We choose to represent such matrix as an
/// upper triangular matrix:
///
/// ```text
/// | a_x   b_x   c_x |
/// |  0    b_y   c_y |
/// |  0     0    c_z |
/// ```
pub struct UnitCell {
    handle: *mut CHFL_CELL,
}

/// An analog to a reference to an unit cell (`&UnitCell`)
pub struct UnitCellRef<'a> {
    inner: UnitCell,
    marker: PhantomData<&'a UnitCell>,
}

impl<'a> Deref for UnitCellRef<'a> {
    type Target = UnitCell;
    fn deref(&self) -> &UnitCell {
        &self.inner
    }
}

/// An analog to a mutable reference to an unit cell (`&mut UnitCell`)
pub struct UnitCellMut<'a> {
    inner: UnitCell,
    marker: PhantomData<&'a mut UnitCell>,
}

impl<'a> Deref for UnitCellMut<'a> {
    type Target = UnitCell;
    fn deref(&self) -> &UnitCell {
        &self.inner
    }
}

impl<'a> DerefMut for UnitCellMut<'a> {
    fn deref_mut(&mut self) -> &mut UnitCell {
        &mut self.inner
    }
}

impl Clone for UnitCell {
    fn clone(&self) -> UnitCell {
        unsafe {
            let new_handle = chfl_cell_copy(self.as_ptr());
            UnitCell::from_ptr(new_handle)
        }
    }
}

impl UnitCell {
    /// Create an owned `UnitCell` from a C pointer.
    ///
    /// This function is unsafe because no validity check is made on the pointer.
    #[inline]
    pub(crate) unsafe fn from_ptr(ptr: *mut CHFL_CELL) -> UnitCell {
        check_not_null(ptr);
        UnitCell { handle: ptr }
    }

    /// Create a borrowed `UnitCell` from a C pointer.
    ///
    /// This function is unsafe because no validity check is made on the
    /// pointer, and the caller is responsible for setting the right lifetime.
    #[inline]
    pub(crate) unsafe fn ref_from_ptr<'a>(ptr: *const CHFL_CELL) -> UnitCellRef<'a> {
        UnitCellRef {
            inner: UnitCell::from_ptr(ptr as *mut CHFL_CELL),
            marker: PhantomData,
        }
    }

    /// Create a borrowed `UnitCell` from a C pointer.
    ///
    /// This function is unsafe because no validity check is made on the
    /// pointer, except for it being non-null, and the caller is responsible for
    /// setting the right lifetime
    #[inline]
    pub(crate) unsafe fn ref_mut_from_ptr<'a>(ptr: *mut CHFL_CELL) -> UnitCellMut<'a> {
        UnitCellMut {
            inner: UnitCell::from_ptr(ptr),
            marker: PhantomData,
        }
    }

    /// Get the underlying C pointer as a const pointer.
    #[inline]
    pub(crate) fn as_ptr(&self) -> *const CHFL_CELL {
        self.handle
    }

    /// Get the underlying C pointer as a mutable pointer.
    #[inline]
    pub(crate) fn as_mut_ptr(&mut self) -> *mut CHFL_CELL {
        self.handle
    }

    /// Create an `Orthorhombic` `UnitCell` from the three lengths, in Angstroms.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::{UnitCell, CellShape};
    /// let cell = UnitCell::new([30.0, 30.0, 23.0]);
    ///
    /// assert_eq!(cell.lengths(), [30.0, 30.0, 23.0]);
    /// assert_eq!(cell.angles(), [90.0, 90.0, 90.0]);
    /// assert_eq!(cell.shape(), CellShape::Orthorhombic);
    /// ```
    pub fn new(lengths: [f64; 3]) -> UnitCell {
        unsafe {
            let handle = chfl_cell(lengths.as_ptr(), ptr::null());
            UnitCell::from_ptr(handle)
        }
    }

    /// Create an `Infinite` `UnitCell`.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::{UnitCell, CellShape};
    /// let cell = UnitCell::infinite();
    ///
    /// assert_eq!(cell.lengths(), [0.0, 0.0, 0.0]);
    /// assert_eq!(cell.angles(), [90.0, 90.0, 90.0]);
    /// assert_eq!(cell.shape(), CellShape::Infinite);
    /// ```
    pub fn infinite() -> UnitCell {
        let mut cell = UnitCell::new([0.0, 0.0, 0.0]);
        cell.set_shape(CellShape::Infinite).expect("could not set cell shape");
        return cell;
    }

    /// Create an `Triclinic` `UnitCell` from the three lengths (in Angstroms)
    /// and three angles (in degree). `alpha` is the angle between the vectors
    /// `b` and `c`; `beta` is the between the vectors `a` and `c` and `gamma`
    /// is the angle between the vectors `a` and `b`.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::{UnitCell, CellShape};
    /// let cell = UnitCell::triclinic([10.0, 10.0, 10.0], [98.0, 99.0, 90.0]);
    ///
    /// assert_eq!(cell.lengths(), [10.0, 10.0, 10.0]);
    /// assert_eq!(cell.angles()[0], 98.0);
    /// // Rounding errors might occur due to internal representation
    /// assert!((cell.angles()[1] - 99.0).abs() < 1e-12);
    /// assert_eq!(cell.angles()[2], 90.0);
    /// assert_eq!(cell.shape(), CellShape::Triclinic);
    /// ```
    pub fn triclinic(lengths: [f64; 3], angles: [f64; 3]) -> UnitCell {
        unsafe {
            let handle = chfl_cell(lengths.as_ptr(), angles.as_ptr());
            UnitCell::from_ptr(handle)
        }
    }

    /// Create an `UnitCell` from a cell matrix. If `matrix` contains only
    /// zeros, then an `Infinite` cell is created. If only the diagonal of the
    /// matrix is non-zero, then the cell is `Orthorhombic`. Else a
    /// `Triclinic` cell is created. The matrix entries should be in Angstroms.
    ///
    /// # Panics
    ///
    /// If the matrix has a negative determinant, or more generally is not
    /// representing a unit cell.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::{UnitCell, CellShape};
    /// let cell = UnitCell::from_matrix([
    ///     [1.0, 0.0, 0.0], [0.0, 2.0, 0.0], [0.0, 0.0, 3.0]
    /// ]);
    ///
    /// assert_eq!(cell.lengths(), [1.0, 2.0, 3.0]);
    /// assert_eq!(cell.angles(), [90.0, 90.0, 90.0]);
    /// assert_eq!(cell.shape(), CellShape::Orthorhombic);
    /// ```
    pub fn from_matrix(mut matrix: [[f64; 3]; 3]) -> UnitCell {
        unsafe {
            let handle = chfl_cell_from_matrix(matrix.as_mut_ptr());
            UnitCell::from_ptr(handle)
        }
    }

    /// Get the three lengths of the cell, in Angstroms.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::UnitCell;
    /// let cell = UnitCell::new([30.0, 30.0, 23.0]);
    /// assert_eq!(cell.lengths(), [30.0, 30.0, 23.0]);
    /// ```
    pub fn lengths(&self) -> [f64; 3] {
        let mut lengths = [0.0; 3];
        unsafe {
            check_success(chfl_cell_lengths(self.as_ptr(), lengths.as_mut_ptr()));
        }
        return lengths;
    }

    /// Set the three lengths of the cell, in Angstroms.
    ///
    /// # Errors
    ///
    /// This function fails if the unit cell is infinite, or if one of the
    /// lengths is negative.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::UnitCell;
    /// let mut cell = UnitCell::new([30.0, 30.0, 23.0]);
    ///
    /// cell.set_lengths([10.0, 30.0, 42.0]).unwrap();
    /// assert_eq!(cell.lengths(), [10.0, 30.0, 42.0]);
    ///
    /// assert!(UnitCell::infinite().set_lengths([1.0, 1.0, 1.0]).is_err());
    /// ```
    pub fn set_lengths(&mut self, lengths: [f64; 3]) -> Result<(), Error> {
        unsafe { check(chfl_cell_set_lengths(self.as_mut_ptr(), lengths.as_ptr())) }
    }

    /// Get the three angles of the cell, in degrees.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::UnitCell;
    /// let cell = UnitCell::new([20.0, 20.0, 20.0]);
    /// assert_eq!(cell.angles(), [90.0, 90.0, 90.0]);
    ///
    /// let cell = UnitCell::triclinic([20.0, 20.0, 20.0], [100.0, 120.0, 90.0]);
    /// assert_eq!(cell.angles()[0], 100.0);
    /// // Rounding errors might occur due to internal representation
    /// assert!((cell.angles()[1] - 120.0).abs() < 1e-12);
    /// assert_eq!(cell.angles()[2], 90.0);
    /// ```
    pub fn angles(&self) -> [f64; 3] {
        let mut angles = [0.0; 3];
        unsafe {
            check_success(chfl_cell_angles(self.as_ptr(), angles.as_mut_ptr()));
        }
        return angles;
    }

    /// Set the three angles of the cell, in degrees.
    ///
    /// # Errors
    ///
    /// This function fails if the unit cell is not `Triclinic`.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::UnitCell;
    /// let mut cell = UnitCell::triclinic([20.0, 20.0, 20.0], [100.0, 120.0, 90.0]);
    /// assert_eq!(cell.angles()[0], 100.0);
    /// // Rounding errors might occur due to internal representation
    /// assert!((cell.angles()[1] - 120.0).abs() < 1e-12);
    /// assert_eq!(cell.angles()[2], 90.0);
    ///
    /// cell.set_angles([90.0, 90.0, 90.0]).unwrap();
    /// assert_eq!(cell.angles(), [90.0, 90.0, 90.0]);
    /// ```
    pub fn set_angles(&mut self, angles: [f64; 3]) -> Result<(), Error> {
        unsafe { check(chfl_cell_set_angles(self.as_mut_ptr(), angles.as_ptr())) }
    }

    /// Get the unit cell matricial representation.
    ///
    /// The unit cell representation is obtained by aligning the a vector along
    /// the *x* axis and putting the b vector in the *xy* plane. This make the
    /// matrix an upper triangular matrix:
    ///
    /// ```text
    /// | a_x   b_x   c_x |
    /// |  0    b_y   c_y |
    /// |  0     0    c_z |
    /// ```
    ///
    /// # Example
    /// ```
    /// # use chemfiles::UnitCell;
    /// let cell = UnitCell::new([10.0, 20.0, 30.0]);
    ///
    /// let matrix = cell.matrix();
    ///
    /// assert_eq!(matrix[0][0], 10.0);
    /// assert_eq!(matrix[1][1], 20.0);
    /// assert_eq!(matrix[2][2], 30.0);
    ///
    /// assert!(matrix[1][2].abs() < 1e-9);
    /// ```
    pub fn matrix(&self) -> [[f64; 3]; 3] {
        let mut matrix = [[0.0; 3]; 3];
        unsafe {
            check_success(chfl_cell_matrix(self.as_ptr(), matrix.as_mut_ptr()));
        }
        return matrix;
    }

    /// Get the shape of the unit cell.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::{UnitCell, CellShape};
    /// let cell = UnitCell::new([10.0, 20.0, 30.0]);
    /// assert_eq!(cell.shape(), CellShape::Orthorhombic);
    /// ```
    pub fn shape(&self) -> CellShape {
        let mut shape = chfl_cellshape::CHFL_CELL_INFINITE;
        unsafe {
            check_success(chfl_cell_shape(self.as_ptr(), &mut shape));
        }
        return CellShape::from(shape);
    }

    /// Set the shape of the unit cell to `shape`.
    ///
    /// # Errors
    ///
    /// This can fail if the cell length or angles are incompatible with the
    /// new shape.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::{UnitCell, CellShape};
    /// let mut cell = UnitCell::new([10.0, 20.0, 30.0]);
    /// assert_eq!(cell.shape(), CellShape::Orthorhombic);
    ///
    /// cell.set_shape(CellShape::Triclinic).unwrap();
    /// assert_eq!(cell.shape(), CellShape::Triclinic);
    /// ```
    pub fn set_shape(&mut self, shape: CellShape) -> Result<(), Error> {
        unsafe { check(chfl_cell_set_shape(self.as_mut_ptr(), shape.into())) }
    }

    /// Get the volume of the unit cell.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::UnitCell;
    /// let cell = UnitCell::new([10.0, 20.0, 30.0]);
    /// assert_eq!(cell.volume(), 10.0 * 20.0 * 30.0);
    /// ```
    pub fn volume(&self) -> f64 {
        let mut volume = 0.0;
        unsafe {
            check_success(chfl_cell_volume(self.as_ptr(), &mut volume));
        }
        return volume;
    }

    /// Wrap a `vector` in this unit cell.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::UnitCell;
    /// let cell = UnitCell::new([10.0, 20.0, 30.0]);
    ///
    /// let mut vector = [12.0, 5.2, -45.3];
    /// cell.wrap(&mut vector);
    /// assert_eq!(vector, [2.0, 5.2, 14.700000000000003]);
    /// ```
    pub fn wrap(&self, vector: &mut [f64; 3]) {
        unsafe {
            check_success(chfl_cell_wrap(self.as_ptr(), vector.as_mut_ptr()));
        }
    }
}

impl Drop for UnitCell {
    fn drop(&mut self) {
        unsafe {
            let _ = chfl_free(self.as_ptr().cast());
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn clone() {
        let mut cell = UnitCell::new([2.0, 3.0, 4.0]);
        assert_eq!(cell.lengths(), [2.0, 3.0, 4.0]);

        let copy = cell.clone();
        assert_eq!(copy.lengths(), [2.0, 3.0, 4.0]);

        cell.set_lengths([10.0, 12.0, 11.0]).unwrap();
        assert_eq!(cell.lengths(), [10.0, 12.0, 11.0]);
        assert_eq!(copy.lengths(), [2.0, 3.0, 4.0]);
    }

    #[test]
    fn lengths() {
        let mut cell = UnitCell::new([2.0, 3.0, 4.0]);
        assert_eq!(cell.lengths(), [2.0, 3.0, 4.0]);
        cell.set_lengths([10.0, 12.0, 11.0]).unwrap();
        assert_eq!(cell.lengths(), [10.0, 12.0, 11.0]);
    }

    #[test]
    fn angles() {
        let mut cell = UnitCell::new([2.0, 3.0, 4.0]);
        crate::assert_vector3d_eq(&cell.angles(), &[90.0, 90.0, 90.0], 1e-6);

        cell.set_shape(CellShape::Triclinic).unwrap();
        cell.set_angles([80.0, 89.0, 100.0]).unwrap();

        crate::assert_vector3d_eq(&cell.angles(), &[80.0, 89.0, 100.0], 1e-6);

        let cell = UnitCell::triclinic([1., 2., 3.], [80., 90., 100.]);
        crate::assert_vector3d_eq(&cell.angles(), &[80.0, 90.0, 100.0], 1e-6);
    }

    #[test]
    fn volume() {
        let cell = UnitCell::new([2.0, 3.0, 4.0]);
        assert_eq!(cell.volume(), 2.0 * 3.0 * 4.0);
    }

    #[test]
    fn wrap() {
        let cell = UnitCell::new([10.0, 20.0, 30.0]);
        let mut vector = [12.0, 5.2, -45.3];
        cell.wrap(&mut vector);
        crate::assert_vector3d_eq(&vector, &[2.0, 5.2, 14.7], 1e-6);
    }

    #[test]
    fn matrix() {
        let cell = UnitCell::new([2.0, 3.0, 4.0]);

        let matrix = cell.matrix();
        let result = [[2.0, 0.0, 0.0], [0.0, 3.0, 0.0], [0.0, 0.0, 4.0]];

        for i in 0..3 {
            for j in 0..3 {
                assert_ulps_eq!(matrix[i][j], result[i][j], epsilon = 1e-12);
            }
        }
    }

    #[test]
    fn from_matrix() {
        let cell = UnitCell::from_matrix([[10.0, 0.0, 0.0], [0.0, 21.0, 0.0], [0.0, 0.0, 32.0]]);
        assert_eq!(cell.shape(), CellShape::Orthorhombic);
        assert_eq!(cell.lengths(), [10.0, 21.0, 32.0]);

        let result_matrix = [[123.0, 4.08386, 71.7295], [0.0, 233.964, 133.571], [0.0, 0.0, 309.901]];
        let cell = UnitCell::from_matrix(result_matrix);

        assert_eq!(cell.shape(), CellShape::Triclinic);
        for i in 0..3 {
            assert_ulps_eq!(cell.lengths()[i], [123.0, 234.0, 345.0][i], epsilon = 1e-3);
            assert_ulps_eq!(cell.angles()[i], [67.0, 78.0, 89.0][i], epsilon = 1e-3);
        }

        let matrix = cell.matrix();
        for i in 0..3 {
            for j in 0..3 {
                assert_ulps_eq!(matrix[i][j], result_matrix[i][j], epsilon = 1e-12);
            }
        }
    }

    #[test]
    fn shape() {
        let cell = UnitCell::new([2.0, 3.0, 4.0]);
        assert_eq!(cell.shape(), CellShape::Orthorhombic);

        let cell = UnitCell::infinite();
        assert_eq!(cell.shape(), CellShape::Infinite);

        let cell = UnitCell::triclinic([1.0, 2.0, 3.0], [80.0, 90.0, 100.0]);
        assert_eq!(cell.shape(), CellShape::Triclinic);

        let mut cell = UnitCell::new([10.0, 10.0, 10.0]);
        assert_eq!(cell.shape(), CellShape::Orthorhombic);
        cell.set_shape(CellShape::Triclinic).unwrap();
        assert_eq!(cell.shape(), CellShape::Triclinic);
    }
}

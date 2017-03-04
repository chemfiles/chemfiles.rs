// Chemfiles, a modern library for chemistry file reading and writing
// Copyright (C) 2015-2017 Guillaume Fraux
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/
use std::ops::Drop;
use std::u64;

use chemfiles_sys::*;
use errors::{check, Error};
use strings;
use Result;

/// A `Residue` is a group of atoms belonging to the same logical unit. They
/// can be small molecules, amino-acids in a protein, monomers in polymers,
/// *etc.*
pub struct Residue {
    handle: *const CHFL_RESIDUE
}

impl Clone for Residue {
    fn clone(&self) -> Residue {
        unsafe {
            let new_handle = chfl_residue_copy(self.as_ptr());
            Residue::from_ptr(new_handle).expect(
                "Out of memory when copying a Residue"
            )
        }
    }
}

impl Residue {
    /// Create a `Residue` from a C pointer.
    ///
    /// This function is unsafe because no validity check is made on the pointer,
    /// except for it being non-null.
    #[inline]
    #[doc(hidden)]
    pub unsafe fn from_ptr(ptr: *const CHFL_RESIDUE) -> Result<Residue> {
        if ptr.is_null() {
            Err(Error::null_ptr())
        } else {
            Ok(Residue{handle: ptr})
        }
    }

    /// Get the underlying C pointer as a const pointer.
    #[inline]
    #[doc(hidden)]
    pub fn as_ptr(&self) -> *const CHFL_RESIDUE {
        self.handle
    }

    /// Get the underlying C pointer as a mutable pointer.
    #[inline]
    #[doc(hidden)]
    pub fn as_mut_ptr(&mut self) -> *mut CHFL_RESIDUE {
        self.handle as *mut CHFL_RESIDUE
    }

    /// Create a new residue with the given `name`
    ///
    /// # Example
    /// ```
    /// # use chemfiles::Residue;
    /// let residue = Residue::new("ALA").unwrap();
    /// assert_eq!(residue.name(), Ok(String::from("ALA")));
    /// ```
    pub fn new<'a, S>(name: S) -> Result<Residue> where S: Into<&'a str> {
        return Residue::with_id(name, u64::MAX)
    }

    /// Create a new residue with the given `name` and `id` as identifier.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::Residue;
    /// let residue = Residue::with_id("ALA", 67).unwrap();
    /// assert_eq!(residue.name(), Ok(String::from("ALA")));
    /// assert_eq!(residue.id(), Ok(67));
    /// ```
    pub fn with_id<'a, S>(name: S, id: u64) -> Result<Residue> where S: Into<&'a str> {
        let handle: *const CHFL_RESIDUE;
        let buffer = strings::to_c(name.into());
        unsafe {
            handle = chfl_residue(buffer.as_ptr(), id);
        }

        if handle.is_null() {
            Err(Error::null_ptr())
        } else {
            Ok(Residue{handle: handle})
        }
    }

    /// Get the number of atoms in this residue.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::Residue;
    /// let mut residue = Residue::new("water").unwrap();
    /// assert_eq!(residue.natoms(), Ok(0));
    ///
    /// residue.add_atom(0);
    /// residue.add_atom(1);
    /// residue.add_atom(2);
    /// assert_eq!(residue.natoms(), Ok(3));
    /// ```
    pub fn natoms(&self) -> Result<u64> {
        let mut natoms = 0;
        unsafe {
            try!(check(chfl_residue_atoms_count(self.as_ptr(), &mut natoms)));
        }
        return Ok(natoms);
    }

    /// Get the identifier of this residue in the initial topology file.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::Residue;
    /// let residue = Residue::with_id("", 42).unwrap();
    /// assert_eq!(residue.id(), Ok(42));
    /// ```
    pub fn id(&self) -> Result<u64> {
        let mut resid = 0;
        unsafe {
            try!(check(chfl_residue_id(self.as_ptr(), &mut resid)));
        }
        return Ok(resid);
    }

    /// Get the name of this residue.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::Residue;
    /// let residue = Residue::new("water").unwrap();
    /// assert_eq!(residue.name(), Ok(String::from("water")));
    /// ```
    pub fn name(&self) -> Result<String> {
        let name = try!(strings::call_autogrow_buffer(64, |ptr, len| unsafe {
            chfl_residue_name(self.as_ptr(), ptr, len)
        }));
        return Ok(strings::from_c(name.as_ptr()));
    }

    /// Add the atom at index `i` in this residue.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::Residue;
    /// let mut residue = Residue::new("water").unwrap();
    /// assert_eq!(residue.contains(56), Ok(false));
    ///
    /// residue.add_atom(56).unwrap();
    /// assert_eq!(residue.contains(56), Ok(true));
    /// ```
    pub fn add_atom(&mut self, atom: u64) -> Result<()> {
        unsafe {
            try!(check(chfl_residue_add_atom(self.as_mut_ptr(), atom)));
        }
        return Ok(());
    }

    /// Check if the atom at index `i` is in this residue
    ///
    /// # Example
    /// ```
    /// # use chemfiles::Residue;
    /// let mut residue = Residue::new("water").unwrap();
    /// assert_eq!(residue.contains(56), Ok(false));
    ///
    /// residue.add_atom(56).unwrap();
    /// assert_eq!(residue.contains(56), Ok(true));
    /// ```
    pub fn contains(&self, atom: u64) -> Result<bool> {
        let mut res = 0;
        unsafe {
            try!(check(chfl_residue_contains(self.as_ptr(), atom, &mut res)));
        }
        return Ok(res != 0);
    }
}

impl Drop for Residue {
    fn drop(&mut self) {
        unsafe {
            let status = chfl_residue_free(self.as_mut_ptr());
            debug_assert_eq!(status, chfl_status::CHFL_SUCCESS);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::u64;

    #[test]
    fn clone() {
        let mut residue = Residue::new("A").unwrap();
        assert_eq!(residue.natoms(), Ok(0));

        let copy = residue.clone();
        assert_eq!(copy.natoms(), Ok(0));

        residue.add_atom(3).unwrap();
        residue.add_atom(7).unwrap();
        assert_eq!(residue.natoms(), Ok(2));
        assert_eq!(copy.natoms(), Ok(0));
    }

    #[test]
    fn name() {
        let residue = Residue::new("A").unwrap();
        assert_eq!(residue.name(), Ok("A".into()));
    }

    #[test]
    fn id() {
        let residue = Residue::new("A").unwrap();
        assert_eq!(residue.id(), Ok(u64::MAX));

        let residue = Residue::with_id("A", 42).unwrap();
        assert_eq!(residue.id(), Ok(42));
    }

    #[test]
    fn atoms() {
        let mut residue = Residue::new("A").unwrap();
        assert_eq!(residue.natoms(), Ok(0));

        residue.add_atom(0).unwrap();
        residue.add_atom(3).unwrap();
        residue.add_atom(45).unwrap();
        assert_eq!(residue.natoms(), Ok(3));

        assert_eq!(residue.contains(3), Ok(true));
        assert_eq!(residue.contains(5), Ok(false));
    }
}

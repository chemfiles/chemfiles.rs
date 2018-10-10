// Chemfiles, a modern library for chemistry file reading and writing
// Copyright (C) 2015-2018 Guillaume Fraux -- BSD licensed
use std::ops::{Drop, Deref};
use std::marker::PhantomData;
use std::u64;

use chemfiles_sys::*;
use errors::{check_not_null, check_success, Error};
use strings;

/// A `Residue` is a group of atoms belonging to the same logical unit. They
/// can be small molecules, amino-acids in a protein, monomers in polymers,
/// *etc.*
pub struct Residue {
    handle: *mut CHFL_RESIDUE,
}

/// An analog to a reference to a residue (`&Residue`)
pub struct ResidueRef<'a> {
    inner: Residue,
    marker: PhantomData<&'a Residue>
}

impl<'a> Deref for ResidueRef<'a> {
    type Target = Residue;
    fn deref(&self) -> &Residue {
        &self.inner
    }
}

impl Clone for Residue {
    fn clone(&self) -> Residue {
        unsafe {
            let new_handle = chfl_residue_copy(self.as_ptr());
            Residue::from_ptr(new_handle)
        }
    }
}

impl Residue {
    /// Create a `Residue` from a C pointer.
    ///
    /// This function is unsafe because no validity check is made on the pointer.
    #[inline]
    pub(crate) unsafe fn from_ptr(ptr: *mut CHFL_RESIDUE) -> Residue {
        check_not_null(ptr);
        Residue {
            handle: ptr
        }
    }

    /// Create a borrowed `Residue` from a C pointer.
    ///
    /// This function is unsafe because no validity check is made on the
    /// pointer, except for it being non-null, and the caller is responsible
    /// for setting the right lifetime
    #[inline]
    pub(crate) unsafe fn ref_from_ptr<'a>(ptr: *const CHFL_RESIDUE) -> ResidueRef<'a> {
        ResidueRef {
            inner: Residue::from_ptr(ptr as *mut CHFL_RESIDUE),
            marker: PhantomData,
        }
    }

    /// Get the underlying C pointer as a const pointer.
    #[inline]
    pub(crate) fn as_ptr(&self) -> *const CHFL_RESIDUE {
        self.handle
    }

    /// Get the underlying C pointer as a mutable pointer.
    #[inline]
    pub(crate) fn as_mut_ptr(&mut self) -> *mut CHFL_RESIDUE {
        self.handle
    }

    /// Create a new residue with the given `name`
    ///
    /// # Example
    /// ```
    /// # use chemfiles::Residue;
    /// let residue = Residue::new("ALA");
    /// assert_eq!(residue.name(), "ALA");
    /// assert_eq!(residue.id(), None);
    /// ```
    pub fn new<'a>(name: impl Into<&'a str>) -> Residue {
        let buffer = strings::to_c(name.into());
        unsafe {
            let handle = chfl_residue(buffer.as_ptr());
            Residue::from_ptr(handle)
        }
    }

    /// Create a new residue with the given `name` and `id` as identifier.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::Residue;
    /// let residue = Residue::with_id("ALA", 67);
    /// assert_eq!(residue.name(), "ALA");
    /// assert_eq!(residue.id(), Some(67));
    /// ```
    pub fn with_id<'a>(name: impl Into<&'a str>, id: u64) -> Residue {
        let buffer = strings::to_c(name.into());
        unsafe {
            let handle = chfl_residue_with_id(buffer.as_ptr(), id);
            Residue::from_ptr(handle)
        }
    }

    /// Get the number of atoms in this residue.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::Residue;
    /// let mut residue = Residue::new("water");
    /// assert_eq!(residue.size(), 0);
    ///
    /// residue.add_atom(0);
    /// residue.add_atom(1);
    /// residue.add_atom(2);
    /// assert_eq!(residue.size(), 3);
    /// ```
    pub fn size(&self) -> u64 {
        let mut size = 0;
        unsafe {
            check_success(chfl_residue_atoms_count(self.as_ptr(), &mut size));
        }
        return size;
    }

    /// Get the identifier of this residue in the initial topology file.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::Residue;
    /// let residue = Residue::with_id("", 42);
    /// assert_eq!(residue.id(), Some(42));
    /// ```
    pub fn id(&self) -> Option<u64> {
        let mut resid = 0;
        let status = unsafe {
            chfl_residue_id(self.as_ptr(), &mut resid)
        };

        if status == chfl_status::CHFL_SUCCESS {
            return Some(resid);
        } else if status == chfl_status::CHFL_GENERIC_ERROR {
            return None;
        } else {
            panic!("unexpected failure: {}", Error::last_error());
        }
    }

    /// Get the name of this residue.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::Residue;
    /// let residue = Residue::new("water");
    /// assert_eq!(residue.name(), "water");
    /// ```
    pub fn name(&self) -> String {
        let get_name = |ptr, len| unsafe { chfl_residue_name(self.as_ptr(), ptr, len) };
        let name = strings::call_autogrow_buffer(64, get_name).expect("getting residue name failed");
        return strings::from_c(name.as_ptr());
    }

    /// Add the atom at index `atom` in this residue.
    ///
    /// This will fail if the atom is already in the residue.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::Residue;
    /// let mut residue = Residue::new("water");
    /// assert_eq!(residue.size(), 0);
    /// assert_eq!(residue.contains(56), false);
    ///
    /// residue.add_atom(56);
    /// assert_eq!(residue.size(), 1);
    /// assert_eq!(residue.contains(56), true);
    ///
    /// // Adding the same atom twice is fine
    /// residue.add_atom(56);
    /// assert_eq!(residue.size(), 1);
    /// ```
    pub fn add_atom(&mut self, atom: u64) {
        unsafe {
            check_success(chfl_residue_add_atom(self.as_mut_ptr(), atom));
        }
    }

    /// Check if the atom at index `i` is in this residue
    ///
    /// # Example
    /// ```
    /// # use chemfiles::Residue;
    /// let mut residue = Residue::new("water");
    /// assert_eq!(residue.contains(56), false);
    ///
    /// residue.add_atom(56);
    /// assert_eq!(residue.contains(56), true);
    /// ```
    pub fn contains(&self, atom: u64) -> bool {
        let mut inside = 0;
        unsafe {
            check_success(chfl_residue_contains(self.as_ptr(), atom, &mut inside));
        }
        return inside != 0;
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

    #[test]
    fn clone() {
        let mut residue = Residue::new("A");
        assert_eq!(residue.size(), 0);

        let copy = residue.clone();
        assert_eq!(copy.size(), 0);

        residue.add_atom(3);
        residue.add_atom(7);
        assert_eq!(residue.size(), 2);
        assert_eq!(copy.size(), 0);
    }

    #[test]
    fn name() {
        let residue = Residue::new("A");
        assert_eq!(residue.name(), "A");
    }

    #[test]
    fn id() {
        let residue = Residue::new("A");
        assert_eq!(residue.id(), None);

        let residue = Residue::with_id("A", 42);
        assert_eq!(residue.id(), Some(42));
    }

    #[test]
    fn atoms() {
        let mut residue = Residue::new("A");
        assert_eq!(residue.size(), 0);

        residue.add_atom(0);
        residue.add_atom(3);
        residue.add_atom(45);
        assert_eq!(residue.size(), 3);

        assert_eq!(residue.contains(3), true);
        assert_eq!(residue.contains(5), false);
    }
}

// Chemfiles, a modern library for chemistry file reading and writing
// Copyright (C) 2015-2018 Guillaume Fraux -- BSD licensed
use std::marker::PhantomData;

use chemfiles_sys as ffi;

use crate::errors::{check_not_null, check_success};
use crate::property::{PropertiesIter, Property, RawProperty};
use crate::strings;

/// A `Residue` is a group of atoms belonging to the same logical unit. They
/// can be small molecules, amino-acids in a protein, monomers in polymers,
/// *etc.*
#[derive(Debug)]
pub struct Residue {
    handle: *mut ffi::CHFL_RESIDUE,
}

/// An analog to a reference to a residue (`&Residue`)
#[derive(Debug)]
pub struct ResidueRef<'a> {
    inner: Residue,
    marker: PhantomData<&'a Residue>,
}

impl<'a> std::ops::Deref for ResidueRef<'a> {
    type Target = Residue;
    fn deref(&self) -> &Residue {
        &self.inner
    }
}

impl Clone for Residue {
    fn clone(&self) -> Residue {
        unsafe {
            let new_handle = ffi::chfl_residue_copy(self.as_ptr());
            Residue::from_ptr(new_handle)
        }
    }
}

impl Residue {
    /// Create a `Residue` from a C pointer.
    ///
    /// This function is unsafe because no validity check is made on the pointer.
    #[inline]
    pub(crate) unsafe fn from_ptr(ptr: *mut ffi::CHFL_RESIDUE) -> Residue {
        check_not_null(ptr);
        Residue { handle: ptr }
    }

    /// Create a borrowed `Residue` from a C pointer.
    ///
    /// This function is unsafe because no validity check is made on the
    /// pointer, except for it being non-null, and the caller is responsible
    /// for setting the right lifetime
    #[inline]
    #[allow(clippy::ptr_cast_constness)]
    pub(crate) unsafe fn ref_from_ptr<'a>(ptr: *const ffi::CHFL_RESIDUE) -> ResidueRef<'a> {
        ResidueRef {
            inner: Residue::from_ptr(ptr as *mut ffi::CHFL_RESIDUE),
            marker: PhantomData,
        }
    }

    /// Get the underlying C pointer as a const pointer.
    #[inline]
    pub(crate) fn as_ptr(&self) -> *const ffi::CHFL_RESIDUE {
        self.handle
    }

    /// Get the underlying C pointer as a mutable pointer.
    #[inline]
    pub(crate) fn as_mut_ptr(&mut self) -> *mut ffi::CHFL_RESIDUE {
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
            let handle = ffi::chfl_residue(buffer.as_ptr());
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
    pub fn with_id<'a>(name: impl Into<&'a str>, id: i64) -> Residue {
        let buffer = strings::to_c(name.into());
        unsafe {
            let handle = ffi::chfl_residue_with_id(buffer.as_ptr(), id);
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
    pub fn size(&self) -> usize {
        let mut size = 0;
        unsafe {
            check_success(ffi::chfl_residue_atoms_count(self.as_ptr(), &mut size));
        }
        #[allow(clippy::cast_possible_truncation)]
        return size as usize;
    }

    /// Get the identifier of this residue in the initial topology file.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::Residue;
    /// let residue = Residue::with_id("", 42);
    /// assert_eq!(residue.id(), Some(42));
    /// ```
    pub fn id(&self) -> Option<i64> {
        let mut resid = 0;
        let status = unsafe { ffi::chfl_residue_id(self.as_ptr(), &mut resid) };

        if status == ffi::chfl_status::CHFL_SUCCESS {
            return Some(resid);
        } else if status == ffi::chfl_status::CHFL_GENERIC_ERROR {
            return None;
        }

        // call check_success to panic in case of error
        check_success(status);
        unreachable!();
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
        let get_name = |ptr, len| unsafe { ffi::chfl_residue_name(self.as_ptr(), ptr, len) };
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
    pub fn add_atom(&mut self, atom: usize) {
        unsafe {
            check_success(ffi::chfl_residue_add_atom(self.as_mut_ptr(), atom as u64));
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
    pub fn contains(&self, atom: usize) -> bool {
        let mut inside = 0;
        unsafe {
            check_success(ffi::chfl_residue_contains(self.as_ptr(), atom as u64, &mut inside));
        }
        return inside != 0;
    }

    /// Get the list of atoms of this residue.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::Residue;
    /// let mut residue = Residue::new("water");
    /// assert_eq!(residue.atoms(), vec![]);
    ///
    /// residue.add_atom(56);
    /// assert_eq!(residue.atoms(), vec![56]);
    /// ```
    pub fn atoms(&self) -> Vec<usize> {
        let size = self.size();
        let count = size as u64;
        let mut indices = vec![u64::max_value(); size];
        unsafe {
            check_success(ffi::chfl_residue_atoms(self.as_ptr(), indices.as_mut_ptr(), count));
        }
        #[allow(clippy::cast_possible_truncation)]
        return indices.into_iter().map(|idx| idx as usize).collect();
    }

    /// Add a new `property` with the given `name` to this residue.
    ///
    /// If a property with the same name already exists, this function override
    /// the existing property with the new one.
    ///
    /// # Examples
    /// ```
    /// # use chemfiles::{Residue, Property};
    /// let mut residue = Residue::new("ALA");
    /// residue.set("a string", "hello");
    /// residue.set("a double", 3.2);
    ///
    /// assert_eq!(residue.get("a string"), Some(Property::String("hello".into())));
    /// assert_eq!(residue.get("a double"), Some(Property::Double(3.2)));
    /// ```
    pub fn set(&mut self, name: &str, property: impl Into<Property>) {
        let buffer = strings::to_c(name);
        let property = property.into().as_raw();
        unsafe {
            check_success(ffi::chfl_residue_set_property(
                self.as_mut_ptr(),
                buffer.as_ptr(),
                property.as_ptr(),
            ));
        }
    }

    /// Get a property with the given `name` in this frame, if it exist.
    ///
    /// # Examples
    /// ```
    /// # use chemfiles::{Residue, Property};
    /// let mut residue = Residue::new("ALA");
    /// residue.set("foo", Property::Double(22.2));
    ///
    /// assert_eq!(residue.get("foo"), Some(Property::Double(22.2)));
    /// assert_eq!(residue.get("Bar"), None);
    /// ```
    pub fn get(&self, name: &str) -> Option<Property> {
        let buffer = strings::to_c(name);
        unsafe {
            let handle = ffi::chfl_residue_get_property(self.as_ptr(), buffer.as_ptr());
            if handle.is_null() {
                None
            } else {
                let raw = RawProperty::from_ptr(handle);
                Some(Property::from_raw(raw))
            }
        }
    }

    /// Get an iterator over all (name, property) pairs for this frame
    ///
    /// # Examples
    /// ```
    /// # use chemfiles::{Residue, Property};
    /// let mut residue = Residue::new("ALA");
    /// residue.set("foo", Property::Double(22.2));
    /// residue.set("bar", Property::Bool(false));
    ///
    /// for (name, property) in residue.properties() {
    ///     if name == "foo" {
    ///         assert_eq!(property, Property::Double(22.2));
    ///     } else if name == "bar" {
    ///         assert_eq!(property, Property::Bool(false));
    ///     }
    /// }
    /// ```
    pub fn properties(&self) -> PropertiesIter {
        let mut count = 0;
        unsafe {
            check_success(ffi::chfl_residue_properties_count(self.as_ptr(), &mut count));
        }

        #[allow(clippy::cast_possible_truncation)]
        let size = count as usize;
        let mut c_names = vec![std::ptr::null_mut(); size];
        unsafe {
            check_success(ffi::chfl_residue_list_properties(
                self.as_ptr(),
                c_names.as_mut_ptr(),
                count,
            ));
        }

        let mut names = Vec::new();
        for ptr in c_names {
            names.push(strings::from_c(ptr));
        }

        PropertiesIter {
            names: names.into_iter(),
            getter: Box::new(move |name| self.get(name).expect("failed to get property")),
        }
    }
}

impl Drop for Residue {
    fn drop(&mut self) {
        unsafe {
            let _ = ffi::chfl_free(self.as_ptr().cast());
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

        let residue = Residue::with_id("A", -3);
        assert_eq!(residue.id(), Some(-3));
    }

    #[test]
    fn atoms() {
        let mut residue = Residue::new("A");
        assert_eq!(residue.size(), 0);

        residue.add_atom(0);
        residue.add_atom(3);
        residue.add_atom(45);
        assert_eq!(residue.size(), 3);

        assert!(residue.contains(3));
        assert!(!residue.contains(5));

        assert_eq!(residue.atoms(), vec![0, 3, 45]);
    }

    #[test]
    fn property() {
        let mut residue = Residue::new("ALA");

        residue.set("foo", -22.0);
        assert_eq!(residue.get("foo"), Some(Property::Double(-22.0)));
        assert_eq!(residue.get("bar"), None);

        residue.set("bar", Property::String("here".into()));
        for (name, property) in residue.properties() {
            if name == "foo" {
                assert_eq!(property, Property::Double(-22.0));
            } else if name == "bar" {
                assert_eq!(property, Property::String("here".into()));
            }
        }
    }
}

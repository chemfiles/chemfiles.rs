// Chemfiles, a modern library for chemistry file reading and writing
// Copyright (C) 2015-2017 Guillaume Fraux
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/
use std::ops::Drop;

use chemfiles_sys::*;
use errors::{check, Error};
use strings;
use Result;

use property::{Property, RawProperty};

/// An `Atom` is a particle in the current `Frame`. It stores the following
/// atomic properties:
///
/// - atom name;
/// - atom type;
/// - atom mass;
/// - atom charge.
///
/// The atom name is usually an unique identifier (`H1`, `C_a`) while the
/// atom type will be shared between all particles of the same type: `H`,
/// `Ow`, `CH3`.
pub struct Atom {
    handle: *mut CHFL_ATOM,
}

impl Clone for Atom {
    fn clone(&self) -> Atom {
        unsafe {
            let new_handle = chfl_atom_copy(self.as_ptr());
            Atom::from_ptr(new_handle).expect("Out of memory when copying an Atom")
        }
    }
}

impl Atom {
    /// Create an `Atom` from a C pointer.
    ///
    /// This function is unsafe because no validity check is made on the
    /// pointer, except for it being non-null.
    #[inline]
    #[doc(hidden)]
    pub unsafe fn from_ptr(ptr: *mut CHFL_ATOM) -> Result<Atom> {
        if ptr.is_null() {
            Err(Error::null_ptr())
        } else {
            Ok(Atom { handle: ptr })
        }
    }

    /// Get the underlying C pointer as a const pointer.
    #[inline]
    #[doc(hidden)]
    pub fn as_ptr(&self) -> *const CHFL_ATOM {
        self.handle
    }

    /// Get the underlying C pointer as a mutable pointer.
    #[inline]
    #[doc(hidden)]
    pub fn as_mut_ptr(&mut self) -> *mut CHFL_ATOM {
        self.handle
    }

    /// Create an atom with the given `name`, and set the atom type to `name`.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::Atom;
    /// let atom = Atom::new("He").unwrap();
    /// assert_eq!(atom.name(), Ok(String::from("He")));
    /// ```
    pub fn new<'a, S>(name: S) -> Result<Atom>
    where
        S: Into<&'a str>,
    {
        let buffer = strings::to_c(name.into());
        unsafe {
            let handle = chfl_atom(buffer.as_ptr());
            Atom::from_ptr(handle)
        }
    }

    /// Get the atom mass, in atomic mass units.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::Atom;
    /// let atom = Atom::new("He").unwrap();
    /// assert_eq!(atom.mass(), Ok(4.002602));
    /// ```
    pub fn mass(&self) -> Result<f64> {
        let mut mass = 0.0;
        unsafe {
            try!(check(chfl_atom_mass(self.as_ptr(), &mut mass)));
        }
        return Ok(mass);
    }

    /// Set the atom mass to `mass`, in atomic mass units.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::Atom;
    /// let mut atom = Atom::new("He").unwrap();
    ///
    /// atom.set_mass(34.9).unwrap();
    /// assert_eq!(atom.mass(), Ok(34.9));
    /// ```
    pub fn set_mass(&mut self, mass: f64) -> Result<()> {
        unsafe {
            try!(check(chfl_atom_set_mass(self.as_mut_ptr(), mass)));
        }
        return Ok(());
    }

    /// Get the atom charge, in number of the electron charge *e*.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::Atom;
    /// let atom = Atom::new("He").unwrap();
    /// assert_eq!(atom.charge(), Ok(0.0));
    /// ```
    pub fn charge(&self) -> Result<f64> {
        let mut charge = 0.0;
        unsafe {
            try!(check(chfl_atom_charge(self.as_ptr(), &mut charge)));
        }
        return Ok(charge);
    }

    /// Set the atom charge to `charge`, in number of the electron charge *e*.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::Atom;
    /// let mut atom = Atom::new("He").unwrap();
    ///
    /// atom.set_charge(-2.0).unwrap();
    /// assert_eq!(atom.charge(), Ok(-2.0));
    /// ```
    pub fn set_charge(&mut self, charge: f64) -> Result<()> {
        unsafe {
            try!(check(chfl_atom_set_charge(self.as_mut_ptr(), charge)));
        }
        return Ok(());
    }

    /// Get the atom name.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::Atom;
    /// let atom = Atom::new("He").unwrap();
    /// assert_eq!(atom.name(), Ok(String::from("He")));
    /// ```
    pub fn name(&self) -> Result<String> {
        let get_name = |ptr, len| unsafe { chfl_atom_name(self.as_ptr(), ptr, len) };
        let name = try!(strings::call_autogrow_buffer(10, get_name));
        return Ok(strings::from_c(name.as_ptr()));
    }

    /// Get the atom type.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::Atom;
    /// let atom = Atom::new("He").unwrap();
    /// assert_eq!(atom.atomic_type(), Ok(String::from("He")));
    /// ```
    pub fn atomic_type(&self) -> Result<String> {
        let get_type = |ptr, len| unsafe { chfl_atom_type(self.as_ptr(), ptr, len) };
        let buffer = try!(strings::call_autogrow_buffer(10, get_type));
        return Ok(strings::from_c(buffer.as_ptr()));
    }

    /// Set the atom name to `name`.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::Atom;
    /// let mut atom = Atom::new("He").unwrap();
    ///
    /// atom.set_name("Zn3").unwrap();
    /// assert_eq!(atom.name(), Ok(String::from("Zn3")));
    /// ```
    pub fn set_name<'a, S>(&mut self, name: S) -> Result<()>
    where
        S: Into<&'a str>,
    {
        let buffer = strings::to_c(name.into());
        unsafe {
            try!(check(chfl_atom_set_name(self.as_mut_ptr(), buffer.as_ptr())));
        }
        return Ok(());
    }

    /// Set the atom type to `atomic_type`.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::Atom;
    /// let mut atom = Atom::new("He").unwrap();
    ///
    /// atom.set_atomic_type("F").unwrap();
    /// assert_eq!(atom.atomic_type(), Ok(String::from("F")));
    /// ```
    pub fn set_atomic_type<'a, S>(&mut self, atomic_type: S) -> Result<()>
    where
        S: Into<&'a str>,
    {
        let buffer = strings::to_c(atomic_type.into());
        unsafe {
            try!(check(chfl_atom_set_type(self.as_mut_ptr(), buffer.as_ptr())));
        }
        return Ok(());
    }

    /// Try to get the full name of the atom from the atomic type. For example,
    /// the full name of "He" is "Helium", and so on. If the name can not be
    /// found, this function returns the empty string.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::Atom;
    /// let atom = Atom::new("Zn").unwrap();
    /// assert_eq!(atom.full_name(), Ok(String::from("Zinc")));
    /// ```
    pub fn full_name(&self) -> Result<String> {
        let get_full_name = |ptr, len| unsafe { chfl_atom_full_name(self.as_ptr(), ptr, len) };
        let name = try!(strings::call_autogrow_buffer(10, get_full_name));
        return Ok(strings::from_c(name.as_ptr()));
    }

    /// Try to get the Van der Waals radius of the atom from the atomic type.
    /// If the radius can not be found, returns -1.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::Atom;
    /// let atom = Atom::new("He").unwrap();
    /// assert_eq!(atom.vdw_radius(), Ok(1.4));
    /// ```
    pub fn vdw_radius(&self) -> Result<f64> {
        let mut radius: f64 = 0.0;
        unsafe {
            try!(check(chfl_atom_vdw_radius(self.as_ptr(), &mut radius)));
        }
        return Ok(radius);
    }

    /// Try to get the covalent radius of the atom from the atomic type. If the
    /// radius can not be found, returns -1.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::Atom;
    /// let atom = Atom::new("He").unwrap();
    /// assert_eq!(atom.covalent_radius(), Ok(0.32));
    /// ```
    pub fn covalent_radius(&self) -> Result<f64> {
        let mut radius: f64 = 0.0;
        unsafe {
            try!(check(chfl_atom_covalent_radius(self.as_ptr(), &mut radius)));
        }
        return Ok(radius);
    }

    /// Try to get the atomic number of the atom from the atomic type. If the
    /// number can not be found, returns -1.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::Atom;
    /// let atom = Atom::new("He").unwrap();
    /// assert_eq!(atom.atomic_number(), Ok(2));
    /// ```
    pub fn atomic_number(&self) -> Result<u64> {
        let mut number = 0;
        unsafe {
            try!(check(chfl_atom_atomic_number(self.as_ptr(), &mut number)));
        }
        return Ok(number);
    }

    /// Add a new `property` with the given `name` to this atom.
    ///
    /// If a property with the same name already exists, this function override
    /// the existing property with the new one.
    ///
    /// # Examples
    /// ```
    /// # use chemfiles::{Atom, Property};
    /// let mut atom = Atom::new("He").unwrap();
    /// atom.set("a bool value", Property::Bool(true));
    ///
    /// assert_eq!(atom.get("a bool value").unwrap(), Some(Property::Bool(true)));
    /// ```
    #[allow(needless_pass_by_value)]  // property
    pub fn set(&mut self, name: &str, property: Property) -> Result<()> {
        let buffer = strings::to_c(name);
        let property = try!(property.as_raw());
        unsafe {
            try!(check(
                chfl_atom_set_property(self.as_mut_ptr(), buffer.as_ptr(), property.as_ptr())
            ));
        }
        return Ok(());
    }


    /// Get a property with the given `name` in this atom, if it exist.
    ///
    /// # Examples
    /// ```
    /// # use chemfiles::{Atom, Property};
    /// let mut atom = Atom::new("He").unwrap();
    /// atom.set("foo", Property::Double(22.2));
    ///
    /// assert_eq!(atom.get("foo").unwrap(), Some(Property::Double(22.2)));
    /// assert_eq!(atom.get("Bar").unwrap(), None);
    /// ```
    pub fn get(&mut self, name: &str) -> Result<Option<Property>> {
        let buffer = strings::to_c(name);
        unsafe {
            let handle = chfl_atom_get_property(self.as_ptr(), buffer.as_ptr());
            if handle.is_null() {
                Ok(None)
            } else {
                let raw = try!(RawProperty::from_ptr(handle));
                let property = try!(Property::from_raw(raw));
                Ok(Some(property))
            }
        }
    }
}

impl Drop for Atom {
    fn drop(&mut self) {
        unsafe {
            let status = chfl_atom_free(self.as_mut_ptr());
            debug_assert_eq!(status, chfl_status::CHFL_SUCCESS);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn clone() {
        let mut atom = Atom::new("He").unwrap();
        assert_eq!(atom.name().unwrap(), "He");

        let copy = atom.clone();
        assert_eq!(copy.name().unwrap(), "He");

        atom.set_name("Na").unwrap();
        assert_eq!(atom.name().unwrap(), "Na");
        assert_eq!(copy.name().unwrap(), "He");
    }

    #[test]
    fn mass() {
        let mut atom = Atom::new("He").unwrap();
        assert_ulps_eq!(atom.mass().unwrap(), 4.002602);

        assert!(atom.set_mass(15.0).is_ok());
        assert_eq!(atom.mass(), Ok(15.0));
    }

    #[test]
    fn charge() {
        let mut atom = Atom::new("He").unwrap();
        assert_eq!(atom.charge(), Ok(0.0));

        assert!(atom.set_charge(-1.5).is_ok());
        assert_eq!(atom.charge(), Ok(-1.5));
    }

    #[test]
    fn name() {
        let mut atom = Atom::new("He").unwrap();
        assert_eq!(atom.name(), Ok(String::from("He")));

        assert!(atom.set_name("Zn-12").is_ok());
        assert_eq!(atom.name(), Ok(String::from("Zn-12")));
    }

    #[test]
    fn atomic_type() {
        let mut atom = Atom::new("He").unwrap();
        assert_eq!(atom.atomic_type(), Ok(String::from("He")));
        assert_eq!(atom.full_name(), Ok(String::from("Helium")));

        assert!(atom.set_atomic_type("Zn").is_ok());
        assert_eq!(atom.atomic_type(), Ok(String::from("Zn")));
        assert_eq!(atom.full_name(), Ok(String::from("Zinc")));
    }

    #[test]
    fn radii() {
        let atom = Atom::new("He").unwrap();
        assert_ulps_eq!(atom.vdw_radius().unwrap(), 1.4);
        assert_ulps_eq!(atom.covalent_radius().unwrap(), 0.32);
    }

    #[test]
    fn atomic_number() {
        let atom = Atom::new("He").unwrap();
        assert_eq!(atom.atomic_number(), Ok(2));
    }

    #[test]
    fn property() {
        let mut atom = Atom::new("F").unwrap();
        assert_eq!(atom.set("foo", Property::Double(-22.0)), Ok(()));
        assert_eq!(atom.get("foo"), Ok(Some(Property::Double(-22.0))));
    }
}

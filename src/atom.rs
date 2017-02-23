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
use strings;
use Result;

/// An Atom is a particle in the current Frame. It can be used to store and
/// retrieve informations about a particle, such as mass, name, atomic number,
/// *etc.*
pub struct Atom {
    handle: *const CHFL_ATOM
}

impl Clone for Atom {
    fn clone(&self) -> Atom {
        unsafe {
            let new_handle = chfl_atom_copy(self.as_ptr());
            Atom::from_ptr(new_handle)
                 .expect("Out of memory when copying an Atom")
        }
    }
}

impl Atom {
    /// Create an `Atom` from a C pointer.
    ///
    /// This function is unsafe because no validity check is made on the pointer,
    /// except for it being non-null.
    #[inline]
    #[doc(hidden)]
    pub unsafe fn from_ptr(ptr: *const CHFL_ATOM) -> Result<Atom> {
        if ptr.is_null() {
            Err(Error::null_ptr())
        } else {
            Ok(Atom{handle: ptr})
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
        self.handle as *mut CHFL_ATOM
    }

    /// Create a new `Atom` from a `name`.
    pub fn new<'a, S>(name: S) -> Result<Atom> where S: Into<&'a str>{
        let buffer = strings::to_c(name.into());
        unsafe {
            let handle = chfl_atom(buffer.as_ptr());
            Atom::from_ptr(handle)
        }
    }

    /// Get the `Atom` mass, in atomic mass units
    pub fn mass(&self) -> Result<f64> {
        let mut mass = 0.0;
        unsafe {
            try!(check(chfl_atom_mass(self.as_ptr(), &mut mass)));
        }
        return Ok(mass);
    }

    /// Set the `Atom` mass, in atomic mass units
    pub fn set_mass(&mut self, mass: f64) -> Result<()> {
        unsafe {
            try!(check(chfl_atom_set_mass(self.as_mut_ptr(), mass)));
        }
        return Ok(());
    }

    /// Get the `Atom` charge, in number of the electron charge *e*
    pub fn charge(&self) -> Result<f64> {
        let mut charge = 0.0;
        unsafe {
            try!(check(chfl_atom_charge(self.as_ptr(), &mut charge)));
        }
        return Ok(charge);
    }

    /// Set the `Atom` charge, in number of the electron charge *e*
    pub fn set_charge(&mut self, charge: f64) -> Result<()> {
        unsafe {
            try!(check(chfl_atom_set_charge(self.as_mut_ptr(), charge)));
        }
        return Ok(());
    }

    /// Get the `Atom` name
    pub fn name(&self) -> Result<String> {
        let name = try!(strings::call_autogrow_buffer(10, |ptr, len| unsafe {
            chfl_atom_name(self.as_ptr(), ptr, len)
        }));
        return Ok(strings::from_c(name.as_ptr()));
    }

    /// Set the `Atom` type
    pub fn set_atom_type<'a, S>(&mut self, name: S) -> Result<()> where S: Into<&'a str>{
        let buffer = strings::to_c(name.into());
        unsafe {
            try!(check(chfl_atom_set_type(self.as_mut_ptr(), buffer.as_ptr())));
        }
        return Ok(());
    }

    /// Get the `Atom` type
    pub fn atom_type(&self) -> Result<String> {
        let buffer = try!(strings::call_autogrow_buffer(10, |ptr, len| unsafe {
            chfl_atom_type(self.as_ptr(), ptr, len)
        }));
        return Ok(strings::from_c(buffer.as_ptr()));
    }

    /// Set the `Atom` name
    pub fn set_name<'a, S>(&mut self, name: S) -> Result<()> where S: Into<&'a str>{
        let buffer = strings::to_c(name.into());
        unsafe {
            try!(check(chfl_atom_set_name(self.as_mut_ptr(), buffer.as_ptr())));
        }
        return Ok(());
    }

    /// Try to get the full name of the `Atom`. The full name of "He" is
    /// "Helium", and so on. If the name can not be found, returns the empty
    /// string.
    pub fn full_name(&mut self) -> Result<String> {
        let name = try!(strings::call_autogrow_buffer(10, |ptr, len| unsafe {
            chfl_atom_full_name(self.as_ptr(), ptr, len)
        }));
        return Ok(strings::from_c(name.as_ptr()));
    }

    /// Try to get the Van der Waals radius of the `Atom`. If the radius can not
    /// be found, returns -1.
    pub fn vdw_radius(&self) -> Result<f64> {
        let mut radius: f64 = 0.0;
        unsafe {
            try!(check(chfl_atom_vdw_radius(self.as_ptr(), &mut radius)));
        }
        return Ok(radius);
    }

    /// Try to get the covalent radius of the `Atom`. If the radius can not be
    /// found, returns -1.
    pub fn covalent_radius(&self) -> Result<f64> {
        let mut radius: f64 = 0.0;
        unsafe {
            try!(check(chfl_atom_covalent_radius(self.as_ptr(), &mut radius)));
        }
        return Ok(radius);
    }

    /// Try to get the atomic number of the `Atom`. If the number can not be
    /// found, returns -1.
    pub fn atomic_number(&self) -> Result<i64> {
        let mut number = 0;
        unsafe {
            try!(check(chfl_atom_atomic_number(self.as_ptr(), &mut number)));
        }
        return Ok(number);
    }
}

impl Drop for Atom {
    fn drop(&mut self) {
        unsafe {
            let status = chfl_atom_free(self.as_mut_ptr());
            debug_assert!(status == chfl_status::CHFL_SUCCESS);
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
        assert_approx_eq!(atom.mass().unwrap(), 4.002602, 1e-6);

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
    fn atom_type() {
        let mut atom = Atom::new("He").unwrap();
        assert_eq!(atom.atom_type(), Ok(String::from("He")));
        assert_eq!(atom.full_name(), Ok(String::from("Helium")));

        assert!(atom.set_atom_type("Zn").is_ok());
        assert_eq!(atom.atom_type(), Ok(String::from("Zn")));
        assert_eq!(atom.full_name(), Ok(String::from("Zinc")));
    }

    #[test]
    fn radii() {
        let atom = Atom::new("He").unwrap();
        assert_approx_eq!(atom.vdw_radius().unwrap(), 1.4, 1e-2);
        assert_approx_eq!(atom.covalent_radius().unwrap(), 0.32, 1e-3);
    }

    #[test]
    fn atomic_number() {
        let atom = Atom::new("He").unwrap();
        assert_eq!(atom.atomic_number(), Ok(2));
    }
}

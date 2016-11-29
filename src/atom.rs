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
use string;
use Result;

/// An Atom is a particle in the current Frame. It can be used to store and
/// retrieve informations about a particle, such as mass, name, atomic number,
/// *etc.*
pub struct Atom {
    handle: *const CHFL_ATOM
}

impl Atom {
    /// Create a new `Atom` from a `name`.
    pub fn new<'a, S>(name: S) -> Result<Atom> where S: Into<&'a str>{
        let handle: *const CHFL_ATOM;
        let buffer = string::to_c(name.into());
        unsafe {
            handle = chfl_atom(buffer.as_ptr());
        }

        if handle.is_null() {
            Err(Error::null_ptr())
        } else {
            Ok(Atom{handle: handle})
        }
    }

    /// Get the `Atom` mass, in atomic mass units
    pub fn mass(&self) -> Result<f64> {
        let mut mass = 0.0;
        unsafe {
            try!(check(chfl_atom_mass(self.handle, &mut mass)));
        }
        return Ok(mass);
    }

    /// Set the `Atom` mass, in atomic mass units
    pub fn set_mass(&mut self, mass: f64) -> Result<()> {
        unsafe {
            try!(check(chfl_atom_set_mass(self.handle as *mut CHFL_ATOM, mass)));
        }
        return Ok(());
    }

    /// Get the `Atom` charge, in number of the electron charge *e*
    pub fn charge(&self) -> Result<f64> {
        let mut charge = 0.0;
        unsafe {
            try!(check(chfl_atom_charge(self.handle, &mut charge)));
        }
        return Ok(charge);
    }

    /// Set the `Atom` charge, in number of the electron charge *e*
    pub fn set_charge(&mut self, charge: f64) -> Result<()> {
        unsafe {
            try!(check(chfl_atom_set_charge(self.handle as *mut CHFL_ATOM, charge)));
        }
        return Ok(());
    }

    /// Get the `Atom` name
    pub fn name(&self) -> Result<String> {
        let mut buffer = vec![0; 10];
        unsafe {
            try!(check(chfl_atom_name(self.handle, &mut buffer[0], buffer.len() as u64)));
        }
        return Ok(string::from_c(&buffer[0]));
    }

    /// Set the `Atom` name
    pub fn set_name<'a, S>(&mut self, name: S) -> Result<()> where S: Into<&'a str>{
        let buffer = string::to_c(name.into());
        unsafe {
            try!(check(chfl_atom_set_name(self.handle as *mut CHFL_ATOM, buffer.as_ptr())));
        }
        return Ok(());
    }

    /// Try to get the full name of the `Atom`. The full name of "He" is
    /// "Helium", and so on. If the name can not be found, returns the empty
    /// string.
    pub fn full_name(&mut self) -> Result<String> {
        let mut buffer = vec![0; 10];
        unsafe {
            try!(check(chfl_atom_full_name(self.handle, &mut buffer[0], buffer.len() as u64)));
        }
        return Ok(string::from_c(&buffer[0]));
    }

    /// Try to get the Van der Waals radius of the `Atom`. If the radius can not
    /// be found, returns -1.
    pub fn vdw_radius(&self) -> Result<f64> {
        let mut radius: f64 = 0.0;
        unsafe {
            try!(check(chfl_atom_vdw_radius(self.handle, &mut radius)));
        }
        return Ok(radius);
    }

    /// Try to get the covalent radius of the `Atom`. If the radius can not be
    /// found, returns -1.
    pub fn covalent_radius(&self) -> Result<f64> {
        let mut radius: f64 = 0.0;
        unsafe {
            try!(check(chfl_atom_covalent_radius(self.handle, &mut radius)));
        }
        return Ok(radius);
    }

    /// Try to get the atomic number of the `Atom`. If the number can not be
    /// found, returns -1.
    pub fn atomic_number(&self) -> Result<i64> {
        let mut number = 0;
        unsafe {
            try!(check(chfl_atom_atomic_number(self.handle, &mut number)));
        }
        return Ok(number);
    }

    /// Create an `Atom` from a C pointer. This function is unsafe because no
    /// validity check is made on the pointer.
    pub unsafe fn from_ptr(ptr: *const CHFL_ATOM) -> Atom {
        Atom{handle: ptr}
    }

    /// Get the underlying C pointer. This function is unsafe because no
    /// lifetime guarantee is made on the pointer.
    pub unsafe fn as_ptr(&self) -> *const CHFL_ATOM {
        self.handle
    }
}

impl Drop for Atom {
    fn drop(&mut self) {
        unsafe {
            check(
                chfl_atom_free(self.handle as *mut CHFL_ATOM)
            ).ok().expect("Error while freeing memory!");
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn mass() {
        let mut at = Atom::new("He").unwrap();
        assert_approx_eq!(at.mass().unwrap(), 4.002602, 1e-6);

        assert!(at.set_mass(15.0).is_ok());
        assert_eq!(at.mass(), Ok(15.0));
    }

    #[test]
    fn charge() {
        let mut at = Atom::new("He").unwrap();
        assert_eq!(at.charge(), Ok(0.0));

        assert!(at.set_charge(-1.5).is_ok());
        assert_eq!(at.charge(), Ok(-1.5));
    }

    #[test]
    fn name() {
        let mut at = Atom::new("He").unwrap();
        assert_eq!(at.name(), Ok(String::from("He")));
        assert_eq!(at.full_name(), Ok(String::from("Helium")));

        assert!(at.set_name("Zn").is_ok());
        assert_eq!(at.name(), Ok(String::from("Zn")));
        // assert_eq!(at.full_name(), Ok(String::from("Zinc")));
    }

    #[test]
    fn radii() {
        let at = Atom::new("He").unwrap();
        assert_approx_eq!(at.vdw_radius().unwrap(), 1.4, 1e-2);
        assert_approx_eq!(at.covalent_radius().unwrap(), 0.32, 1e-3);
    }

    #[test]
    fn atomic_number() {
        let at = Atom::new("He").unwrap();
        assert_eq!(at.atomic_number(), Ok(2));
    }
}

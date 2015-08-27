/*
 * Chemharp, an efficient IO library for chemistry file formats
 * Copyright (C) 2015 Guillaume Fraux
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/
*/

extern crate libc;

use std::ops::Drop;
use std::ffi::CString;

use ::ffi::*;
use ::errors::{check, Error};
use ::errors::from_c_str;

pub struct Atom {
    handle: *const CHRP_ATOM
}

impl Atom {
    pub fn new<'a, S>(name: S) -> Result<Atom, Error> where S: Into<&'a str>{
        let mut handle : *const CHRP_ATOM;
        let buffer = CString::new(name.into()).ok().expect("Got invalid C string from Rust!");
        unsafe {
            handle = chrp_atom(buffer.as_ptr());
        }
        if handle.is_null() {
            return Err(Error::ChemharpCppError{message: Error::last_error()})
        }
        Ok(Atom{handle: handle})
    }

    pub fn mass(&self) -> Result<f32, Error> {
        let mut mass: f32 = 0.0;
        unsafe {
            try!(check(chrp_atom_mass(self.handle, &mut mass)));
        }
        return Ok(mass);
    }

    pub fn set_mass(&mut self, mass: f32) -> Result<(), Error> {
        unsafe {
            try!(check(chrp_atom_set_mass(self.handle as *mut CHRP_ATOM, mass)));
        }
        return Ok(());
    }

    pub fn charge(&self) -> Result<f32, Error> {
        let mut charge: f32 = 0.0;
        unsafe {
            try!(check(chrp_atom_charge(self.handle, &mut charge)));
        }
        return Ok(charge);
    }

    pub fn set_charge(&mut self, charge: f32) -> Result<(), Error> {
        unsafe {
            try!(check(chrp_atom_set_charge(self.handle as *mut CHRP_ATOM, charge)));
        }
        return Ok(());
    }

    pub fn name(&self) -> Result<String, Error> {
        let mut buffer = vec![0; 10];
        unsafe {
            try!(check(chrp_atom_name(self.handle, &mut buffer[0], buffer.len() as u64)));
        }
        return Ok(from_c_str(&buffer[0]));
    }

    pub fn set_name<'a, S>(&mut self, name: S) -> Result<(), Error> where S: Into<&'a str>{
        let buffer = CString::new(name.into()).ok().expect("Got invalid C string from Rust!");
        unsafe {
            try!(check(chrp_atom_set_name(self.handle as *mut CHRP_ATOM, buffer.as_ptr())));
        }
        return Ok(());
    }

    pub fn full_name(&mut self) -> Result<String, Error> {
        let mut buffer = vec![0; 10];
        unsafe {
            try!(check(chrp_atom_full_name(self.handle, &mut buffer[0], buffer.len() as u64)));
        }
        return Ok(from_c_str(&buffer[0]));
    }

    pub fn vdw_radius(&self) -> Result<f64, Error> {
        let mut radius: f64 = 0.0;
        unsafe {
            try!(check(chrp_atom_vdw_radius(self.handle, &mut radius)));
        }
        return Ok(radius);
    }

    pub fn covalent_radius(&self) -> Result<f64, Error> {
        let mut radius: f64 = 0.0;
        unsafe {
            try!(check(chrp_atom_covalent_radius(self.handle, &mut radius)));
        }
        return Ok(radius);
    }

    pub fn atomic_number(&self) -> Result<i32, Error> {
        let mut number: i32 = 0;
        unsafe {
            try!(check(chrp_atom_atomic_number(self.handle, &mut number)));
        }
        return Ok(number);
    }

    /// Create an `Atom` from a C pointer. This function is unsafe because no
    /// validity check is made on the pointer.
    pub unsafe fn from_ptr(ptr: *const CHRP_ATOM) -> Atom {
        Atom{handle: ptr}
    }

    /// Get the underlying C pointer. This function is unsafe because no
    /// lifetime guarantee is made on the pointer.
    pub unsafe fn as_ptr(&self) -> *const CHRP_ATOM {
        self.handle
    }
}

impl Drop for Atom {
    fn drop(&mut self) {
        unsafe {
            check(
                chrp_atom_free(self.handle as *mut CHRP_ATOM)
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

        assert!(at.set_mass(15.0f32).is_ok());
        assert_eq!(at.mass(), Ok(15.0));
    }

    #[test]
    fn charge() {
        let mut at = Atom::new("He").unwrap();
        assert_eq!(at.charge(), Ok(0.0));

        assert!(at.set_charge(-1.5f32).is_ok());
        assert_eq!(at.charge(), Ok(-1.5));
    }

    #[test]
    fn name() {
        let mut at = Atom::new("He").unwrap();
        assert_eq!(at.name(), Ok(String::from("He")));
        assert_eq!(at.full_name(), Ok(String::from("Helium")));

        assert!(at.set_name("Zn").is_ok());
        assert_eq!(at.name(), Ok(String::from("Zn")));
        assert_eq!(at.full_name(), Ok(String::from("Zinc")));
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

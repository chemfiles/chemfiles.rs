// Chemfiles, a modern library for chemistry file reading and writing
// Copyright (C) 2015-2018 Guillaume Fraux -- BSD licensed
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut, Drop};
use std::ptr;

use chemfiles_sys::*;
use errors::{check_not_null, check_success};
use strings;

use property::{PropertiesIter, Property, RawProperty};

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

/// An analog to a reference to an atom (`&Atom`)
pub struct AtomRef<'a> {
    inner: Atom,
    marker: PhantomData<&'a Atom>,
}

impl<'a> Deref for AtomRef<'a> {
    type Target = Atom;
    fn deref(&self) -> &Atom {
        &self.inner
    }
}

/// An analog to a mutable reference to an atom (`&mut Atom`)
pub struct AtomMut<'a> {
    inner: Atom,
    marker: PhantomData<&'a mut Atom>,
}

impl<'a> Deref for AtomMut<'a> {
    type Target = Atom;
    fn deref(&self) -> &Atom {
        &self.inner
    }
}

impl<'a> DerefMut for AtomMut<'a> {
    fn deref_mut(&mut self) -> &mut Atom {
        &mut self.inner
    }
}

impl Clone for Atom {
    fn clone(&self) -> Atom {
        unsafe {
            let new_handle = chfl_atom_copy(self.as_ptr());
            Atom::from_ptr(new_handle)
        }
    }
}

impl Atom {
    /// Create an owned `Atom` from a C pointer.
    ///
    /// This function is unsafe because no validity check is made on the
    /// pointer.
    #[inline]
    pub(crate) unsafe fn from_ptr(ptr: *mut CHFL_ATOM) -> Atom {
        check_not_null(ptr);
        Atom { handle: ptr }
    }

    /// Create a borrowed `Atom` from a C pointer.
    ///
    /// This function is unsafe because no validity check is made on the
    /// pointer, and the caller is responsible for setting the right lifetime
    #[inline]
    pub(crate) unsafe fn ref_from_ptr<'a>(ptr: *const CHFL_ATOM) -> AtomRef<'a> {
        AtomRef {
            inner: Atom::from_ptr(ptr as *mut CHFL_ATOM),
            marker: PhantomData,
        }
    }

    /// Create a mutably borrowed `Atom` from a C pointer.
    ///
    /// This function is unsafe because no validity check is made on the
    /// pointer, and the caller is responsible for setting the right lifetime
    #[inline]
    pub(crate) unsafe fn ref_mut_from_ptr<'a>(ptr: *mut CHFL_ATOM) -> AtomMut<'a> {
        AtomMut {
            inner: Atom::from_ptr(ptr),
            marker: PhantomData,
        }
    }

    /// Get the underlying C pointer as a const pointer.
    #[inline]
    pub(crate) fn as_ptr(&self) -> *const CHFL_ATOM {
        self.handle
    }

    /// Get the underlying C pointer as a mutable pointer.
    #[inline]
    pub(crate) fn as_mut_ptr(&mut self) -> *mut CHFL_ATOM {
        self.handle
    }

    /// Create an atom with the given `name`, and set the atom type to `name`.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::Atom;
    /// let atom = Atom::new("He");
    /// assert_eq!(atom.name(), "He");
    /// ```
    pub fn new<'a>(name: impl Into<&'a str>) -> Atom {
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
    /// let atom = Atom::new("He");
    /// assert_eq!(atom.mass(), 4.002602);
    /// ```
    pub fn mass(&self) -> f64 {
        let mut mass = 0.0;
        unsafe {
            check_success(chfl_atom_mass(self.as_ptr(), &mut mass));
        }
        return mass;
    }

    /// Set the atom mass to `mass`, in atomic mass units.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::Atom;
    /// let mut atom = Atom::new("He");
    ///
    /// atom.set_mass(34.9);
    /// assert_eq!(atom.mass(), 34.9);
    /// ```
    pub fn set_mass(&mut self, mass: f64) {
        unsafe {
            check_success(chfl_atom_set_mass(self.as_mut_ptr(), mass));
        }
    }

    /// Get the atom charge, in number of the electron charge *e*.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::Atom;
    /// let atom = Atom::new("He");
    /// assert_eq!(atom.charge(), 0.0);
    /// ```
    pub fn charge(&self) -> f64 {
        let mut charge = 0.0;
        unsafe {
            check_success(chfl_atom_charge(self.as_ptr(), &mut charge));
        }
        return charge;
    }

    /// Set the atom charge to `charge`, in number of the electron charge *e*.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::Atom;
    /// let mut atom = Atom::new("He");
    ///
    /// atom.set_charge(-2.0);
    /// assert_eq!(atom.charge(), -2.0);
    /// ```
    pub fn set_charge(&mut self, charge: f64) {
        unsafe {
            check_success(chfl_atom_set_charge(self.as_mut_ptr(), charge));
        }
    }

    /// Get the atom name.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::Atom;
    /// let atom = Atom::new("He");
    /// assert_eq!(atom.name(), "He");
    /// ```
    pub fn name(&self) -> String {
        let get_name = |ptr, len| unsafe { chfl_atom_name(self.as_ptr(), ptr, len) };
        let name = strings::call_autogrow_buffer(10, get_name).expect("getting name failed");
        return strings::from_c(name.as_ptr());
    }

    /// Get the atom type.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::Atom;
    /// let atom = Atom::new("He");
    /// assert_eq!(atom.atomic_type(), "He");
    /// ```
    pub fn atomic_type(&self) -> String {
        let get_type = |ptr, len| unsafe { chfl_atom_type(self.as_ptr(), ptr, len) };
        let buffer = strings::call_autogrow_buffer(10, get_type).expect("getting type failed");
        return strings::from_c(buffer.as_ptr());
    }

    /// Set the atom name to `name`.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::Atom;
    /// let mut atom = Atom::new("He");
    ///
    /// atom.set_name("Zn3");
    /// assert_eq!(atom.name(), "Zn3");
    /// ```
    pub fn set_name<'a>(&mut self, name: impl Into<&'a str>) {
        let buffer = strings::to_c(name.into());
        unsafe {
            check_success(chfl_atom_set_name(self.as_mut_ptr(), buffer.as_ptr()));
        }
    }

    /// Set the atom type to `atomic_type`.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::Atom;
    /// let mut atom = Atom::new("He");
    ///
    /// atom.set_atomic_type("F");
    /// assert_eq!(atom.atomic_type(), "F");
    /// ```
    pub fn set_atomic_type<'a>(&mut self, atomic_type: impl Into<&'a str>) {
        let buffer = strings::to_c(atomic_type.into());
        unsafe {
            check_success(chfl_atom_set_type(self.as_mut_ptr(), buffer.as_ptr()));
        }
    }

    /// Try to get the full name of the atom from the atomic type. For example,
    /// the full name of "He" is "Helium", and so on. If the name can not be
    /// found, this function returns the empty string.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::Atom;
    /// let atom = Atom::new("Zn");
    /// assert_eq!(atom.full_name(), "Zinc");
    /// ```
    pub fn full_name(&self) -> String {
        let get_full_name = |ptr, len| unsafe { chfl_atom_full_name(self.as_ptr(), ptr, len) };
        let name = strings::call_autogrow_buffer(10, get_full_name).expect("getting full name failed");
        return strings::from_c(name.as_ptr());
    }

    /// Try to get the Van der Waals radius of the atom from the atomic type.
    /// If the radius can not be found, returns 0.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::Atom;
    /// assert_eq!(Atom::new("He").vdw_radius(), 1.4);
    /// assert_eq!(Atom::new("Xxx").vdw_radius(), 0.0);
    /// ```
    pub fn vdw_radius(&self) -> f64 {
        let mut radius: f64 = 0.0;
        unsafe {
            check_success(chfl_atom_vdw_radius(self.as_ptr(), &mut radius));
        }
        return radius;
    }

    /// Try to get the covalent radius of the atom from the atomic type. If the
    /// radius can not be found, returns 0.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::Atom;
    /// assert_eq!(Atom::new("He").covalent_radius(), 0.32);
    /// assert_eq!(Atom::new("Xxx").covalent_radius(), 0.0);
    /// ```
    pub fn covalent_radius(&self) -> f64 {
        let mut radius: f64 = 0.0;
        unsafe {
            check_success(chfl_atom_covalent_radius(self.as_ptr(), &mut radius));
        }
        return radius;
    }

    /// Try to get the atomic number of the atom from the atomic type. If the
    /// number can not be found, returns 0.
    ///
    /// # Example
    /// ```
    /// # use chemfiles::Atom;
    /// assert_eq!(Atom::new("He").atomic_number(), 2);
    /// assert_eq!(Atom::new("Xxx").atomic_number(), 0);
    /// ```
    pub fn atomic_number(&self) -> u64 {
        let mut number = 0;
        unsafe {
            check_success(chfl_atom_atomic_number(self.as_ptr(), &mut number));
        }
        return number;
    }

    /// Add a new `property` with the given `name` to this atom.
    ///
    /// If a property with the same name already exists, this function override
    /// the existing property with the new one.
    ///
    /// # Examples
    /// ```
    /// # use chemfiles::{Atom, Property};
    /// let mut atom = Atom::new("He");
    /// atom.set("a bool", true);
    /// atom.set("a string", "test");
    ///
    /// assert_eq!(atom.get("a bool"), Some(Property::Bool(true)));
    /// assert_eq!(atom.get("a string"), Some(Property::String("test".into())));
    /// ```
    pub fn set(&mut self, name: &str, property: impl Into<Property>) {
        let buffer = strings::to_c(name);
        let property = property.into().as_raw();
        unsafe {
            check_success(chfl_atom_set_property(
                self.as_mut_ptr(),
                buffer.as_ptr(),
                property.as_ptr(),
            ));
        }
    }

    /// Get a property with the given `name` in this atom, if it exist.
    ///
    /// # Examples
    /// ```
    /// # use chemfiles::{Atom, Property};
    /// let mut atom = Atom::new("He");
    /// atom.set("foo", Property::Double(22.2));
    ///
    /// assert_eq!(atom.get("foo"), Some(Property::Double(22.2)));
    /// assert_eq!(atom.get("Bar"), None);
    /// ```
    pub fn get(&self, name: &str) -> Option<Property> {
        let buffer = strings::to_c(name);
        unsafe {
            let handle = chfl_atom_get_property(self.as_ptr(), buffer.as_ptr());
            if handle.is_null() {
                None
            } else {
                let raw = RawProperty::from_ptr(handle);
                let property = Property::from_raw(raw);
                Some(property)
            }
        }
    }

    /// Get an iterator over all (name, property) pairs for this atom
    ///
    /// # Examples
    /// ```
    /// # use chemfiles::{Atom, Property};
    /// let mut atom = Atom::new("He");
    /// atom.set("foo", Property::Double(22.2));
    /// atom.set("bar", Property::Bool(false));
    ///
    /// for (name, property) in atom.properties() {
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
            check_success(chfl_atom_properties_count(self.as_ptr(), &mut count));
        }

        #[allow(clippy::cast_possible_truncation)]
        let size = count as usize;
        let mut c_names = vec![ptr::null_mut(); size];
        unsafe {
            check_success(chfl_atom_list_properties(self.as_ptr(), c_names.as_mut_ptr(), count));
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

impl Drop for Atom {
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
        let mut atom = Atom::new("He");
        assert_eq!(atom.name(), "He");

        let copy = atom.clone();
        assert_eq!(copy.name(), "He");

        atom.set_name("Na");
        assert_eq!(atom.name(), "Na");
        assert_eq!(copy.name(), "He");
    }

    #[test]
    fn mass() {
        let mut atom = Atom::new("He");
        assert_ulps_eq!(atom.mass(), 4.002602);
        atom.set_mass(15.0);
        assert_eq!(atom.mass(), 15.0);
    }

    #[test]
    fn charge() {
        let mut atom = Atom::new("He");
        assert_eq!(atom.charge(), 0.0);
        atom.set_charge(-1.5);
        assert_eq!(atom.charge(), -1.5);
    }

    #[test]
    fn name() {
        let mut atom = Atom::new("He");
        assert_eq!(atom.name(), "He");
        atom.set_name("Zn-12");
        assert_eq!(atom.name(), "Zn-12");
    }

    #[test]
    fn atomic_type() {
        let mut atom = Atom::new("He");
        assert_eq!(atom.atomic_type(), "He");
        atom.set_atomic_type("Zn");
        assert_eq!(atom.atomic_type(), "Zn");
    }

    #[test]
    fn full_name() {
        let mut atom = Atom::new("He");
        assert_eq!(atom.full_name(), "Helium");

        atom.set_atomic_type("Zn");
        assert_eq!(atom.full_name(), "Zinc");

        let atom = Atom::new("Unknown");
        assert_eq!(atom.full_name(), "");
    }

    #[test]
    fn radii() {
        let atom = Atom::new("He");
        assert_ulps_eq!(atom.vdw_radius(), 1.4);
        assert_ulps_eq!(atom.covalent_radius(), 0.32);

        let atom = Atom::new("Unknown");
        assert_eq!(atom.vdw_radius(), 0.0);
        assert_eq!(atom.covalent_radius(), 0.0);
    }

    #[test]
    fn atomic_number() {
        let atom = Atom::new("He");
        assert_eq!(atom.atomic_number(), 2);

        let atom = Atom::new("Unknown");
        assert_eq!(atom.atomic_number(), 0);
    }

    #[test]
    fn property() {
        let mut atom = Atom::new("F");

        atom.set("foo", -22.0);
        assert_eq!(atom.get("foo"), Some(Property::Double(-22.0)));
        assert_eq!(atom.get("bar"), None);

        atom.set("bar", Property::String("here".into()));
        for (name, property) in atom.properties() {
            if name == "foo" {
                assert_eq!(property, Property::Double(-22.0));
            } else if name == "bar" {
                assert_eq!(property, Property::String("here".into()));
            }
        }
    }
}

// Chemfiles, a modern library for chemistry file reading and writing
// Copyright (C) 2015-2018 Guillaume Fraux -- BSD licensed
use chemfiles_sys as ffi;

use crate::errors::{check, check_not_null, check_success, Error};
use crate::strings;

/// A thin wrapper around `ffi::CHFL_PROPERTY`
#[derive(Debug)]
pub(crate) struct RawProperty {
    handle: *mut ffi::CHFL_PROPERTY,
}

impl RawProperty {
    /// Create a `RawProperty` from a C pointer.
    ///
    /// This function is unsafe because no validity check is made on the pointer.
    pub unsafe fn from_ptr(ptr: *mut ffi::CHFL_PROPERTY) -> RawProperty {
        check_not_null(ptr);
        RawProperty { handle: ptr }
    }

    /// Get the underlying C pointer as a const pointer.
    pub fn as_ptr(&self) -> *const ffi::CHFL_PROPERTY {
        self.handle
    }

    fn double(value: f64) -> RawProperty {
        unsafe {
            let handle = ffi::chfl_property_double(value);
            RawProperty::from_ptr(handle)
        }
    }

    fn bool(value: bool) -> RawProperty {
        unsafe {
            let handle = ffi::chfl_property_bool(u8::from(value));
            RawProperty::from_ptr(handle)
        }
    }

    fn vector3d(value: [f64; 3]) -> RawProperty {
        unsafe {
            let handle = ffi::chfl_property_vector3d(value.as_ptr());
            RawProperty::from_ptr(handle)
        }
    }

    fn string(value: &str) -> RawProperty {
        let buffer = strings::to_c(value);
        unsafe {
            let handle = ffi::chfl_property_string(buffer.as_ptr());
            RawProperty::from_ptr(handle)
        }
    }

    fn get_kind(&self) -> ffi::chfl_property_kind {
        let mut kind = ffi::chfl_property_kind::CHFL_PROPERTY_BOOL;
        unsafe {
            check_success(ffi::chfl_property_get_kind(self.as_ptr(), &mut kind));
        }
        return kind;
    }

    fn get_bool(&self) -> Result<bool, Error> {
        let mut value = 0;
        unsafe {
            check(ffi::chfl_property_get_bool(self.as_ptr(), &mut value))?;
        }
        return Ok(value != 0);
    }

    fn get_double(&self) -> Result<f64, Error> {
        let mut value = 0.0;
        unsafe {
            check(ffi::chfl_property_get_double(self.as_ptr(), &mut value))?;
        }
        return Ok(value);
    }

    fn get_string(&self) -> Result<String, Error> {
        let get_string = |ptr, len| unsafe { ffi::chfl_property_get_string(self.as_ptr(), ptr, len) };
        let value = strings::call_autogrow_buffer(64, get_string)?;
        return Ok(strings::from_c(value.as_ptr()));
    }

    fn get_vector3d(&self) -> Result<[f64; 3], Error> {
        let mut value = [0.0; 3];
        unsafe {
            check(ffi::chfl_property_get_vector3d(self.as_ptr(), value.as_mut_ptr()))?;
        }
        return Ok(value);
    }
}

impl Drop for RawProperty {
    fn drop(&mut self) {
        unsafe {
            let _ = ffi::chfl_free(self.as_ptr().cast());
        }
    }
}

/// A `Property` is a piece of data that can be associated with an `Atom` or a
/// `Frame`.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Property {
    /// Boolean property
    Bool(bool),
    /// Floating point property
    Double(f64),
    /// String property
    String(String),
    /// 3-dimensional vector property
    Vector3D([f64; 3]),
}

impl From<bool> for Property {
    fn from(value: bool) -> Self {
        Property::Bool(value)
    }
}

impl From<f64> for Property {
    fn from(value: f64) -> Self {
        Property::Double(value)
    }
}

impl From<String> for Property {
    fn from(value: String) -> Self {
        Property::String(value)
    }
}

impl<'a> From<&'a str> for Property {
    fn from(value: &'a str) -> Self {
        Property::String(value.into())
    }
}

impl From<[f64; 3]> for Property {
    fn from(value: [f64; 3]) -> Self {
        Property::Vector3D(value)
    }
}

impl Property {
    pub(crate) fn as_raw(&self) -> RawProperty {
        match *self {
            Property::Bool(value) => RawProperty::bool(value),
            Property::Double(value) => RawProperty::double(value),
            Property::String(ref value) => RawProperty::string(value),
            Property::Vector3D(value) => RawProperty::vector3d(value),
        }
    }

    #[allow(clippy::needless_pass_by_value)] // raw property
    pub(crate) fn from_raw(raw: RawProperty) -> Property {
        match raw.get_kind() {
            ffi::chfl_property_kind::CHFL_PROPERTY_BOOL => Self::Bool(raw.get_bool().expect("should be a bool")),
            ffi::chfl_property_kind::CHFL_PROPERTY_DOUBLE => {
                Self::Double(raw.get_double().expect("should be a double"))
            }
            ffi::chfl_property_kind::CHFL_PROPERTY_STRING => {
                Self::String(raw.get_string().expect("should be a string"))
            }
            ffi::chfl_property_kind::CHFL_PROPERTY_VECTOR3D => {
                Property::Vector3D(raw.get_vector3d().expect("should be a vector3d"))
            }
        }
    }
}

/// An iterator over the properties in an atom/frame/residue
pub struct PropertiesIter<'a> {
    pub(crate) names: std::vec::IntoIter<String>,
    pub(crate) getter: Box<dyn Fn(&str) -> Property + 'a>,
}

impl<'a> Iterator for PropertiesIter<'a> {
    type Item = (String, Property);
    fn next(&mut self) -> Option<Self::Item> {
        self.names.next().map(|name| {
            let property = (self.getter)(&name);
            (name, property)
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.names.size_hint()
    }

    fn count(self) -> usize {
        self.names.count()
    }
}

#[cfg(test)]
mod tests {
    mod raw {
        use super::super::*;

        #[test]
        fn bool() {
            let property = RawProperty::bool(false);
            assert_eq!(property.get_kind(), ffi::chfl_property_kind::CHFL_PROPERTY_BOOL);
            assert_eq!(property.get_bool(), Ok(false));
        }

        #[test]
        fn double() {
            let property = RawProperty::double(45.0);
            assert_eq!(property.get_kind(), ffi::chfl_property_kind::CHFL_PROPERTY_DOUBLE);
            assert_eq!(property.get_double(), Ok(45.0));
        }

        #[test]
        fn string() {
            let property = RawProperty::string("test");
            assert_eq!(property.get_kind(), ffi::chfl_property_kind::CHFL_PROPERTY_STRING);
            assert_eq!(property.get_string(), Ok("test".into()));
        }

        #[test]
        fn vector3d() {
            let property = RawProperty::vector3d([1.2, 3.4, 5.6]);
            assert_eq!(property.get_kind(), ffi::chfl_property_kind::CHFL_PROPERTY_VECTOR3D);
            assert_eq!(property.get_vector3d(), Ok([1.2, 3.4, 5.6]));
        }
    }

    mod rust {
        use super::super::*;

        #[test]
        fn bool() {
            let property = Property::Bool(false);

            let raw = property.as_raw();
            assert_eq!(raw.get_kind(), ffi::chfl_property_kind::CHFL_PROPERTY_BOOL);
            assert_eq!(raw.get_bool(), Ok(false));

            assert_eq!(Property::from_raw(raw), property);
        }

        #[test]
        fn double() {
            let property = Property::Double(45.0);

            let raw = property.as_raw();
            assert_eq!(raw.get_kind(), ffi::chfl_property_kind::CHFL_PROPERTY_DOUBLE);
            assert_eq!(raw.get_double(), Ok(45.0));

            assert_eq!(Property::from_raw(raw), property);
        }

        #[test]
        fn string() {
            let property = Property::String("test".into());

            let raw = property.as_raw();
            assert_eq!(raw.get_kind(), ffi::chfl_property_kind::CHFL_PROPERTY_STRING);
            assert_eq!(raw.get_string(), Ok("test".into()));

            assert_eq!(Property::from_raw(raw), property);

            let property = Property::String("long string ".repeat(128));
            let raw = property.as_raw();
            assert_eq!(raw.get_string(), Ok("long string ".repeat(128)));
        }

        #[test]
        fn vector3d() {
            let property = Property::Vector3D([1.2, 3.4, 5.6]);

            let raw = property.as_raw();
            assert_eq!(raw.get_kind(), ffi::chfl_property_kind::CHFL_PROPERTY_VECTOR3D);
            assert_eq!(raw.get_vector3d(), Ok([1.2, 3.4, 5.6]));

            assert_eq!(Property::from_raw(raw), property);
        }
    }
}

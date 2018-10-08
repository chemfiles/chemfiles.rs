// Chemfiles, a modern library for chemistry file reading and writing
// Copyright (C) 2015-2018 Guillaume Fraux -- BSD licensed
use std::ops::Drop;

use chemfiles_sys::*;
use errors::{check, Error};
use strings;
use Result;

/// A thin wrapper around `CHFL_PROPERTY`
pub(crate) struct RawProperty {
    handle: *mut CHFL_PROPERTY,
}

impl RawProperty {
    /// Create a `RawProperty` from a C pointer.
    ///
    /// This function is unsafe because no validity check is made on the pointer,
    /// except for it being non-null.
    pub unsafe fn from_ptr(ptr: *mut CHFL_PROPERTY) -> Result<RawProperty> {
        if ptr.is_null() {
            Err(Error::null_ptr())
        } else {
            Ok(RawProperty { handle: ptr })
        }
    }

    /// Get the underlying C pointer as a const pointer.
    pub fn as_ptr(&self) -> *const CHFL_PROPERTY {
        self.handle
    }

    /// Get the underlying C pointer as a mutable pointer.
    pub fn as_mut_ptr(&mut self) -> *mut CHFL_PROPERTY {
        self.handle
    }

    fn double(value: f64) -> Result<RawProperty> {
        unsafe {
            let handle = chfl_property_double(value);
            RawProperty::from_ptr(handle)
        }
    }

    fn bool(value: bool) -> Result<RawProperty> {
        let value = if value { 1 } else { 0 };
        unsafe {
            let handle = chfl_property_bool(value);
            RawProperty::from_ptr(handle)
        }
    }

    fn vector3d(value: [f64; 3]) -> Result<RawProperty> {
        unsafe {
            let handle = chfl_property_vector3d(value.as_ptr());
            RawProperty::from_ptr(handle)
        }
    }

    fn string(value: &str) -> Result<RawProperty> {
        let buffer = strings::to_c(value);
        unsafe {
            let handle = chfl_property_string(buffer.as_ptr());
            RawProperty::from_ptr(handle)
        }
    }

    fn get_kind(&self) -> Result<chfl_property_kind> {
        let mut kind = chfl_property_kind::CHFL_PROPERTY_BOOL;
        unsafe {
            try!(check(chfl_property_get_kind(self.as_ptr(), &mut kind)));
        }
        return Ok(kind);
    }

    fn get_bool(&self) -> Result<bool> {
        let mut value = 0;
        unsafe {
            try!(check(chfl_property_get_bool(self.as_ptr(), &mut value)));
        }
        return Ok(value != 0);
    }

    fn get_double(&self) -> Result<f64> {
        let mut value = 0.0;
        unsafe {
            try!(check(chfl_property_get_double(self.as_ptr(), &mut value)));
        }
        return Ok(value);
    }

    fn get_string(&self) -> Result<String> {
        let get_string = |ptr, len| unsafe { chfl_property_get_string(self.as_ptr(), ptr, len) };
        let value = try!(strings::call_autogrow_buffer(64, get_string));
        return Ok(strings::from_c(value.as_ptr()));
    }

    fn get_vector3d(&self) -> Result<[f64; 3]> {
        let mut value = [0.0; 3];
        unsafe {
            try!(check(chfl_property_get_vector3d(self.as_ptr(), value.as_mut_ptr())));
        }
        return Ok(value);
    }
}

impl Drop for RawProperty {
    fn drop(&mut self) {
        unsafe {
            let status = chfl_property_free(self.as_mut_ptr());
            debug_assert_eq!(status, chfl_status::CHFL_SUCCESS);
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

impl Property {
    pub(crate) fn as_raw(&self) -> Result<RawProperty> {
        match *self {
            Property::Bool(value) => RawProperty::bool(value),
            Property::Double(value) => RawProperty::double(value),
            Property::String(ref value) => RawProperty::string(value),
            Property::Vector3D(value) => RawProperty::vector3d(value),
        }
    }

    #[allow(needless_pass_by_value)]  // raw
    pub(crate) fn from_raw(raw: RawProperty) -> Result<Property> {
        match raw.get_kind()? {
            chfl_property_kind::CHFL_PROPERTY_BOOL => Ok(Property::Bool(raw.get_bool()?)),
            chfl_property_kind::CHFL_PROPERTY_DOUBLE => Ok(Property::Double(raw.get_double()?)),
            chfl_property_kind::CHFL_PROPERTY_STRING => Ok(Property::String(raw.get_string()?)),
            chfl_property_kind::CHFL_PROPERTY_VECTOR3D => {
                Ok(Property::Vector3D(raw.get_vector3d()?))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    mod raw {
        use super::super::*;

        #[test]
        fn bool() {
            let property = RawProperty::bool(false).unwrap();
            assert_eq!(property.get_kind(), Ok(chfl_property_kind::CHFL_PROPERTY_BOOL));
            assert_eq!(property.get_bool(), Ok(false));
        }

        #[test]
        fn double() {
            let property = RawProperty::double(45.0).unwrap();
            assert_eq!(property.get_kind(), Ok(chfl_property_kind::CHFL_PROPERTY_DOUBLE));
            assert_eq!(property.get_double(), Ok(45.0));
        }

        #[test]
        fn string() {
            let property = RawProperty::string("test").unwrap();
            assert_eq!(property.get_kind(), Ok(chfl_property_kind::CHFL_PROPERTY_STRING));
            assert_eq!(property.get_string(), Ok("test".into()));
        }

        #[test]
        fn vector3d() {
            let property = RawProperty::vector3d([1.2, 3.4, 5.6]).unwrap();
            assert_eq!(property.get_kind(), Ok(chfl_property_kind::CHFL_PROPERTY_VECTOR3D));
            assert_eq!(property.get_vector3d(), Ok([1.2, 3.4, 5.6]));
        }
    }

    mod rust {
        use super::super::*;

        #[test]
        fn bool() {
            let property = Property::Bool(false);

            let raw = property.as_raw().unwrap();
            assert_eq!(raw.get_kind(), Ok(chfl_property_kind::CHFL_PROPERTY_BOOL));
            assert_eq!(raw.get_bool(), Ok(false));

            assert_eq!(Property::from_raw(raw), Ok(property));
        }

        #[test]
        fn double() {
            let property = Property::Double(45.0);

            let raw = property.as_raw().unwrap();
            assert_eq!(raw.get_kind(), Ok(chfl_property_kind::CHFL_PROPERTY_DOUBLE));
            assert_eq!(raw.get_double(), Ok(45.0));

            assert_eq!(Property::from_raw(raw), Ok(property));
        }

        #[test]
        fn string() {
            let property = Property::String("test".into());

            let raw = property.as_raw().unwrap();
            assert_eq!(raw.get_kind(), Ok(chfl_property_kind::CHFL_PROPERTY_STRING));
            assert_eq!(raw.get_string(), Ok("test".into()));

            assert_eq!(Property::from_raw(raw), Ok(property));
        }

        #[test]
        fn vector3d() {
            let property = Property::Vector3D([1.2, 3.4, 5.6]);

            let raw = property.as_raw().unwrap();
            assert_eq!(raw.get_kind(), Ok(chfl_property_kind::CHFL_PROPERTY_VECTOR3D));
            assert_eq!(raw.get_vector3d(), Ok([1.2, 3.4, 5.6]));

            assert_eq!(Property::from_raw(raw), Ok(property));
        }
    }
}

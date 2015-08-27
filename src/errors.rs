/*
 * Chemharp, an efficient IO library for chemistry file formats
 * Copyright (C) 2015 Guillaume Fraux
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/
*/

extern crate libc;

use std::ffi::CStr;
use std::str;

use ::ffi::*;

/// Create a Rust string from a C string.
pub fn from_c_str(buffer: *const i8) -> String {
    let mut res = String::new();
    unsafe {
        let c_string = CStr::from_ptr(buffer);
        let rust_str = str::from_utf8(c_string.to_bytes())
                            .ok()
                            .expect("Got invalid UTF8 string from C!");
        res.push_str(rust_str);
    }
    return res;
}

#[derive(Clone, Debug)]
/// Possible causes of error in Chemharp
pub enum Error {
    /// Exception in the C++ standard library
    CppStdError{message: String},
    /// Exception in the C++ Chemharp library
    ChemharpCppError{message: String},
    /// Error in memory
    MemoryError,
    /// Error while reading or writing a file
    FileError,
    /// Error in file formatting
    FormatError,
}

impl From<CHRP_STATUS> for Error {
    fn from(status: CHRP_STATUS) -> Error {
        match status {
            1 => Error::CppStdError{message: Error::last_error()},
            2 => Error::ChemharpCppError{message: Error::last_error()},
            3 => Error::MemoryError,
            4 => Error::FileError,
            5 => Error::FormatError,
            _ => unreachable!()
        }
    }
}

impl From<Error> for CHRP_STATUS {
    fn from(error: Error) -> CHRP_STATUS {
        match error {
            Error::CppStdError{..} => 1,
            Error::ChemharpCppError{..} => 2,
            Error::MemoryError => 3,
            Error::FileError => 4,
            Error::FormatError => 5
        }
    }
}

impl Error {
    /// Get the message associated with this error.
    pub fn message(&self) -> String {
        let error = self.clone();
        match error {
            Error::CppStdError{message} | Error::ChemharpCppError{message} => message,
            _ => {
                unsafe {
                    from_c_str(chrp_strerror(CHRP_STATUS::from(error)))
                }
            }
        }
    }

    /// Get the last error message.
    pub fn last_error() -> String {
        unsafe {
            from_c_str(chrp_last_error())
        }
    }
}

/// Check return value of a C function, and get the error if needed.
pub fn check(status: CHRP_STATUS) -> Result<(), Error> {
    if status != 0 {
        return Err(Error::from(status));
    }
    return Ok(());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn errors() {
        let error = Error::MemoryError;

        assert_eq!(error.message(), "Memory error.");
        assert_eq!(Error::last_error(), "");
    }
}

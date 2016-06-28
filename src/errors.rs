/* Chemfiles, an efficient IO library for chemistry file formats
 * Copyright (C) 2015 Guillaume Fraux
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/
*/
use std::error;
use std::fmt;
use std::result;
use std::path::Path;

use chemfiles_sys::*;
use string;
use Result;

#[derive(Clone, Debug, PartialEq)]
/// Error type for Chemfiles.
pub struct Error {
    /// The error kind
    pub kind: ErrorKind,
    /// A message describing the error cause
    pub message: String
}

#[derive(Clone, Debug, PartialEq)]
/// Possible causes of error in chemfiles
pub enum ErrorKind {
    /// Exception in the C++ standard library
    CppStdError,
    /// Exception in the C++ chemfiles library
    ChemfilesCppError,
    /// Error in memory allocations
    MemoryError,
    /// Error while reading or writing a file
    FileError,
    /// Error in file formatting, *i.e.* the file is invalid
    FormatError,
    /// Error in selection string syntax
    SelectionError,
    /// The given path is not valid UTF8
    UTF8PathError,
    /// We got a null pointer from C++
    NullPtr,
}

impl From<CHFL_STATUS> for Error {
    fn from(status: CHFL_STATUS) -> Error {
        let kind = match status {
            CHFL_CXX_ERROR => ErrorKind::CppStdError,
            CHFL_GENERIC_ERROR => ErrorKind::ChemfilesCppError,
            CHFL_MEMORY_ERROR => ErrorKind::MemoryError,
            CHFL_FILE_ERROR => ErrorKind::FileError,
            CHFL_FORMAT_ERROR => ErrorKind::FormatError,
            CHFL_SELECTION_ERROR => ErrorKind::SelectionError,
            _ => unreachable!()
        };
        Error {
            kind: kind,
            message: Error::last_error()
        }
    }
}

impl From<Error> for CHFL_STATUS {
    fn from(error: Error) -> CHFL_STATUS {
        match error.kind {
            ErrorKind::CppStdError => CHFL_CXX_ERROR,
            ErrorKind::ChemfilesCppError => CHFL_GENERIC_ERROR,
            ErrorKind::MemoryError => CHFL_MEMORY_ERROR,
            ErrorKind::FileError => CHFL_FILE_ERROR,
            ErrorKind::FormatError => CHFL_FORMAT_ERROR,
            ErrorKind::SelectionError => CHFL_SELECTION_ERROR,
            ErrorKind::UTF8PathError | ErrorKind::NullPtr  => {
                panic!(
                    "Can not convert this error to error code. \
                    It is a Rust-side only error."
                )
            },
        }
    }
}

impl Error {
    /// Create a new error because the given `path` is invalid UTF-8 data
    #[doc(hidden)]
    pub fn utf8_path_error(path: &Path) -> Error {
        Error {
            kind: ErrorKind::UTF8PathError,
            message: format!("Could not convert '{}' to UTF8", path.display())
        }
    }

    /// Create a new error because we got a null pointer from C++
    #[doc(hidden)]
    pub fn null_ptr() -> Error {
        Error {
            kind: ErrorKind::NullPtr,
            message: Error::last_error()
        }
    }

    /// Get the last error message from the C++ library.
    pub fn last_error() -> String {
        unsafe {
            string::from_c(chfl_last_error())
        }
    }
}

/// Check return value of a C function, and get the error if needed.
pub fn check(status: CHFL_STATUS) -> Result<()> {
    if status != CHFL_SUCCESS {
        return Err(Error::from(status));
    }
    return Ok(());
}


impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        write!(fmt, "{}", self.message)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match self.kind {
            ErrorKind::CppStdError => "Exception from the C++ standard library",
            ErrorKind::ChemfilesCppError => "Exception from the chemfiles library",
            ErrorKind::MemoryError => "Error in memory allocations",
            ErrorKind::FileError => "Error while reading or writing a file",
            ErrorKind::FormatError => "Error in file formatting, i.e. the file is invalid",
	        ErrorKind::SelectionError => "Error in selection string syntax",
	        ErrorKind::UTF8PathError => "The given path is not valid UTF8",
            ErrorKind::NullPtr => "We got a NULL pointer from C++"
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn errors() {
        assert_eq!(Error::last_error(), "");
    }
}

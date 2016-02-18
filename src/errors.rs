/* Chemfiles, an efficient IO library for chemistry file formats
 * Copyright (C) 2015 Guillaume Fraux
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/
*/
use chemfiles_sys::*;
use string;

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
            ErrorKind::UTF8PathError => {
                panic!("Can not convert UTF8PathError to C error code. It is a Rust-side error.")
            },
        }
    }
}

impl Error {
    /// Create a new error of the given `kind` and the last error message from
    /// the C++ library.
    pub fn new(kind: ErrorKind) -> Error {
        Error {
            kind: kind,
            message: Error::last_error()
        }
    }

    /// Get the message associated with this error.
    pub fn message(&self) -> String {
        self.message.clone()
    }

    /// Get the last error message.
    pub fn last_error() -> String {
        unsafe {
            string::from_c(chfl_last_error())
        }
    }
}

/// Check return value of a C function, and get the error if needed.
pub fn check(status: CHFL_STATUS) -> Result<(), Error> {
    if status != CHFL_SUCCESS {
        return Err(Error::from(status));
    }
    return Ok(());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn errors() {
        assert_eq!(Error::last_error(), "");
    }
}

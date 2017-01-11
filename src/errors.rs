/* Chemfiles, an efficient IO library for chemistry file formats
 * Copyright (C) 2015 Guillaume Fraux
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/
*/
extern crate libc;

use std::error;
use std::fmt;
use std::sync::Mutex;
use std::result;
use std::path::Path;

use self::libc::c_char;

use chemfiles_sys::*;
use string;
use Result;

#[derive(Clone, Debug, PartialEq)]
/// Error type for Chemfiles.
pub struct Error {
    /// The error status code
    pub status: Status,
    /// A message describing the error cause
    pub message: String
}

#[derive(Clone, Debug, PartialEq)]
/// Possible causes of error in chemfiles
pub enum Status {
    /// No error
    Success,
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

impl From<chfl_status> for Error {
    fn from(status: chfl_status) -> Error {
        let status = match status {
            chfl_status::CHFL_SUCCESS => Status::Success,
            chfl_status::CHFL_CXX_ERROR => Status::CppStdError,
            chfl_status::CHFL_GENERIC_ERROR => Status::ChemfilesCppError,
            chfl_status::CHFL_MEMORY_ERROR => Status::MemoryError,
            chfl_status::CHFL_FILE_ERROR => Status::FileError,
            chfl_status::CHFL_FORMAT_ERROR => Status::FormatError,
            chfl_status::CHFL_SELECTION_ERROR => Status::SelectionError,
        };
        Error {
            status: status,
            message: Error::last_error()
        }
    }
}

impl Error {
    /// Create a new error because the given `path` is invalid UTF-8 data
    #[doc(hidden)]
    pub fn utf8_path_error(path: &Path) -> Error {
        Error {
            status: Status::UTF8PathError,
            message: format!("Could not convert '{}' to UTF8", path.display())
        }
    }

    /// Create a new error because we got a null pointer from C++
    #[doc(hidden)]
    pub fn null_ptr() -> Error {
        Error {
            status: Status::NullPtr,
            message: Error::last_error()
        }
    }

    /// Get the last error message from the C++ library.
    pub fn last_error() -> String {
        unsafe {
            string::from_c(chfl_last_error())
        }
    }

    /// Clear any error from the C++ library
    pub fn cleanup() {
        unsafe {
            // TODO check the status
            let _ = chfl_clear_errors();
        }
    }
}

/// Check return value of a C function, and get the error if needed.
pub fn check(status: chfl_status) -> Result<()> {
    if status != chfl_status::CHFL_SUCCESS {
        return Err(Error::from(status));
    }
    return Ok(());
}

// FIXME: there must be a better way to do this ...
static mut LOGGING_CALLBACK: Option<*const Fn(&str)> = None;
extern "C" fn warning_callback(message: *const c_char) {
    unsafe {
        let callback = LOGGING_CALLBACK.expect(
            "No callback provided, this is an internal bug"
        );
        (*callback)(&string::from_c(message));
    }
}

/// Use `callback` for every chemfiles warning. The callback will be passed
/// the warning message.
///
/// # Caveats
///
/// This function will box and forget the callback, effectivelly leaking it.
/// Calling this function multiple time will leak all callbacks.
///
/// This function hold a `Mutex` under the hood, and will block when called
/// from multiple threads. You should really call this function once, at the
/// beggining of your application.
pub fn set_warning_callback<F>(callback: F) -> Result<()> where F: Fn(&str) + 'static {
    // Grab a mutex to prevent concurent modifications of the warning callback
    let mutex = Mutex::new(0);
    let _guard = mutex.lock().expect("Could not get the mutex in set_warning_callback");

    // Put the callback on the heap to be sure it survives long enough. This
    // mean than we leak all the callbacks here.
    let callback = Box::into_raw(Box::new(callback));
    unsafe {
        LOGGING_CALLBACK = Some(callback);
        try!(check(chfl_set_warning_callback(warning_callback)));
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
        match self.status {
            Status::Success => "Success",
            Status::CppStdError => "Exception from the C++ standard library",
            Status::ChemfilesCppError => "Exception from the chemfiles library",
            Status::MemoryError => "Error in memory allocations",
            Status::FileError => "Error while reading or writing a file",
            Status::FormatError => "Error in file formatting, i.e. the file is invalid",
	        Status::SelectionError => "Error in selection string syntax",
	        Status::UTF8PathError => "The given path is not valid UTF8",
            Status::NullPtr => "We got a NULL pointer from C++",
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use Trajectory;
    use std::error::Error as ErrorTrait;

    #[test]
    fn errors() {
        Error::cleanup();
        assert_eq!(Error::last_error(), "");
        assert!(Trajectory::open("nope").is_err());
        assert_eq!(Error::last_error(), "Can not find a format associated with the \"\" extension.");
        Error::cleanup();
        assert_eq!(Error::last_error(), "");
    }

    #[test]
    fn codes() {
        assert_eq!(Error::from(chfl_status::CHFL_SUCCESS).status, Status::Success);
        assert_eq!(Error::from(chfl_status::CHFL_CXX_ERROR).status, Status::CppStdError);
        assert_eq!(Error::from(chfl_status::CHFL_GENERIC_ERROR).status, Status::ChemfilesCppError);
        assert_eq!(Error::from(chfl_status::CHFL_MEMORY_ERROR).status, Status::MemoryError);
        assert_eq!(Error::from(chfl_status::CHFL_FILE_ERROR).status, Status::FileError);
        assert_eq!(Error::from(chfl_status::CHFL_FORMAT_ERROR).status, Status::FormatError);
        assert_eq!(Error::from(chfl_status::CHFL_SELECTION_ERROR).status, Status::SelectionError);
    }

    #[test]
    fn messages() {
        assert!(Error::from(chfl_status::CHFL_SUCCESS).description().contains("Success"));
        assert!(Error::from(chfl_status::CHFL_CXX_ERROR).description().contains("C++ standard library"));
        assert!(Error::from(chfl_status::CHFL_GENERIC_ERROR).description().contains("chemfiles library"));
        assert!(Error::from(chfl_status::CHFL_MEMORY_ERROR).description().contains("memory"));
        assert!(Error::from(chfl_status::CHFL_FILE_ERROR).description().contains("file"));
        assert!(Error::from(chfl_status::CHFL_FORMAT_ERROR).description().contains("format"));
        assert!(Error::from(chfl_status::CHFL_SELECTION_ERROR).description().contains("selection"));
    }
}

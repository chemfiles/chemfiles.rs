// Chemfiles, a modern library for chemistry file reading and writing
// Copyright (C) 2015-2018 Guillaume Fraux -- BSD licensed
use std::os::raw::c_char;
use std::panic::{self, RefUnwindSafe};
use std::path::Path;

use chemfiles_sys::*;

use crate::strings;

#[derive(Clone, Debug, PartialEq, Eq)]
/// Error type for Chemfiles.
pub struct Error {
    /// The error status code
    pub status: Status,
    /// A message describing the error cause
    pub message: String,
}

#[repr(C)]
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Eq)]
/// Possible causes of error in chemfiles
pub enum Status {
    /// No error
    Success = chfl_status::CHFL_SUCCESS as isize,
    /// Error in memory allocations
    MemoryError = chfl_status::CHFL_MEMORY_ERROR as isize,
    /// Error while reading or writing a file
    FileError = chfl_status::CHFL_FILE_ERROR as isize,
    /// Error in file formatting, *i.e.* the file is invalid
    FormatError = chfl_status::CHFL_FORMAT_ERROR as isize,
    /// Error in selection string syntax
    SelectionError = chfl_status::CHFL_SELECTION_ERROR as isize,
    /// Error in configuration files syntax
    ConfigurationError = chfl_status::CHFL_CONFIGURATION_ERROR as isize,
    /// Error for out of bounds indexing
    OutOfBounds = chfl_status::CHFL_OUT_OF_BOUNDS as isize,
    /// Error related to properties
    PropertyError = chfl_status::CHFL_PROPERTY_ERROR as isize,
    /// Exception in the C++ chemfiles library
    ChemfilesError = chfl_status::CHFL_GENERIC_ERROR as isize,
    /// Exception in the C++ standard library
    StdCppError = chfl_status::CHFL_CXX_ERROR as isize,
    /// The given path is not valid UTF8
    // TODO: rename this to UTF8Error in the next breaking release
    UTF8PathError,
}

impl From<chfl_status> for Error {
    fn from(status: chfl_status) -> Error {
        let status = match status {
            chfl_status::CHFL_SUCCESS => Status::Success,
            chfl_status::CHFL_CXX_ERROR => Status::StdCppError,
            chfl_status::CHFL_GENERIC_ERROR => Status::ChemfilesError,
            chfl_status::CHFL_MEMORY_ERROR => Status::MemoryError,
            chfl_status::CHFL_FILE_ERROR => Status::FileError,
            chfl_status::CHFL_FORMAT_ERROR => Status::FormatError,
            chfl_status::CHFL_SELECTION_ERROR => Status::SelectionError,
            chfl_status::CHFL_CONFIGURATION_ERROR => Status::ConfigurationError,
            chfl_status::CHFL_OUT_OF_BOUNDS => Status::OutOfBounds,
            chfl_status::CHFL_PROPERTY_ERROR => Status::PropertyError,
        };

        let message = Error::last_error();
        Error { status, message }
    }
}

impl From<std::str::Utf8Error> for Error {
    fn from(_: std::str::Utf8Error) -> Self {
        Error {
            status: Status::UTF8PathError,
            message: "failed to convert data to UTF8 string".into(),
        }
    }
}

impl Error {
    /// Create a new error because the given `path` is invalid UTF-8 data
    pub(crate) fn utf8_path_error(path: &Path) -> Error {
        Error {
            status: Status::UTF8PathError,
            message: format!("Could not convert '{}' to UTF8", path.display()),
        }
    }

    /// Get the last error message from the C++ library.
    pub fn last_error() -> String {
        unsafe { strings::from_c(chfl_last_error()) }
    }

    /// Clear any error from the C++ library
    pub fn cleanup() {
        unsafe {
            check(chfl_clear_errors()).expect("error in chfl_clear_errors. Things went very bad");
        }
    }
}

/// Check return value of a C function, and get the error if needed.
pub(crate) fn check(status: chfl_status) -> Result<(), Error> {
    if status == chfl_status::CHFL_SUCCESS {
        Ok(())
    } else {
        Err(Error::from(status))
    }
}

/// Check return value of a C function, panic if it failed.
pub(crate) fn check_success(status: chfl_status) {
    assert!(
        status == chfl_status::CHFL_SUCCESS,
        "unexpected failure: {}",
        Error::last_error()
    );
}

/// Check a pointer for null.
pub(crate) fn check_not_null<T>(ptr: *const T) {
    assert!(!ptr.is_null(), "unexpected null pointer: {}", Error::last_error());
}

pub trait WarningCallback: RefUnwindSafe + Fn(&str) {}
impl<T> WarningCallback for T where T: RefUnwindSafe + Fn(&str) {}

static mut LOGGING_CALLBACK: Option<*mut dyn WarningCallback<Output = ()>> = None;

extern "C" fn warning_callback(message: *const c_char) {
    unsafe {
        let callback = &*LOGGING_CALLBACK.expect("No callback provided, this is an internal bug");
        // ignore result. If a panic happened, everything is going badly anyway
        let _result = panic::catch_unwind(|| {
            callback(&strings::from_c(message));
        });
    }
}

/// Use `callback` for every chemfiles warning. The callback will be passed
/// the warning message. This will drop any previous warning callback.
pub fn set_warning_callback<F>(callback: F)
where
    F: WarningCallback + 'static,
{
    // box callback to ensure it stays accessible
    let callback = Box::into_raw(Box::new(callback));
    unsafe {
        if let Some(previous) = LOGGING_CALLBACK {
            // drop the previous callback
            let previous = Box::from_raw(previous);
            std::mem::drop(previous);
            // set the LOGGING_CALLBACK to the new one
            LOGGING_CALLBACK = Some(callback);
        } else {
            // set the LOGGING_CALLBACK
            LOGGING_CALLBACK = Some(callback);
            // Tell C code to use Rust-provided callback
            check_success(chfl_set_warning_callback(warning_callback));
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(fmt, "{}", self.message)
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match self.status {
            Status::Success => "Success",
            Status::StdCppError => "Exception from the C++ standard library",
            Status::ChemfilesError => "Exception from the chemfiles library",
            Status::MemoryError => "Error in memory allocations",
            Status::FileError => "Error while reading or writing a file",
            Status::FormatError => "Error in file formatting, i.e. the file is invalid",
            Status::SelectionError => "Error in selection string syntax",
            Status::UTF8PathError => "A string is not valid UTF8",
            Status::ConfigurationError => "Error in configuration files",
            Status::OutOfBounds => "Out of bounds indexing",
            Status::PropertyError => "Error in property",
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Trajectory;

    #[test]
    fn errors() {
        Error::cleanup();
        assert_eq!(Error::last_error(), "");
        assert!(Trajectory::open("nope", 'r').is_err());
        assert_eq!(
            Error::last_error(),
            "file at \'nope\' does not have an extension, provide a format name to read it"
        );
        Error::cleanup();
        assert_eq!(Error::last_error(), "");
    }

    #[test]
    fn codes() {
        assert_eq!(Error::from(chfl_status::CHFL_SUCCESS).status, Status::Success);
        assert_eq!(Error::from(chfl_status::CHFL_CXX_ERROR).status, Status::StdCppError);
        assert_eq!(
            Error::from(chfl_status::CHFL_GENERIC_ERROR).status,
            Status::ChemfilesError
        );
        assert_eq!(Error::from(chfl_status::CHFL_MEMORY_ERROR).status, Status::MemoryError);
        assert_eq!(Error::from(chfl_status::CHFL_FILE_ERROR).status, Status::FileError);
        assert_eq!(Error::from(chfl_status::CHFL_FORMAT_ERROR).status, Status::FormatError);
        assert_eq!(
            Error::from(chfl_status::CHFL_SELECTION_ERROR).status,
            Status::SelectionError
        );
        assert_eq!(Error::from(chfl_status::CHFL_OUT_OF_BOUNDS).status, Status::OutOfBounds);
        assert_eq!(
            Error::from(chfl_status::CHFL_PROPERTY_ERROR).status,
            Status::PropertyError
        );
    }
}

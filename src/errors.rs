/*
 * Chemharp, an efficient IO library for chemistry file formats
 * Copyright (C) 2015 Guillaume Fraux
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/
*/

extern crate libc;

use ::ffi::*;
use ::string;

#[derive(Clone, Debug, PartialEq)]
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
                    string::from_c(chrp_strerror(CHRP_STATUS::from(error)))
                }
            }
        }
    }

    /// Get the last error message.
    pub fn last_error() -> String {
        unsafe {
            string::from_c(chrp_last_error())
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

/******************************************************************************/

/// Available log levels
#[derive(Clone, Debug, PartialEq)]
pub enum LogLevel {
    /// Do not log anything
    NONE = NONE as isize,
    /// Only log errors
    ERROR = ERROR as isize,
    /// Log errors and warnings
    WARNING = WARNING as isize,
    /// Log errors, warnings and informations
    INFO = INFO as isize,
    /// Log everything (errors, warnings, informations and debug informations)
    DEBUG = DEBUG as isize,
}

impl From<CHRP_LOG_LEVEL> for LogLevel {
    fn from(level: CHRP_LOG_LEVEL) -> LogLevel {
        match level {
            NONE => LogLevel::NONE,
            ERROR => LogLevel::ERROR,
            WARNING => LogLevel::WARNING,
            INFO => LogLevel::INFO,
            DEBUG => LogLevel::DEBUG,
            _ => unreachable!()
        }
    }
}

pub struct Logging;

impl Logging {
    /// Get current logging level
    pub fn level() -> Result<LogLevel, Error> {
        let mut level = 0;
        unsafe {
            try!(check(chrp_loglevel(&mut level)));
        }
        Ok(LogLevel::from(level))
    }


    /// Set the logging level to `level`
    pub fn set_level(level: LogLevel) -> Result<(), Error> {
        unsafe {
            try!(check(chrp_set_loglevel(level as CHRP_LOG_LEVEL)));
        }
        Ok(())
    }

    /// Write logs to the file at `path`, creating it if needed.
    pub fn log_to_file<'a, S>(path: S) -> Result<(), Error> where S: Into<&'a str> {
        let buffer = string::to_c(path.into());
        unsafe {
            try!(check(chrp_logfile(buffer)));
        }
        Ok(())
    }

    /// Write logs to the standard error stream. This is the default.
    pub fn log_to_stderr() -> Result<(), Error> {
        unsafe {
            try!(check(chrp_log_stderr()));
        }
        Ok(())
    }
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

    #[test]
    fn logging() {
        use std::fs;
        let filename = "test.log";

        Logging::log_to_file(filename).unwrap();
        // Check that file exists
        assert!(fs::metadata(filename).is_ok());
        fs::remove_file(filename).unwrap();

        // Just calling the function and ensuring that the return code is OK.
        assert!(Logging::log_to_stderr().is_ok());

        let level = Logging::level().unwrap();
        assert_eq!(level, LogLevel::WARNING);
        assert!(Logging::set_level(LogLevel::ERROR).is_ok());

        let level = Logging::level().unwrap();
        assert_eq!(level, LogLevel::ERROR);
    }
}

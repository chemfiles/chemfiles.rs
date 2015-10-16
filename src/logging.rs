/* Chemfiles, an efficient IO library for chemistry file formats
 * Copyright (C) 2015 Guillaume Fraux
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/
*/
//! Logging utilities
extern crate chemfiles_sys;
use self::chemfiles_sys::*;

use std::path::Path;

use string;
use errors::{Error, check};

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

impl From<CHFL_LOG_LEVEL> for LogLevel {
    fn from(level: CHFL_LOG_LEVEL) -> LogLevel {
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

/// Get current logging level
pub fn level() -> Result<LogLevel, Error> {
    let mut level = 0;
    unsafe {
        try!(check(chfl_loglevel(&mut level)));
    }
    Ok(LogLevel::from(level))
}


/// Set the logging level to `level`
pub fn set_level(level: LogLevel) -> Result<(), Error> {
    unsafe {
        try!(check(chfl_set_loglevel(level as CHFL_LOG_LEVEL)));
    }
    Ok(())
}

/// Write logs to the file at `path`, creating it if needed.
pub fn log_to_file<P>(filename: P) -> Result<(), Error> where P: AsRef<Path> {
    let filename = match filename.as_ref().to_str() {
        Some(val) => val,
        None => {
            return Err(
                Error::UTF8PathError{message: format!("Could not convert '{}' to UTF8 string", filename.as_ref().display())}
            )
        }
    };

    let filename = string::to_c(filename);
    unsafe {
        try!(check(chfl_logfile(filename.as_ptr())));
    }
    Ok(())
}

/// Write logs to the standard error stream. This is the default.
pub fn log_to_stderr() -> Result<(), Error> {
    unsafe {
        try!(check(chfl_log_stderr()));
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn logging() {
        use std::fs;
        let filename = "test.log";

        log_to_file(filename).unwrap();
        // Check that file exists
        assert!(fs::metadata(filename).is_ok());
        fs::remove_file(filename).unwrap();

        // Just calling the function and ensuring that the return code is OK.
        assert!(log_to_stderr().is_ok());

        let log_level = level().unwrap();
        assert_eq!(log_level, LogLevel::WARNING);
        assert!(set_level(LogLevel::ERROR).is_ok());

        let log_level = level().unwrap();
        assert_eq!(log_level, LogLevel::ERROR);
    }
}

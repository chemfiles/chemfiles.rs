/* Chemfiles, an efficient IO library for chemistry file formats
 * Copyright (C) 2015 Guillaume Fraux
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/
*/
//! Logging utilities
extern crate libc;
use self::libc::c_char;

use std::path::Path;
use std::sync::{MutexGuard, Mutex};

use chemfiles_sys::*;
use string;
use errors::{Error, ErrorKind, check};

/// Available log levels
#[derive(Clone, Debug, PartialEq)]
pub enum LogLevel {
    /// Only log errors
    Error = ERROR as isize,
    /// Log errors and warnings
    Warning = WARNING as isize,
    /// Log errors, warnings and informations
    Info = INFO as isize,
    /// Log everything (errors, warnings, informations and debug informations)
    Debug = DEBUG as isize,
}

impl From<CHFL_LOG_LEVEL> for LogLevel {
    fn from(level: CHFL_LOG_LEVEL) -> LogLevel {
        match level {
            ERROR => LogLevel::Error,
            WARNING => LogLevel::Warning,
            INFO => LogLevel::Info,
            DEBUG => LogLevel::Debug,
            _ => unreachable!()
        }
    }
}

/// This struct give access to the logging system.
///
/// As it is a global system, it must be aquired before any operations.
pub struct Logger<'a> {
     _guard: MutexGuard<'a, ()>,
}

impl<'a> Logger<'a> {
    /// Get an handle to the logging system. This function blocks, waiting for a
    /// mutex to be available. You should probably call this function from one
    /// thread only.
    pub fn get() -> Logger<'a> {
        lazy_static! {
            static ref LOGGER_MUTEX: Mutex<()> = Mutex::new(());
        }

        let guard = LOGGER_MUTEX.lock().expect("Could not lock the logging system");
        Logger {
            _guard: guard
        }
    }

    /// Get the current maximal logging level
    pub fn level(&self) -> Result<LogLevel, Error> {
        let mut level = 0;
        unsafe {
            try!(check(chfl_loglevel(&mut level)));
        }
        Ok(LogLevel::from(level))
    }


    /// Set the maximal logging level to `level`
    pub fn set_level(&self, level: LogLevel) -> Result<(), Error> {
        unsafe {
            try!(check(chfl_set_loglevel(level as CHFL_LOG_LEVEL)));
        }
        Ok(())
    }

    /// Write logs to the file at `path`, creating it if needed.
    pub fn log_to_file<P>(&self, filename: P) -> Result<(), Error> where P: AsRef<Path> {
        let filename = match filename.as_ref().to_str() {
            Some(val) => val,
            None => {
                return Err(
                    Error{
                        kind: ErrorKind::UTF8PathError,
                        message: format!("Could not convert '{}' to UTF8 string", filename.as_ref().display())}
                )
            }
        };

        let filename = string::to_c(filename);
        unsafe {
            try!(check(chfl_logfile(filename.as_ptr())));
        }
        Ok(())
    }

    /// Redirect the logs to the standard error stream. This is the default.
    pub fn log_to_stderr(&self) -> Result<(), Error> {
        unsafe {
            try!(check(chfl_log_stderr()));
        }
        Ok(())
    }

    /// Redirect the logs to the standard output.
    pub fn log_to_stdout(&self) -> Result<(), Error> {
        unsafe {
            try!(check(chfl_log_stdout()));
        }
        Ok(())
    }

    /// Remove all logging output.
    pub fn log_silent(&self) -> Result<(), Error> {
        unsafe {
            try!(check(chfl_log_silent()));
        }
        Ok(())
    }

    /// Redirect all logging to user-provided logging. The `callback` function will
    /// be called at each loggin operation with the level of the message, and the
    /// the message itself.
    pub fn log_callback<F>(&self, callback: F) -> Result<(), Error> where F: Fn(LogLevel, &str) + 'static {
        let callback = Box::into_raw(Box::new(callback));
        unsafe {
            LOGGING_CALLBACK = Some(callback);
            try!(check(chfl_log_callback(logging_callback)));
        }
        return Ok(());
    }
}

static mut LOGGING_CALLBACK: Option<*const Fn(LogLevel, &str)> = None;
extern "C" fn logging_callback(level: CHFL_LOG_LEVEL, message: *const c_char) {
    unsafe {
        let callback = LOGGING_CALLBACK.expect("No callback provided! Argl ...");
        (*callback)(LogLevel::from(level), &string::from_c(message));
    }
}

#[cfg(test)]
mod test {
    use std::fs;
    use std::io::prelude::*;

    use super::*;
    use Trajectory;

    #[test]
    fn file() {
        let filename = "test.log";
        let logger = Logger::get();

        logger.log_to_file(filename).unwrap();
        // Check that file exists
        assert!(fs::metadata(filename).is_ok());
        fs::remove_file(filename).unwrap();

        assert!(logger.log_to_stderr().is_ok());
    }

    #[test]
    fn log_level() {
        let logger = Logger::get();

        let log_level = logger.level().unwrap();
        assert_eq!(log_level, LogLevel::Warning);

        assert!(logger.set_level(LogLevel::Error).is_ok());
        let log_level = logger.level().unwrap();
        assert_eq!(log_level, LogLevel::Error);
    }

    #[test]
    fn callback() {
        let logger = Logger::get();
        fn cb(level: LogLevel, message: &str) {
            let mut file = fs::File::create("test.log").unwrap();
            writeln!(file, "{:?}: {}", level, message).unwrap();
        };

        logger.log_callback(cb).unwrap();
        assert!(Trajectory::open("nothere").is_err());
        assert!(logger.log_to_stdout().is_ok());

        let mut file = fs::File::open("test.log").unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        assert_eq!(content, "Error: Can not find a format associated with the \"\" extension.\n");
    }
}

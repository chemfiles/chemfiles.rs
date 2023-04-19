// Chemfiles, a modern library for chemistry file reading and writing
// Copyright (C) 2015-2018 Guillaume Fraux -- BSD licensed

//! String conversions between C and Rust
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

use chemfiles_sys::chfl_status;
use errors::{check, Error};

/// Create a Rust string from a C string. Clones all characters in `buffer`.
pub fn from_c(buffer: *const c_char) -> String {
    unsafe {
        let rust_str = CStr::from_ptr(buffer).to_str().expect("Invalid Rust string from C");
        return String::from(rust_str);
    }
}

/// Create a C string from a Rust string.
pub fn to_c(string: &str) -> CString {
    CString::new(string).expect("Invalid C string from Rust")
}

/// Check if a string buffer was big enough when passed to a C function
fn buffer_was_big_enough(buffer: &[c_char]) -> bool {
    let len = buffer.len();
    if len < 2 {
        false
    } else {
        // The C code should always set the last element to 0
        debug_assert_eq!(buffer[len - 1], 0);
        buffer[len - 2] == 0
    }
}

/// Call `callback` C function with a string buffer and it length, using
/// `initial` as the buffer initial size. If the buffer was filled and the
/// result truncated by the C library, grow the buffer and try again until we
/// get all the data. Then return the filled buffer to the caller.
pub fn call_autogrow_buffer<F>(initial: usize, callback: F) -> Result<Vec<c_char>, Error>
where
    F: Fn(*mut c_char, u64) -> chfl_status,
{
    let mut size = initial;
    let mut buffer = vec![0; size];
    check(callback(buffer.as_mut_ptr(), buffer.len() as u64))?;

    while !buffer_was_big_enough(&buffer) {
        // Grow the buffer and retry
        size *= 2;
        buffer.resize(size, 0);
        check(callback(buffer.as_mut_ptr(), buffer.len() as u64))?;
    }

    Ok(buffer)
}

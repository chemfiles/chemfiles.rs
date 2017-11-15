// Chemfiles, a modern library for chemistry file reading and writing
// Copyright (C) 2015-2017 Guillaume Fraux
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

//! String convertions between C and Rust
use std::ffi::{CStr, CString};
use std::str;

use chemfiles_sys::chfl_status;
use errors::check;
use Result;

/// Create a Rust string from a C string.
pub fn from_c(buffer: *const i8) -> String {
    let mut res = String::new();
    unsafe {
        let c_string = CStr::from_ptr(buffer);
        let rust_str = str::from_utf8(c_string.to_bytes()).expect("invalid Rust string from C");
        res.push_str(rust_str);
    }
    return res;
}


/// Create a C string from a Rust string.
pub fn to_c(string: &str) -> CString {
    CString::new(string).expect("Invalid C string from Rust")
}

/// Check if a string buffer was big enough when passed to a C function
fn buffer_was_big_enough(buffer: &[i8]) -> bool {
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
pub fn call_autogrow_buffer<F>(initial: usize, callback: F) -> Result<Vec<i8>>
where
    F: Fn(*mut i8, u64) -> chfl_status,
{
    let mut size = initial;
    let mut buffer = vec![0; size];
    try!(check(callback(buffer.as_mut_ptr(), buffer.len() as u64)));

    while !buffer_was_big_enough(&buffer) {
        // Grow the buffer and retry
        size *= 2;
        buffer.resize(size, 0);
        try!(check(callback(buffer.as_mut_ptr(), buffer.len() as u64)));
    }

    Ok(buffer)
}

/* Chemfiles, an efficient IO library for chemistry file formats
 * Copyright (C) 2015 Guillaume Fraux
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/
*/
//! String convertions between C and Rust
use std::ffi::{CStr, CString};
use std::str;

/// Create a Rust string from a C string.
pub fn from_c(buffer: *const i8) -> String {
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


/// Create a C string from a Rust string.
pub fn to_c<'a>(string: &'a str) -> CString {
    CString::new(string).ok().expect("Got invalid C string from Rust!")
}

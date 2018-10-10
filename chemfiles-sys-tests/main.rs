#![allow(bad_style, unused)]

extern crate chemfiles_sys;
extern crate libc;

use libc::*;
use chemfiles_sys::*;

include!(concat!(env!("OUT_DIR"), "/ctest.rs"));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main()
    }
}

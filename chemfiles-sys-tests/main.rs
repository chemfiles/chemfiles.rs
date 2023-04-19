#![allow(bad_style, unused)]
use chemfiles_sys::*;
use libc::*;

#[allow(warnings)]
include!(concat!(env!("OUT_DIR"), "/ctest.rs"));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main()
    }
}

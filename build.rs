extern crate cmake;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    // Building the chemharp C++ library
    let dst = cmake::build("external").join("build");
    println!("cargo:rustc-link-search=native={}/lib", dst.display());

    // Getting the list of needed C++ libraries
    let mut dirs_file = File::open(dst.join("cxx_link_dirs.cmake")).unwrap();
    let mut content = String::new();
    dirs_file.read_to_string(&mut content).unwrap();
    for dir in content.lines() {
        println!("cargo:rustc-link-search=native={}", dir);
    }

    let mut libs_file = File::open(dst.join("cxx_link_libs.cmake")).unwrap();
    let mut content = String::new();
    libs_file.read_to_string(&mut content).unwrap();
    for lib in content.lines() {
        // Workaround a libclang_rt.osx.a not found error. This library is not
        // necessary for Chemharp, so let's just ignore it.
        if !lib.contains("libclang_rt.osx.a") {
            println!("cargo:rustc-link-lib={}", lib);
        }
    }
}

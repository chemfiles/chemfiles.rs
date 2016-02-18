extern crate cmake;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

fn main() {
    let path = Path::new("chemfiles").join("CMakeLists.txt");
    if !path.exists() {
        panic!("The git submodule for chemfiles is not initalized.\n\
                Please run `git submodule update --init.`")
    }

    let mut cfg = cmake::Config::new(".");
    cfg.define("BUILD_SHARED_LIBS", "OFF");
    cfg.define("CMAKE_POSITION_INDEPENDENT_CODE", "ON");

    // Building the chemfiles C++ library
    let dst = cfg.build().join("build");
    println!("cargo:rustc-link-search=native={}/chemfiles", dst.display());

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
        // necessary for chemfiles, so let's just ignore it.
        if !lib.contains("libclang_rt.osx.a") {
            println!("cargo:rustc-link-lib={}", lib);
        }
    }
}

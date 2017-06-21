extern crate cmake;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::env;

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
    let mut chemfiles_lib_path = dst.join("chemfiles");
    let target = env::var("TARGET").expect("TARGET is not set");
    if target.contains("-windows-msvc") {
        let profile = env::var("PROFILE").expect("PROFILE is not set");
        chemfiles_lib_path = chemfiles_lib_path.join(profile);
    }
    println!("cargo:rustc-link-search=native={}", chemfiles_lib_path.display());

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
        /// Exclude bogus libraries: libclang_rt.osx.a is not found by the
        /// linker, and to_library is a bug with cmake
        fn exclude(name: &str) -> bool {
            name.contains("libclang_rt.osx.a") || name == "to_library"
        }
        if !exclude(&lib) {
            println!("cargo:rustc-link-lib={}", lib);
        }
    }
}

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

    let out_dir = cmake::Config::new(".").build();
    let lib = out_dir.join("lib");
    let build = out_dir.join("build");

    println!("cargo:rustc-link-search=native={}", lib.display());

    // Getting the list of needed C++ libraries
    let mut dirs_file = File::open(build.join("cxx_link_dirs.cmake")).unwrap();
    let mut content = String::new();
    dirs_file.read_to_string(&mut content).unwrap();
    for dir in content.lines() {
        println!("cargo:rustc-link-search=native={}", dir);
    }

    let mut libs_file = File::open(build.join("cxx_link_libs.cmake")).unwrap();
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

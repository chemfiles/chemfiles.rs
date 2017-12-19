extern crate cmake;

use std::io::prelude::*;
use std::fs::File;
use std::path::{Path, PathBuf};

fn main() {
    let out_dir = build_chemfiles();
    list_cxx_libs(&out_dir.join("build"));
}

fn build_chemfiles() -> PathBuf {
    let path = Path::new("chemfiles").join("CMakeLists.txt");
    if !path.exists() {
        panic!("uninitalized git submodule. Please run `git submodule update --init.`")
    }

    let out_dir = cmake::Config::new(".").build();
    let lib = out_dir.join("lib");
    println!("cargo:rustc-link-search=native={}", lib.display());

    return out_dir;
}

fn list_cxx_libs(build: &Path) {
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
        if !is_excluded_lib(&lib) {
            println!("cargo:rustc-link-lib={}", lib);
        }
    }
}

#[cfg(target_os = "macos")]
fn is_excluded_lib(name: &str) -> bool {
    // libclang_rt.osx.a is not found by the linker, and to_library is a bug
    // with cmake (trying to parse -lto_library=<...>)
    name.contains("libclang_rt.osx.a") || name == "to_library"
}

#[cfg(target_os = "linux")]
fn is_excluded_lib(name: &str) -> bool {
    // Fiw warnings about redundant linker flag
    name == "gcc" || name == "gcc_s"
}

#[cfg(not(any(target_os = "linux", target_os = "macos")))]
fn is_excluded_lib(name: &str) -> bool {
    false
}

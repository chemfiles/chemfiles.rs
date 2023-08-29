#![allow(clippy::needless_return)]

use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

fn main() {
    let out_dir = build_chemfiles();
    list_cxx_libs(&out_dir.join("build"));
}

mod prebuilt;

fn build_chemfiles() -> PathBuf {
    let path = Path::new("chemfiles").join("CMakeLists.txt");
    if !path.exists() {
        panic!("uninitialized git submodule. Please run `git submodule update --init`.")
    }

    let mut cmake = cmake::Config::new(".");
    cmake.define("CHEMFILES_VERSION", "0.10.4");

    let target = std::env::var("TARGET").expect("cargo should set TARGET");
    if !cfg!(feature = "build-from-sources") {
        if let Some((target, sha1)) = prebuilt::get_prebuilt_info(&target) {
            cmake.define("CHFL_RUST_PREBUILT_TARGET", target);
            cmake.define("CHFL_RUST_PREBUILT_SHA256", sha1);
        }
    }

    let out_dir = cmake.build();
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
        if !is_excluded_lib(lib) {
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
fn is_excluded_lib(_: &str) -> bool {
    false
}

extern crate cmake;

fn main() {
    let target = std::env::var("TARGET").unwrap();
    let darwin = target.contains("darwin");
    let windows = target.contains("windows");
    let msvc = target.contains("msvc");
    let linux = target.contains("linux");
    let gnu = target.contains("gnu");
    let bsd = target.contains("bsd");

    // Getting code to link with C++ runtime
    if darwin {
        println!("cargo:rustc-link-lib=c++");
    } else if windows && msvc {
        // TODO
    } else if windows && !msvc {
        println!("cargo:rustc-link-lib=libc++");
    } else if linux && gnu {
        println!("cargo:rustc-link-lib=libc++");
    } else if linux && bsd {
        println!("cargo:rustc-link-lib=c++");
    } else {
        panic!("Unknown C++ runtime name! Please edit build.rs, and submit your changes!")
    }

    // Building the chemharp C++ library
    let dst = cmake::build("external");
    println!("cargo:rustc-link-search=native={}/build/lib", dst.display());
}

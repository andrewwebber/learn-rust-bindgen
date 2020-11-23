extern crate bindgen;
extern crate cc;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-search=.");
    println!("cargo:rustc-link-lib=dylib=stdc++");
    cc::Build::new()
        .cpp(true)
        .file("../example-app/add.cpp")
        .include("../example-app")
        .compile("example");
    let bindings = bindgen::Builder::default()
        .header("../example-app/add.hpp")
        .trust_clang_mangling(false)
        .clang_arg("-L .")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

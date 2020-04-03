extern crate bindgen;

use std::{env, path::PathBuf};

const BINDINGS_FILE: &'static str = "bindings.rs";
const WRAPPER_HEADER: &'static str = "wrapper.h";

fn main() {
    // Rust target spec is needed for now so that auto-generated tests pass.
    // https://github.com/rust-lang-nursery/rust-bindgen/issues/1370#issuecomment-426597356
    let bindings = bindgen::Builder::default()
        .header(WRAPPER_HEADER)
        .rust_target(bindgen::RustTarget::Stable_1_26)
        .generate()
        .expect("bindgen failed");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join(BINDINGS_FILE))
        .expect("Couldn't write bindings!");
}

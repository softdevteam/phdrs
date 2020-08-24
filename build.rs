extern crate bindgen;

use std::{env, path::PathBuf};

const BINDINGS_FILE: &'static str = "bindings.rs";
const WRAPPER_HEADER: &'static str = "wrapper.h";

fn main() {
    let bindings = bindgen::Builder::default()
        .header(WRAPPER_HEADER)
        .generate()
        .expect("bindgen failed");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join(BINDINGS_FILE))
        .expect("Couldn't write bindings!");
}

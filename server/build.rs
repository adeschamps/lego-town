extern crate protobuf_build;

use std::env;
use std::path::PathBuf;
use std::fs;

fn main() {
    let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let source = root.join("../arduino-api");
    println!("cargo:rerun-if-changed={}", source.to_str().unwrap());

    // This would be preferred, but since module level attributes
    // are not allowed in included code, there are complicatinos
    // which are difficult to work around on rust stable.
    // let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let out_dir = root.join("src/");
    println!("cargo:rerun-if-changed={}", out_dir.to_str().unwrap());
    fs::create_dir_all(out_dir.to_str().unwrap()).unwrap();

    let mut compiler = protobuf_build::Compiler::new(&source, &out_dir);
    compiler.compile("messages.proto").unwrap();
}

extern crate protobuf_build;

use std::env;
use std::path::PathBuf;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter, Read, Write};

fn main() {
    let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let source = root.join("../arduino-api");
    println!("cargo:rerun-if-changed={}", source.to_str().unwrap());

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    println!("cargo:rerun-if-changed={}", out_dir.to_str().unwrap());

    let mut compiler = protobuf_build::Compiler::new(&source, &out_dir);
    compiler.compile("messages.proto").unwrap();

    // TODO: Clean this up
    // Wrap the generated file in pub mod messages { ... }
    // This allows it to be included in the source with
    // use messages;
    let file_path = out_dir.join("messages.rs");
    let file = match File::open(file_path.clone()) {
        Ok(file) => file,
        Err(e) => panic!("Failed to open file: {}", e)
    };
    let mut reader = BufReader::new(&file);
    let buffer_string = &mut String::new();
    reader.read_to_string(buffer_string).unwrap();
    let mut options = OpenOptions::new();
    options.write(true);
    let file = match options.open(&file_path) {
        Ok(file) => file,
        Err(e) => panic!("Failed to open file: {}", e)
    };
    let mut writer = BufWriter::new(&file);
    // writer.write_fmt(buffer_string);
    write!(writer, "pub mod messages {{\n{}\n}}", buffer_string).unwrap();
}

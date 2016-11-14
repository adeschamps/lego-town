#[cfg(not(feature="protobuf_build"))]
use std::env;

fn main() {
    #[cfg(feature="protobuf_build")]
    match protobuf::generate_protobufs() {
        Ok(()) => {},
        Err(e) => {
            println!("cargo:warning=Failed to generate protobufs: {}.", e);
            println!("cargo:warning=Try switching to the 'with-generated-code' branch.");
            println!("cargo:warning=It has the protobufs already generated in the src directory.");
            println!("cargo:warning=You may need to rebase this branch before you compile.")
        }
    };

    #[cfg(not(feature="protobuf_build"))]
    {
        let out_dir = env::var("OUT_DIR").unwrap();
        println!("cargo:warning=Protobuf generation is disabled.");
        println!("cargo:warning=Make sure you have a current generated file.");
        println!("cargo:warning=It should preferably be in the in the src directory,");
        println!("cargo:warning=although it can also go in {}", out_dir);
    }
}

#[cfg(feature="protobuf_build")]
mod protobuf {
    extern crate protobuf_build;

    use std::env;
    use std::fs::{File, OpenOptions};
    use std::io::{BufReader, BufWriter, Read, Write};
    use std::path::PathBuf;

    pub fn generate_protobufs() -> Result<(), String> {
        let manifest_dir = env::var("CARGO_MANIFEST_DIR").map_err(|e| format!("{}", e))?;
        let root = PathBuf::from(manifest_dir);

        let source = root.join("../arduino-api");
        println!("cargo:rerun-if-changed={}", source.to_str().unwrap());

        let out_dir = env::var("OUT_DIR").map_err(|e| format!("{}", e))?;
        let out_dir = PathBuf::from(out_dir);
        println!("cargo:rerun-if-changed={}", out_dir.to_str().unwrap());

        let mut compiler = protobuf_build::Compiler::new(&source, &out_dir);
        compiler.compile("messages.proto").map_err(|_| format!("Error in the protobuf compiler"))?;

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
        reader.read_to_string(buffer_string).map_err(|e| format!("{}", e))?;
        let mut options = OpenOptions::new();
        options.write(true);
        let file = options.open(&file_path).map_err(|e| format!("{}", e))?;
        let mut writer = BufWriter::new(&file);
        // writer.write_fmt(buffer_string);
        write!(writer, "pub mod messages {{\n{}\n}}", buffer_string).map_err(|e| format!("{}", e))?;

        Ok(())
    }
}

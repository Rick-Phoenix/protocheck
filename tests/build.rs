use std::{env, path::PathBuf};

use prost_build::Config;
use protocheck_build::{compile_protos_with_validators, get_proto_files_recursive};

fn main() -> Result<(), Box<dyn std::error::Error>> {
  println!("cargo:rerun-if-changed=proto/");
  println!("cargo:rerun-if-changed=proto_deps/");

  let out_dir = env::var("OUT_DIR")
    .map(PathBuf::from)
    .unwrap_or(env::temp_dir());
  let descriptor_path = out_dir.join("file_descriptor_set.bin");

  let proto_include_paths = &["proto", "proto_deps"];

  let files = get_proto_files_recursive(PathBuf::from("proto/myapp/v1"))?;

  let mut config = Config::new();
  config
    .file_descriptor_set_path(&descriptor_path)
    .extern_path(".google.type", "::proto_types")
    .extern_path(".google.rpc", "::proto_types")
    .bytes(["."])
    .enable_type_names()
    .type_attribute(".", "#[derive(::serde::Serialize, ::serde::Deserialize)]")
    .out_dir(&out_dir);

  compile_protos_with_validators(&mut config, &files, proto_include_paths, &["myapp.v1"])?;

  config.compile_protos(&files, proto_include_paths)?;

  println!(
    "cargo:rustc-env=PROTO_DESCRIPTOR_SET={}",
    descriptor_path.display()
  );

  Ok(())
}

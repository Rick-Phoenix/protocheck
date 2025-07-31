use prost_build::Config;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  println!("cargo:rerun-if-changed=proto/");
  println!("cargo:rerun-if-changed=proto_deps/");

  let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR").expect("Could not find OUT_DIR"));
  let descriptor_path = out_dir.join("file_descriptor_set.bin");

  let mut config = Config::new();
  config
    .file_descriptor_set_path(descriptor_path.clone())
    .extern_path(".google.protobuf", "crate::protobuf")
    .compile_well_known_types()
    .out_dir(out_dir.clone());

  let proto_include_paths = &["proto"];

  config.compile_protos(&["proto/buf/validate/validate.proto"], proto_include_paths)?;

  println!(
    "cargo:rustc-env=PROTO_DESCRIPTOR_SET={}",
    descriptor_path.display()
  );

  Ok(())
}

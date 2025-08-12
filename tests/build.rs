use prost_build::Config;
use protocheck_build::compile_protos_with_validators;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  println!("cargo:rerun-if-changed=proto/");
  println!("cargo:rerun-if-changed=proto_deps/");

  let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR").expect("Could not find OUT_DIR"));
  let final_descriptor_path = out_dir.join("file_descriptor_set.bin");

  let proto_include_paths = &["proto", "proto_deps"];

  let proto_files = &["proto/myapp/v1/tests.proto"];

  let mut config = Config::new();
  config
    .file_descriptor_set_path(final_descriptor_path.clone())
    .bytes(["."])
    .enable_type_names()
    .type_attribute(".", "#[derive(::serde::Serialize, ::serde::Deserialize)]")
    .out_dir(out_dir.clone());

  compile_protos_with_validators(&mut config, proto_files, proto_include_paths, &["myapp.v1"])?;

  config.compile_protos(proto_files, proto_include_paths)?;

  println!(
    "cargo:rustc-env=PROTO_DESCRIPTOR_SET={}",
    final_descriptor_path.display()
  );

  Ok(())
}

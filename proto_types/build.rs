use prost_build::Config;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  println!("cargo:rerun-if-changed=proto/");
  println!("cargo:rerun-if-changed=proto_deps/");

  let dummy_path = "/tmp/my_test_descriptor.bin";
  println!("cargo:rustc-env=CEL_DESCRIPTOR_SET_PATH={}", dummy_path);

  let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR").expect("Could not find OUT_DIR"));
  let descriptor_path = out_dir.join("file_descriptor_set.bin");

  let mut config = Config::new();
  config
    .file_descriptor_set_path(descriptor_path.clone())
    .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
    .compile_well_known_types()
    .out_dir(out_dir.clone());

  let proto_include_paths = &["proto"];

  config.compile_protos(
    &[
      "proto/buf/validate/validate.proto",
      "proto/google/protobuf/descriptor.proto",
      "proto/google/protobuf/duration.proto",
      "proto/google/protobuf/timestamp.proto",
      "proto/google/protobuf/empty.proto",
      "proto/google/protobuf/field_mask.proto",
    ],
    proto_include_paths,
  )?;

  println!(
    "cargo:rustc-env=PROTO_DESCRIPTOR_SET={}",
    descriptor_path.display()
  );

  Ok(())
}

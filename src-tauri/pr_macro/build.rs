use prost_build::Config;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  println!("cargo:rerun-if-changed=proto/");
  println!("cargo:rerun-if-changed=proto_deps/");

  let dummy_path = "/tmp/my_test_descriptor.bin";
  println!("cargo:rustc-env=CEL_DESCRIPTOR_SET_PATH={}", dummy_path);

  let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
  let descriptor_path = out_dir.join("file_descriptor_set.bin");

  let mut config = Config::new();
  config
    .file_descriptor_set_path(descriptor_path.clone())
    .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
    .compile_well_known_types()
    .out_dir(out_dir.clone());

  let proto_include_paths = &["proto", "proto_deps"];

  config.compile_protos(
    &[
      "proto/myapp/v1/user.proto",
      "proto_deps/buf/validate/validate.proto",
      "proto_deps/google/protobuf/descriptor.proto",
      "proto_deps/google/protobuf/duration.proto",
      "proto_deps/google/protobuf/timestamp.proto",
      "proto_deps/google/protobuf/empty.proto",
      "proto_deps/google/protobuf/field_mask.proto",
      // "proto/myapp/v1/post.proto",
    ],
    proto_include_paths,
  )?;

  println!(
    "cargo:rustc-env=PROTO_DESCRIPTOR_SET={}",
    descriptor_path.display()
  );

  Ok(())
}

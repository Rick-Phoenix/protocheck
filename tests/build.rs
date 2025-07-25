use std::io::Read;

use prost_build::Config;
use prost_reflect::{prost::Message, prost_types::FileDescriptorSet};

fn main() -> Result<(), Box<dyn std::error::Error>> {
  println!("cargo:rerun-if-changed=proto/");
  println!("cargo:rerun-if-changed=proto_deps/");

  let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
  let final_descriptor_path = out_dir.join("file_descriptor_set.bin");

  let temp_descriptor_path = out_dir.join("temp_file_descriptor_set.bin");

  let proto_include_paths = &["proto", "proto_deps"];

  let proto_files = &[
    "proto/myapp/v1/user.proto",
    "proto_deps/buf/validate/validate.proto",
    // "proto/myapp/v1/post.proto",
  ];

  {
    let mut temp_config = prost_build::Config::new();
    temp_config.file_descriptor_set_path(temp_descriptor_path.clone());
    temp_config.compile_protos(proto_files, proto_include_paths)?;
  }

  let mut fds_file = std::fs::File::open(&temp_descriptor_path)?;
  let mut fds_bytes = Vec::new();
  fds_file.read_to_end(&mut fds_bytes)?;
  let fds = FileDescriptorSet::decode(fds_bytes.as_slice())?;
  let pool = prost_reflect::DescriptorPool::from_file_descriptor_set(fds)?;

  let mut full_message_names = Vec::new();
  for message_desc in pool.all_messages() {
    if message_desc.full_name().starts_with("myapp.v1") {
      full_message_names.push(message_desc.full_name().to_string());
    }
  }

  let mut config = Config::new();
  config
    .file_descriptor_set_path(final_descriptor_path.clone())
    .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
    .extern_path(".google.protobuf", "protocheck::types::protobuf")
    .extern_path(".buf.validate", "protocheck::types::protovalidate")
    .compile_well_known_types()
    .out_dir(out_dir.clone());

  for full_name in full_message_names {
    let attribute_str = format!(
      r#"#[protocheck::macros::protobuf_validate("{}")]"#,
      full_name
    );
    config.message_attribute(full_name, &attribute_str);
  }

  config.compile_protos(proto_files, proto_include_paths)?;

  std::fs::remove_file(&temp_descriptor_path)?;

  println!(
    "cargo:rustc-env=PROTO_DESCRIPTOR_SET={}",
    final_descriptor_path.display()
  );

  Ok(())
}

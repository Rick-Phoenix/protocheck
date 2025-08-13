#![cfg_attr(docsrs, feature(doc_auto_cfg))]

//! Build-time entrypoint for [`protocheck`](https://docs.rs/protocheck/0.1.0/protocheck/)

use std::{
  env,
  error::Error,
  io::Read,
  path::{Path, PathBuf},
};

use prost_build::Config;
use prost_reflect::{prost::Message, prost_types::FileDescriptorSet};

#[cfg(not(feature = "cel"))]
fn enable_cel() -> bool {
  false
}
#[cfg(feature = "cel")]
fn enable_cel() -> bool {
  true
}

pub fn compile_protos_with_validators(
  config: &mut Config,
  proto_files: &[impl AsRef<Path>],
  proto_include_paths: &[impl AsRef<Path>],
  packages: &[&str],
) -> Result<(), Box<dyn Error>> {
  let out_dir = PathBuf::from(env::var("OUT_DIR").expect("Could not find OUT_DIR."));

  let temp_descriptor_path = out_dir.join("temp_file_descriptor_set_for_protocheck.bin");
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

  for message_desc in pool.all_messages() {
    let message_name = message_desc.full_name();
    if packages.contains(&message_desc.package_name()) {
      let attribute_str = format!(
        r#"#[::protocheck::macros::protobuf_validate("{}")]"#,
        message_name
      );
      config.message_attribute(message_name, &attribute_str);

      if enable_cel() {
        config.message_attribute(
          message_name,
          "#[derive(::protocheck::macros::TryIntoCelValue)]",
        );
      }

      for oneof in message_desc.oneofs() {
        let oneof_name = oneof.full_name();
        config.type_attribute(
          oneof_name,
          format!(
            r#"#[::protocheck::macros::protobuf_validate_oneof("{}")]"#,
            oneof_name
          ),
        );

        config.type_attribute(oneof_name, r#"#[derive(::protocheck::macros::Oneof)]"#);

        if enable_cel() {
          config.type_attribute(
            oneof_name,
            r#"#[derive(::protocheck::macros::OneofTryIntoCelValue)]"#,
          );
        }

        for field in oneof.fields() {
          config.field_attribute(
            format!("{}.{}", oneof_name, field.name()),
            format!(r#"#[protocheck(proto_name = "{}")]"#, field.name()),
          );
        }
      }
    }
  }

  config.extern_path(".buf.validate", "::protocheck::types::protovalidate");
  config
    .extern_path(".google.protobuf", "::protocheck::types")
    .compile_well_known_types();

  std::fs::remove_file(&temp_descriptor_path)?;

  Ok(())
}

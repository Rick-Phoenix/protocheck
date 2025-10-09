#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![doc = include_str!("../README.md")]
use std::{
  env,
  error::Error,
  fs,
  io::{self, Read},
  path::{Path, PathBuf},
};

use prost_build::Config;
use prost_reflect::{prost::Message, prost_types::FileDescriptorSet};

/// This function compiles the proto_files in the list, it creates an intermediary file descriptor and it uses it to extract information about the messages, enums and oneofs which can later be used to generate the validation logic with protocheck.
pub fn compile_protos_with_validators(
  config: &mut Config,
  proto_files: &[impl AsRef<Path>],
  proto_include_paths: &[impl AsRef<Path>],
  packages: &[&str],
) -> Result<(), Box<dyn Error>> {
  let out_dir = env::var("OUT_DIR")
    .map(PathBuf::from)
    .unwrap_or_else(|_| env::temp_dir());

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

      if cfg!(feature = "cel") {
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

        if cfg!(feature = "cel") {
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

/// A helper to use when gathering the names of proto files to pass to [`prost_build::Config::compile_protos`].
/// Recursively collects all .proto files in a given directory and its subdirectories.
pub fn get_proto_files_recursive(base_dir: &Path) -> io::Result<Vec<String>> {
  let mut proto_files = Vec::new();

  if !base_dir.is_dir() {
    return Err(io::Error::new(
      io::ErrorKind::InvalidInput,
      format!("Path {:?} is not a directory.", base_dir),
    ));
  }

  // We'll use a helper function to do the actual recursive work
  // This helps keep the public function's signature clean.
  collect_proto_files_recursive_helper(base_dir, &mut proto_files)?;

  Ok(proto_files)
}

fn collect_proto_files_recursive_helper(
  current_dir: &Path,
  proto_files: &mut Vec<String>,
) -> io::Result<()> {
  for entry in fs::read_dir(current_dir)? {
    let entry = entry?;
    let path = entry.path();

    if path.is_file() {
      if path.extension().is_some_and(|ext| ext == "proto") {
        proto_files.push(
          path
            .to_str()
            .ok_or_else(|| {
              io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Path {:?} contains invalid Unicode.", path),
              )
            })?
            .to_owned(),
        );
      }
    } else if path.is_dir() {
      collect_proto_files_recursive_helper(&path, proto_files)?;
    }
  }
  Ok(())
}

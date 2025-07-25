pub use proto_types as types;
pub use protocheck_core::{field_data, validators};
pub use protocheck_proc_macros as macros;

pub mod build {
  use std::{
    env,
    error::Error,
    io::Read,
    path::{Path, PathBuf},
  };

  use prost_build::Config;
  use prost_reflect::{prost::Message, prost_types::FileDescriptorSet};

  pub fn compile_protos_with_validators(
    config: &mut Config,
    proto_files: &[impl AsRef<Path>],
    proto_include_paths: &[impl AsRef<Path>],
    app_package_prefix: &str,
  ) -> Result<(), Box<dyn Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

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

    let mut full_message_names = Vec::new();
    for message_desc in pool.all_messages() {
      if message_desc.full_name().starts_with(app_package_prefix) {
        full_message_names.push(message_desc.full_name().to_string());
      }
    }

    for full_name in full_message_names {
      let attribute_str = format!(
        r#"#[protocheck::macros::protobuf_validate("{}")]"#,
        full_name
      );
      config.message_attribute(full_name, &attribute_str);
    }

    config.compile_protos(proto_files, proto_include_paths)?;

    std::fs::remove_file(&temp_descriptor_path)?;

    Ok(())
  }
}

use std::path::PathBuf;

use prost_build::Config;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let mut config = Config::new();
  config
    .bytes(["."])
    .extern_path(".google.protobuf", "crate::protobuf")
    .compile_well_known_types()
    .out_dir(PathBuf::from("../proto_types/src/protovalidate"));

  config.compile_protos(&["proto/buf/validate/validate.proto"], &["proto"])?;

  Ok(())
}

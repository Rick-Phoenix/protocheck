use prost_build::Config;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  println!("cargo:rerun-if-changed=proto/");

  let mut config = Config::new();
  config
    .bytes(["."])
    .extern_path(".google.protobuf", "crate::protobuf")
    .compile_well_known_types();

  let proto_include_paths = &["proto"];

  config.compile_protos(&["proto/buf/validate/validate.proto"], proto_include_paths)?;

  Ok(())
}

use std::{
  fs, io,
  path::{Path, PathBuf},
};

use prost_build::Config;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let mut config = Config::new();
  config
    .bytes(["."])
    .extern_path(".google.protobuf", "crate::protobuf")
    .compile_well_known_types()
    .out_dir(PathBuf::from("../proto_types/src/protovalidate"));

  let root = PathBuf::from("proto/buf/validate");

  let files = get_proto_files_in_dir(&root)?;

  config.compile_protos(&files, &["proto"])?;

  Ok(())
}

fn get_proto_files_in_dir(proto_dir: &Path) -> io::Result<Vec<String>> {
  let mut proto_files = Vec::new();

  if !proto_dir.is_dir() {
    return Err(io::Error::new(
      io::ErrorKind::InvalidInput,
      format!("Path {:?} is not a directory.", proto_dir),
    ));
  }

  for entry in fs::read_dir(proto_dir)? {
    let entry = entry?;

    let path = entry.path();

    if path.is_file() && path.extension().is_some_and(|ext| ext == "proto") {
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
  }

  Ok(proto_files)
}

# ⚒️ Protocheck-build

This is the build-time entrypoint for [protocheck](https://docs.rs/protocheck/0.1.5/protocheck/). 

## ➡️ Getting started 

To get started, you need to use [`protocheck-build`] as a build dependency in your crate, which will use [`protocheck-proc-macro`](https://docs.rs/protocheck-proc-macro/latest/protocheck_proc_macro/index.html) to add all the validation logic to your structs. The setup will look more or less like this (this is taken directly from the [`tests`](https://github.com/Rick-Phoenix/protocheck/tree/main/tests) crate)

```rust
// In your build.rs file
use prost_build::Config;
use protocheck_build::{compile_protos_with_validators, get_proto_files_recursive};
use std::{
  env,
  path::PathBuf
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
  println!("cargo:rerun-if-changed=proto/");

  let out_dir = env::var("OUT_DIR")
    .map(PathBuf::from)
    .unwrap_or(env::temp_dir());

  let descriptor_path = out_dir.join("file_descriptor_set.bin");

  let proto_include_paths = &["proto", "proto_deps"];

  // Use the helper to get all proto files recursively in a directory
  let proto_files = get_proto_files_recursive("proto")?;

  let mut config = Config::new();
  config
    .file_descriptor_set_path(&descriptor_path)
    // Enable the use of bytes::Bytes for `bytes` fields
    .bytes(["."])
    .out_dir(&out_dir);

  // Call the build helper
  compile_protos_with_validators(&mut config, &proto_files, proto_include_paths, &["myapp.v1"])?;

  // Compile protos
  config.compile_protos(&proto_files, proto_include_paths)?;

  // Set the env for the file descriptor location
  println!(
    "cargo:rustc-env=PROTO_DESCRIPTOR_SET={}",
    descriptor_path.display()
  );

  Ok(())
}
```

[`compile_protos_with_validators`](https://docs.rs/protocheck-build/0.1.0/protocheck_build/fn.compile_protos_with_validators.html) takes these arguments:

1. The [`config`](https://docs.rs/prost-build/latest/prost_build/struct.Config.html) struct.
2. The proto files and include paths being used by the [`config`](https://docs.rs/prost-build/latest/prost_build/struct.Config.html).
3. The list of packages to apply validators to. If a given message contains Cel validation or is validated as a field by another message, its package name must be included in this list.

The function will then:

1. Compile your protos to create an intermediary descriptor
2. Iterate its messages, and use the information extracted from them to add the derives and attributes to the actual [`config`](https://docs.rs/prost-build/latest/prost_build/struct.Config.html) that are needed by [`protocheck-proc-macro`](https://docs.rs/protocheck-proc-macro/0.1.0/protocheck_proc_macro/index.html) to add the validation logic.
3. Add the derive macros used by `protocheck`
4. Use `proto_types` as the source for the well known types and the protovalidate types (it is re-exported by protocheck, so there is no need to add it as a dependency).

## 🍸 Tonic example

When using [`tonic-prost-build`](https://crates.io/crates/tonic-prost-build), the workflow is almost identical:

```rust
// In your build.rs file
use tonic_prost_build::Config;
use protocheck_build::{compile_protos_with_validators, get_proto_files_recursive};
use std::{
  env,
  path::PathBuf
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
  println!("cargo:rerun-if-changed=proto/");

  let out_dir = env::var("OUT_DIR")
    .map(PathBuf::from)
    .unwrap_or(env::temp_dir());

  let descriptor_path = out_dir.join("file_descriptor_set.bin");

  let proto_include_paths = &["proto", "proto_deps"];

  // Use the helper to get all proto files recursively in a directory
  let proto_files = get_proto_files_recursive("proto")?;

  let mut config = Config::new();
  config
    .file_descriptor_set_path(&descriptor_path)
    // Enable the use of bytes::Bytes for `bytes` fields
    .bytes(["."])
    .out_dir(&out_dir);

  // Call the build helper
  compile_protos_with_validators(&mut config, &proto_files, proto_include_paths, &["myapp.v1"])?;

  // Compile protos <-- Only this part is changed <--
  tonic_prost_build::configure()
    // We pass the config here
    .compile_with_config(config, proto_files, proto_include_paths)?;

  // Set the env for the file descriptor location
  println!(
    "cargo:rustc-env=PROTO_DESCRIPTOR_SET={}",
    descriptor_path.display()
  );

  Ok(())
}
```


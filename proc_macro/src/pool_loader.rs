use std::{fs, sync::LazyLock};

use prost_reflect::{prost::Message, DescriptorPool};

pub static DESCRIPTOR_POOL: LazyLock<DescriptorPool> = LazyLock::new(|| {
  let descriptor_set_path = std::env::var("PROTO_DESCRIPTOR_SET")
        .expect("PROTO_DESCRIPTOR_SET environment variable not set. This is required by `proc_macro` to access Protobuf schema information.");
  let descriptor_set_bytes = fs::read(&descriptor_set_path).unwrap_or_else(|e| {
    panic!(
      "Failed to read descriptor set from {}: {}",
      descriptor_set_path, e
    )
  });
  DescriptorPool::decode(descriptor_set_bytes.as_slice())
        .expect("Failed to decode DescriptorPool from descriptor set bytes. Ensure your protobuf definitions are valid.")
});

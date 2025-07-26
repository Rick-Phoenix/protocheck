use std::{fs, sync::LazyLock};

use prost_reflect::{DescriptorPool, ExtensionDescriptor};
use syn::{DeriveInput, Error};

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

pub static FIELD_RULES_EXT_DESCRIPTOR: LazyLock<ExtensionDescriptor> = LazyLock::new(|| {
  DESCRIPTOR_POOL
    .get_extension_by_name("buf.validate.field")
    .expect("buf.validate.field extension not found in descriptor pool")
});

pub static ONEOF_RULES_EXT_DESCRIPTOR: LazyLock<ExtensionDescriptor> = LazyLock::new(|| {
  DESCRIPTOR_POOL
    .get_extension_by_name("buf.validate.oneof")
    .expect("buf.validate.oneof extension not found in descriptor pool")
});

pub(crate) fn get_rule_extensions_descriptors(
  input_tokens: DeriveInput,
) -> Result<
  (
    prost_reflect::ExtensionDescriptor,
    prost_reflect::ExtensionDescriptor,
    prost_reflect::ExtensionDescriptor,
  ),
  Error,
> {
  let field_ext_descriptor = DESCRIPTOR_POOL
    .get_extension_by_name("buf.validate.field")
    .ok_or(Error::new_spanned(
      &input_tokens,
      "buf.validate.field extension not found in descriptor pool",
    ))?;
  let message_ext_descriptor = DESCRIPTOR_POOL
    .get_extension_by_name("buf.validate.message")
    .ok_or(Error::new_spanned(
      &input_tokens,
      "buf.validate.message extension not found in descriptor pool",
    ))?;
  let oneof_ext_descriptor = DESCRIPTOR_POOL
    .get_extension_by_name("buf.validate.oneof")
    .ok_or(Error::new_spanned(
      &input_tokens,
      "buf.validate.oneof extension not found in descriptor pool",
    ))?;
  Ok((
    field_ext_descriptor,
    message_ext_descriptor,
    oneof_ext_descriptor,
  ))
}

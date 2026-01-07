#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]

use proto_types::protovalidate::{FieldPath, FieldPathElement};
pub(crate) use proto_types::{field_descriptor_proto::Type as ProtoType, protovalidate};

/// The context about the field being validated that is passed to the validators.
pub mod field_data;

/// The functions executing the validation logic. These are called by the validators added by [`protocheck-proc-macro`](https://docs.rs/protocheck-proc-macro/0.1.0/protocheck_proc_macro/) to the target structs.
#[cfg(feature = "validators")]
pub mod validators;

#[cfg(feature = "cel")]
pub use cel;

pub use ordered_float;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct EnumVariant(pub i32);

impl std::fmt::Display for EnumVariant {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}
impl std::ops::Deref for EnumVariant {
  type Target = i32;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}
impl std::cmp::PartialEq<i32> for EnumVariant {
  fn eq(&self, other: &i32) -> bool {
    self.0 == *other
  }
}

impl core::ops::DerefMut for EnumVariant {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

impl From<EnumVariant> for i32 {
  fn from(value: EnumVariant) -> Self {
    value.0
  }
}

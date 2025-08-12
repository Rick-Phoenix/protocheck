#![doc = include_str!("../README.md")]

pub mod types {
  pub use proto_types::{FieldType, *};
}

/// The shared trait for all structs that have validators in them. The `validate` method is available on the structs themselves, so it is not necessary to import the trait just for validation, but this is useful for making functions that accept any struct implementing ProtoValidator, such as a Tower layer.
pub trait ProtoValidator {
  fn validate(&self) -> Result<(), Violations>;
}

use proto_types::protovalidate::Violations;
pub use protocheck_core::*;
#[doc(inline)]
pub use protocheck_proc_macro as macros;

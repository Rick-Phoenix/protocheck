#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]

pub(crate) use proto_types::{field_descriptor_proto::Type as ProtoType, protovalidate};

/// The context about the field being validated that is passed to the validators.
pub mod field_data;

/// The functions executing the validation logic. These are called by the validators added by [`protocheck-proc-macro`](https://docs.rs/protocheck-proc-macro/0.1.0/protocheck_proc_macro/) to the target structs.
pub mod validators;

#[cfg(feature = "cel")]
pub use cel;

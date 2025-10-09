#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![doc = include_str!("../README.md")]

//! # Feature flags
#![doc = document_features::document_features!()]

/// Generated rust code from buf.validate protobuf package, with some added methods and structs.
#[cfg(feature = "protovalidate")]
pub mod protovalidate;

/// Implementations to allow conversion from well known types to [`cel::Value`](::cel::Value)
#[cfg(feature = "cel")]
pub mod cel;

#[cfg(feature = "rpc")]
mod rpc;
#[cfg(feature = "rpc")]
pub use rpc::*;

mod common;
pub use common::*;
pub use protobuf::*;
mod protobuf;
mod protobuf_impls;

/// Implementations and units for Duration structs.
pub mod duration;

pub mod timestamp;

mod any;
mod any_impls;

mod field_mask;

mod field_type;
#[doc(inline)]
pub use field_type::FieldType;

mod empty;

mod constants;
mod conversions;
mod datetime_internal;
mod type_url;

use core::{convert::TryFrom, fmt, time};
use std::str::FromStr;

use prost::{
  alloc::{format, string::String, vec::Vec},
  DecodeError, EncodeError, Message, Name,
};
pub(crate) use type_url::{type_url_for, TypeUrl};

/// The `Status` type defines a logical error model that is suitable for
/// different programming environments, including REST APIs and RPC APIs. It is
/// used by [gRPC](<https://github.com/grpc>). Each `Status` message contains
/// three pieces of data: error code, error message, and error details.
///
/// You can find out more about this error model and how to work with it in the
/// [API Design Guide](<https://cloud.google.com/apis/design/errors>).
#[derive(Clone, PartialEq, ::prost::Message)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Status {
  /// The status code, which should be an enum value of
  /// [google.rpc.Code][google.rpc.Code].
  #[prost(int32, tag = "1")]
  pub code: i32,
  /// A developer-facing error message, which should be in English. Any
  /// user-facing error message should be localized and sent in the
  /// [google.rpc.Status.details][google.rpc.Status.details] field, or localized
  /// by the client.
  #[prost(string, tag = "2")]
  pub message: ::prost::alloc::string::String,
  /// A list of messages that carry the error details.  There is a common set of
  /// message types for APIs to use.
  #[prost(message, repeated, tag = "3")]
  pub details: ::prost::alloc::vec::Vec<crate::protobuf::Any>,
}

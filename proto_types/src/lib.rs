#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]

//! # Feature flags
#![doc = document_features::document_features!()]

/// Generated rust code from buf.validate protobuf package, with some added methods and structs.
#[cfg(feature = "protovalidate")]
pub mod protovalidate;

/// Implementations to allow conversion from well known types to [`cel::Value`](::cel::Value)
#[cfg(feature = "cel")]
pub mod cel;

mod rpc;
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
#[cfg(any(
  feature = "diesel-postgres",
  feature = "diesel-sqlite",
  feature = "diesel-mysql"
))]
mod diesel_impls;

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

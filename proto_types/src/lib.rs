// no_std support is planned at a certain point, but not yet implemented
#![no_std]
#![deny(clippy::alloc_instead_of_core)]
#![deny(clippy::std_instead_of_alloc)]
#![deny(clippy::std_instead_of_core)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]

//! # Feature flags
#![doc = document_features::document_features!()]

#[cfg(any(test, feature = "std"))]
extern crate std;

extern crate alloc;

/// Generated rust code from buf.validate protobuf package, with some added methods and structs.
#[cfg(feature = "protovalidate")]
pub mod protovalidate;

/// Implementations to allow conversion from well known types to [`cel::Value`](::cel::Value)
#[cfg(feature = "cel")]
pub mod cel;

mod rpc;
pub use rpc::*;

pub mod num_wrappers;

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

use alloc::{format, string::String, string::ToString, vec::Vec};
use core::str::FromStr;
use core::{convert::TryFrom, fmt, time};
use core::{
  fmt::{Debug, Display},
  hash::Hash,
};

use prost::{DecodeError, EncodeError, Message, Name};
pub(crate) use type_url::{TypeUrl, type_url_for};

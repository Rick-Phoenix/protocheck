pub mod protovalidate;

#[cfg(feature = "cel")]
pub mod cel;

pub use protobuf::*;
mod protobuf;
mod protobuf_impls;

pub mod duration;
pub use duration::*;

mod timestamp;
pub use timestamp::*;

mod any;
mod any_impls;

mod field_mask;

mod field_type;
pub use field_type::FieldType;

mod empty;

mod conversions;
mod datetime;
mod type_url;

use core::{convert::TryFrom, fmt, time};
use std::str::FromStr;

use prost::{
  alloc::{format, string::String, vec::Vec},
  DecodeError, EncodeError, Message, Name,
};
pub(crate) use type_url::{type_url_for, TypeUrl};

const NANOS_PER_SECOND: i32 = 1_000_000_000;

const NANOS_MAX: i32 = NANOS_PER_SECOND - 1;

const PACKAGE_PREFIX: &str = "google.protobuf";

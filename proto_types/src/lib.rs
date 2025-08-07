mod buf;

pub use protobuf::*;
mod protobuf;
mod protobuf_impls;

mod field_type;
pub use field_type::FieldType;

mod any;
mod any_impls;
mod conversions;
mod datetime;
pub mod protovalidate_impls;
mod type_url;

#[allow(unused_imports)]
pub use empty::*;
mod empty;

use core::{convert::TryFrom, fmt, time};
use std::str::FromStr;

pub use buf::validate as protovalidate;
pub(crate) use proc_macro2::TokenStream as TokenStream2;
use prost::{
  alloc::{format, string::String, vec::Vec},
  DecodeError, EncodeError, Message, Name,
};
pub(crate) use type_url::{type_url_for, TypeUrl};

const NANOS_PER_SECOND: i32 = 1_000_000_000;

const NANOS_MAX: i32 = NANOS_PER_SECOND - 1;

const PACKAGE_PREFIX: &str = "google.protobuf";

pub mod duration;
pub use duration::*;

mod timestamp;
pub use timestamp::*;

pub mod cel;

mod field_mask_impls;

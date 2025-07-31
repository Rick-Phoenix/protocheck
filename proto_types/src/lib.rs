mod buf;

mod protobuf;
mod protobuf_impls;

pub use protobuf::*;
mod any;
mod conversions;
pub mod datetime;
mod type_url;

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

const PACKAGE: &str = "google.protobuf";

mod duration;
mod duration_impls;
pub use duration::DurationError;

mod timestamp;
mod timestamp_impls;
pub use timestamp::TimestampError;

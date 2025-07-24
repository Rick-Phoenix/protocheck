#![allow(clippy::all, dead_code, unused)]
use std::{collections::HashMap, sync::LazyLock};

use bytes::Bytes;
use proc_macro::TokenStream;
pub(crate) use prost_reflect::Kind as ProtoFieldKind;
use prost_reflect::{prost::Message, ExtensionDescriptor, MessageDescriptor, Value};
pub(crate) use proto_types::{protobuf, protovalidate};
pub(crate) use protobuf::field_descriptor_proto::Type as ProtoTypes;
pub(crate) use protocheck_core::{
  field_data::{FieldContext, FieldData},
  validator_template::{GeneratedCodeKind, ValidatorCallTemplate},
};
use protovalidate::{
  field_path_element::Subscript, field_rules, FieldPath, FieldPathElement, FieldRules, Ignore,
  MessageRules, OneofRules, PredefinedRules, Rule,
};
pub(crate) use protovalidate::{
  field_rules, FieldPath, FieldPathElement, FieldRules, Ignore, Violation,
};
use regex::Regex;
use syn::DeriveInput;

pub mod any_rules;
pub mod bool_rules;
pub mod bytes_rules;
pub mod cel_rules;
pub mod core;
pub mod duration_rules;
pub mod enum_rules;
pub mod extract_validators;
pub mod map_rules;
pub mod numeric_rules;
pub mod repeated_rules;
pub mod string_rules;
pub mod timestamp_rules;

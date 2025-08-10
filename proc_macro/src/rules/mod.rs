#[cfg(not(feature = "cel"))]
use proc_macro2::TokenStream;
pub(crate) use proto_types::{field_descriptor_proto::Type as ProtoType, protovalidate};
pub(crate) use protovalidate::{field_rules, Ignore};
use protovalidate::{MessageRules, OneofRules, Rule};
#[cfg(not(feature = "cel"))]
use syn::Error;

#[cfg(not(feature = "cel"))]
use crate::cel_rule_template::CelRuleTemplateTarget;

#[cfg(not(feature = "cel"))]
pub fn get_cel_rules(
  _: &CelRuleTemplateTarget,
  _: &[Rule],
  _: &mut Vec<TokenStream>,
) -> Result<TokenStream, Error> {
  Ok(TokenStream::new())
}

pub mod any_rules;
pub mod bool_rules;
pub mod bytes_rules;
#[cfg(feature = "cel")]
pub mod cel_rules;
pub mod core;
pub mod duration_rules;
pub mod enum_rules;
pub mod extract_validators;
pub mod map_rules;
pub mod numeric_rules;
pub mod repeated_rules;
pub mod special_field_names;
pub mod string_rules;
pub mod timestamp_rules;

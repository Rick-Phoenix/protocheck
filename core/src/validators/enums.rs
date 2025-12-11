use crate::{
  field_data::FieldContext,
  protovalidate::{violations_data::ENUM_DEFINED_ONLY_VIOLATION, Violation},
  validators::static_data::base_violations::create_violation,
};

pub fn defined_only(field_context: &FieldContext, enum_name: &str) -> Violation {
  create_violation(
    field_context,
    &ENUM_DEFINED_ONLY_VIOLATION,
    &format!("must be a defined value of `{enum_name}`"),
  )
}

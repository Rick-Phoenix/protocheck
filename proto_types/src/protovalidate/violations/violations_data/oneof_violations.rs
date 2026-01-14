use super::*;

pub const ONEOF_REQUIRED_VIOLATION: ViolationData = ViolationData {
  name: "oneof.required",
  elements: &[ConstPathElement {
    name: "required",
    field_type: Type::Bool,
    number: 1,
  }],
};

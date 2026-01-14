use super::*;

pub const CEL_VIOLATION: ViolationData = ViolationData {
  name: "cel",
  elements: &[ConstPathElement {
    name: "cel",
    field_type: Type::Message,
    number: 23,
  }],
};

pub const REQUIRED_VIOLATION: ViolationData = ViolationData {
  name: "required",
  elements: &[ConstPathElement {
    name: "required",
    field_type: Type::Bool,
    number: 25,
  }],
};

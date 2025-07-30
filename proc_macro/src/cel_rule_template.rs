use prost_reflect::{FieldDescriptor, MessageDescriptor};

use crate::rules::FieldData;

#[derive(Debug, Clone)]
pub enum CelRuleTemplateTarget {
  Message(MessageDescriptor),
  Field(FieldDescriptor, FieldData),
}

impl CelRuleTemplateTarget {
  pub fn get_validation_type(&self) -> &str {
    match self {
      CelRuleTemplateTarget::Field(_, _) => "field",
      CelRuleTemplateTarget::Message(_) => "message",
    }
  }

  pub fn get_name(&self) -> &str {
    match self {
      CelRuleTemplateTarget::Field(_, field_data) => &field_data.proto_name,
      CelRuleTemplateTarget::Message(message_desc) => message_desc.name(),
    }
  }
}

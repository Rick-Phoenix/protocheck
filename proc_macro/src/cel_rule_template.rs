use prost_reflect::{FieldDescriptor, MessageDescriptor};

use crate::validation_data::ValidationData;

#[derive(Debug, Clone)]
pub enum CelRuleTemplateTarget {
  Message(MessageDescriptor),
  Field {
    field_desc: FieldDescriptor,
    validation_data: ValidationData,
    is_boxed: bool,
  },
}

impl CelRuleTemplateTarget {
  pub fn get_validation_type(&self) -> &str {
    match self {
      CelRuleTemplateTarget::Field { .. } => "field",
      CelRuleTemplateTarget::Message(_) => "message",
    }
  }

  pub fn get_name(&self) -> &str {
    match self {
      CelRuleTemplateTarget::Field {
        validation_data, ..
      } => &validation_data.field_data.proto_name,
      CelRuleTemplateTarget::Message(message_desc) => message_desc.name(),
    }
  }

  pub fn get_full_name(&self) -> String {
    match self {
      CelRuleTemplateTarget::Field { field_desc, .. } => field_desc.full_name().replace(".", "_"),
      CelRuleTemplateTarget::Message(message_desc) => message_desc.full_name().replace(".", "_"),
    }
  }
}

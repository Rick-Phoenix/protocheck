use crate::*;

#[derive(Debug, Clone)]
pub enum CelRuleTemplateTarget<'a> {
  Message {
    message_desc: &'a MessageDescriptor,
    parent_messages_ident: Ident,
    violations_ident: Ident,
  },
  Field {
    field_desc: &'a FieldDescriptor,
    validation_data: &'a ValidationData<'a>,
  },
}

impl CelRuleTemplateTarget<'_> {
  pub fn get_validation_type(&self) -> &str {
    match self {
      CelRuleTemplateTarget::Field { .. } => "field",
      CelRuleTemplateTarget::Message { .. } => "message",
    }
  }

  pub fn get_full_name(&self) -> String {
    match self {
      CelRuleTemplateTarget::Field { field_desc, .. } => field_desc.full_name().replace(".", "_"),
      CelRuleTemplateTarget::Message { message_desc, .. } => {
        message_desc.full_name().replace(".", "_")
      }
    }
  }

  pub fn get_idents(&self) -> (&Ident, &Ident) {
    match self {
      CelRuleTemplateTarget::Field {
        validation_data, ..
      } => (
        validation_data.parent_messages_ident,
        validation_data.violations_ident,
      ),
      CelRuleTemplateTarget::Message {
        parent_messages_ident,
        violations_ident,
        ..
      } => (parent_messages_ident, violations_ident),
    }
  }
}

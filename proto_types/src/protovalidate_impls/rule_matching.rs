use proc_macro2::Span;
use syn::Error;

use crate::{
  field_descriptor_proto::Type as ProtoType,
  protovalidate::{
    field_rules::Type as RulesType, BoolRules, BytesRules, DoubleRules, EnumRules, Fixed32Rules,
    Fixed64Rules, FloatRules, Int32Rules, Int64Rules, SFixed32Rules, SFixed64Rules, SInt32Rules,
    SInt64Rules, StringRules, UInt32Rules, UInt64Rules,
  },
};

impl RulesType {
  pub fn matches_type(
    &self,
    field_type: &ProtoType,
    field_span: Span,
    error_prefix: &str,
  ) -> Result<(), Error> {
    match self {
      RulesType::Float(v) => v.matches_type(field_type, field_span, error_prefix),
      RulesType::Double(v) => v.matches_type(field_type, field_span, error_prefix),
      RulesType::Int32(v) => v.matches_type(field_type, field_span, error_prefix),
      RulesType::Int64(v) => v.matches_type(field_type, field_span, error_prefix),
      RulesType::Uint32(v) => v.matches_type(field_type, field_span, error_prefix),
      RulesType::Uint64(v) => v.matches_type(field_type, field_span, error_prefix),
      RulesType::Sint32(v) => v.matches_type(field_type, field_span, error_prefix),
      RulesType::Sint64(v) => v.matches_type(field_type, field_span, error_prefix),
      RulesType::Fixed32(v) => v.matches_type(field_type, field_span, error_prefix),
      RulesType::Fixed64(v) => v.matches_type(field_type, field_span, error_prefix),
      RulesType::Sfixed32(v) => v.matches_type(field_type, field_span, error_prefix),
      RulesType::Sfixed64(v) => v.matches_type(field_type, field_span, error_prefix),
      RulesType::Bool(v) => v.matches_type(field_type, field_span, error_prefix),
      RulesType::String(v) => v.matches_type(field_type, field_span, error_prefix),
      RulesType::Bytes(v) => v.matches_type(field_type, field_span, error_prefix),
      RulesType::Enum(v) => v.matches_type(field_type, field_span, error_prefix),
      RulesType::Repeated(_) => Ok(()),
      RulesType::Map(_) => Ok(()),
      RulesType::Any(_) => Ok(()),
      RulesType::Duration(_) => Ok(()),
      RulesType::Timestamp(_) => Ok(()),
    }
  }
}

pub trait RuleMatches {
  const RULE_TYPE: ProtoType;
  fn matches_type(
    &self,
    field_type: &ProtoType,
    field_span: Span,
    error_prefix: &str,
  ) -> Result<(), Error>;
}

fn get_wrong_rule_error(
  field_span: Span,
  error_prefix: &str,
  expected: &ProtoType,
  actual: &ProtoType,
) -> Error {
  Error::new(
    field_span,
    format!(
      "{} wrong rule type. Expected {:#?}, got {:#?}",
      error_prefix, expected, actual
    ),
  )
}

impl RuleMatches for FloatRules {
  const RULE_TYPE: ProtoType = ProtoType::Float;
  fn matches_type(
    &self,
    field_type: &ProtoType,
    field_span: Span,
    error_prefix: &str,
  ) -> Result<(), Error> {
    if !matches!(*field_type, Self::RULE_TYPE) {
      Err(get_wrong_rule_error(
        field_span,
        error_prefix,
        &Self::RULE_TYPE,
        field_type,
      ))
    } else {
      Ok(())
    }
  }
}

impl RuleMatches for DoubleRules {
  const RULE_TYPE: ProtoType = ProtoType::Double;
  fn matches_type(
    &self,
    field_type: &ProtoType,
    field_span: Span,
    error_prefix: &str,
  ) -> Result<(), Error> {
    if !matches!(*field_type, Self::RULE_TYPE) {
      Err(get_wrong_rule_error(
        field_span,
        error_prefix,
        &Self::RULE_TYPE,
        field_type,
      ))
    } else {
      Ok(())
    }
  }
}

impl RuleMatches for Int32Rules {
  const RULE_TYPE: ProtoType = ProtoType::Int32;
  fn matches_type(
    &self,
    field_type: &ProtoType,
    field_span: Span,
    error_prefix: &str,
  ) -> Result<(), Error> {
    if !matches!(*field_type, Self::RULE_TYPE) {
      Err(get_wrong_rule_error(
        field_span,
        error_prefix,
        &Self::RULE_TYPE,
        field_type,
      ))
    } else {
      Ok(())
    }
  }
}

impl RuleMatches for Int64Rules {
  const RULE_TYPE: ProtoType = ProtoType::Int64;
  fn matches_type(
    &self,
    field_type: &ProtoType,
    field_span: Span,
    error_prefix: &str,
  ) -> Result<(), Error> {
    if !matches!(*field_type, Self::RULE_TYPE) {
      Err(get_wrong_rule_error(
        field_span,
        error_prefix,
        &Self::RULE_TYPE,
        field_type,
      ))
    } else {
      Ok(())
    }
  }
}

impl RuleMatches for UInt32Rules {
  const RULE_TYPE: ProtoType = ProtoType::Uint32;
  fn matches_type(
    &self,
    field_type: &ProtoType,
    field_span: Span,
    error_prefix: &str,
  ) -> Result<(), Error> {
    if !matches!(*field_type, Self::RULE_TYPE) {
      Err(get_wrong_rule_error(
        field_span,
        error_prefix,
        &Self::RULE_TYPE,
        field_type,
      ))
    } else {
      Ok(())
    }
  }
}

impl RuleMatches for UInt64Rules {
  const RULE_TYPE: ProtoType = ProtoType::Uint64;
  fn matches_type(
    &self,
    field_type: &ProtoType,
    field_span: Span,
    error_prefix: &str,
  ) -> Result<(), Error> {
    if !matches!(*field_type, Self::RULE_TYPE) {
      Err(get_wrong_rule_error(
        field_span,
        error_prefix,
        &Self::RULE_TYPE,
        field_type,
      ))
    } else {
      Ok(())
    }
  }
}

impl RuleMatches for SInt32Rules {
  const RULE_TYPE: ProtoType = ProtoType::Sint32;
  fn matches_type(
    &self,
    field_type: &ProtoType,
    field_span: Span,
    error_prefix: &str,
  ) -> Result<(), Error> {
    if !matches!(*field_type, Self::RULE_TYPE) {
      Err(get_wrong_rule_error(
        field_span,
        error_prefix,
        &Self::RULE_TYPE,
        field_type,
      ))
    } else {
      Ok(())
    }
  }
}

impl RuleMatches for SInt64Rules {
  const RULE_TYPE: ProtoType = ProtoType::Sint64;
  fn matches_type(
    &self,
    field_type: &ProtoType,
    field_span: Span,
    error_prefix: &str,
  ) -> Result<(), Error> {
    if !matches!(*field_type, Self::RULE_TYPE) {
      Err(get_wrong_rule_error(
        field_span,
        error_prefix,
        &Self::RULE_TYPE,
        field_type,
      ))
    } else {
      Ok(())
    }
  }
}

impl RuleMatches for Fixed32Rules {
  const RULE_TYPE: ProtoType = ProtoType::Fixed32;
  fn matches_type(
    &self,
    field_type: &ProtoType,
    field_span: Span,
    error_prefix: &str,
  ) -> Result<(), Error> {
    if !matches!(*field_type, Self::RULE_TYPE) {
      Err(get_wrong_rule_error(
        field_span,
        error_prefix,
        &Self::RULE_TYPE,
        field_type,
      ))
    } else {
      Ok(())
    }
  }
}

impl RuleMatches for Fixed64Rules {
  const RULE_TYPE: ProtoType = ProtoType::Fixed64;
  fn matches_type(
    &self,
    field_type: &ProtoType,
    field_span: Span,
    error_prefix: &str,
  ) -> Result<(), Error> {
    if !matches!(*field_type, Self::RULE_TYPE) {
      Err(get_wrong_rule_error(
        field_span,
        error_prefix,
        &Self::RULE_TYPE,
        field_type,
      ))
    } else {
      Ok(())
    }
  }
}

impl RuleMatches for SFixed32Rules {
  const RULE_TYPE: ProtoType = ProtoType::Sfixed32;
  fn matches_type(
    &self,
    field_type: &ProtoType,
    field_span: Span,
    error_prefix: &str,
  ) -> Result<(), Error> {
    if !matches!(*field_type, Self::RULE_TYPE) {
      Err(get_wrong_rule_error(
        field_span,
        error_prefix,
        &Self::RULE_TYPE,
        field_type,
      ))
    } else {
      Ok(())
    }
  }
}

impl RuleMatches for SFixed64Rules {
  const RULE_TYPE: ProtoType = ProtoType::Sfixed64;
  fn matches_type(
    &self,
    field_type: &ProtoType,
    field_span: Span,
    error_prefix: &str,
  ) -> Result<(), Error> {
    if !matches!(*field_type, Self::RULE_TYPE) {
      Err(get_wrong_rule_error(
        field_span,
        error_prefix,
        &Self::RULE_TYPE,
        field_type,
      ))
    } else {
      Ok(())
    }
  }
}

impl RuleMatches for BoolRules {
  const RULE_TYPE: ProtoType = ProtoType::Bool;
  fn matches_type(
    &self,
    field_type: &ProtoType,
    field_span: Span,
    error_prefix: &str,
  ) -> Result<(), Error> {
    if !matches!(*field_type, Self::RULE_TYPE) {
      Err(get_wrong_rule_error(
        field_span,
        error_prefix,
        &Self::RULE_TYPE,
        field_type,
      ))
    } else {
      Ok(())
    }
  }
}

impl RuleMatches for EnumRules {
  const RULE_TYPE: ProtoType = ProtoType::Enum;
  fn matches_type(
    &self,
    field_type: &ProtoType,
    field_span: Span,
    error_prefix: &str,
  ) -> Result<(), Error> {
    if !matches!(*field_type, Self::RULE_TYPE) {
      Err(get_wrong_rule_error(
        field_span,
        error_prefix,
        &Self::RULE_TYPE,
        field_type,
      ))
    } else {
      Ok(())
    }
  }
}

impl RuleMatches for StringRules {
  const RULE_TYPE: ProtoType = ProtoType::String;
  fn matches_type(
    &self,
    field_type: &ProtoType,
    field_span: Span,
    error_prefix: &str,
  ) -> Result<(), Error> {
    if !matches!(*field_type, Self::RULE_TYPE) {
      Err(get_wrong_rule_error(
        field_span,
        error_prefix,
        &Self::RULE_TYPE,
        field_type,
      ))
    } else {
      Ok(())
    }
  }
}

impl RuleMatches for BytesRules {
  const RULE_TYPE: ProtoType = ProtoType::Bytes;
  fn matches_type(
    &self,
    field_type: &ProtoType,
    field_span: Span,
    error_prefix: &str,
  ) -> Result<(), Error> {
    if !matches!(*field_type, Self::RULE_TYPE) {
      Err(get_wrong_rule_error(
        field_span,
        error_prefix,
        &Self::RULE_TYPE,
        field_type,
      ))
    } else {
      Ok(())
    }
  }
}

use proc_macro2::Span;
use syn::Error;

use crate::{
  protovalidate::{
    field_rules::Type as RulesType, AnyRules, BoolRules, BytesRules, DoubleRules, DurationRules,
    EnumRules, Fixed32Rules, Fixed64Rules, FloatRules, Int32Rules, Int64Rules, SFixed32Rules,
    SFixed64Rules, SInt32Rules, SInt64Rules, StringRules, TimestampRules, UInt32Rules, UInt64Rules,
  },
  FieldType,
};

impl RulesType {
  pub fn matches_type(
    &self,
    field_type: &FieldType,
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
  const RULE_TYPE: FieldType;
  fn matches_type(
    &self,
    field_type: &FieldType,
    field_span: Span,
    error_prefix: &str,
  ) -> Result<(), Error>;
}

fn get_wrong_rule_error(
  field_span: Span,
  error_prefix: &str,
  expected: &FieldType,
  actual: &FieldType,
) -> Error {
  Error::new(
    field_span,
    format!(
      "{} wrong rule type. Expected {:#?}, got {:#?}",
      error_prefix, expected, actual
    ),
  )
}

impl RuleMatches for DurationRules {
  const RULE_TYPE: FieldType = FieldType::Duration;
  fn matches_type(
    &self,
    field_type: &FieldType,
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

impl RuleMatches for TimestampRules {
  const RULE_TYPE: FieldType = FieldType::Timestamp;
  fn matches_type(
    &self,
    field_type: &FieldType,
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

impl RuleMatches for AnyRules {
  const RULE_TYPE: FieldType = FieldType::Any;
  fn matches_type(
    &self,
    field_type: &FieldType,
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

impl RuleMatches for FloatRules {
  const RULE_TYPE: FieldType = FieldType::Float;
  fn matches_type(
    &self,
    field_type: &FieldType,
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
  const RULE_TYPE: FieldType = FieldType::Double;
  fn matches_type(
    &self,
    field_type: &FieldType,
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
  const RULE_TYPE: FieldType = FieldType::Int32;
  fn matches_type(
    &self,
    field_type: &FieldType,
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
  const RULE_TYPE: FieldType = FieldType::Int64;
  fn matches_type(
    &self,
    field_type: &FieldType,
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
  const RULE_TYPE: FieldType = FieldType::Uint32;
  fn matches_type(
    &self,
    field_type: &FieldType,
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
  const RULE_TYPE: FieldType = FieldType::Uint64;
  fn matches_type(
    &self,
    field_type: &FieldType,
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
  const RULE_TYPE: FieldType = FieldType::Sint32;
  fn matches_type(
    &self,
    field_type: &FieldType,
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
  const RULE_TYPE: FieldType = FieldType::Sint64;
  fn matches_type(
    &self,
    field_type: &FieldType,
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
  const RULE_TYPE: FieldType = FieldType::Fixed32;
  fn matches_type(
    &self,
    field_type: &FieldType,
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
  const RULE_TYPE: FieldType = FieldType::Fixed64;
  fn matches_type(
    &self,
    field_type: &FieldType,
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
  const RULE_TYPE: FieldType = FieldType::Sfixed32;
  fn matches_type(
    &self,
    field_type: &FieldType,
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
  const RULE_TYPE: FieldType = FieldType::Sfixed64;
  fn matches_type(
    &self,
    field_type: &FieldType,
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
  const RULE_TYPE: FieldType = FieldType::Bool;
  fn matches_type(
    &self,
    field_type: &FieldType,
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
  const RULE_TYPE: FieldType = FieldType::Enum;
  fn matches_type(
    &self,
    field_type: &FieldType,
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
  const RULE_TYPE: FieldType = FieldType::String;
  fn matches_type(
    &self,
    field_type: &FieldType,
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
  const RULE_TYPE: FieldType = FieldType::Bytes;
  fn matches_type(
    &self,
    field_type: &FieldType,
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

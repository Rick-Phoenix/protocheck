mod comparable_rules;
mod containing_rules;
mod into_comparable;
mod numeric_rules;
mod rule_matching;
mod violations;

use std::fmt::{self, Display};

pub use comparable_rules::{
  ComparableGreaterThan, ComparableLessThan, ComparableRules, LengthRules,
};
pub use containing_rules::ContainingRules;
pub use numeric_rules::NumericRules;

use crate::protovalidate::field_path_element::Subscript;

impl Display for Subscript {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Subscript::Index(val) => write!(f, "{}", val),
      Subscript::BoolKey(val) => write!(f, "{}", val),
      Subscript::IntKey(val) => write!(f, "{}", val),
      Subscript::UintKey(val) => write!(f, "{}", val),
      Subscript::StringKey(val) => write!(f, "{}", val),
    }
  }
}

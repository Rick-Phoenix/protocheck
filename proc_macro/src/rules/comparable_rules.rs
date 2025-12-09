use crate::*;

pub enum ComparableLessThan<T: ComparableError> {
  Lt(T),
  Lte(T),
}

impl<T> ComparableLessThan<T>
where
  T: Copy + ComparableError,
{
  pub fn value(&self) -> T {
    match self {
      Self::Lte(val) => *val,
      Self::Lt(val) => *val,
    }
  }

  pub fn error_message(&self) -> String {
    let value_str = self.value().string_representation();
    let smaller_adjective = T::smaller_adjective();

    match self {
      ComparableLessThan::Lt(_) => format!("must be {smaller_adjective} than {value_str}"),
      ComparableLessThan::Lte(_) => {
        format!("must be {smaller_adjective} than or equal to {value_str}")
      }
    }
  }
}

impl<T> ComparableGreaterThan<T>
where
  T: Copy + ComparableError,
{
  pub fn value(&self) -> T {
    match self {
      Self::Gte(val) => *val,
      Self::Gt(val) => *val,
    }
  }

  pub fn error_message(&self) -> String {
    let value_str = self.value().string_representation();
    let greater_adjective = T::greater_adjective();

    match self {
      ComparableGreaterThan::Gt(_) => format!("must be {greater_adjective} than {value_str}"),
      ComparableGreaterThan::Gte(_) => {
        format!("must be {greater_adjective} than or equal to {value_str}")
      }
    }
  }
}

pub enum ComparableGreaterThan<T: ComparableError> {
  Gt(T),
  Gte(T),
}

impl ComparableError for Timestamp {
  fn greater_adjective() -> &'static str {
    "later"
  }

  fn smaller_adjective() -> &'static str {
    "earlier"
  }

  fn string_representation(&self) -> String {
    self
      .format("%d %b %Y %R %Z")
      .unwrap_or_else(|_| self.to_string())
  }
}

impl ComparableError for Duration {
  fn greater_adjective() -> &'static str {
    "longer"
  }

  fn smaller_adjective() -> &'static str {
    "shorter"
  }
}

pub trait ComparableError: Display {
  fn smaller_adjective() -> &'static str {
    "less"
  }

  fn greater_adjective() -> &'static str {
    "greater"
  }

  fn string_representation(&self) -> String {
    self.to_string()
  }
}

pub struct ComparableRules<T>
where
  T: PartialOrd + PartialEq + ComparableError,
{
  pub less_than: Option<ComparableLessThan<T>>,
  pub greater_than: Option<ComparableGreaterThan<T>>,
}

impl<T> ComparableRules<T>
where
  T: PartialOrd + PartialEq + ComparableError,
{
  pub fn validate(self) -> Result<Self, &'static str> {
    if let Some(ref gt_rule) = self.greater_than
      && let Some(ref lt_rule) = self.less_than {
        match gt_rule {
          ComparableGreaterThan::Gte (gte_val) => {
            match lt_rule {
              ComparableLessThan::Lte (lte_val) => {
                if lte_val < gte_val {
                  return Err("Lte cannot be smaller than Gte");
                }
              }
              ComparableLessThan::Lt (lt_val) => {
                if lt_val <= gte_val {
                  return Err("Lt cannot be smaller than Gte");
                }
              }
            };
          }
          ComparableGreaterThan::Gt (gt_val) => {
            match lt_rule {
              ComparableLessThan::Lte (lte_val) => {
                if lte_val <= gt_val {
                  return Err("Lte cannot be smaller than or equal to Gt");
                }
              }
              ComparableLessThan::Lt (lt_val) => {
                if lt_val <= gt_val {
                  return Err("Lt cannot be smaller than or equal to Gt");
                }
              }
            };
          }
        };
      }
    Ok(self)
  }
}

use crate::timestamp_rules::{GreaterThan as TimestampGreaterThan, LessThan as TimestampLessThan};

pub trait RulesWithComparables<T>
where
  T: PartialOrd + PartialEq + ComparableError,
{
  fn comparable_rules(&self) -> ComparableRules<T>;
}

impl RulesWithComparables<Duration> for DurationRules {
  fn comparable_rules(&self) -> ComparableRules<Duration> {
    let less_than = self.less_than.as_ref().map(|less_than| match less_than {
      duration_rules::LessThan::Lt(v) => ComparableLessThan::Lt(*v),
      duration_rules::LessThan::Lte(v) => ComparableLessThan::Lte(*v),
    });

    let greater_than = self
      .greater_than
      .as_ref()
      .map(|greater_than| match greater_than {
        duration_rules::GreaterThan::Gt(v) => ComparableGreaterThan::Gt(*v),
        duration_rules::GreaterThan::Gte(v) => ComparableGreaterThan::Gte(*v),
      });

    ComparableRules {
      less_than,
      greater_than,
    }
  }
}

impl RulesWithComparables<Timestamp> for TimestampRules {
  fn comparable_rules(&self) -> ComparableRules<Timestamp> {
    let mut greater_than: Option<ComparableGreaterThan<Timestamp>> = None;

    if let Some(gt_rule) = self.greater_than {
      greater_than = match gt_rule {
        TimestampGreaterThan::Gt(v) => Some(ComparableGreaterThan::Gt(v)),

        TimestampGreaterThan::Gte(v) => Some(ComparableGreaterThan::Gte(v)),
        _ => None,
      }
    }

    let mut less_than: Option<ComparableLessThan<Timestamp>> = None;

    if let Some(gt_rule) = self.less_than {
      less_than = match gt_rule {
        TimestampLessThan::Lt(v) => Some(ComparableLessThan::Lt(v)),

        TimestampLessThan::Lte(v) => Some(ComparableLessThan::Lte(v)),
        _ => None,
      }
    }

    ComparableRules {
      less_than,
      greater_than,
    }
  }
}

impl ComparableError for i32 {}
impl ComparableError for i64 {}
impl ComparableError for u32 {}
impl ComparableError for u64 {}
impl ComparableError for f32 {}
impl ComparableError for f64 {}

macro_rules! impl_comparable {
  ($target_type:ty, $rule_type:ident) => {
    paste::paste! {
      impl RulesWithComparables<$target_type> for [< $rule_type Rules >] {
        fn comparable_rules(&self) -> ComparableRules<$target_type> {
          let less_than = self.less_than.as_ref().map(|less_than| match less_than {
            [< $rule_type:snake _rules >]::LessThan::Lt(v) => ComparableLessThan::Lt(*v),
            [< $rule_type:snake _rules >]::LessThan::Lte(v) => ComparableLessThan::Lte(*v),
          });

          let greater_than = self
          .greater_than
          .as_ref()
          .map(|greater_than| match greater_than {
            [< $rule_type:snake _rules >]::GreaterThan::Gt(v) => ComparableGreaterThan::Gt(*v),
            [< $rule_type:snake _rules >]::GreaterThan::Gte(v) => ComparableGreaterThan::Gte(*v),
          });

          ComparableRules {
            less_than,
            greater_than,
          }
        }
      }
    }
  };
}

impl_comparable!(f32, Float);
impl_comparable!(f64, Double);
impl_comparable!(i64, Int64);
impl_comparable!(i32, Int32);
impl_comparable!(i64, SInt64);
impl_comparable!(i32, SInt32);
impl_comparable!(i64, SFixed64);
impl_comparable!(i32, SFixed32);
impl_comparable!(u64, UInt64);
impl_comparable!(u32, UInt32);
impl_comparable!(u64, Fixed64);
impl_comparable!(u32, Fixed32);

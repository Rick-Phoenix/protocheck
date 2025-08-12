use std::fmt::Debug;

use quote::ToTokens;

use crate::Timestamp;

pub struct TimestampComparableRules {
  pub comparable_rules: ComparableRules<Timestamp>,
  pub lt_now: bool,
  pub gt_now: bool,
}

pub enum ComparableLessThan<T> {
  Lt { val: T, error_message: String },
  Lte { val: T, error_message: String },
}

impl<T> ComparableLessThan<T>
where
  T: Copy,
{
  pub fn value(&self) -> T {
    match self {
      Self::Lte { val, .. } => *val,
      Self::Lt { val, .. } => *val,
    }
  }
}

impl<T> ComparableGreaterThan<T>
where
  T: Copy,
{
  pub fn value(&self) -> T {
    match self {
      Self::Gte { val, .. } => *val,
      Self::Gt { val, .. } => *val,
    }
  }
}

pub enum ComparableGreaterThan<T> {
  Gt { val: T, error_message: String },
  Gte { val: T, error_message: String },
}

pub struct ComparableRules<T>
where
  T: PartialOrd + PartialEq + Debug + ToTokens,
{
  pub less_than: Option<ComparableLessThan<T>>,
  pub greater_than: Option<ComparableGreaterThan<T>>,
}

impl ComparableRules<Timestamp> {}

impl<T> ComparableRules<T>
where
  T: PartialOrd + PartialEq + Debug + ToTokens,
{
  pub fn validate(self) -> Result<Self, &'static str> {
    if let Some(ref gt_rule) = self.greater_than
      && let Some(ref lt_rule) = self.less_than {
        match gt_rule {
          ComparableGreaterThan::Gte { val: gte_val,.. } => {
            match lt_rule {
              ComparableLessThan::Lte { val:lte_val,.. } => {
                if lte_val < gte_val {
                  return Err("Lte cannot be smaller than Gte");
                }
              }
              ComparableLessThan::Lt { val:lt_val,.. } => {
                if lt_val <= gte_val {
                  return Err("Lt cannot be smaller than Gte");
                }
              }
            };
          }
          ComparableGreaterThan::Gt { val: gt_val, .. } => {
            match lt_rule {
              ComparableLessThan::Lte { val: lte_val, .. } => {
                if lte_val <= gt_val {
                  return Err("Lte cannot be smaller than or equal to Gt");
                }
              }
              ComparableLessThan::Lt { val: lt_val, .. } => {
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

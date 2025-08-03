use crate::{
  protovalidate::{
    DoubleRules, Fixed32Rules, Fixed64Rules, FloatRules, Int32Rules, Int64Rules, SFixed32Rules,
    SFixed64Rules, SInt32Rules, SInt64Rules, UInt32Rules, UInt64Rules,
  },
  protovalidate_impls::into_numeric::{IntoNumeric, NumericGreaterThan, NumericLessThan},
};

pub mod into_numeric;

pub trait NumericRules {
  type Unit;
  fn constant(&self) -> Option<Self::Unit>;
  fn not_in_list(&self) -> Vec<Self::Unit>;
  fn in_list(&self) -> Vec<Self::Unit>;
  fn finite(&self) -> bool;
  fn greater_than(&self) -> Option<NumericGreaterThan<Self::Unit>>;
  fn less_than(&self) -> Option<NumericLessThan<Self::Unit>>;
}

impl NumericRules for FloatRules {
  type Unit = f32;
  fn constant(&self) -> Option<Self::Unit> {
    self.r#const
  }
  fn finite(&self) -> bool {
    self.finite()
  }
  fn greater_than(&self) -> Option<NumericGreaterThan<Self::Unit>> {
    self.greater_than.map(|r| r.into_numeric())
  }
  fn less_than(&self) -> Option<NumericLessThan<Self::Unit>> {
    self.less_than.map(|r| r.into_numeric())
  }
  fn not_in_list(&self) -> Vec<Self::Unit> {
    self.not_in.clone()
  }
  fn in_list(&self) -> Vec<Self::Unit> {
    self.r#in.clone()
  }
}

impl NumericRules for DoubleRules {
  type Unit = f64;
  fn constant(&self) -> Option<Self::Unit> {
    self.r#const
  }
  fn finite(&self) -> bool {
    self.finite()
  }
  fn greater_than(&self) -> Option<NumericGreaterThan<Self::Unit>> {
    self.greater_than.map(|r| r.into_numeric())
  }
  fn less_than(&self) -> Option<NumericLessThan<Self::Unit>> {
    self.less_than.map(|r| r.into_numeric())
  }
  fn not_in_list(&self) -> Vec<Self::Unit> {
    self.not_in.clone()
  }
  fn in_list(&self) -> Vec<Self::Unit> {
    self.r#in.clone()
  }
}

impl NumericRules for Int64Rules {
  type Unit = i64;
  fn constant(&self) -> Option<Self::Unit> {
    self.r#const
  }
  fn finite(&self) -> bool {
    false
  }
  fn greater_than(&self) -> Option<NumericGreaterThan<Self::Unit>> {
    self.greater_than.map(|r| r.into_numeric())
  }
  fn less_than(&self) -> Option<NumericLessThan<Self::Unit>> {
    self.less_than.map(|r| r.into_numeric())
  }
  fn not_in_list(&self) -> Vec<Self::Unit> {
    self.not_in.clone()
  }
  fn in_list(&self) -> Vec<Self::Unit> {
    self.r#in.clone()
  }
}

impl NumericRules for SInt64Rules {
  type Unit = i64;
  fn constant(&self) -> Option<Self::Unit> {
    self.r#const
  }
  fn finite(&self) -> bool {
    false
  }
  fn greater_than(&self) -> Option<NumericGreaterThan<Self::Unit>> {
    self.greater_than.map(|r| r.into_numeric())
  }
  fn less_than(&self) -> Option<NumericLessThan<Self::Unit>> {
    self.less_than.map(|r| r.into_numeric())
  }
  fn not_in_list(&self) -> Vec<Self::Unit> {
    self.not_in.clone()
  }
  fn in_list(&self) -> Vec<Self::Unit> {
    self.r#in.clone()
  }
}

impl NumericRules for SFixed64Rules {
  type Unit = i64;
  fn constant(&self) -> Option<Self::Unit> {
    self.r#const
  }
  fn finite(&self) -> bool {
    false
  }
  fn greater_than(&self) -> Option<NumericGreaterThan<Self::Unit>> {
    self.greater_than.map(|r| r.into_numeric())
  }
  fn less_than(&self) -> Option<NumericLessThan<Self::Unit>> {
    self.less_than.map(|r| r.into_numeric())
  }
  fn not_in_list(&self) -> Vec<Self::Unit> {
    self.not_in.clone()
  }
  fn in_list(&self) -> Vec<Self::Unit> {
    self.r#in.clone()
  }
}

impl NumericRules for Int32Rules {
  type Unit = i32;
  fn constant(&self) -> Option<Self::Unit> {
    self.r#const
  }
  fn finite(&self) -> bool {
    false
  }
  fn greater_than(&self) -> Option<NumericGreaterThan<Self::Unit>> {
    self.greater_than.map(|r| r.into_numeric())
  }
  fn less_than(&self) -> Option<NumericLessThan<Self::Unit>> {
    self.less_than.map(|r| r.into_numeric())
  }
  fn not_in_list(&self) -> Vec<Self::Unit> {
    self.not_in.clone()
  }
  fn in_list(&self) -> Vec<Self::Unit> {
    self.r#in.clone()
  }
}

impl NumericRules for SInt32Rules {
  type Unit = i32;
  fn constant(&self) -> Option<Self::Unit> {
    self.r#const
  }
  fn finite(&self) -> bool {
    false
  }
  fn greater_than(&self) -> Option<NumericGreaterThan<Self::Unit>> {
    self.greater_than.map(|r| r.into_numeric())
  }
  fn less_than(&self) -> Option<NumericLessThan<Self::Unit>> {
    self.less_than.map(|r| r.into_numeric())
  }
  fn not_in_list(&self) -> Vec<Self::Unit> {
    self.not_in.clone()
  }
  fn in_list(&self) -> Vec<Self::Unit> {
    self.r#in.clone()
  }
}

impl NumericRules for SFixed32Rules {
  type Unit = i32;
  fn constant(&self) -> Option<Self::Unit> {
    self.r#const
  }
  fn finite(&self) -> bool {
    false
  }
  fn greater_than(&self) -> Option<NumericGreaterThan<Self::Unit>> {
    self.greater_than.map(|r| r.into_numeric())
  }
  fn less_than(&self) -> Option<NumericLessThan<Self::Unit>> {
    self.less_than.map(|r| r.into_numeric())
  }
  fn not_in_list(&self) -> Vec<Self::Unit> {
    self.not_in.clone()
  }
  fn in_list(&self) -> Vec<Self::Unit> {
    self.r#in.clone()
  }
}

impl NumericRules for UInt64Rules {
  type Unit = u64;
  fn constant(&self) -> Option<Self::Unit> {
    self.r#const
  }
  fn finite(&self) -> bool {
    false
  }
  fn greater_than(&self) -> Option<NumericGreaterThan<Self::Unit>> {
    self.greater_than.map(|r| r.into_numeric())
  }
  fn less_than(&self) -> Option<NumericLessThan<Self::Unit>> {
    self.less_than.map(|r| r.into_numeric())
  }
  fn not_in_list(&self) -> Vec<Self::Unit> {
    self.not_in.clone()
  }
  fn in_list(&self) -> Vec<Self::Unit> {
    self.r#in.clone()
  }
}

impl NumericRules for Fixed64Rules {
  type Unit = u64;
  fn constant(&self) -> Option<Self::Unit> {
    self.r#const
  }
  fn finite(&self) -> bool {
    false
  }
  fn greater_than(&self) -> Option<NumericGreaterThan<Self::Unit>> {
    self.greater_than.map(|r| r.into_numeric())
  }
  fn less_than(&self) -> Option<NumericLessThan<Self::Unit>> {
    self.less_than.map(|r| r.into_numeric())
  }
  fn not_in_list(&self) -> Vec<Self::Unit> {
    self.not_in.clone()
  }
  fn in_list(&self) -> Vec<Self::Unit> {
    self.r#in.clone()
  }
}

impl NumericRules for UInt32Rules {
  type Unit = u32;
  fn constant(&self) -> Option<Self::Unit> {
    self.r#const
  }
  fn finite(&self) -> bool {
    false
  }
  fn greater_than(&self) -> Option<NumericGreaterThan<Self::Unit>> {
    self.greater_than.map(|r| r.into_numeric())
  }
  fn less_than(&self) -> Option<NumericLessThan<Self::Unit>> {
    self.less_than.map(|r| r.into_numeric())
  }
  fn not_in_list(&self) -> Vec<Self::Unit> {
    self.not_in.clone()
  }
  fn in_list(&self) -> Vec<Self::Unit> {
    self.r#in.clone()
  }
}

impl NumericRules for Fixed32Rules {
  type Unit = u32;
  fn constant(&self) -> Option<Self::Unit> {
    self.r#const
  }
  fn finite(&self) -> bool {
    false
  }
  fn greater_than(&self) -> Option<NumericGreaterThan<Self::Unit>> {
    self.greater_than.map(|r| r.into_numeric())
  }
  fn less_than(&self) -> Option<NumericLessThan<Self::Unit>> {
    self.less_than.map(|r| r.into_numeric())
  }
  fn not_in_list(&self) -> Vec<Self::Unit> {
    self.not_in.clone()
  }
  fn in_list(&self) -> Vec<Self::Unit> {
    self.r#in.clone()
  }
}

use proc_macro2::Span;
use quote::ToTokens;
use syn::LitByteStr;

use crate::{
  protovalidate::{
    BoolRules, BytesRules, DoubleRules, DurationRules, EnumRules, Fixed32Rules, Fixed64Rules,
    FloatRules, Int32Rules, Int64Rules, SFixed32Rules, SFixed64Rules, SInt32Rules, SInt64Rules,
    StringRules, TimestampRules, UInt32Rules, UInt64Rules,
  },
  Duration, Timestamp,
};

pub struct ConstRule<T>
where
  T: ToTokens,
{
  pub val: T,
  pub error_message: String,
}

macro_rules! const_rule {
  ($rule_type:ty, $target_type:ty) => {
    impl $rule_type {
      pub fn const_rule(&self) -> Option<ConstRule<$target_type>> {
        self.r#const.as_ref().map(|v| ConstRule {
          val: v.clone(),
          error_message: format!("must be equal to {:?}", v),
        })
      }
    }
  };
}

impl BytesRules {
  pub fn const_rule(&self) -> Option<ConstRule<LitByteStr>> {
    self.r#const.as_ref().map(|v| ConstRule {
      val: LitByteStr::new(v, Span::call_site()),
      error_message: format!("must be equal to {:?}", v),
    })
  }
}

const_rule!(DurationRules, Duration);
const_rule!(TimestampRules, Timestamp);
const_rule!(StringRules, String);
const_rule!(BoolRules, bool);
const_rule!(EnumRules, i32);
const_rule!(FloatRules, f32);
const_rule!(DoubleRules, f64);
const_rule!(Int64Rules, i64);
const_rule!(SInt64Rules, i64);
const_rule!(SFixed64Rules, i64);
const_rule!(Fixed64Rules, u64);
const_rule!(UInt64Rules, u64);
const_rule!(Int32Rules, i32);
const_rule!(SInt32Rules, i32);
const_rule!(SFixed32Rules, i32);
const_rule!(Fixed32Rules, u32);
const_rule!(UInt32Rules, u32);

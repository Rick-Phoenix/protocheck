use crate::*;

pub struct ConstRule<T>
where
  T: ToTokens,
{
  pub val: T,
  pub error_message: String,
}

pub trait RuleWithConst<T: ToTokens> {
  fn const_rule(&self) -> Option<ConstRule<T>>;
}

macro_rules! const_rule {
  ($rule_type:ty, $target_type:ty) => {
    paste::paste! {
      impl RuleWithConst<$target_type> for $rule_type {
        fn const_rule(&self) -> Option<ConstRule<$target_type>> {
          self.r#const.as_ref().map(|v| ConstRule {
            val: v.clone(),
            error_message: format!("must be equal to {v}")
          })
        }
      }
    }
  };
}

impl RuleWithConst<TokenStream2> for BytesRules {
  fn const_rule(&self) -> Option<ConstRule<TokenStream2>> {
    self.r#const.as_ref().map(|v| ConstRule {
      val: {
        let byte_str = LitByteStr::new(v, Span::call_site());
        // Need to assert it as a slice because Bytes does not have PartialEq with arrays
        quote! { &#byte_str[..] }
      },
      error_message: format!("must be equal to {}", v.escape_ascii()),
    })
  }
}

#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Copy)]
pub struct DurationTokens(pub Duration);

impl Display for DurationTokens {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl ToTokens for DurationTokens {
  fn to_tokens(&self, tokens: &mut TokenStream2) {
    let Duration { nanos, seconds } = self.0;
    tokens.extend(quote! {
      ::protocheck::types::Duration {
        seconds: #seconds,
        nanos: #nanos
      }
    });
  }
}

#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Copy)]
pub struct TimestampTokens(pub Timestamp);

impl Display for TimestampTokens {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl ToTokens for TimestampTokens {
  fn to_tokens(&self, tokens: &mut TokenStream2) {
    let Timestamp { nanos, seconds } = self.0;
    tokens.extend(quote! {
      ::protocheck::types::Timestamp {
        seconds: #seconds,
        nanos: #nanos
      }
    });
  }
}

impl RuleWithConst<DurationTokens> for DurationRules {
  fn const_rule(&self) -> Option<ConstRule<DurationTokens>> {
    self.r#const.as_ref().map(|v| ConstRule {
      val: DurationTokens(*v),
      error_message: format!("must be equal to {v}"),
    })
  }
}

impl RuleWithConst<TimestampTokens> for TimestampRules {
  fn const_rule(&self) -> Option<ConstRule<TimestampTokens>> {
    self.r#const.as_ref().map(|v| ConstRule {
      val: TimestampTokens(*v),
      error_message: format!("must be equal to {v}"),
    })
  }
}

const_rule!(StringRules, String);
const_rule!(BoolRules, bool);
const_rule!(EnumRules, i32);
const_rule!(FloatRules, f32);
const_rule!(DoubleRules, f64);
const_rule!(Int64Rules, i64);
const_rule!(Int32Rules, i32);
const_rule!(SInt64Rules, i64);
const_rule!(SInt32Rules, i32);
const_rule!(SFixed64Rules, i64);
const_rule!(SFixed32Rules, i32);
const_rule!(Fixed64Rules, u64);
const_rule!(Fixed32Rules, u32);
const_rule!(UInt64Rules, u64);
const_rule!(UInt32Rules, u32);

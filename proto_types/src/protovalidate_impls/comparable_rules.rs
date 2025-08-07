use std::fmt::Debug;

use proc_macro2::Span;
use quote::ToTokens;
use syn::Error;

use crate::protovalidate::{BytesRules, StringRules};

pub enum ComparableLessThan<T> {
  Lt(T),
  Lte(T),
}

impl<T> ComparableLessThan<T>
where
  T: Copy,
{
  pub fn value(&self) -> T {
    match self {
      Self::Lte(v) => *v,
      Self::Lt(v) => *v,
    }
  }
}

impl<T> ComparableGreaterThan<T>
where
  T: Copy,
{
  pub fn value(&self) -> T {
    match self {
      Self::Gte(v) => *v,
      Self::Gt(v) => *v,
    }
  }
}

pub enum ComparableGreaterThan<T> {
  Gt(T),
  Gte(T),
}

pub struct LengthRules {
  pub len: Option<u64>,
  pub min_len: Option<u64>,
  pub max_len: Option<u64>,
}

impl BytesRules {
  pub fn length_rules(&self, field_span: Span, error_prefix: &str) -> Result<LengthRules, Error> {
    let len = self.len;
    let min_len = self.min_len;
    let max_len = self.max_len;

    if len.is_some() && (min_len.is_some() || max_len.is_some()) {
      return Err(syn::Error::new(
        field_span,
        format!(
          "{} len cannot be used with min_len or max_len",
          error_prefix,
        ),
      ));
    }

    if let Some(min) = min_len
      && let Some(max) = max_len
        && min > max {
          return Err(syn::Error::new(
            field_span,
            format!("{} min_len cannot be larger than max_len", error_prefix,),
          ));
        }

    Ok(LengthRules {
      len,
      min_len,
      max_len,
    })
  }
}

impl StringRules {
  pub fn length_rules(&self, field_span: Span, error_prefix: &str) -> Result<LengthRules, Error> {
    let len = self.len;
    let min_len = self.min_len;
    let max_len = self.max_len;

    if len.is_some() && (min_len.is_some() || max_len.is_some()) {
      return Err(syn::Error::new(
        field_span,
        format!(
          "{} len cannot be used with min_len or max_len",
          error_prefix,
        ),
      ));
    }

    if let Some(min) = min_len
      && let Some(max) = max_len
        && min > max {
          return Err(syn::Error::new(
            field_span,
            format!("{} min_len cannot be larger than max_len", error_prefix,),
          ));
        }

    Ok(LengthRules {
      len,
      min_len,
      max_len,
    })
  }

  pub fn bytes_length_rules(
    &self,
    field_span: Span,
    error_prefix: &str,
  ) -> Result<LengthRules, Error> {
    let len = self.len_bytes;
    let min_len = self.min_bytes;
    let max_len = self.max_bytes;

    if len.is_some() && (min_len.is_some() || max_len.is_some()) {
      return Err(syn::Error::new(
        field_span,
        format!(
          "{} len_bytes cannot be used with min_bytes or max_bytes",
          error_prefix,
        ),
      ));
    }

    if let Some(min) = min_len
      && let Some(max) = max_len
        && min > max {
          return Err(syn::Error::new(
            field_span,
            format!("{} min_bytes cannot be larger than max_bytes", error_prefix,),
          ));
        }

    Ok(LengthRules {
      len,
      min_len,
      max_len,
    })
  }
}

pub struct ComparableRules<T>
where
  T: PartialOrd + PartialEq + Debug + ToTokens,
{
  pub less_than: Option<ComparableLessThan<T>>,
  pub greater_than: Option<ComparableGreaterThan<T>>,
}

impl<T> ComparableRules<T>
where
  T: PartialOrd + PartialEq + Debug + ToTokens,
{
  pub fn validate(self, field_span: Span, error_prefix: &str) -> Result<Self, Error> {
    if let Some(ref gt_rule) = self.greater_than
      && let Some(ref lt_rule) = self.less_than {
        match gt_rule {
          ComparableGreaterThan::Gte(gte_val) => {
            match lt_rule {
              ComparableLessThan::Lte(lte_val) => {
                if lte_val < gte_val {
                  return Err(Error::new(
                    field_span,
                    format!("{} Lte cannot be smaller than Gte", error_prefix),
                  ));
                }
              }
              ComparableLessThan::Lt(lt_val) => {
                if lt_val <= gte_val {
                  return Err(Error::new(
                    field_span,
                    format!("{} Lt cannot be smaller than Gte", error_prefix),
                  ));
                }
              }
            };
          }
          ComparableGreaterThan::Gt(gt_val) => {
            match lt_rule {
              ComparableLessThan::Lte(lte_val) => {
                if lte_val <= gt_val {
                  return Err(Error::new(
                    field_span,
                    format!("{} Lte cannot be smaller than or equal to Gt", error_prefix),
                  ));
                }
              }
              ComparableLessThan::Lt(lt_val) => {
                if lt_val <= gt_val {
                  return Err(Error::new(
                    field_span,
                    format!("{} Lt cannot be smaller than or equal to Gt", error_prefix),
                  ));
                }
              }
            };
          }
        };
      }
    Ok(self)
  }
}

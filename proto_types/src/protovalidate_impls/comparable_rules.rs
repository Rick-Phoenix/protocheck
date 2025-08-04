use std::fmt::Debug;

use proc_macro2::Span;
use quote::ToTokens;
use syn::Error;

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

pub fn validate_len(
  len: Option<u64>,
  min_len: Option<u64>,
  max_len: Option<u64>,
  error_prefix: &str,
  is_bytes: bool,
  field_span: Span,
) -> Result<(), Error> {
  let (len_name, min_name, max_name) = if is_bytes {
    ("len_bytes", "min_bytes", "max_bytes")
  } else {
    ("len", "min_len", "max_len")
  };

  if len.is_some() && (min_len.is_some() || max_len.is_some()) {
    return Err(syn::Error::new(
      field_span,
      format!(
        "{} {} cannot be used with {} or {}",
        error_prefix, len_name, min_name, max_name
      ),
    ));
  }

  if let Some(min) = min_len {
    if let Some(max) = max_len {
      if min > max {
        return Err(syn::Error::new(
          field_span,
          format!(
            "{} {} cannot be larger than {}",
            error_prefix, min_name, max_name
          ),
        ));
      }
    }
  }

  Ok(())
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
    if let Some(ref gt_rule) = self.greater_than {
      if let Some(ref lt_rule) = self.less_than {
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
    }
    Ok(self)
  }
}

use std::{
  collections::HashSet,
  fmt::{Debug, Display},
  hash::Hash,
};

use itertools::Itertools;
use proc_macro2::Span;
use quote::ToTokens;
use syn::{Error, LitByteStr};

use crate::{
  protovalidate::{AnyRules, BytesRules, DurationRules, EnumRules, StringRules},
  Duration,
};

pub struct ContainingRules<T>
where
  T: Debug + ToTokens + Eq + Hash,
{
  pub in_list: Option<(HashSet<T>, String)>,
  pub not_in_list: Option<(HashSet<T>, String)>,
}

impl DurationRules {
  pub fn containing_rules(
    &self,
    field_span: Span,
    error_prefix: &str,
  ) -> Result<ContainingRules<Duration>, Error> {
    let (in_list, not_in_list) = validate_lists(&self.r#in, &self.not_in, true)
      .map_err(|invalid_items| invalid_lists_error(field_span, error_prefix, &invalid_items))?;

    Ok(ContainingRules {
      in_list,
      not_in_list,
    })
  }
}

impl AnyRules {
  pub fn containing_rules(
    &self,
    field_span: Span,
    error_prefix: &str,
  ) -> Result<ContainingRules<String>, Error> {
    let (in_list, not_in_list) = validate_lists(&self.r#in, &self.not_in, true)
      .map_err(|invalid_items| invalid_lists_error(field_span, error_prefix, &invalid_items))?;

    Ok(ContainingRules {
      in_list,
      not_in_list,
    })
  }
}

impl EnumRules {
  pub fn containing_rules(
    &self,
    field_span: Span,
    error_prefix: &str,
  ) -> Result<ContainingRules<i32>, Error> {
    let (in_list, not_in_list) = validate_lists(&self.r#in, &self.not_in, false)
      .map_err(|invalid_items| invalid_lists_error(field_span, error_prefix, &invalid_items))?;

    Ok(ContainingRules {
      in_list,
      not_in_list,
    })
  }
}

impl StringRules {
  pub fn containing_rules(
    &self,
    field_span: Span,
    error_prefix: &str,
  ) -> Result<ContainingRules<String>, Error> {
    let (in_list, not_in_list) = validate_lists(&self.r#in, &self.not_in, true)
      .map_err(|invalid_items| invalid_lists_error(field_span, error_prefix, &invalid_items))?;

    Ok(ContainingRules {
      in_list,
      not_in_list,
    })
  }
}

impl BytesRules {
  pub fn containing_rules(
    &self,
    field_span: Span,
    error_prefix: &str,
  ) -> Result<ContainingRules<LitByteStr>, Error> {
    let in_list_hashset: HashSet<LitByteStr> = self
      .r#in
      .iter()
      .map(|b| LitByteStr::new(b, Span::call_site()))
      .collect();

    let not_in_list_hashset: HashSet<LitByteStr> = self
      .not_in
      .iter()
      .map(|b| LitByteStr::new(b, Span::call_site()))
      .collect();

    let invalid_items: Vec<LitByteStr> = in_list_hashset
      .intersection(&not_in_list_hashset)
      .cloned()
      .collect();

    if !invalid_items.is_empty() {
      return Err(invalid_lists_error(
        field_span,
        error_prefix,
        &invalid_items,
      ));
    }

    let in_list = (!in_list_hashset.is_empty()).then(|| {
      let in_list_str = self.r#in.iter().map(|d| format!("{:?}", d)).join(", ");

      (in_list_hashset, in_list_str)
    });

    let not_in_list = (!not_in_list_hashset.is_empty()).then(|| {
      let not_in_list_str = self.not_in.iter().map(|d| format!("{:?}", d)).join(", ");

      (not_in_list_hashset, not_in_list_str)
    });

    Ok(ContainingRules {
      in_list,
      not_in_list,
    })
  }
}

pub(crate) fn invalid_lists_error<T>(
  field_span: Span,
  error_prefix: &str,
  invalid_items: &[T],
) -> Error
where
  T: Debug,
{
  Error::new(
    field_span,
    format!(
      "{} the following values are contained by 'in' and 'not_in': {:?}",
      error_prefix, invalid_items
    ),
  )
}

#[allow(clippy::type_complexity)]
pub(crate) fn validate_lists<T>(
  in_list: &[T],
  not_in_list: &[T],
  wrap_in_quotes: bool,
) -> Result<(Option<(HashSet<T>, String)>, Option<(HashSet<T>, String)>), Vec<T>>
where
  T: Clone + Hash + Eq + Debug + ToTokens + Display,
{
  let in_list_hashset: HashSet<T> = in_list.iter().cloned().collect();
  let not_in_list_hashset: HashSet<T> = not_in_list.iter().cloned().collect();

  let invalid_items: Vec<T> = in_list_hashset
    .intersection(&not_in_list_hashset)
    .cloned()
    .collect();

  if !invalid_items.is_empty() {
    return Err(invalid_items);
  }

  let in_list = (!in_list_hashset.is_empty()).then(|| {
    let in_list_str = if wrap_in_quotes {
      in_list_hashset
        .iter()
        .map(|d| format!("'{}'", d))
        .join(", ")
    } else {
      in_list_hashset.iter().join(", ")
    };

    (in_list_hashset, in_list_str)
  });

  let not_in_list = (!not_in_list_hashset.is_empty()).then(|| {
    let not_in_list_str = if wrap_in_quotes {
      not_in_list_hashset
        .iter()
        .map(|d| format!("'{}'", d))
        .join(", ")
    } else {
      not_in_list_hashset.iter().join(", ")
    };

    (not_in_list_hashset, not_in_list_str)
  });

  Ok((in_list, not_in_list))
}

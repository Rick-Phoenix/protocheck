use proc_macro2::Span;
use syn::Error;

#[derive(Debug)]
pub struct Lt<T>
where
  T: PartialEq + PartialOrd,
{
  pub val: T,
  pub eq: bool,
}

#[derive(Debug)]
pub struct Gt<T>
where
  T: PartialEq + PartialOrd,
{
  pub val: T,
  pub eq: bool,
}

pub fn validate_gt_lt<T>(
  gt: &Option<Gt<T>>,
  lt: &Option<Lt<T>>,
  error_prefix: &str,
  field_span: Span,
) -> Result<(), Error>
where
  T: PartialEq + PartialOrd,
{
  if let Some(gt_val) = gt {
    if let Some(lt_val) = lt {
      if lt_val.eq && gt_val.eq && lt_val.val > gt_val.val {
        return Err(Error::new(
          field_span,
          format!("{} Lte cannot be larger than Gte", error_prefix),
        ));
      }
      if !lt_val.eq && !gt_val.eq && lt_val.val >= gt_val.val {
        return Err(Error::new(
          field_span,
          format!("{} Lt cannot be larger than or equal to Gt", error_prefix),
        ));
      }
      if lt_val.eq && !gt_val.eq && lt_val.val >= gt_val.val {
        return Err(Error::new(
          field_span,
          format!("{} Lte cannot be larger than or equal to Gt", error_prefix),
        ));
      }
      if !lt_val.eq && gt_val.eq && lt_val.val > gt_val.val {
        return Err(Error::new(
          field_span,
          format!("{} Lt cannot be larger than Gte", error_prefix),
        ));
      }
    }
  }

  Ok(())
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

use std::{collections::HashSet, fmt::Debug, hash::Hash};

use itertools::Itertools;
use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::Error;

use super::{
  comparable_rules::ComparableRules,
  containing_rules::{invalid_lists_error, validate_lists, ContainingRules},
  into_comparable::IntoComparable,
};
use crate::protovalidate::{
  DoubleRules, Fixed32Rules, Fixed64Rules, FloatRules, Int32Rules, Int64Rules, SFixed32Rules,
  SFixed64Rules, SInt32Rules, SInt64Rules, UInt32Rules, UInt64Rules,
};

pub trait NumericRules<HashableType>
where
  HashableType: Debug + Copy + ToTokens + Eq + PartialOrd + Hash,
{
  type Unit: ToTokens + PartialEq + PartialOrd + Debug;
  const UNIT_NAME: &'static str;
  fn constant(&self) -> Option<Self::Unit>;
  fn containing_rules(
    &self,
    field_span: Span,
    error_prefix: &str,
  ) -> Result<ContainingRules<HashableType>, Error>;
  fn finite(&self) -> Option<TokenStream>;
  fn comparable_rules(
    &self,
    field_span: Span,
    error_prefix: &str,
  ) -> Result<ComparableRules<Self::Unit>, Error>;
  fn hashable_type_tokens(&self) -> TokenStream;
}

impl NumericRules<u32> for FloatRules {
  type Unit = f32;
  const UNIT_NAME: &'static str = "float";

  fn hashable_type_tokens(&self) -> TokenStream {
    quote! { u32 }
  }
  fn constant(&self) -> Option<Self::Unit> {
    self.r#const
  }
  fn finite(&self) -> Option<TokenStream> {
    self
      .finite()
      .then_some(quote! { protocheck::validators::floats::f32_is_finite })
  }
  fn containing_rules(
    &self,
    field_span: Span,
    error_prefix: &str,
  ) -> Result<ContainingRules<u32>, Error> {
    let in_list_hashset: HashSet<u32> = self.r#in.iter().map(|n| n.to_bits()).collect();
    let not_in_list_hashset: HashSet<u32> = self.not_in.iter().map(|n| n.to_bits()).collect();

    let invalid_items: Vec<f32> = in_list_hashset
      .intersection(&not_in_list_hashset)
      .map(|n| f32::from_bits(*n))
      .collect();

    if !invalid_items.is_empty() {
      return Err(invalid_lists_error(
        field_span,
        error_prefix,
        &invalid_items,
      ));
    }

    let in_list = (!in_list_hashset.is_empty()).then(|| {
      let in_list_str = self.r#in.iter().map(|d| format!("{}", d)).join(", ");

      (in_list_hashset, in_list_str)
    });

    let not_in_list = (!not_in_list_hashset.is_empty()).then(|| {
      let not_in_list_str = self.not_in.iter().map(|d| format!("{}", d)).join(", ");

      (not_in_list_hashset, not_in_list_str)
    });

    Ok(ContainingRules {
      in_list,
      not_in_list,
    })
  }
  fn comparable_rules(
    &self,
    field_span: Span,
    error_prefix: &str,
  ) -> Result<ComparableRules<Self::Unit>, Error> {
    let rules = ComparableRules {
      greater_than: self.greater_than.map(|gt| gt.into_comparable()),
      less_than: self.less_than.map(|lt| lt.into_comparable()),
    };
    rules.validate(field_span, error_prefix)
  }
}

impl NumericRules<u64> for DoubleRules {
  type Unit = f64;
  const UNIT_NAME: &'static str = "double";

  fn hashable_type_tokens(&self) -> TokenStream {
    quote! { u64 }
  }
  fn constant(&self) -> Option<Self::Unit> {
    self.r#const
  }
  fn finite(&self) -> Option<TokenStream> {
    self
      .finite()
      .then_some(quote! { protocheck::validators::floats::f64_is_finite })
  }
  fn comparable_rules(
    &self,
    field_span: Span,
    error_prefix: &str,
  ) -> Result<ComparableRules<Self::Unit>, Error> {
    let rules = ComparableRules {
      greater_than: self.greater_than.map(|gt| gt.into_comparable()),
      less_than: self.less_than.map(|lt| lt.into_comparable()),
    };
    rules.validate(field_span, error_prefix)
  }

  fn containing_rules(
    &self,
    field_span: Span,
    error_prefix: &str,
  ) -> Result<ContainingRules<u64>, Error> {
    let in_list_hashset: HashSet<u64> = self.r#in.iter().map(|n| n.to_bits()).collect();
    let not_in_list_hashset: HashSet<u64> = self.not_in.iter().map(|n| n.to_bits()).collect();

    let invalid_items: Vec<f64> = in_list_hashset
      .intersection(&not_in_list_hashset)
      .map(|n| f64::from_bits(*n))
      .collect();

    if !invalid_items.is_empty() {
      return Err(invalid_lists_error(
        field_span,
        error_prefix,
        &invalid_items,
      ));
    }

    let in_list = (!in_list_hashset.is_empty()).then(|| {
      let in_list_str = self.r#in.iter().map(|d| format!("{}", d)).join(", ");

      (in_list_hashset, in_list_str)
    });

    let not_in_list = (!not_in_list_hashset.is_empty()).then(|| {
      let not_in_list_str = self.not_in.iter().map(|d| format!("{}", d)).join(", ");

      (not_in_list_hashset, not_in_list_str)
    });

    Ok(ContainingRules {
      in_list,
      not_in_list,
    })
  }
}

impl NumericRules<i64> for Int64Rules {
  type Unit = i64;
  const UNIT_NAME: &'static str = "int64";

  fn hashable_type_tokens(&self) -> TokenStream {
    quote! { i64 }
  }
  fn constant(&self) -> Option<Self::Unit> {
    self.r#const
  }
  fn finite(&self) -> Option<TokenStream> {
    None
  }
  fn comparable_rules(
    &self,
    field_span: Span,
    error_prefix: &str,
  ) -> Result<ComparableRules<Self::Unit>, Error> {
    let rules = ComparableRules {
      greater_than: self.greater_than.map(|gt| gt.into_comparable()),
      less_than: self.less_than.map(|lt| lt.into_comparable()),
    };
    rules.validate(field_span, error_prefix)
  }
  fn containing_rules(
    &self,
    field_span: Span,
    error_prefix: &str,
  ) -> Result<ContainingRules<Self::Unit>, Error> {
    let (in_list, not_in_list) = validate_lists(&self.r#in, &self.not_in, false)
      .map_err(|invalid_items| invalid_lists_error(field_span, error_prefix, &invalid_items))?;

    Ok(ContainingRules {
      in_list,
      not_in_list,
    })
  }
}

impl NumericRules<i64> for SInt64Rules {
  type Unit = i64;
  const UNIT_NAME: &'static str = "sint64";

  fn hashable_type_tokens(&self) -> TokenStream {
    quote! { i64 }
  }
  fn constant(&self) -> Option<Self::Unit> {
    self.r#const
  }
  fn finite(&self) -> Option<TokenStream> {
    None
  }
  fn comparable_rules(
    &self,
    field_span: Span,
    error_prefix: &str,
  ) -> Result<ComparableRules<Self::Unit>, Error> {
    let rules = ComparableRules {
      greater_than: self.greater_than.map(|gt| gt.into_comparable()),
      less_than: self.less_than.map(|lt| lt.into_comparable()),
    };
    rules.validate(field_span, error_prefix)
  }
  fn containing_rules(
    &self,
    field_span: Span,
    error_prefix: &str,
  ) -> Result<ContainingRules<Self::Unit>, Error> {
    let (in_list, not_in_list) = validate_lists(&self.r#in, &self.not_in, false)
      .map_err(|invalid_items| invalid_lists_error(field_span, error_prefix, &invalid_items))?;

    Ok(ContainingRules {
      in_list,
      not_in_list,
    })
  }
}

impl NumericRules<i64> for SFixed64Rules {
  type Unit = i64;
  const UNIT_NAME: &'static str = "sfixed64";

  fn hashable_type_tokens(&self) -> TokenStream {
    quote! { i64 }
  }

  fn constant(&self) -> Option<Self::Unit> {
    self.r#const
  }
  fn finite(&self) -> Option<TokenStream> {
    None
  }
  fn comparable_rules(
    &self,
    field_span: Span,
    error_prefix: &str,
  ) -> Result<ComparableRules<Self::Unit>, Error> {
    let rules = ComparableRules {
      greater_than: self.greater_than.map(|gt| gt.into_comparable()),
      less_than: self.less_than.map(|lt| lt.into_comparable()),
    };
    rules.validate(field_span, error_prefix)
  }
  fn containing_rules(
    &self,
    field_span: Span,
    error_prefix: &str,
  ) -> Result<ContainingRules<Self::Unit>, Error> {
    let (in_list, not_in_list) = validate_lists(&self.r#in, &self.not_in, false)
      .map_err(|invalid_items| invalid_lists_error(field_span, error_prefix, &invalid_items))?;

    Ok(ContainingRules {
      in_list,
      not_in_list,
    })
  }
}

impl NumericRules<i32> for Int32Rules {
  type Unit = i32;
  const UNIT_NAME: &'static str = "int32";

  fn hashable_type_tokens(&self) -> TokenStream {
    quote! { i32 }
  }

  fn constant(&self) -> Option<Self::Unit> {
    self.r#const
  }
  fn finite(&self) -> Option<TokenStream> {
    None
  }
  fn comparable_rules(
    &self,
    field_span: Span,
    error_prefix: &str,
  ) -> Result<ComparableRules<Self::Unit>, Error> {
    let rules = ComparableRules {
      greater_than: self.greater_than.map(|gt| gt.into_comparable()),
      less_than: self.less_than.map(|lt| lt.into_comparable()),
    };
    rules.validate(field_span, error_prefix)
  }
  fn containing_rules(
    &self,
    field_span: Span,
    error_prefix: &str,
  ) -> Result<ContainingRules<Self::Unit>, Error> {
    let (in_list, not_in_list) = validate_lists(&self.r#in, &self.not_in, false)
      .map_err(|invalid_items| invalid_lists_error(field_span, error_prefix, &invalid_items))?;

    Ok(ContainingRules {
      in_list,
      not_in_list,
    })
  }
}

impl NumericRules<i32> for SInt32Rules {
  type Unit = i32;
  const UNIT_NAME: &'static str = "sint32";

  fn hashable_type_tokens(&self) -> TokenStream {
    quote! { i32 }
  }

  fn constant(&self) -> Option<Self::Unit> {
    self.r#const
  }
  fn finite(&self) -> Option<TokenStream> {
    None
  }
  fn comparable_rules(
    &self,
    field_span: Span,
    error_prefix: &str,
  ) -> Result<ComparableRules<Self::Unit>, Error> {
    let rules = ComparableRules {
      greater_than: self.greater_than.map(|gt| gt.into_comparable()),
      less_than: self.less_than.map(|lt| lt.into_comparable()),
    };
    rules.validate(field_span, error_prefix)
  }
  fn containing_rules(
    &self,
    field_span: Span,
    error_prefix: &str,
  ) -> Result<ContainingRules<Self::Unit>, Error> {
    let (in_list, not_in_list) = validate_lists(&self.r#in, &self.not_in, false)
      .map_err(|invalid_items| invalid_lists_error(field_span, error_prefix, &invalid_items))?;

    Ok(ContainingRules {
      in_list,
      not_in_list,
    })
  }
}

impl NumericRules<i32> for SFixed32Rules {
  type Unit = i32;
  const UNIT_NAME: &'static str = "sfixed32";

  fn hashable_type_tokens(&self) -> TokenStream {
    quote! { i32 }
  }

  fn constant(&self) -> Option<Self::Unit> {
    self.r#const
  }
  fn finite(&self) -> Option<TokenStream> {
    None
  }
  fn comparable_rules(
    &self,
    field_span: Span,
    error_prefix: &str,
  ) -> Result<ComparableRules<Self::Unit>, Error> {
    let rules = ComparableRules {
      greater_than: self.greater_than.map(|gt| gt.into_comparable()),
      less_than: self.less_than.map(|lt| lt.into_comparable()),
    };
    rules.validate(field_span, error_prefix)
  }
  fn containing_rules(
    &self,
    field_span: Span,
    error_prefix: &str,
  ) -> Result<ContainingRules<Self::Unit>, Error> {
    let (in_list, not_in_list) = validate_lists(&self.r#in, &self.not_in, false)
      .map_err(|invalid_items| invalid_lists_error(field_span, error_prefix, &invalid_items))?;

    Ok(ContainingRules {
      in_list,
      not_in_list,
    })
  }
}

impl NumericRules<u64> for UInt64Rules {
  type Unit = u64;
  const UNIT_NAME: &'static str = "uint64";

  fn hashable_type_tokens(&self) -> TokenStream {
    quote! { u64 }
  }

  fn constant(&self) -> Option<Self::Unit> {
    self.r#const
  }
  fn finite(&self) -> Option<TokenStream> {
    None
  }
  fn comparable_rules(
    &self,
    field_span: Span,
    error_prefix: &str,
  ) -> Result<ComparableRules<Self::Unit>, Error> {
    let rules = ComparableRules {
      greater_than: self.greater_than.map(|gt| gt.into_comparable()),
      less_than: self.less_than.map(|lt| lt.into_comparable()),
    };
    rules.validate(field_span, error_prefix)
  }
  fn containing_rules(
    &self,
    field_span: Span,
    error_prefix: &str,
  ) -> Result<ContainingRules<Self::Unit>, Error> {
    let (in_list, not_in_list) = validate_lists(&self.r#in, &self.not_in, false)
      .map_err(|invalid_items| invalid_lists_error(field_span, error_prefix, &invalid_items))?;

    Ok(ContainingRules {
      in_list,
      not_in_list,
    })
  }
}

impl NumericRules<u64> for Fixed64Rules {
  type Unit = u64;
  const UNIT_NAME: &'static str = "fixed64";

  fn hashable_type_tokens(&self) -> TokenStream {
    quote! { u64 }
  }

  fn constant(&self) -> Option<Self::Unit> {
    self.r#const
  }
  fn finite(&self) -> Option<TokenStream> {
    None
  }
  fn comparable_rules(
    &self,
    field_span: Span,
    error_prefix: &str,
  ) -> Result<ComparableRules<Self::Unit>, Error> {
    let rules = ComparableRules {
      greater_than: self.greater_than.map(|gt| gt.into_comparable()),
      less_than: self.less_than.map(|lt| lt.into_comparable()),
    };
    rules.validate(field_span, error_prefix)
  }
  fn containing_rules(
    &self,
    field_span: Span,
    error_prefix: &str,
  ) -> Result<ContainingRules<Self::Unit>, Error> {
    let (in_list, not_in_list) = validate_lists(&self.r#in, &self.not_in, false)
      .map_err(|invalid_items| invalid_lists_error(field_span, error_prefix, &invalid_items))?;

    Ok(ContainingRules {
      in_list,
      not_in_list,
    })
  }
}

impl NumericRules<u32> for UInt32Rules {
  type Unit = u32;
  const UNIT_NAME: &'static str = "uint32";

  fn hashable_type_tokens(&self) -> TokenStream {
    quote! { u32 }
  }

  fn constant(&self) -> Option<Self::Unit> {
    self.r#const
  }
  fn finite(&self) -> Option<TokenStream> {
    None
  }
  fn comparable_rules(
    &self,
    field_span: Span,
    error_prefix: &str,
  ) -> Result<ComparableRules<Self::Unit>, Error> {
    let rules = ComparableRules {
      greater_than: self.greater_than.map(|gt| gt.into_comparable()),
      less_than: self.less_than.map(|lt| lt.into_comparable()),
    };
    rules.validate(field_span, error_prefix)
  }
  fn containing_rules(
    &self,
    field_span: Span,
    error_prefix: &str,
  ) -> Result<ContainingRules<Self::Unit>, Error> {
    let (in_list, not_in_list) = validate_lists(&self.r#in, &self.not_in, false)
      .map_err(|invalid_items| invalid_lists_error(field_span, error_prefix, &invalid_items))?;

    Ok(ContainingRules {
      in_list,
      not_in_list,
    })
  }
}

impl NumericRules<u32> for Fixed32Rules {
  type Unit = u32;
  const UNIT_NAME: &'static str = "fixed32";

  fn hashable_type_tokens(&self) -> TokenStream {
    quote! { u32 }
  }

  fn constant(&self) -> Option<Self::Unit> {
    self.r#const
  }
  fn finite(&self) -> Option<TokenStream> {
    None
  }
  fn comparable_rules(
    &self,
    field_span: Span,
    error_prefix: &str,
  ) -> Result<ComparableRules<Self::Unit>, Error> {
    let rules = ComparableRules {
      greater_than: self.greater_than.map(|gt| gt.into_comparable()),
      less_than: self.less_than.map(|lt| lt.into_comparable()),
    };
    rules.validate(field_span, error_prefix)
  }
  fn containing_rules(
    &self,
    field_span: Span,
    error_prefix: &str,
  ) -> Result<ContainingRules<Self::Unit>, Error> {
    let (in_list, not_in_list) = validate_lists(&self.r#in, &self.not_in, false)
      .map_err(|invalid_items| invalid_lists_error(field_span, error_prefix, &invalid_items))?;

    Ok(ContainingRules {
      in_list,
      not_in_list,
    })
  }
}

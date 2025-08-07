use std::{fmt::Debug, hash::Hash};

use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::Error;

use crate::{
  protovalidate::{
    DoubleRules, Fixed32Rules, Fixed64Rules, FloatRules, Int32Rules, Int64Rules, SFixed32Rules,
    SFixed64Rules, SInt32Rules, SInt64Rules, UInt32Rules, UInt64Rules,
  },
  protovalidate_impls::{
    comparable_rules::ComparableRules,
    containing_rules::{validate_in_not_in, validate_in_not_in_floats, ContainingRules},
    into_comparable::IntoComparable,
  },
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
    let in_list = self.r#in.clone();
    let not_in_list = self.not_in.clone();
    let (in_hashset, not_in_hashset) =
      validate_in_not_in_floats(&in_list, &not_in_list, field_span, error_prefix)?;

    Ok(ContainingRules {
      in_list: in_hashset,
      not_in_list: not_in_hashset,
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
    let in_list = self.r#in.clone();
    let not_in_list = self.not_in.clone();

    let (in_hashset, not_in_hashset) =
      validate_in_not_in_floats(&in_list, &not_in_list, field_span, error_prefix)?;

    Ok(ContainingRules {
      in_list: in_hashset,
      not_in_list: not_in_hashset,
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
    let in_list = self.r#in.clone();
    let not_in_list = self.not_in.clone();

    let (in_hashset, not_in_hashset) =
      validate_in_not_in(&in_list, &not_in_list, field_span, error_prefix)?;

    Ok(ContainingRules {
      in_list: in_hashset,
      not_in_list: not_in_hashset,
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
    let in_list = self.r#in.clone();
    let not_in_list = self.not_in.clone();

    let (in_hashset, not_in_hashset) =
      validate_in_not_in(&in_list, &not_in_list, field_span, error_prefix)?;

    Ok(ContainingRules {
      in_list: in_hashset,
      not_in_list: not_in_hashset,
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
    let in_list = self.r#in.clone();
    let not_in_list = self.not_in.clone();

    let (in_hashset, not_in_hashset) =
      validate_in_not_in(&in_list, &not_in_list, field_span, error_prefix)?;

    Ok(ContainingRules {
      in_list: in_hashset,
      not_in_list: not_in_hashset,
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
    let in_list = self.r#in.clone();
    let not_in_list = self.not_in.clone();

    let (in_hashset, not_in_hashset) =
      validate_in_not_in(&in_list, &not_in_list, field_span, error_prefix)?;

    Ok(ContainingRules {
      in_list: in_hashset,
      not_in_list: not_in_hashset,
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
    let in_list = self.r#in.clone();
    let not_in_list = self.not_in.clone();

    let (in_hashset, not_in_hashset) =
      validate_in_not_in(&in_list, &not_in_list, field_span, error_prefix)?;

    Ok(ContainingRules {
      in_list: in_hashset,
      not_in_list: not_in_hashset,
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
    let in_list = self.r#in.clone();
    let not_in_list = self.not_in.clone();

    let (in_hashset, not_in_hashset) =
      validate_in_not_in(&in_list, &not_in_list, field_span, error_prefix)?;

    Ok(ContainingRules {
      in_list: in_hashset,
      not_in_list: not_in_hashset,
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
    let in_list = self.r#in.clone();
    let not_in_list = self.not_in.clone();

    let (in_hashset, not_in_hashset) =
      validate_in_not_in(&in_list, &not_in_list, field_span, error_prefix)?;

    Ok(ContainingRules {
      in_list: in_hashset,
      not_in_list: not_in_hashset,
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
    let in_list = self.r#in.clone();
    let not_in_list = self.not_in.clone();

    let (in_hashset, not_in_hashset) =
      validate_in_not_in(&in_list, &not_in_list, field_span, error_prefix)?;

    Ok(ContainingRules {
      in_list: in_hashset,
      not_in_list: not_in_hashset,
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
    let in_list = self.r#in.clone();
    let not_in_list = self.not_in.clone();

    let (in_hashset, not_in_hashset) =
      validate_in_not_in(&in_list, &not_in_list, field_span, error_prefix)?;

    Ok(ContainingRules {
      in_list: in_hashset,
      not_in_list: not_in_hashset,
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
    let in_list = self.r#in.clone();
    let not_in_list = self.not_in.clone();

    let (in_hashset, not_in_hashset) =
      validate_in_not_in(&in_list, &not_in_list, field_span, error_prefix)?;

    Ok(ContainingRules {
      in_list: in_hashset,
      not_in_list: not_in_hashset,
    })
  }
}

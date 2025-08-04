use std::fmt::Debug;

use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::Error;

use crate::{
  field_descriptor_proto::Type as ProtoType,
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

pub trait NumericRules {
  type Unit: ToTokens + PartialEq + PartialOrd + Debug;
  const UNIT_NAME: &'static str;
  fn constant(&self) -> Option<Self::Unit>;
  fn containing_rules(
    &self,
    field_span: Span,
    error_prefix: &str,
  ) -> Result<ContainingRules<Self::Unit>, Error>;
  fn finite(&self) -> Option<TokenStream>;
  fn comparable_rules(
    &self,
    field_span: Span,
    error_prefix: &str,
  ) -> Result<ComparableRules<Self::Unit>, Error>;
  fn matches_type(&self, ty: &ProtoType) -> bool;
}

impl NumericRules for FloatRules {
  type Unit = f32;
  const UNIT_NAME: &'static str = "float";
  fn matches_type(&self, ty: &ProtoType) -> bool {
    matches!(ty, ProtoType::Float)
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
  ) -> Result<ContainingRules<Self::Unit>, Error> {
    let in_list = self.r#in.clone();
    let not_in_list = self.not_in.clone();
    validate_in_not_in_floats(&in_list, &not_in_list, field_span, error_prefix)?;

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

impl NumericRules for DoubleRules {
  type Unit = f64;
  const UNIT_NAME: &'static str = "double";
  fn matches_type(&self, ty: &ProtoType) -> bool {
    matches!(ty, ProtoType::Double)
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
  ) -> Result<ContainingRules<Self::Unit>, Error> {
    let in_list = self.r#in.clone();
    let not_in_list = self.not_in.clone();
    validate_in_not_in_floats(&in_list, &not_in_list, field_span, error_prefix)?;

    Ok(ContainingRules {
      in_list,
      not_in_list,
    })
  }
}

impl NumericRules for Int64Rules {
  type Unit = i64;
  const UNIT_NAME: &'static str = "int64";
  fn matches_type(&self, ty: &ProtoType) -> bool {
    matches!(ty, ProtoType::Int64)
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
    validate_in_not_in(&in_list, &not_in_list, field_span, error_prefix)?;

    Ok(ContainingRules {
      in_list,
      not_in_list,
    })
  }
}

impl NumericRules for SInt64Rules {
  type Unit = i64;
  const UNIT_NAME: &'static str = "sint64";
  fn matches_type(&self, ty: &ProtoType) -> bool {
    matches!(ty, ProtoType::Sint64)
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
    validate_in_not_in(&in_list, &not_in_list, field_span, error_prefix)?;

    Ok(ContainingRules {
      in_list,
      not_in_list,
    })
  }
}

impl NumericRules for SFixed64Rules {
  type Unit = i64;
  const UNIT_NAME: &'static str = "sfixed64";
  fn matches_type(&self, ty: &ProtoType) -> bool {
    matches!(ty, ProtoType::Sfixed64)
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
    validate_in_not_in(&in_list, &not_in_list, field_span, error_prefix)?;

    Ok(ContainingRules {
      in_list,
      not_in_list,
    })
  }
}

impl NumericRules for Int32Rules {
  type Unit = i32;
  const UNIT_NAME: &'static str = "int32";
  fn matches_type(&self, ty: &ProtoType) -> bool {
    matches!(ty, ProtoType::Int32)
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
    validate_in_not_in(&in_list, &not_in_list, field_span, error_prefix)?;

    Ok(ContainingRules {
      in_list,
      not_in_list,
    })
  }
}

impl NumericRules for SInt32Rules {
  type Unit = i32;
  const UNIT_NAME: &'static str = "sint32";
  fn matches_type(&self, ty: &ProtoType) -> bool {
    matches!(ty, ProtoType::Sint32)
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
    validate_in_not_in(&in_list, &not_in_list, field_span, error_prefix)?;

    Ok(ContainingRules {
      in_list,
      not_in_list,
    })
  }
}

impl NumericRules for SFixed32Rules {
  type Unit = i32;
  const UNIT_NAME: &'static str = "sfixed32";
  fn matches_type(&self, ty: &ProtoType) -> bool {
    matches!(ty, ProtoType::Sfixed32)
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
    validate_in_not_in(&in_list, &not_in_list, field_span, error_prefix)?;

    Ok(ContainingRules {
      in_list,
      not_in_list,
    })
  }
}

impl NumericRules for UInt64Rules {
  type Unit = u64;
  const UNIT_NAME: &'static str = "uint64";
  fn matches_type(&self, ty: &ProtoType) -> bool {
    matches!(ty, ProtoType::Uint64)
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
    validate_in_not_in(&in_list, &not_in_list, field_span, error_prefix)?;

    Ok(ContainingRules {
      in_list,
      not_in_list,
    })
  }
}

impl NumericRules for Fixed64Rules {
  type Unit = u64;
  const UNIT_NAME: &'static str = "fixed64";
  fn matches_type(&self, ty: &ProtoType) -> bool {
    matches!(ty, ProtoType::Fixed64)
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
    validate_in_not_in(&in_list, &not_in_list, field_span, error_prefix)?;

    Ok(ContainingRules {
      in_list,
      not_in_list,
    })
  }
}

impl NumericRules for UInt32Rules {
  type Unit = u32;
  const UNIT_NAME: &'static str = "uint32";
  fn matches_type(&self, ty: &ProtoType) -> bool {
    matches!(ty, ProtoType::Uint32)
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
    validate_in_not_in(&in_list, &not_in_list, field_span, error_prefix)?;

    Ok(ContainingRules {
      in_list,
      not_in_list,
    })
  }
}

impl NumericRules for Fixed32Rules {
  type Unit = u32;
  const UNIT_NAME: &'static str = "fixed32";
  fn matches_type(&self, ty: &ProtoType) -> bool {
    matches!(ty, ProtoType::Fixed32)
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
    validate_in_not_in(&in_list, &not_in_list, field_span, error_prefix)?;

    Ok(ContainingRules {
      in_list,
      not_in_list,
    })
  }
}

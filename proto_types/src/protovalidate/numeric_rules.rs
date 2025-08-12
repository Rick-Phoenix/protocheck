use std::{
  collections::HashSet,
  fmt::{Debug, Display},
  hash::Hash,
};

use quote::{format_ident, quote, ToTokens};

use super::{
  comparable_rules::ComparableRules, containing_rules::ContainingRules,
  into_comparable::IntoComparable,
};
use crate::protovalidate::{
  containing_rules::{hashset_to_tokens, ItemList},
  ConstRule, DoubleRules, Fixed32Rules, Fixed64Rules, FloatRules, Int32Rules, Int64Rules,
  SFixed32Rules, SFixed64Rules, SInt32Rules, SInt64Rules, UInt32Rules, UInt64Rules,
};

pub trait NumericRules<HashableType>
where
  HashableType: Debug + Copy + ToTokens + Eq + PartialOrd + Hash,
{
  type Unit: ToTokens + PartialEq + PartialOrd + Debug + Display;
  fn constant(&self) -> Option<ConstRule<Self::Unit>>;
  fn num_containing_rules(&self, field_full_name: &str)
    -> Result<ContainingRules, Vec<Self::Unit>>;
  fn finite(&self) -> bool;
  fn comparable_rules(&self) -> Result<ComparableRules<Self::Unit>, &'static str>;
}

impl NumericRules<u32> for FloatRules {
  type Unit = f32;

  fn constant(&self) -> Option<ConstRule<Self::Unit>> {
    self.const_rule()
  }
  fn finite(&self) -> bool {
    self.finite()
  }

  fn num_containing_rules(
    &self,
    field_full_name: &str,
  ) -> Result<ContainingRules, Vec<Self::Unit>> {
    let in_list_slice = &self.r#in;
    let not_in_list_slice = &self.not_in;

    let in_list_hashset: HashSet<u32> = in_list_slice.iter().map(|n| n.to_bits()).collect();
    let not_in_list_hashset: HashSet<u32> = not_in_list_slice.iter().map(|n| n.to_bits()).collect();

    let invalid_items: Vec<f32> = in_list_hashset
      .intersection(&not_in_list_hashset)
      .map(|n| f32::from_bits(*n))
      .collect();

    if !invalid_items.is_empty() {
      return Err(invalid_items);
    }

    let in_list = (!in_list_hashset.is_empty()).then(|| {
      let stringified_list = self
        .r#in
        .iter()
        .map(|b| b.to_string())
        .collect::<Vec<String>>()
        .join(", ");

      let error_message = format!("must be one of these values: [ {} ]", stringified_list);

      if in_list_slice.len() >= 16 {
        let static_ident = format_ident!("__{}_IN_LIST", field_full_name);
        ItemList::HashSet {
          error_message,
          tokens: hashset_to_tokens(in_list_hashset, quote! { u32 }, &static_ident),
          static_ident,
        }
      } else {
        ItemList::Slice {
          error_message,
          tokens: quote! { [ #(#in_list_slice),* ] },
        }
      }
    });

    let not_in_list = (!not_in_list_hashset.is_empty()).then(|| {
      let stringified_list = self
        .r#in
        .iter()
        .map(|b| b.to_string())
        .collect::<Vec<String>>()
        .join(", ");

      let error_message = format!("cannot be one of these values: [ {} ]", stringified_list);

      if not_in_list_slice.len() >= 16 {
        let static_ident = format_ident!("__{}_NOT_IN_LIST", field_full_name);
        ItemList::HashSet {
          error_message,
          tokens: hashset_to_tokens(not_in_list_hashset, quote! { u32 }, &static_ident),
          static_ident,
        }
      } else {
        ItemList::Slice {
          error_message,
          tokens: quote! { [ #(#not_in_list_slice),* ] },
        }
      }
    });

    Ok(ContainingRules {
      in_list_rule: in_list,
      not_in_list_rule: not_in_list,
    })
  }

  fn comparable_rules(&self) -> Result<ComparableRules<Self::Unit>, &'static str> {
    let rules = ComparableRules {
      greater_than: self.greater_than.map(|gt| gt.into_comparable()),
      less_than: self.less_than.map(|lt| lt.into_comparable()),
    };
    rules.validate()
  }
}

impl NumericRules<u64> for DoubleRules {
  type Unit = f64;

  fn constant(&self) -> Option<ConstRule<Self::Unit>> {
    self.const_rule()
  }
  fn finite(&self) -> bool {
    self.finite()
  }
  fn comparable_rules(&self) -> Result<ComparableRules<Self::Unit>, &'static str> {
    let rules = ComparableRules {
      greater_than: self.greater_than.map(|gt| gt.into_comparable()),
      less_than: self.less_than.map(|lt| lt.into_comparable()),
    };
    rules.validate()
  }

  fn num_containing_rules(
    &self,
    field_full_name: &str,
  ) -> Result<ContainingRules, Vec<Self::Unit>> {
    let in_list_slice = &self.r#in;
    let not_in_list_slice = &self.not_in;

    let in_list_hashset: HashSet<u64> = in_list_slice.iter().map(|n| n.to_bits()).collect();
    let not_in_list_hashset: HashSet<u64> = not_in_list_slice.iter().map(|n| n.to_bits()).collect();

    let invalid_items: Vec<f64> = in_list_hashset
      .intersection(&not_in_list_hashset)
      .map(|n| f64::from_bits(*n))
      .collect();

    if !invalid_items.is_empty() {
      return Err(invalid_items);
    }

    let in_list = (!in_list_hashset.is_empty()).then(|| {
      let stringified_list = self
        .r#in
        .iter()
        .map(|b| b.to_string())
        .collect::<Vec<String>>()
        .join(", ");

      let error_message = format!("must be one of these values: [ {} ]", stringified_list);

      if in_list_slice.len() >= 16 {
        let static_ident = format_ident!("__{}_IN_LIST", field_full_name);
        ItemList::HashSet {
          error_message,
          tokens: hashset_to_tokens(in_list_hashset, quote! { u64 }, &static_ident),
          static_ident,
        }
      } else {
        ItemList::Slice {
          error_message,
          tokens: quote! { [ #(#in_list_slice),* ] },
        }
      }
    });

    let not_in_list = (!not_in_list_hashset.is_empty()).then(|| {
      let stringified_list = self
        .r#in
        .iter()
        .map(|b| b.to_string())
        .collect::<Vec<String>>()
        .join(", ");

      let error_message = format!("cannot be one of these values: [ {} ]", stringified_list);

      if not_in_list_slice.len() >= 16 {
        let static_ident = format_ident!("__{}_NOT_IN_LIST", field_full_name);
        ItemList::HashSet {
          error_message,
          tokens: hashset_to_tokens(not_in_list_hashset, quote! { u64 }, &static_ident),
          static_ident,
        }
      } else {
        ItemList::Slice {
          error_message,
          tokens: quote! { [ #(#not_in_list_slice),* ] },
        }
      }
    });

    Ok(ContainingRules {
      in_list_rule: in_list,
      not_in_list_rule: not_in_list,
    })
  }
}

impl NumericRules<i64> for Int64Rules {
  type Unit = i64;

  fn constant(&self) -> Option<ConstRule<Self::Unit>> {
    self.const_rule()
  }
  fn finite(&self) -> bool {
    false
  }
  fn comparable_rules(&self) -> Result<ComparableRules<Self::Unit>, &'static str> {
    let rules = ComparableRules {
      greater_than: self.greater_than.map(|gt| gt.into_comparable()),
      less_than: self.less_than.map(|lt| lt.into_comparable()),
    };
    rules.validate()
  }
  fn num_containing_rules(
    &self,
    field_full_name: &str,
  ) -> Result<ContainingRules, Vec<Self::Unit>> {
    self.containing_rules(field_full_name)
  }
}

impl NumericRules<i64> for SInt64Rules {
  type Unit = i64;

  fn constant(&self) -> Option<ConstRule<Self::Unit>> {
    self.const_rule()
  }
  fn finite(&self) -> bool {
    false
  }
  fn comparable_rules(&self) -> Result<ComparableRules<Self::Unit>, &'static str> {
    let rules = ComparableRules {
      greater_than: self.greater_than.map(|gt| gt.into_comparable()),
      less_than: self.less_than.map(|lt| lt.into_comparable()),
    };
    rules.validate()
  }
  fn num_containing_rules(
    &self,
    field_full_name: &str,
  ) -> Result<ContainingRules, Vec<Self::Unit>> {
    self.containing_rules(field_full_name)
  }
}

impl NumericRules<i64> for SFixed64Rules {
  type Unit = i64;

  fn constant(&self) -> Option<ConstRule<Self::Unit>> {
    self.const_rule()
  }
  fn finite(&self) -> bool {
    false
  }
  fn comparable_rules(&self) -> Result<ComparableRules<Self::Unit>, &'static str> {
    let rules = ComparableRules {
      greater_than: self.greater_than.map(|gt| gt.into_comparable()),
      less_than: self.less_than.map(|lt| lt.into_comparable()),
    };
    rules.validate()
  }
  fn num_containing_rules(
    &self,
    field_full_name: &str,
  ) -> Result<ContainingRules, Vec<Self::Unit>> {
    self.containing_rules(field_full_name)
  }
}

impl NumericRules<i32> for Int32Rules {
  type Unit = i32;

  fn constant(&self) -> Option<ConstRule<Self::Unit>> {
    self.const_rule()
  }
  fn finite(&self) -> bool {
    false
  }
  fn comparable_rules(&self) -> Result<ComparableRules<Self::Unit>, &'static str> {
    let rules = ComparableRules {
      greater_than: self.greater_than.map(|gt| gt.into_comparable()),
      less_than: self.less_than.map(|lt| lt.into_comparable()),
    };
    rules.validate()
  }
  fn num_containing_rules(
    &self,
    field_full_name: &str,
  ) -> Result<ContainingRules, Vec<Self::Unit>> {
    self.containing_rules(field_full_name)
  }
}

impl NumericRules<i32> for SInt32Rules {
  type Unit = i32;

  fn constant(&self) -> Option<ConstRule<Self::Unit>> {
    self.const_rule()
  }
  fn finite(&self) -> bool {
    false
  }
  fn comparable_rules(&self) -> Result<ComparableRules<Self::Unit>, &'static str> {
    let rules = ComparableRules {
      greater_than: self.greater_than.map(|gt| gt.into_comparable()),
      less_than: self.less_than.map(|lt| lt.into_comparable()),
    };
    rules.validate()
  }
  fn num_containing_rules(
    &self,
    field_full_name: &str,
  ) -> Result<ContainingRules, Vec<Self::Unit>> {
    self.containing_rules(field_full_name)
  }
}

impl NumericRules<i32> for SFixed32Rules {
  type Unit = i32;

  fn constant(&self) -> Option<ConstRule<Self::Unit>> {
    self.const_rule()
  }
  fn finite(&self) -> bool {
    false
  }
  fn comparable_rules(&self) -> Result<ComparableRules<Self::Unit>, &'static str> {
    let rules = ComparableRules {
      greater_than: self.greater_than.map(|gt| gt.into_comparable()),
      less_than: self.less_than.map(|lt| lt.into_comparable()),
    };
    rules.validate()
  }
  fn num_containing_rules(
    &self,
    field_full_name: &str,
  ) -> Result<ContainingRules, Vec<Self::Unit>> {
    self.containing_rules(field_full_name)
  }
}

impl NumericRules<u64> for UInt64Rules {
  type Unit = u64;

  fn constant(&self) -> Option<ConstRule<Self::Unit>> {
    self.const_rule()
  }
  fn finite(&self) -> bool {
    false
  }
  fn comparable_rules(&self) -> Result<ComparableRules<Self::Unit>, &'static str> {
    let rules = ComparableRules {
      greater_than: self.greater_than.map(|gt| gt.into_comparable()),
      less_than: self.less_than.map(|lt| lt.into_comparable()),
    };
    rules.validate()
  }
  fn num_containing_rules(
    &self,
    field_full_name: &str,
  ) -> Result<ContainingRules, Vec<Self::Unit>> {
    self.containing_rules(field_full_name)
  }
}

impl NumericRules<u64> for Fixed64Rules {
  type Unit = u64;

  fn constant(&self) -> Option<ConstRule<Self::Unit>> {
    self.const_rule()
  }
  fn finite(&self) -> bool {
    false
  }
  fn comparable_rules(&self) -> Result<ComparableRules<Self::Unit>, &'static str> {
    let rules = ComparableRules {
      greater_than: self.greater_than.map(|gt| gt.into_comparable()),
      less_than: self.less_than.map(|lt| lt.into_comparable()),
    };
    rules.validate()
  }
  fn num_containing_rules(
    &self,
    field_full_name: &str,
  ) -> Result<ContainingRules, Vec<Self::Unit>> {
    self.containing_rules(field_full_name)
  }
}

impl NumericRules<u32> for UInt32Rules {
  type Unit = u32;

  fn constant(&self) -> Option<ConstRule<Self::Unit>> {
    self.const_rule()
  }
  fn finite(&self) -> bool {
    false
  }
  fn comparable_rules(&self) -> Result<ComparableRules<Self::Unit>, &'static str> {
    let rules = ComparableRules {
      greater_than: self.greater_than.map(|gt| gt.into_comparable()),
      less_than: self.less_than.map(|lt| lt.into_comparable()),
    };
    rules.validate()
  }
  fn num_containing_rules(
    &self,
    field_full_name: &str,
  ) -> Result<ContainingRules, Vec<Self::Unit>> {
    self.containing_rules(field_full_name)
  }
}

impl NumericRules<u32> for Fixed32Rules {
  type Unit = u32;

  fn constant(&self) -> Option<ConstRule<Self::Unit>> {
    self.const_rule()
  }
  fn finite(&self) -> bool {
    false
  }
  fn comparable_rules(&self) -> Result<ComparableRules<Self::Unit>, &'static str> {
    let rules = ComparableRules {
      greater_than: self.greater_than.map(|gt| gt.into_comparable()),
      less_than: self.less_than.map(|lt| lt.into_comparable()),
    };
    rules.validate()
  }
  fn num_containing_rules(
    &self,
    field_full_name: &str,
  ) -> Result<ContainingRules, Vec<Self::Unit>> {
    self.containing_rules(field_full_name)
  }
}

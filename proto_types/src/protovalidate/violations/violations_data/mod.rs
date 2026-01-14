use crate::protovalidate::*;

macro_rules! violation_data {
  ( $typ:ident, $num:literal, $name:ident, $viol_num:literal, $field_type:ident ) => {
    paste::paste! {
      pub const [< $typ:upper _ $name:upper _VIOLATION >]: ViolationData = ViolationData {
        name: concat!(stringify!($typ),".", stringify!($name)),
        elements: &[
          ConstPathElement {
            name: stringify!($typ),
            number: $num,
            field_type: Type::Message,
          },
          ConstPathElement {
            name: stringify!($name),
            number: $viol_num,
            field_type: Type::$field_type,
          },
        ]
      };
    }
  };
}

violation_data!(bool, 13, const, 1, Bool);

pub mod base_violations;

pub mod field_mask_violations;
pub use field_mask_violations::*;
pub mod duration_violations;
pub use duration_violations::*;
pub mod any_violations;
pub use any_violations::*;
pub mod int_violations;
pub use int_violations::*;
pub mod timestamp_violations;
pub use timestamp_violations::*;
pub mod repeated_violations;
pub use repeated_violations::*;
pub mod oneof_violations;
pub use oneof_violations::*;
pub mod map_violations;
pub use map_violations::*;
pub mod float_violations;
pub use float_violations::*;
pub mod enum_violations;
pub use enum_violations::*;
pub mod string_violations;
pub use string_violations::*;
pub mod bytes_violations;
pub use base_violations::*;
pub use bytes_violations::*;

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConstPathElement {
  pub name: &'static str,
  pub number: i32,
  pub field_type: Type,
}

impl ConstPathElement {
  #[must_use]
  #[inline]
  pub fn as_path_element(&self) -> FieldPathElement {
    FieldPathElement {
      field_name: Some(self.name.to_string()),
      field_number: Some(self.number),
      field_type: Some(self.field_type as i32),
      ..Default::default()
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ViolationData {
  pub name: &'static str,
  pub elements: &'static [ConstPathElement],
}

impl ViolationData {
  #[must_use]
  #[inline]
  pub fn elements_iter(&self) -> impl ExactSizeIterator<Item = FieldPathElement> {
    self.elements.iter().map(|e| e.as_path_element())
  }

  #[must_use]
  pub fn to_elements_vec(&self) -> Vec<FieldPathElement> {
    self.elements_iter().collect()
  }
}

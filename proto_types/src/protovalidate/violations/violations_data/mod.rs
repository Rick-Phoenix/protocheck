use crate::{Vec, protovalidate::*};

#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ViolationType {
  Required,
  RequiredOneof,
  Float(FloatViolation),
  Double(DoubleViolation),
  Int32(Int32Violation),
  Int64(Int64Violation),
  Sint32(Sint32Violation),
  Sint64(Sint64Violation),
  Sfixed32(Sfixed32Violation),
  Sfixed64(Sfixed64Violation),
  Uint32(Uint32Violation),
  Uint64(Uint64Violation),
  Fixed32(Fixed32Violation),
  Fixed64(Fixed64Violation),
  String(StringViolation),
  Bool(BoolViolation),
  Bytes(BytesViolation),
  Enum(EnumViolation),
  Repeated(RepeatedViolation),
  Map(MapViolation),
  Any(AnyViolation),
  Duration(DurationViolation),
  FieldMask(FieldMaskViolation),
  Timestamp(TimestampViolation),
}

impl ViolationType {
  #[must_use]
  pub const fn data(&self) -> ViolationData {
    match self {
      Self::Required => REQUIRED_VIOLATION,
      Self::RequiredOneof => ONEOF_REQUIRED_VIOLATION,
      Self::Float(v) => v.data(),
      Self::Double(v) => v.data(),
      Self::Int32(v) => v.data(),
      Self::Int64(v) => v.data(),
      Self::Sint32(v) => v.data(),
      Self::Sint64(v) => v.data(),
      Self::Sfixed32(v) => v.data(),
      Self::Sfixed64(v) => v.data(),
      Self::Uint32(v) => v.data(),
      Self::Uint64(v) => v.data(),
      Self::Fixed32(v) => v.data(),
      Self::Fixed64(v) => v.data(),
      Self::String(v) => v.data(),
      Self::Bytes(v) => v.data(),
      Self::Enum(v) => v.data(),
      Self::Repeated(v) => v.data(),
      Self::Map(v) => v.data(),
      Self::Any(v) => v.data(),
      Self::Duration(v) => v.data(),
      Self::FieldMask(v) => v.data(),
      Self::Timestamp(v) => v.data(),
      Self::Bool(v) => v.data(),
    }
  }
}

macro_rules! violations_enum {
  ($target:ident, $($names:ident),*) => {
    paste::paste! {
      #[non_exhaustive]
      #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
      pub enum [< $target Violation >] {
        $(
          [< $names:camel >]
        ),*
      }

      impl [< $target Violation >] {
        pub const fn data(&self) -> ViolationData {
          match self {
            $(
              Self::[< $names:camel >] => [< $target:snake:upper _ $names:snake:upper _VIOLATION  >]
            ),*
          }
        }
      }

      impl From<[< $target Violation >]> for ViolationType {
        fn from(value: [< $target Violation >]) -> Self {
          Self::$target(value)
        }
      }
    }
  };
}

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
violations_enum!(Bool, const);

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

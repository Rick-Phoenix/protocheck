use ordered_float::OrderedFloat;
use proto_types::{Any, Duration};

use super::*;
use crate::protovalidate::violations_data::{in_violations::*, not_in_violations::*};

pub trait ListRules: Sized {
  type LookupTarget: PartialEq + PartialOrd + Ord;
  const IN_VIOLATION: &'static LazyLock<ViolationData>;
  const NOT_IN_VIOLATION: &'static LazyLock<ViolationData>;

  fn is_in(&self, list: &SortedList<Self::LookupTarget>) -> bool;
}

pub struct SortedList<T: Ord> {
  items: Box<[T]>,
}

impl<T> SortedList<T>
where
  T: Ord,
{
  pub fn new<I: IntoIterator<Item = T>>(iter: I) -> Self {
    let mut items: Vec<T> = iter.into_iter().collect();

    items.sort();

    Self {
      items: items.into_boxed_slice(),
    }
  }

  pub fn contains(&self, item: &T) -> bool {
    self.items.binary_search(item).is_ok()
  }
}

macro_rules! impl_lookup {
  ($typ:ty, $proto_type:ident) => {
    paste::paste! {
      impl ListRules for $typ {
        type LookupTarget = Self;
        const IN_VIOLATION: &'static LazyLock<ViolationData> = &[< $proto_type _IN_VIOLATION >];
        const NOT_IN_VIOLATION: &'static LazyLock<ViolationData> = &[< $proto_type _NOT_IN_VIOLATION >];

        fn is_in(&self, list: &SortedList<Self::LookupTarget>) -> bool {
          list.contains(self)
        }
      }
    }
  };

  // Wrappers
  ($wrapper:ty, $target:ty, $proto_type:ident) => {
    paste::paste! {
      impl ListRules for $wrapper {
        type LookupTarget = $target;
        const IN_VIOLATION: &'static LazyLock<ViolationData> = &[< $proto_type _IN_VIOLATION >];
        const NOT_IN_VIOLATION: &'static LazyLock<ViolationData> = &[< $proto_type _NOT_IN_VIOLATION >];

        fn is_in(&self, list: &SortedList<Self::LookupTarget>) -> bool {
          list.contains(self)
        }
      }
    }
  };
}

impl ListRules for f32 {
  type LookupTarget = OrderedFloat<Self>;
  const IN_VIOLATION: &'static LazyLock<ViolationData> = &FLOAT_IN_VIOLATION;
  const NOT_IN_VIOLATION: &'static LazyLock<ViolationData> = &FLOAT_NOT_IN_VIOLATION;

  fn is_in(&self, list: &SortedList<Self::LookupTarget>) -> bool {
    list.contains(&((*self).into()))
  }
}

impl ListRules for f64 {
  type LookupTarget = OrderedFloat<Self>;
  const IN_VIOLATION: &'static LazyLock<ViolationData> = &DOUBLE_IN_VIOLATION;
  const NOT_IN_VIOLATION: &'static LazyLock<ViolationData> = &DOUBLE_NOT_IN_VIOLATION;

  fn is_in(&self, list: &SortedList<Self::LookupTarget>) -> bool {
    list.contains(&((*self).into()))
  }
}

// Wrappers
impl_lookup!(EnumVariant, i32, ENUM);
impl_lookup!(Sint64, i64, SINT64);
impl_lookup!(Sint32, i32, SINT32);
impl_lookup!(Sfixed64, i64, SFIXED64);
impl_lookup!(Sfixed32, i32, SFIXED32);
impl_lookup!(Fixed64, u64, FIXED64);
impl_lookup!(Fixed32, u32, FIXED32);

impl_lookup!(i64, INT64);
impl_lookup!(i32, INT32);
impl_lookup!(u64, UINT64);
impl_lookup!(u32, UINT32);
impl_lookup!(Duration, DURATION);

impl ListRules for &::bytes::Bytes {
  type LookupTarget = &'static [u8];
  const IN_VIOLATION: &'static LazyLock<ViolationData> = &BYTES_IN_VIOLATION;
  const NOT_IN_VIOLATION: &'static LazyLock<ViolationData> = &BYTES_NOT_IN_VIOLATION;

  fn is_in(&self, list: &SortedList<Self::LookupTarget>) -> bool {
    list.contains(&self.as_ref())
  }
}

impl ListRules for &Any {
  type LookupTarget = &'static str;
  const IN_VIOLATION: &'static LazyLock<ViolationData> = &ANY_IN_VIOLATION;
  const NOT_IN_VIOLATION: &'static LazyLock<ViolationData> = &ANY_NOT_IN_VIOLATION;

  fn is_in(&self, list: &SortedList<Self::LookupTarget>) -> bool {
    list.contains(&self.type_url.as_str())
  }
}

impl ListRules for &str {
  type LookupTarget = &'static str;
  const IN_VIOLATION: &'static LazyLock<ViolationData> = &STRING_IN_VIOLATION;
  const NOT_IN_VIOLATION: &'static LazyLock<ViolationData> = &STRING_NOT_IN_VIOLATION;

  fn is_in(&self, list: &SortedList<Self::LookupTarget>) -> bool {
    list.contains(self)
  }
}

// We always receive refs or Copy types here anyway
#[allow(clippy::needless_pass_by_value)]
pub fn in_list<T>(
  field_context: &FieldContext,
  parent_elements: &[FieldPathElement],
  value: T,
  list: &SortedList<T::LookupTarget>,
  error_message: &str,
) -> Result<(), Violation>
where
  T: ListRules,
{
  let has_item = value.is_in(list);

  if has_item {
    Ok(())
  } else {
    Err(create_violation(
      field_context,
      T::IN_VIOLATION,
      error_message,
      parent_elements,
    ))
  }
}

#[allow(clippy::needless_pass_by_value)]
pub fn not_in_list<T>(
  field_context: &FieldContext,
  parent_elements: &[FieldPathElement],
  value: T,
  list: &SortedList<T::LookupTarget>,
  error_message: &str,
) -> Result<(), Violation>
where
  T: ListRules,
{
  let has_item = value.is_in(list);

  if has_item {
    Err(create_violation(
      field_context,
      T::NOT_IN_VIOLATION,
      error_message,
      parent_elements,
    ))
  } else {
    Ok(())
  }
}

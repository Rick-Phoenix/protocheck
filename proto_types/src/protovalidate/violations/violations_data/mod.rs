use crate::protovalidate::*;

pub mod base_violations;

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
pub mod not_in_violations;
pub use not_in_violations::*;
pub mod lte_violations;
pub use lte_violations::*;
pub mod lt_violations;
pub use lt_violations::*;
pub mod in_violations;
pub use in_violations::*;
pub mod gte_violations;
pub use gte_violations::*;
pub mod gt_violations;
pub use gt_violations::*;
pub mod const_violations;
pub use const_violations::*;
pub mod bytes_violations;
pub use base_violations::*;
pub use bytes_violations::*;

use super::*;

pub struct ViolationData {
  pub name: &'static str,
  pub elements: &'static [FieldPathElement],
}

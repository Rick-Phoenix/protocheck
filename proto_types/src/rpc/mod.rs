#![allow(clippy::doc_overindented_list_items)]

pub type QuotaFailureViolation = quota_failure::Violation;
pub type PreconditionFailureViolation = precondition_failure::Violation;

include!("./google.rpc.rs");

macro_rules! has_impl {
  ($name:ident) => {
    paste::paste! {
      #[doc = "Returns true if the `" $name "` matches the given value."]
      pub fn [< has_ $name >](&self, $name: &str) -> bool {
        self.$name == $name
      }
    }
  };

  ($name:ident, $name_override:ident) => {
    paste::paste! {
      #[doc = "Returns true if the `" $name "` matches the given value."]
      pub fn [< has_ $name >](&self, $name_override: &str) -> bool {
        self.$name_override == $name_override
      }
    }
  };

  ($name:ident, $ty:ty) => {
    paste::paste! {
      #[doc = "Returns true if the `" $name "` matches the given value."]
      pub fn [< has_ $name >](&self, $name: $ty) -> bool {
        self.$name == $name
      }
    }
  };
}

#[cfg(feature = "serde")]
mod rpc_serde_impls;

pub mod error_details;
pub mod http;

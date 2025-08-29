#![allow(clippy::doc_overindented_list_items)]

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
}

#[cfg(feature = "serde")]
mod rpc_serde_impls;

mod error_details;
mod http;
#[cfg(feature = "cel")]
mod rpc_cel_impls;

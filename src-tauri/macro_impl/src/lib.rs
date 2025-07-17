use std::collections::HashMap;

pub mod macros {
  pub use pr_macro::*;
}

pub trait ProtoMessage {
  fn get_fields(&self) -> MessageData;
}

#[derive(Debug)]
pub struct MessageData {
  pub fields: HashMap<String, ProtoField>,
}

#[derive(Debug)]
pub struct ProtoField {
  pub field_num: i32,
  pub name: String,
  pub rust_type: String,
  pub proto_type: String,
}

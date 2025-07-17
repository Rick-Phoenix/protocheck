use std::collections::HashMap;

pub mod macros {
  pub use pr_macro::*;
}

pub trait ProtoMessage {
  fn data(&self) -> MessageData;
  fn fields(&self) -> HashMap<String, ProtoField>;
}

#[derive(Debug)]
pub struct MessageData {
  pub name: String,
  pub fields: HashMap<String, ProtoField>,
  pub reserved_nums: Vec<i32>,
  pub reserved_ranges: Vec<(i32, i32)>,
  pub reserved_names: Vec<String>,
}

#[derive(Debug)]
pub struct ProtoField {
  pub field_num: i32,
  pub name: String,
  pub rust_type: String,
  pub proto_type: String,
  pub options: Option<String>,
  pub repeated: bool,
  pub optional: bool,
}

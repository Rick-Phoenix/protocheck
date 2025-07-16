pub mod macros {
  pub use pr_macro::*;
}

pub trait ProtoMessage {
  fn get_fields(&self) -> MessageData;
}

#[derive(Debug)]
pub struct MessageData {
  pub fields: Vec<(i32, String, String)>,
}

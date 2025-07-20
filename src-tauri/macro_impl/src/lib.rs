use std::collections::HashMap;

pub mod macros {
  pub use pr_macro::*;

  #[macro_export]
  macro_rules! reserved_numbers {
    () => {
        String::new()
    };

    ($num:literal, $($rest:tt)*) => {{
        let mut s = String::new();
        s.push_str(stringify!($num));
        s.push_str(", ");
        s.push_str(&$crate::reserved_numbers!($($rest)*));
        s
    }};

    ($start:literal to $end:literal, $($rest:tt)*) => {{
        let mut s = String::new();
        s.push_str(stringify!($start to $end));
        s.push_str(", ");
        s.push_str(&$crate::reserved_numbers!($($rest)*));
        s
    }};

    ($num:literal) => {
        stringify!($num).to_string()
    };

    ($start:literal to $end:literal) => {
        stringify!($start to $end).to_string()
    };
}
}

pub trait ProtoMessage {
  fn data(&self) -> MessageData;
  fn fields(&self) -> HashMap<String, ProtoField>;
  fn get_name(&self) -> &str;
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

pub trait WithValidator {
  fn validate(&self) -> bool;
}

impl ProtoField {}

use crate::rules::get_field_rules;
use bytes::Bytes;
use prost::Message;
use prost_reflect::{DescriptorPool, Value};

use crate::buf::validate::FieldRules;

#[macro_use]
extern crate lazy_static;

mod myapp {
  pub mod v1 {
    include!(concat!(env!("OUT_DIR"), "/myapp.v1.rs"));
  }
}
mod buf {
  pub mod validate {
    include!(concat!(env!("OUT_DIR"), "/buf.validate.rs"));
  }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let descriptor_set_bytes = Bytes::from(std::fs::read(std::env::var("PROTO_DESCRIPTOR_SET")?)?);
  let pool = DescriptorPool::decode(descriptor_set_bytes)?;

  let user_desc = pool
    .get_message_by_name("myapp.v1.User")
    .ok_or("User message not found")?;

  let field_ext_descriptor = pool
    .get_extension_by_name("buf.validate.field")
    .ok_or("buf.validate.field extension not found in descriptor pool")?;

  println!("--- User Message Validation Rules ---");

  for field_desc in user_desc.fields() {
    let field_name = field_desc.name();
    println!("\nField: {}", field_name);

    let field_options = field_desc.options();

    let field_rules_descriptor = field_options.get_extension(&field_ext_descriptor);

    if let Value::Message(field_rules_msg) = field_rules_descriptor.as_ref() {
      let field_rules = FieldRules::decode(field_rules_msg.encode_to_vec().as_slice())?;

      let is_required = field_rules.required();

      if let Some(rules_type) = field_rules.r#type {
        let rules = get_field_rules(&rules_type);
        println!("Rules: {:#?}", rules);
      }
    }
  }

  Ok(())
}

mod rules;

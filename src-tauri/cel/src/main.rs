use bytes::Bytes;
use prost::Message;
use prost_reflect::{DescriptorPool, Value};

use crate::buf::validate::{FieldRules, PredefinedRules}; // Make sure `bytes` crate is in your Cargo.toml

// Import generated types, though we'll primarily use reflection for this task
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
  // Load the compiled descriptor set
  let descriptor_set_bytes = Bytes::from(std::fs::read(std::env::var("PROTO_DESCRIPTOR_SET")?)?);
  let pool = DescriptorPool::decode(descriptor_set_bytes)?;

  // Get the descriptor for the `User` message from your `user.proto`
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

    // Get the FieldOptions for the current field
    let field_options = field_desc.options();

    let field_rules = field_options.get_extension(&field_ext_descriptor);

    if let Value::Message(field_rules_msg) = field_rules.as_ref() {
      println!("  Found buf.validate.field extension.");

      let field_rules = FieldRules::decode(field_rules_msg.encode_to_vec().as_slice())?;

      if let Some(type_oneof) = field_rules.r#type {
        match type_oneof {
          buf::validate::field_rules::Type::String(string_rules) => {
            println!("    Found String Rules:");
            if string_rules.max_len.is_some() {
              println!(
                "      max_len rule is set to: {}",
                string_rules.max_len.unwrap()
              );
            }

            let string_rules_desc = pool
              .get_message_by_name("buf.validate.StringRules")
              .ok_or("StringRules message not found")?;

            let max_len_desc = string_rules_desc
              .get_field_by_name("max_len")
              .ok_or("max_len descriptor not found")?;

            let max_len_options = max_len_desc.options();

            let predefined_descriptor = pool
              .get_extension_by_name("buf.validate.predefined")
              .ok_or("buf.validate.predefined not found")?;

            if let Value::Message(predefined_dynamic_msg) = max_len_options
              .get_extension(&predefined_descriptor)
              .as_ref()
            {
              let predefined_rules =
                PredefinedRules::decode(predefined_dynamic_msg.encode_to_vec().as_slice())?;

              for rule in predefined_rules.cel {
                if rule.id() == "string.max_len" {
                  println!("        String_max_len: {}", rule.expression());
                }
              }
            } else {
              println!(
                "        No predefined rules extension found on string.max_len field descriptor"
              )
            }
          }
          _ => {}
        }
      }

      if !field_rules.cel.is_empty() {
        println!("    Custom Field CEL Rules:");
        for rule in field_rules.cel {
          println!("      ID: {}", rule.id());
          println!("      Message: {}", rule.message());
          println!("      Expression: `{}`", rule.expression());
        }
      }
    }
  }

  Ok(())
}

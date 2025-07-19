#[allow(clippy::all, dead_code, unused)]
use crate::{
  buf::validate::{MessageOneofRule, MessageRules, OneofRules},
  myapp::v1::{Post, User},
  rules::get_field_rules,
};
use buf::validate::Ignore;
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

pub struct MessageOneofRuleData {
  pub fields: Vec<String>,
  pub required: bool,
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

  let message_ext_descriptor = pool
    .get_extension_by_name("buf.validate.message")
    .ok_or("buf.validate.message extension not found in descriptor pool")?;

  let oneof_ext_descriptor = pool
    .get_extension_by_name("buf.validate.oneof")
    .ok_or("buf.validate.oneof extension not found in descriptor pool")?;

  println!("--- User Message Validation Rules ---");

  let message_options = user_desc.options();

  let message_rules_descriptor = message_options.get_extension(&message_ext_descriptor);

  if let Value::Message(message_rules_msg) = message_rules_descriptor.as_ref() {
    let message_rules = MessageRules::decode(message_rules_msg.encode_to_vec().as_slice())?;

    if message_rules.cel.len() > 0 {
      let message_cel_rules = message_rules.cel.clone();
    }

    if message_rules.oneof.len() > 0 {
      let message_oneof_rules = message_rules.oneof;
    }
  }

  for oneof in user_desc.oneofs() {
    if let Value::Message(oneof_rules_msg) = oneof
      .options()
      .get_extension(&oneof_ext_descriptor)
      .as_ref()
    {
      let oneof_rules = OneofRules::decode(oneof_rules_msg.encode_to_vec().as_slice())?;
      if oneof_rules.required() {
        //
      }
    }
  }

  for field_desc in user_desc.fields() {
    let field_name = field_desc.name();
    println!("\nField: {}", field_name);

    let is_repeated = field_desc.is_list();
    let is_map = field_desc.is_map();

    let is_optional = field_desc.supports_presence();

    let field_options = field_desc.options();

    let field_rules_descriptor = field_options.get_extension(&field_ext_descriptor);

    if let Value::Message(field_rules_msg) = field_rules_descriptor.as_ref() {
      let field_rules = FieldRules::decode(field_rules_msg.encode_to_vec().as_slice())?;
      let mut ignore_val: Option<Ignore> = None;

      if field_rules.ignore.is_some() {
        match field_rules.ignore() {
          Ignore::Always => continue,
          Ignore::IfZeroValue => ignore_val = Some(Ignore::IfZeroValue),
          Ignore::Unspecified => ignore_val = Some(Ignore::Unspecified),
        }
      }

      let is_required = field_rules.required();

      if field_rules.cel.len() > 0 {
        let cel_rules = field_rules.cel.clone();
      }

      let rules = get_field_rules(&field_rules);
      println!("Rules: {:#?}", rules);
    }
  }

  let user = User {
    created_at: None,
    id: Some(1),
    name: "Me".to_string(),
    posts: Vec::<Post>::new(),
  };

  Ok(())
}

pub trait WithValidator {
  fn validate(&self) -> bool;
}

impl WithValidator for User {
  fn validate(&self) -> bool {
    let program = Program::compile("this.name == 'nonme'").unwrap();
    let mut context = Context::default();

    context.add_variable("this", self).unwrap();

    let value = program.execute(&context).unwrap();
    cel_interpreter::Value::Bool(value)
  }
}

use cel_interpreter::{Context, Program};
use serde::{Deserialize, Serialize};

mod rules;

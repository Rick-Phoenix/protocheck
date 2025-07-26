use std::collections::HashMap;

use protocheck::validators::WithValidator;

use crate::myapp::v1::{
  user::{OneofFields, Post},
  User,
};

mod myapp {
  pub mod v1 {
    include!(concat!(env!("OUT_DIR"), "/myapp.v1.rs"));
  }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let user = User {
    created_at: None,
    id: 1,
    value: vec![],
    posts: vec![
      Post {
        tags: vec!["me".to_string()],
        name: vec!["not_alfonso".to_string(), "also_not_alfonso".to_string()],
      },
      Post {
        tags: vec!["me".to_string()],
        name: vec!["not_alfonso".to_string(), "also_not_alfonso".to_string()],
      },
    ],
    oneof_fields: Some(OneofFields::Field1("aa".to_string())),
  };

  let result = user.validate();
  println!("{:#?}", result);

  Ok(())
}

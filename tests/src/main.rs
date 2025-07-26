use std::collections::HashMap;

use protocheck::validators::WithValidator;

use crate::myapp::v1::{user::Post, User};

mod myapp {
  pub mod v1 {
    include!(concat!(env!("OUT_DIR"), "/myapp.v1.rs"));
  }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let mut map_field: HashMap<String, Post> = HashMap::new();
  map_field.insert(
    "aa".to_string(),
    Post {
      tags: vec!["me".to_string(), "me".to_string()],
      name: vec!["alfonso".to_string()],
    },
  );
  let mut map_enum_field: HashMap<String, i32> = HashMap::new();
  map_enum_field.insert("enum_map_field1".to_string(), 15);
  map_enum_field.insert("enum_map_field2".to_string(), 16);
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
    map_field,
    enum_field: map_enum_field,
    oneof_fields: None,
  };

  let result = user.validate();
  println!("{:#?}", result);

  Ok(())
}

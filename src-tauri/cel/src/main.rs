use std::collections::HashMap;

use crate::myapp::v1::user::Post;
use crate::myapp::v1::User;
use macro_impl::validators::WithValidator;

mod myapp {
  pub mod v1 {
    include!(concat!(env!("OUT_DIR"), "/myapp.v1.rs"));
  }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let mut map_field: HashMap<String, Post> = HashMap::new();
  map_field.insert(
    "a".to_string(),
    Post {
      tags: vec!["me".to_string(), "me".to_string()],
      name: vec!["alfonso".to_string()],
    },
  );
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
    enum_field: 31,
    oneof_fields: None,
  };

  let result = user.validate();
  println!("{:#?}", result);

  Ok(())
}

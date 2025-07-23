use std::collections::HashMap;

use crate::myapp::v1::user::Post;
use crate::myapp::v1::User;
use cel_test::__protobuf_validators_consts;
use macro_impl::validators::WithValidator;

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
      tags: vec!["mee".to_string(), "mee".to_string(), "mee".to_string()],
    },
  );
  let user = User {
    created_at: None,
    id: 1,
    name: "M".to_string(),
    value: vec![],
    posts: vec![
      Post {
        tags: vec!["m".to_string(), "m".to_string(), "m".to_string()],
      },
      Post {
        tags: vec!["m".to_string(), "m".to_string(), "m".to_string()],
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

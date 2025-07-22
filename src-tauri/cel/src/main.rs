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
  let mut map_field: HashMap<String, String> = HashMap::new();
  map_field.insert("aa".to_string(), "aa".to_string());
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
  };

  let result = user.validate();
  println!("{:#?}", result);

  Ok(())
}

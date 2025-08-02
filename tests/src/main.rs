mod myapp {
  pub mod v1 {
    include!(concat!(env!("OUT_DIR"), "/myapp.v1.rs"));
  }
}

use std::collections::HashMap;

use protocheck::{types::Duration, validators::ProtoValidator};

use crate::myapp::v1::{user::Post, User};

fn main() {
  let mut dur_map: HashMap<String, Duration> = HashMap::new();
  dur_map.insert("abc".to_string(), Duration::new(1000, 0));

  let post = Post { dur_map };

  let mut posts_map: HashMap<String, Post> = HashMap::new();
  posts_map.insert("post1".to_string(), post);

  let user = User {
    posts_map,
    details: None,
  };

  let _result = user.validate();
  println!("{:#?}", _result);
}

mod myapp {
  pub mod v1 {
    include!(concat!(env!("OUT_DIR"), "/myapp.v1.rs"));
  }
}

use std::collections::HashMap;

use protocheck::{types::Duration, validators::ProtoValidator};

use crate::myapp::v1::{user::Post, User};

fn main() {
  let user = User {
    duration_field: Some(Duration::new(1000, 0)),
    post: Some(Post { created_at: None }),
    map_field: HashMap::new(),
  };

  let _result = user.validate();
  // println!("{:#?}", result);
}

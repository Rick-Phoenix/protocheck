mod myapp {
  pub mod v1 {
    include!(concat!(env!("OUT_DIR"), "/myapp.v1.rs"));
  }
}

use std::collections::HashMap;

use protocheck::{types::Duration, validators::ProtoValidator};

use crate::myapp::v1::{user::Person, User};

fn main() {
  // let mut friends: HashMap<String, String> = HashMap::new();
  // friends.insert("friend1".to_string(), "alfio".to_string());

  let name = "alfio".to_string();
  let person = Person { name };

  let user = User {
    duration_field: Some(Duration::default()),
    friend: Some(person.clone()),
  };

  let _result = user.validate();
  println!("{:#?}", _result);
}

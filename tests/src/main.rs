mod myapp {
  pub mod v1 {
    include!(concat!(env!("OUT_DIR"), "/myapp.v1.rs"));
  }
}

use std::collections::HashMap;

use protocheck::{types::Duration, validators::ProtoValidator};

use crate::myapp::v1::{
  user::{self, Friend},
  User,
};

fn main() {
  let mut friends: HashMap<String, Friend> = HashMap::new();
  friends.insert(
    "friend1".to_string(),
    Friend {
      name: "alfio".to_string(),
    },
  );

  let user = User {
    values: Some(user::Values::NestedUser(Box::new(User {
      values: None,
      duration_field: Some(Duration::default()),
    }))),
    duration_field: None,
  };

  let _result = user.validate();
  println!("{:#?}", _result);
}

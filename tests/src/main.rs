mod myapp {
  pub mod v1 {
    include!(concat!(env!("OUT_DIR"), "/myapp.v1.rs"));
  }
}

use protocheck::validators::ProtoValidator;

use crate::myapp::v1::{
  user::{OneofFields, Post},
  User,
};

fn main() {
  let user = User {
    created_at: None,
    id: 1,
    value: vec![],
    post: Some(Post {
      tags: vec!["me".to_string()],
      name: "not_alfonso".to_string(),
      created_at: None,
    }),
    oneof_fields: Some(OneofFields::Field1("abab".to_string())),
    // oneof_fields: None,
  };

  let result = user.validate();
  println!("{:#?}", result);
}

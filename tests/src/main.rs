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
    oneof_fields: Some(OneofFields::OneofEnum2(33)),
    // oneof_fields: Some(OneofFields::Post(Post {
    //   tags: vec!["me".to_string()],
    //   name: "not_alfonso".to_string(),
    //   created_at: None,
    // })),
    outer_enum_field: None,
    post: None,
  };

  let result = user.validate();
  println!("{:#?}", result);
}

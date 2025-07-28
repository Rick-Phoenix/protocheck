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
    posts: vec![
      Post {
        tags: vec!["me".to_string()],
        name: vec!["not_alfonso".to_string(), "also_not_alfonso".to_string()],
      },
      Post {
        tags: vec!["meeeeeee".to_string()],
        name: vec!["not_alfonso".to_string(), "also_not_alfonso".to_string()],
      },
    ],
    oneof_fields: Some(OneofFields::Post(Post {
      tags: vec!["me".to_string()],
      name: vec!["not_alfonso".to_string(), "also_not_alfonso".to_string()],
    })),
    // oneof_fields: None,
  };

  let result = user.validate();
  println!("{:#?}", result);
}

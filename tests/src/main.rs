mod myapp {
  pub mod v1 {
    include!(concat!(env!("OUT_DIR"), "/myapp.v1.rs"));
  }
}

use protocheck::{types::Duration, validators::ProtoValidator};

use crate::myapp::v1::{user::Post, User};

fn main() {
  let post = Post {
    duration: Some(Duration::new(1000, 0)),
    nested_post: None,
  };
  let post2 = Post {
    duration: Some(Duration::new(1000, 0)),
    nested_post: Some(Box::new(post.clone())),
  };

  let user = User {
    posts: Some(post2),
    // posts: vec![post, post2],
    details: None,
  };

  let _result = user.validate();
  println!("{:#?}", _result);
}

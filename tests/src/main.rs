mod myapp {
  pub mod v1 {
    include!(concat!(env!("OUT_DIR"), "/myapp.v1.rs"));
  }
}

use protocheck::{
  types::{field_descriptor_proto::Type, Duration},
  validators::ProtoValidator,
};

use crate::myapp::v1::{
  user::{post::PostOneof, Post},
  User,
};

fn main() {
  let test_duration = Duration::default();

  let post = Post {
    post_oneof: Some(PostOneof::NestedPost(Box::new(Post {
      id: 3,
      outer_duration: Some(test_duration),
      post_oneof: None,
    }))),
    id: 2,
    outer_duration: None,
  };

  let post2 = Post {
    post_oneof: Some(PostOneof::NestedPost(Box::new(Post {
      id: 3,
      outer_duration: Some(test_duration),
      post_oneof: Some(PostOneof::Duration(Duration::new(1000, 0))),
    }))),
    id: 2,
    outer_duration: None,
  };
  let post3 = Post {
    post_oneof: Some(PostOneof::NestedPost(Box::new(Post {
      id: 3,
      outer_duration: Some(test_duration),
      post_oneof: Some(PostOneof::Duration(Duration::new(1000, 0))),
    }))),
    id: 2,
    outer_duration: None,
  };

  let user = User {
    posts: vec![post, post2, post3],
  };

  let _result = user.validate();
  println!("{:#?}", _result);
}

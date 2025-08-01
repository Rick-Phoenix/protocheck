mod myapp {
  pub mod v1 {
    include!(concat!(env!("OUT_DIR"), "/myapp.v1.rs"));
  }
}

// use prost_reflect::DescriptorPool;
use protocheck::{types::Duration, validators::ProtoValidator};

use crate::myapp::v1::{user::Post, User};

fn main() {
  let user = User {
    duration_field: Some(Duration::new(1000, 0)),
    post: Some(Post { created_at: None }),
  };

  let result = user.validate();
  println!("{:#?}", result);

  // let descriptor_pool = DescriptorPool::global();
  // for message in descriptor_pool.all_messages() {
  //   println!("{}", message.full_name());
  // }
}

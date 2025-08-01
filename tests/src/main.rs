mod myapp {
  pub mod v1 {
    include!(concat!(env!("OUT_DIR"), "/myapp.v1.rs"));
  }
}

const FILE_DESCRIPTOR_SET_BYTES: &[u8] =
  include_bytes!(concat!(env!("OUT_DIR"), "/file_descriptor_set.bin"));

static DESCRIPTOR_POOL: LazyLock<DescriptorPool> = LazyLock::new(|| {
  DescriptorPool::decode(FILE_DESCRIPTOR_SET_BYTES).expect("Failed to decode file descriptor set")
});

use std::sync::LazyLock;

use prost_reflect::DescriptorPool;
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
  println!("{}", user.duration_field.unwrap());
}

mod myapp {
  pub mod v1 {
    include!(concat!(env!("OUT_DIR"), "/myapp.v1.rs"));
  }
}

use protocheck::{types::Duration, validators::ProtoValidator};

use crate::myapp::v1::User;

fn main() {
  let user = User {
    duration_field: Some(Duration::default()),
  };

  let result = user.validate();
  println!("{:#?}", result);
}

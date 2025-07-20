use crate::myapp::v1::{Post, User};
use macro_impl::validators::WithValidator;

mod myapp {
  pub mod v1 {
    include!(concat!(env!("OUT_DIR"), "/myapp.v1.rs"));
  }
}
mod buf {
  pub mod validate {
    include!(concat!(env!("OUT_DIR"), "/buf.validate.rs"));
  }
}
mod google {
  pub mod protobuf {
    include!(concat!(env!("OUT_DIR"), "/google.protobuf.rs"));
  }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let user = User {
    created_at: None,
    id: 1,
    name: "Me".to_string(),
    posts: Vec::<Post>::new(),
    value: vec!["me".to_string()],
  };

  let result = user.validate();
  println!("{:#?}", result);

  Ok(())
}

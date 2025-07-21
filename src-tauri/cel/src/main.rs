use crate::myapp::v1::user::Post;
use crate::myapp::v1::User;
use macro_impl::validators::WithValidator;

mod myapp {
  pub mod v1 {
    include!(concat!(env!("OUT_DIR"), "/myapp.v1.rs"));
  }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let user = User {
    created_at: None,
    id: 1,
    name: "M".to_string(),
    value: vec![],
    posts: vec![
      Post {
        tags: vec!["me".to_string(), "mee".to_string(), "meeee".to_string()],
      },
      Post {
        tags: vec!["me".to_string(), "mee".to_string(), "meeee".to_string()],
      },
    ],
  };

  let result = user.validate();
  println!("{:#?}", result);

  Ok(())
}

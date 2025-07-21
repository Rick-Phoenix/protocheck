use crate::validators::buf::validate::Violations;

pub mod buf {
  pub mod validate {
    include!(concat!(env!("OUT_DIR"), "/buf.validate.rs"));
  }
}
mod google {
  pub mod protobuf {
    include!(concat!(env!("OUT_DIR"), "/google.protobuf.rs"));
  }
}

pub trait WithValidator {
  fn validate(&self) -> Result<(), Violations>;
}

pub mod strings;

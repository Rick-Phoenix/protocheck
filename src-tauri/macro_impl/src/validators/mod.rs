use proto_types::buf::validate::Violations;

pub trait WithValidator {
  fn validate(&self) -> Result<(), Violations>;
}

pub mod strings;

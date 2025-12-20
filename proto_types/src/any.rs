// From (prost-types)[https://github.com/tokio-rs/prost/blob/master/prost-types/src/any.rs]
use super::*;
use crate::constants::PACKAGE_PREFIX;

impl Any {
  /// Serialize the given message type `M` as [`Any`].
  pub fn from_msg<M>(msg: &M) -> Result<Self, EncodeError>
  where
    M: Name,
  {
    let type_url = M::type_url();

    let mut value = Vec::new();

    Message::encode(msg, &mut value)?;

    Ok(Self { type_url, value })
  }

  /// Decode the given message type `M` from [`Any`], validating that it has
  /// the expected type URL.
  pub fn to_msg<M>(&self) -> Result<M, DecodeError>
  where
    M: Default + Name + Sized,
  {
    let expected_type_url = M::type_url();

    if let (Some(expected), Some(actual)) = (
      TypeUrl::new(&expected_type_url),
      TypeUrl::new(&self.type_url),
    )
      && expected == actual {
        return M::decode(self.value.as_slice());
      }

    let mut err = DecodeError::new(format!(
      "expected type URL: \"{}\" (got: \"{}\")",
      expected_type_url, &self.type_url
    ));

    err.push("unexpected type URL", "type_url");

    Err(err)
  }
}

impl Name for Any {
  const PACKAGE: &'static str = PACKAGE_PREFIX;

  const NAME: &'static str = "Any";

  fn type_url() -> String {
    type_url_for::<Self>()
  }
}

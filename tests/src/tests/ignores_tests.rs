use bytes::Bytes;

use crate::myapp::v1::{IgnoreAlwaysTest, IgnoreIfZeroValueTest};

#[test]
fn ignore_always_test() {
  let test = IgnoreAlwaysTest {
    name: "abcdefg".to_string(),
    bytes: Bytes::from_static(b"blahblahblah"),
    int64: 1,
    int32: 1,
    uint64: 1,
    uint32: 1,
    sint64: 1,
    sint32: 1,
    fixed64: 1,
    fixed32: 1,
    sfixed64: 1,
    sfixed32: 1,
    enum_field: 1,
  };

  assert!(test.validate().is_ok());
}

#[test]
fn ignore_if_zero_value_test() {
  let test = IgnoreIfZeroValueTest {
    name: "".to_string(),
    bytes: Bytes::default(),
    int64: 0,
    int32: 0,
    uint64: 0,
    uint32: 0,
    sint64: 0,
    sint32: 0,
    fixed64: 0,
    fixed32: 0,
    sfixed64: 0,
    sfixed32: 0,
    enum_field: 0,
  };

  assert!(test.validate().is_ok());
}

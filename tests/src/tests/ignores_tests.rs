use bytes::Bytes;
use protocheck::types::{protovalidate::Violations, Duration, Timestamp};

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
    duration_field: None,
    timestamp_field: None,
  };

  assert!(test.validate().is_ok());

  let test = IgnoreIfZeroValueTest {
    name: "abcde".to_string(),
    bytes: Bytes::from_static(b"they're taking the hobbits to isengard"),
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
    duration_field: Some(Duration::default()),
    timestamp_field: Some(Timestamp::default()),
  };

  let Violations { violations } = test.validate().unwrap_err();

  assert_eq!(violations.len(), 15);

  let test = IgnoreIfZeroValueTest {
    name: "abc".to_string(),
    bytes: Bytes::from_static(b"abc"),
    int64: 15,
    int32: 15,
    uint64: 15,
    uint32: 15,
    sint64: 15,
    sint32: 15,
    fixed64: 15,
    fixed32: 15,
    sfixed64: 15,
    sfixed32: 15,
    enum_field: 15,
    duration_field: Some(Duration::new(3600, 0)),
    timestamp_field: Some(Timestamp::new(3600, 0)),
  };

  assert!(test.validate().is_ok());
}

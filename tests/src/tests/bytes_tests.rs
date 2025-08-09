use bytes::Bytes;
use protocheck::types::protovalidate::Violations;

use crate::myapp::v1::BytesTests;

#[test]
fn bytes_tests() {
  let example1 = b"they're taking the hobbits to isengard";

  let test = BytesTests {
    contains_field: Bytes::from_static(example1),
    prefix_field: Bytes::from_static(example1),
    pattern_field: Bytes::from_static(example1),
    suffix_field: Bytes::from_static(example1),
  };

  let Violations { violations } = test.validate().unwrap_err();

  assert_eq!(violations.len(), 4);

  let correct = b"abc";

  let test = BytesTests {
    contains_field: Bytes::from_static(correct),
    prefix_field: Bytes::from_static(correct),
    pattern_field: Bytes::from_static(correct),
    suffix_field: Bytes::from_static(correct),
  };

  assert!(test.validate().is_ok())
}

use protocheck::types::protovalidate::Violations;

use crate::myapp::v1::OptionalTests;

#[test]
fn optional_tests() {
  let test = OptionalTests {
    double_field: None,
    int64_field: None,
    float_field: None,
    name: None,
    bytes_field: None,
  };

  assert!(test.validate().is_ok());

  let test2 = OptionalTests {
    double_field: Some(1.0),
    bytes_field: Some(bytes::Bytes::new()),
    float_field: Some(1.0),
    name: Some("".to_string()),
    int64_field: Some(1),
  };

  let Violations { violations } = test2.validate().unwrap_err();

  assert_eq!(violations.len(), 5);
}

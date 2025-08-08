use bytes::Bytes;
use protocheck::types::{
  protovalidate::{Violation, Violations},
  Any, Duration,
};

use crate::myapp::v1::ContainingTests;

#[test]
fn containing_tests() {
  let test_message = ContainingTests {
    double_field: 2.0,
    float_field: 2.0,
    duration_field: Some(Duration::default()),
    int64_field: 5,
    name: "ermenegildo".to_string(),
    any_field: Some(Any {
      type_url: "type.googleapis.com/Nope".to_string(),
      value: vec![],
    }),
    enum_field: 15,
    bytes_field: Bytes::from_static(b"\x02\x03"),
  };

  let Violations { violations } = test_message.validate().unwrap_err();

  println!("{:#?}", violations);

  assert_eq!(violations.len(), 17);

  let in_violations: Vec<&Violation> = violations
    .iter()
    .filter(|v| !v.rule_id().ends_with("not_in") && v.rule_id() != "enum.defined_only")
    .collect();

  assert_eq!(in_violations.len(), 8);

  let not_in_violations: Vec<&Violation> = violations
    .iter()
    .filter(|v| v.rule_id().ends_with("not_in"))
    .collect();

  assert_eq!(not_in_violations.len(), 8);

  assert!(violations
    .iter()
    .any(|v| v.rule_id() == "enum.defined_only"));
}

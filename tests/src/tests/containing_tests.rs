use protocheck::{
  types::{
    protovalidate::{Violation, Violations},
    Duration,
  },
  validators::ProtoValidator,
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
  };

  let Violations { violations } = test_message.validate().unwrap_err();

  assert_eq!(violations.len(), 10);

  let in_violations: Vec<&Violation> = violations
    .iter()
    .filter(|v| !v.rule_id().ends_with("not_in"))
    .collect();

  assert_eq!(in_violations.len(), 5);

  let not_in_violations: Vec<&Violation> = violations
    .iter()
    .filter(|v| v.rule_id().ends_with("not_in"))
    .collect();

  assert_eq!(not_in_violations.len(), 5);

  println!("{:#?}", violations);
}

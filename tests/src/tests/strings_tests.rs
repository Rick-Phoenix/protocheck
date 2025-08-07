use protocheck::types::protovalidate::Violations;

use crate::myapp::v1::StringTests;

#[test]
fn string_tests() {
  let invalid_email = "//anakin@darkforce@.com".to_string();
  let example1 = "they're taking the hobbits to isengard".to_string();
  let example2 = "abc".to_string();

  let string_tests = StringTests {
    email: invalid_email,
    regex_test: example1.clone(),
    contains_test: example1.clone(),
    not_contains_test: example2.clone(),
    prefix_test: example1.clone(),
    suffix_test: example1.clone(),
  };

  let result = string_tests.validate().unwrap_err();

  let Violations { violations } = result;
  assert_eq!(violations.len(), 6);

  assert!(violations.iter().any(|v| v.rule_id() == "string.email"));
  assert!(violations.iter().any(|v| v.rule_id() == "string.pattern"));
  assert!(violations.iter().any(|v| v.rule_id() == "string.contains"));
  assert!(violations
    .iter()
    .any(|v| v.rule_id() == "string.not_contains"));
  assert!(violations.iter().any(|v| v.rule_id() == "string.prefix"));
  assert!(violations.iter().any(|v| v.rule_id() == "string.suffix"));

  let valid_test = StringTests {
    email: "obiwan@force.com".to_string(),
    regex_test: example2.clone(),
    contains_test: example2.clone(),
    not_contains_test: example1.clone(),
    prefix_test: example2.clone(),
    suffix_test: example2.clone(),
  };

  let valid_result = valid_test.validate();

  assert!(valid_result.is_ok());
}

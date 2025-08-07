use protocheck::{types::protovalidate::Violation, validators::ProtoValidator};

use crate::myapp::v1::{repeated_test::Person, RepeatedTest};

#[test]
fn repeated_tests() {
  let person = Person {
    name: "lucrezio".to_string(),
  };

  let people = vec![person.clone(), person.clone()];

  let unique_floats: Vec<f32> = vec![1.1, 2.2];
  let unique_doubles: Vec<f64> = vec![1.1, 2.2];
  let unique_strings = vec!["ignazio".to_string(), "ignazio".to_string()];

  let msg = RepeatedTest {
    people,
    unique_floats,
    unique_doubles,
    unique_strings,
  };

  let result = msg.validate().unwrap_err();

  assert_eq!(result.violations.len(), 7);

  let message_level_violations: Vec<&Violation> = result
    .violations
    .iter()
    .filter(|v| v.rule_id() == "message.person_name")
    .collect();

  assert_eq!(message_level_violations.len(), 2);

  for v in &message_level_violations {
    assert_eq!(v.rule_path().unwrap(), "cel");
  }

  let message_field_violations: Vec<&Violation> = result
    .violations
    .iter()
    .filter(|v| v.rule_id() == "message_field.person_name")
    .collect();

  assert_eq!(message_field_violations.len(), 2);

  for v in &message_field_violations {
    assert_eq!(v.parent_field().unwrap().field_name(), "people");
    assert_eq!(v.rule_path().unwrap(), "cel");
  }

  let field_level_violations: Vec<&Violation> = result
    .violations
    .iter()
    .filter(|v| v.rule_id() == "repeated_item.person_name")
    .collect();

  assert_eq!(field_level_violations.len(), 2);

  for v in &field_level_violations {
    assert_eq!(v.rule_path().unwrap(), "repeated.items.cel");
  }
}

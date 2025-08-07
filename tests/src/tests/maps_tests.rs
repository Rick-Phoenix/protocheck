use maplit::hashmap;
use protocheck::{
  types::{
    field_descriptor_proto::Type, protovalidate::field_path_element::Subscript, Duration, Timestamp,
  },
  validators::ProtoValidator,
};

use crate::myapp::v1::{BasicMap, DurationMap, TimestampMap};

#[test]
fn timestamp_map() {
  let timestamp_map = hashmap! { "hello there".to_string() => Timestamp::default(), "general kenobi".to_string() => Timestamp::default() };

  let msg = TimestampMap { timestamp_map };

  let result = msg.validate().unwrap_err();

  assert_eq!(result.violations.len(), 4);

  let values_gt_now_violation = result.violation_by_rule_id("timestamp.gt_now").unwrap();
  let violation_field = values_gt_now_violation.last_field().unwrap();

  assert_eq!(violation_field.key_type(), Type::String);
  assert_eq!(violation_field.value_type(), Type::Message);

  // First violation
  assert!(result
    .violations
    .iter()
    .any(|v| v.rule_id() == "timestamp.gt_now"
      && v.last_field().unwrap().subscript
        == Some(Subscript::StringKey("hello there".to_string()))));

  // Second violation, with correct subscript
  assert!(result
    .violations
    .iter()
    .any(|v| v.rule_id() == "timestamp.gt_now"
      && v.last_field().unwrap().subscript
        == Some(Subscript::StringKey("general kenobi".to_string()))));

  assert_eq!(values_gt_now_violation.for_key(), false);
  assert_eq!(
    values_gt_now_violation.rule_path(),
    Some("map.values.timestamp.gt_now".to_string())
  );

  let values_cel_violation = result.violation_by_rule_id("timestamp_map_value").unwrap();

  // First violation
  assert!(result
    .violations
    .iter()
    .any(|v| v.rule_id() == "timestamp_map_value"
      && v.last_field().unwrap().subscript
        == Some(Subscript::StringKey("hello there".to_string()))));

  // Second violation, with correct subscript
  assert!(result
    .violations
    .iter()
    .any(|v| v.rule_id() == "timestamp_map_value"
      && v.last_field().unwrap().subscript
        == Some(Subscript::StringKey("general kenobi".to_string()))));

  assert_eq!(values_cel_violation.for_key(), false);
  assert_eq!(
    values_cel_violation.rule_path(),
    Some("map.values.cel".to_string())
  );
}

#[test]
fn duration_map() {
  let duration_map = hashmap! { "hello there".to_string() => Duration::default(), "general kenobi".to_string() => Duration::default() };

  let msg = DurationMap { duration_map };

  let result = msg.validate().unwrap_err();

  let values_gt_violation = result.violation_by_rule_id("duration.gt").unwrap();
  let violation_field = values_gt_violation.last_field().unwrap();

  assert_eq!(result.violations.len(), 4);

  assert_eq!(violation_field.key_type(), Type::String);
  assert_eq!(violation_field.value_type(), Type::Message);

  // First violation
  assert!(result
    .violations
    .iter()
    .any(|v| v.rule_id() == "duration.gt"
      && v.last_field().unwrap().subscript
        == Some(Subscript::StringKey("hello there".to_string()))));

  // Second violation, with correct subscript
  assert!(result
    .violations
    .iter()
    .any(|v| v.rule_id() == "duration.gt"
      && v.last_field().unwrap().subscript
        == Some(Subscript::StringKey("general kenobi".to_string()))));

  assert_eq!(values_gt_violation.for_key(), false);
  assert_eq!(
    values_gt_violation.rule_path(),
    Some("map.values.duration.gt".to_string())
  );

  let values_cel_violation = result.violation_by_rule_id("duration_map_value").unwrap();

  // First violation
  assert!(result
    .violations
    .iter()
    .any(|v| v.rule_id() == "duration_map_value"
      && v.last_field().unwrap().subscript
        == Some(Subscript::StringKey("hello there".to_string()))));

  // Second violation, with correct subscript
  assert!(result
    .violations
    .iter()
    .any(|v| v.rule_id() == "duration_map_value"
      && v.last_field().unwrap().subscript
        == Some(Subscript::StringKey("general kenobi".to_string()))));

  assert_eq!(values_cel_violation.for_key(), false);
  assert_eq!(
    values_cel_violation.rule_path(),
    Some("map.values.cel".to_string())
  );
}

#[test]
fn basic_map() {
  let string_map = hashmap! { "hi".to_string() => "there".to_string() };

  let msg = BasicMap { string_map };

  let result = msg.validate().unwrap_err();

  assert_eq!(result.violations.len(), 5);

  let min_pairs_violation = result.violation_by_rule_id("map.min_pairs").unwrap();
  let violation_field = min_pairs_violation.last_field().unwrap();

  assert_eq!(violation_field.key_type(), Type::String);
  assert_eq!(violation_field.value_type(), Type::String);
  assert_eq!(violation_field.subscript, None);
  assert_eq!(min_pairs_violation.for_key(), false);
  assert_eq!(
    min_pairs_violation.rule_path(),
    Some("map.min_pairs".to_string())
  );

  let min_len_violation = result.violation_by_rule_id("string.min_len").unwrap();
  let violation_field = min_len_violation.last_field().unwrap();

  assert_eq!(violation_field.key_type(), Type::String);
  assert_eq!(violation_field.value_type(), Type::String);
  assert_eq!(
    violation_field.subscript,
    Some(Subscript::StringKey("hi".to_string()))
  );
  assert_eq!(min_len_violation.for_key(), true);
  assert_eq!(
    min_len_violation.rule_path(),
    Some("map.keys.string.min_len".to_string())
  );

  let max_len_violation = result.violation_by_rule_id("string.max_len").unwrap();
  let violation_field = max_len_violation.last_field().unwrap();

  assert_eq!(violation_field.key_type(), Type::String);
  assert_eq!(violation_field.value_type(), Type::String);
  assert_eq!(
    violation_field.subscript,
    Some(Subscript::StringKey("hi".to_string()))
  );
  assert_eq!(max_len_violation.for_key(), false);
  assert_eq!(
    max_len_violation.rule_path(),
    Some("map.values.string.max_len".to_string())
  );

  let keys_cel_violation = result.violation_by_rule_id("map_key_cel").unwrap();
  let violation_field = keys_cel_violation.last_field().unwrap();

  assert_eq!(
    violation_field.subscript,
    Some(Subscript::StringKey("hi".to_string()))
  );
  assert_eq!(keys_cel_violation.for_key(), true);

  let values_cel_violation = result.violation_by_rule_id("map_value_cel").unwrap();
  let violation_field = values_cel_violation.last_field().unwrap();

  assert_eq!(
    violation_field.subscript,
    Some(Subscript::StringKey("hi".to_string()))
  );
  assert_eq!(values_cel_violation.for_key(), false);
}

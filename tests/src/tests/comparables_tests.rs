use protocheck::types::{protovalidate::Violations, Duration, Timestamp};

use crate::myapp::v1::ComparableRulesTests;

#[test]
fn comparables() {
  let timestamp1 = Timestamp::new(100, 0);
  let timestamp2 = Timestamp::new(0, 100);
  let timestamp3 = Timestamp::new(200, 0);

  let duration1 = Duration::new(100, 0);
  let duration2 = Duration::new(0, 100);
  let duration3 = Duration::new(200, 0);

  let test = ComparableRulesTests {
    timestamp_gt: Some(timestamp1),
    timestamp_gte: Some(timestamp2),
    timestamp_lt: Some(timestamp1),
    timestamp_lte: Some(timestamp3),
    duration_gt: Some(duration1),
    duration_gte: Some(duration2),
    duration_lt: Some(duration1),
    duration_lte: Some(duration3),
    int64_gt: 5,
    int64_gte: 4,
    int64_lt: 5,
    int64_lte: 6,
  };

  let Violations { violations } = test.validate().unwrap_err();

  assert_eq!(violations.len(), 12);

  let test = ComparableRulesTests {
    timestamp_gt: Some(timestamp3),
    timestamp_gte: Some(timestamp1),
    timestamp_lt: Some(timestamp2),
    timestamp_lte: Some(timestamp1),
    duration_gt: Some(duration3),
    duration_gte: Some(duration1),
    duration_lt: Some(duration2),
    duration_lte: Some(duration1),
    int64_gt: 6,
    int64_gte: 5,
    int64_lt: 4,
    int64_lte: 5,
  };

  assert!(test.validate().is_ok());
}

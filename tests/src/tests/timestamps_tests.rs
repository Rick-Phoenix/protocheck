use protocheck::types::{protovalidate::Violations, Duration, Timestamp};

use crate::myapp::v1::TimestampTests;

#[test]
fn timestamp() {
  let far_past_timestamp = Timestamp::default();
  let future_timestamp = Timestamp::now() + Duration::new(3000, 0);

  let test = TimestampTests {
    gt_now: Some(far_past_timestamp),
    within: Some(far_past_timestamp),
    lt_now: Some(future_timestamp),
  };

  let Violations { violations } = test.validate().unwrap_err();

  assert_eq!(violations.len(), 3);

  let test = TimestampTests {
    gt_now: Some(future_timestamp),
    within: Some(future_timestamp),
    lt_now: Some(far_past_timestamp),
  };

  assert!(test.validate().is_ok())
}

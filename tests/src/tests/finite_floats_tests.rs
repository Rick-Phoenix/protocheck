use protocheck::types::protovalidate::Violations;

use crate::myapp::v1::FiniteRulesTests;

#[test]
fn finite_floats() {
  let test = FiniteRulesTests {
    finite_float: f32::NAN,
    finite_double: f64::NAN,
  };

  let Violations { violations } = test.validate().unwrap_err();

  assert_eq!(violations.len(), 2);
}

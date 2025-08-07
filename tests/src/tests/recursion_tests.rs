use protocheck::{types::protovalidate::Violations, validators::ProtoValidator};

use crate::myapp::v1::{recursive::Oneofs, Recursive};

#[test]
fn recursion_test() {
  let rec1 = Recursive {
    id: 20,
    recursive: None,
    oneofs: None,
  };

  let recursive = Recursive {
    id: 11,
    recursive: Some(Box::new(rec1.clone())),
    oneofs: Some(Oneofs::RecursiveOneof(Box::new(rec1.clone()))),
  };

  let Violations { violations } = recursive.validate().unwrap_err();

  let err1 = violations.iter().any(|v| v.rule_id() == "id.is_2");
  let err2 = violations.iter().any(|v| v.rule_id() == "id.is_3");
  let err3 = violations.iter().any(|v| v.rule_id() == "id.is_4");

  assert!(err1);
  assert!(err2);
  assert!(err3);
}

use protocheck::types::protovalidate::Violations;

use crate::myapp::v1::{Anakin, JediFight, ObiWan, User};

#[test]
fn example() {
  let obi_wan = ObiWan {
    has_high_ground: false,
  };
  let anakin = Anakin {
    has_high_ground: true,
  };
  let jedi_fight = JediFight {
    anakin: Some(anakin),
    obi_wan: Some(obi_wan),
  };

  #[allow(unused_variables)]
  let Violations { violations } = jedi_fight.validate().unwrap_err();

  let outcome: Vec<String> = violations
    .iter()
    .map(|v| {
      format!(
        "Field path: {}, Error message: {}",
        v.field_path_str().unwrap(),
        v.message()
      )
    })
    .collect();

  assert_eq!(
    outcome[0],
    "Field path: anakin.has_high_ground, Error message: must be equal to false",
  );
  assert_eq!(
    outcome[1],
    "Field path: obi_wan, Error message: obi-wan must have the high ground."
  );
  assert_eq!(
    outcome[2],
    "Field path: obi_wan.has_high_ground, Error message: obi-wan must have the high ground."
  );

  let user = User {
    password: "abc".to_string(),
    confirm_password: "abcde".to_string(),
  };

  let Violations { violations } = user.validate().unwrap_err();

  let outcome = format!(
    "Field path: {}, Error message: {}",
    // Message-wide violations do not have a field path unless they are nested in another message
    // But they do have a rule id
    violations[0].rule_id(),
    violations[0].message()
  );

  assert_eq!(
    outcome,
    "Field path: passwords_match, Error message: the two passwords do not match"
  );
}

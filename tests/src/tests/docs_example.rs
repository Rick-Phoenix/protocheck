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

  let Violations { violations } = jedi_fight.validate().unwrap_err();

  violations.iter().for_each(|v| {
    println!(
      "Field path: {}, Error message: {}",
      v.field_path_str().unwrap(),
      v.message()
    )
  });

  let user = User {
    password: "abc".to_string(),
    confirm_password: "abcde".to_string(),
  };

  #[allow(unused_variables)]
  let Violations { violations } = user.validate().unwrap_err();

  // println!(
  //   "Field path: {}, Error message: {}",
  //   // Message-wide violations do not have a field path unless they are nested in another message
  //   // But they do have a rule id
  //   violations[0].rule_id(),
  //   violations[0].message()
  // );
}

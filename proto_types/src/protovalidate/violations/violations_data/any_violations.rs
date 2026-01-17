use super::*;

violations_enum!(Any, in, not_in);

macro_rules! any_violation {
  ($name:ident, $num:literal) => {
    violation_data!(any, 20, $name, $num, String);
  };
}

any_violation!(in, 2);
any_violation!(not_in, 3);

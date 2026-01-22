use super::*;

violations_enum!(with_required, Duration, const, lt, lte, gt, gte, in, not_in);

macro_rules! duration_violation {
  ($name:ident, $num:literal) => {
    violation_data!(duration, 21, $name, $num, Message);
  };
}

duration_violation!(const, 2);
duration_violation!(lt, 3);
duration_violation!(lte, 4);
duration_violation!(gt, 5);
duration_violation!(gte, 6);
duration_violation!(in, 7);
duration_violation!(not_in, 8);

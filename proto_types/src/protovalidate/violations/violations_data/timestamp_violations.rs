use super::*;

violations_enum!(Timestamp, const, lt, lte, lt_now, gt, gte, gt_now);

macro_rules! timestamp_violation {
  ($name:ident, $num:literal, $typ:ident) => {
    violation_data!(timestamp, 22, $name, $num, $typ);
  };
}

timestamp_violation!(const, 2, Message);
timestamp_violation!(lt, 3, Message);
timestamp_violation!(lte, 4, Message);
timestamp_violation!(lt_now, 7, Bool);
timestamp_violation!(gt, 5, Message);
timestamp_violation!(gte, 6, Message);
timestamp_violation!(gt_now, 8, Bool);
timestamp_violation!(within, 9, Message);

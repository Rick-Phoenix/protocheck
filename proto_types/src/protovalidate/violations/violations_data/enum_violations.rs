use super::*;

violations_enum!(Enum, const, defined_only, in, not_in);

macro_rules! enum_violation {
  ($name:ident, $num:literal, $typ:ident) => {
    violation_data!(enum, 16, $name, $num, $typ);
  };
}

enum_violation!(const, 1, Int32);
enum_violation!(defined_only, 2, Bool);
enum_violation!(in, 3, Int32);
enum_violation!(not_in, 4, Int32);

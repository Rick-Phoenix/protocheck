use super::*;

violations_enum!(with_required, FieldMask, const, in, not_in);

macro_rules! field_mask_violation {
  ($name:ident, $num:literal, $typ:ident) => {
    violation_data!(field_mask, 28, $name, $num, String);
  };
}

field_mask_violation!(const, 1, Message);
field_mask_violation!(in, 2, String);
field_mask_violation!(not_in, 3, String);

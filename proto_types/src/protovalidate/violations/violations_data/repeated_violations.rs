use super::*;

violations_enum!(Repeated, min_items, max_items, unique, items);

macro_rules! repeated_violation {
  ($name:ident, $num:literal, $typ:ident) => {
    violation_data!(repeated, 18, $name, $num, $typ);
  };
}

repeated_violation!(min_items, 1, Uint64);
repeated_violation!(max_items, 2, Uint64);
repeated_violation!(unique, 3, Bool);
repeated_violation!(items, 4, Message);

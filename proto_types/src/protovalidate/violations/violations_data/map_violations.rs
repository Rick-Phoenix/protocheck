use super::*;

violations_enum!(Map, min_pairs, max_pairs, keys, values);

macro_rules! map_violation {
  ($name:ident, $num:literal, $typ:ident) => {
    violation_data!(map, 19, $name, $num, $typ);
  };
}

map_violation!(min_pairs, 1, Uint64);
map_violation!(max_pairs, 2, Uint64);
map_violation!(keys, 4, Message);
map_violation!(values, 5, Message);

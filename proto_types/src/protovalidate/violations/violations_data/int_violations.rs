use super::*;

macro_rules! int_violations {
  ($name:ident, $num:literal) => {
    paste::paste! {
      violations_enum!([< $name:camel >], const, lt, lte, gt, gte, in, not_in);

      violation_data!($name, $num, const, 1, [< $name:camel >]);
      violation_data!($name, $num, lt, 2, [< $name:camel >]);
      violation_data!($name, $num, lte, 3, [< $name:camel >]);
      violation_data!($name, $num, gt, 4, [< $name:camel >]);
      violation_data!($name, $num, gte, 5, [< $name:camel >]);
      violation_data!($name, $num, in, 6, [< $name:camel >]);
      violation_data!($name, $num, not_in, 7, [< $name:camel >]);
    }
  };
}

int_violations!(int32, 3);
int_violations!(int64, 4);
int_violations!(uint32, 5);
int_violations!(uint64, 6);
int_violations!(sint32, 7);
int_violations!(sint64, 8);
int_violations!(fixed32, 9);
int_violations!(fixed64, 10);
int_violations!(sfixed32, 11);
int_violations!(sfixed64, 12);

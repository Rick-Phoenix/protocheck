use super::*;

violations_enum!(Float, const, lt, lte, gt, gte, in, not_in, finite);

macro_rules! float_violation {
  ($name:ident, $num:literal, $typ:ident) => {
    violation_data!(float, 1, $name, $num, $typ);
  };
}

float_violation!(const, 1, Float);
float_violation!(lt, 2, Float);
float_violation!(lte, 3, Float);
float_violation!(gt, 4, Float);
float_violation!(gte, 5, Float);
float_violation!(in, 6, Float);
float_violation!(not_in, 7, Float);
float_violation!(finite, 8, Bool);

violations_enum!(Double, const, lt, lte, gt, gte, in, not_in, finite);

macro_rules! double_violation {
  ($name:ident, $num:literal, $typ:ident) => {
    violation_data!(double, 2, $name, $num, $typ);
  };
}

double_violation!(const, 1, Double);
double_violation!(lt, 2, Double);
double_violation!(lte, 3, Double);
double_violation!(gt, 4, Double);
double_violation!(gte, 5, Double);
double_violation!(in, 6, Double);
double_violation!(not_in, 7, Double);
double_violation!(finite, 8, Bool);

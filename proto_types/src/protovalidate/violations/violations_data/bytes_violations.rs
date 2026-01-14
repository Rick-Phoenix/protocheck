use super::*;

macro_rules! bytes_violation {
  ($name:ident, $num:literal, $typ:ident) => {
    violation_data!(bytes, 15, $name, $num, $typ);
  };
}

bytes_violation!(const, 1, Bytes);
bytes_violation!(len, 13, Uint64);
bytes_violation!(min_len, 2, Uint64);
bytes_violation!(max_len, 3, Uint64);
bytes_violation!(pattern, 4, String);
bytes_violation!(prefix, 5, Bytes);
bytes_violation!(suffix, 6, Bytes);
bytes_violation!(contains, 7, Bytes);
bytes_violation!(in, 8, Bytes);
bytes_violation!(not_in, 8, Bytes);
bytes_violation!(ip, 10, Bool);
bytes_violation!(ipv4, 11, Bool);
bytes_violation!(ipv6, 12, Bool);
bytes_violation!(uuid, 15, Bool);

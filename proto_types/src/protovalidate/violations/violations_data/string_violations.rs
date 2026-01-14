use super::*;

macro_rules! string_violation {
  ($name:ident, $num:literal, $typ:ident) => {
    violation_data!(string, 14, $name, $num, $typ);
  };
}

string_violation!(const, 1, String);
string_violation!(len, 19, Uint64);
string_violation!(min_len, 2, Uint64);
string_violation!(max_len, 3, Uint64);
string_violation!(len_bytes, 20, Uint64);
string_violation!(min_bytes, 4, Uint64);
string_violation!(max_bytes, 5, Uint64);
string_violation!(pattern, 6, String);
string_violation!(prefix, 7, String);
string_violation!(suffix, 8, String);
string_violation!(contains, 9, String);
string_violation!(not_contains, 23, String);
string_violation!(in, 10, String);
string_violation!(not_in, 11, String);
string_violation!(email, 12, Bool);
string_violation!(hostname, 13, Bool);
string_violation!(ip, 14, Bool);
string_violation!(ipv4, 15, Bool);
string_violation!(ipv6, 16, Bool);
string_violation!(uri, 17, Bool);
string_violation!(uri_ref, 18, Bool);
string_violation!(address, 21, Bool);
string_violation!(uuid, 22, Bool);
string_violation!(tuuid, 33, Bool);
string_violation!(ip_with_prefixlen, 26, Bool);
string_violation!(ipv4_with_prefixlen, 27, Bool);
string_violation!(ipv6_with_prefixlen, 28, Bool);
string_violation!(ip_prefix, 29, Bool);
string_violation!(ipv4_prefix, 30, Bool);
string_violation!(ipv6_prefix, 31, Bool);
string_violation!(host_and_port, 32, Bool);
string_violation!(ulid, 35, Bool);
string_violation!(well_known_regex, 24, Enum);

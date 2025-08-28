pub(crate) const PACKAGE_PREFIX: &str = "google.protobuf";

pub(crate) const TIME_NANOS_MAX: i32 = NANOS_PER_SECOND - 1;
pub(crate) const NANOS_PER_SECOND: i32 = 1_000_000_000;
pub(crate) const NANOS_PER_MINUTE: i64 = NANOS_PER_SECOND as i64 * 60;
pub(crate) const NANOS_PER_HOUR: i64 = NANOS_PER_MINUTE * 60;

pub(crate) const SECONDS_PER_MINUTE: u64 = 60;
pub(crate) const SECONDS_PER_HOUR: u64 = 3600;
pub(crate) const SECONDS_PER_DAY: u64 = 86400;
pub(crate) const SECONDS_PER_WEEK: u64 = 604800;
pub(crate) const SECONDS_PER_MONTH_AVG: u64 = 2_629_746;
pub(crate) const SECONDS_PER_YEAR_AVG: u64 = 31_556_952;

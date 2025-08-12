use paste::paste;

use super::comparable_rules::{ComparableGreaterThan, ComparableLessThan, ComparableRules};
use crate::{
  protovalidate::{
    double_rules::{GreaterThan as DoubleGreaterThan, LessThan as DoubleLessThan},
    duration_rules::{GreaterThan as DurationGreaterThan, LessThan as DurationLessThan},
    fixed32_rules::{GreaterThan as Fixed32GreaterThan, LessThan as Fixed32LessThan},
    fixed64_rules::{GreaterThan as Fixed64GreaterThan, LessThan as Fixed64LessThan},
    float_rules::{GreaterThan as FloatGreaterThan, LessThan as FloatLessThan},
    int32_rules::{GreaterThan as Int32GreaterThan, LessThan as Int32LessThan},
    int64_rules::{GreaterThan as Int64GreaterThan, LessThan as Int64LessThan},
    s_fixed32_rules::{GreaterThan as SFixed32GreaterThan, LessThan as SFixed32LessThan},
    s_fixed64_rules::{GreaterThan as SFixed64GreaterThan, LessThan as SFixed64LessThan},
    s_int32_rules::{GreaterThan as SInt32GreaterThan, LessThan as SInt32LessThan},
    s_int64_rules::{GreaterThan as SInt64GreaterThan, LessThan as SInt64LessThan},
    timestamp_rules::{GreaterThan as TimestampGreaterThan, LessThan as TimestampLessThan},
    u_int32_rules::{GreaterThan as UInt32GreaterThan, LessThan as UInt32LessThan},
    u_int64_rules::{GreaterThan as UInt64GreaterThan, LessThan as UInt64LessThan},
    DurationRules, TimestampComparableRules, TimestampRules,
  },
  Duration, Timestamp, TimestampError,
};

pub trait IntoComparable<T> {
  fn into_comparable(self) -> T;
}

impl TimestampRules {
  pub fn comparable_rules(&self) -> Result<TimestampComparableRules, String> {
    let format_timestamp = |t: Timestamp, msg: &str| -> Result<String, String> {
      t.format(&format!("{} {}", msg, "%d %b %Y %R %Z"))
        .map_err(|e: TimestampError| {
          format!(
            "failed to convert protobuf timestamp to chrono timestamp: {}",
            e
          )
        })
    };

    let mut greater_than: Option<ComparableGreaterThan<Timestamp>> = None;

    if let Some(gt_rule) = self.greater_than {
      greater_than = match gt_rule {
        TimestampGreaterThan::Gt(v) => Some(ComparableGreaterThan::Gt {
          val: v,
          error_message: format_timestamp(v, "must be later than")?,
        }),

        TimestampGreaterThan::Gte(v) => Some(ComparableGreaterThan::Gte {
          val: v,
          error_message: format_timestamp(v, "cannot be earlier than")?,
        }),
        _ => None,
      }
    }

    let mut less_than: Option<ComparableLessThan<Timestamp>> = None;

    if let Some(gt_rule) = self.less_than {
      less_than = match gt_rule {
        TimestampLessThan::Lt(v) => Some(ComparableLessThan::Lt {
          val: v,
          error_message: format_timestamp(v, "must be earlier than")?,
        }),

        TimestampLessThan::Lte(v) => Some(ComparableLessThan::Lte {
          val: v,
          error_message: format_timestamp(v, "cannot be later than")?,
        }),
        _ => None,
      }
    }

    let comparable_rules = ComparableRules {
      less_than,
      greater_than,
    };

    let lt_now = matches!(self.less_than, Some(TimestampLessThan::LtNow(true)));

    let gt_now = matches!(self.greater_than, Some(TimestampGreaterThan::GtNow(true)));

    Ok(TimestampComparableRules {
      comparable_rules: comparable_rules.validate().map_err(|e| e.to_string())?,
      lt_now,
      gt_now,
    })
  }
}

impl DurationRules {
  pub fn comparable_rules(&self) -> Result<ComparableRules<Duration>, &'static str> {
    let greater_than = self.greater_than.map(|rule| match rule {
      DurationGreaterThan::Gt(val) => ComparableGreaterThan::Gt {
        val,
        error_message: format!("must be longer than {}", val),
      },
      DurationGreaterThan::Gte(val) => ComparableGreaterThan::Gte {
        val,
        error_message: format!("cannot be shorter than {}", val),
      },
    });

    let less_than = self.less_than.map(|rule| match rule {
      DurationLessThan::Lt(val) => ComparableLessThan::Lt {
        val,
        error_message: format!("must be shorter than {}", val),
      },
      DurationLessThan::Lte(val) => ComparableLessThan::Lte {
        val,
        error_message: format!("cannot be longer than {}", val),
      },
    });

    let comparable_rules = ComparableRules {
      greater_than,
      less_than,
    };
    comparable_rules.validate()
  }
}

macro_rules! into_comparable {
  ($target_type:ty, $rule_type:ident) => {
    paste! {
      impl IntoComparable<ComparableLessThan<$target_type>> for [< $rule_type LessThan >] {
        fn into_comparable(self) -> ComparableLessThan<$target_type> {
          match self {
            Self::Lt(val) => ComparableLessThan::Lt { val, error_message: format!("must be smaller than {}", val) },
            Self::Lte(val) => ComparableLessThan::Lte { val, error_message: format!("cannot be greater than {}", val) },
          }
        }
      }

      impl IntoComparable<ComparableGreaterThan<$target_type>> for [< $rule_type GreaterThan >] {
        fn into_comparable(self) -> ComparableGreaterThan<$target_type> {
          match self {
            Self::Gt(val) => ComparableGreaterThan::Gt { val, error_message: format!("must be greater than {}", val) },
            Self::Gte(val) => ComparableGreaterThan::Gte { val, error_message: format!("cannot be smaller than {}", val) },
          }
        }
      }
    }
  };
}

into_comparable!(f32, Float);
into_comparable!(f64, Double);
into_comparable!(i64, Int64);
into_comparable!(i32, Int32);
into_comparable!(i64, SInt64);
into_comparable!(i32, SInt32);
into_comparable!(i64, SFixed64);
into_comparable!(i32, SFixed32);
into_comparable!(u64, UInt64);
into_comparable!(u32, UInt32);
into_comparable!(u64, Fixed64);
into_comparable!(u32, Fixed32);

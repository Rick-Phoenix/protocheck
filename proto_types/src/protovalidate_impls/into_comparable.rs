use proc_macro2::Span;
use syn::Error;

use crate::{
  protovalidate::{
    double_rules::{GreaterThan as DoubleGreaterThan, LessThan as DoubleLessThan},
    duration_rules::{GreaterThan as DurationGreaterThan, LessThan as DurationLessThan},
    fixed32_rules::{GreaterThan as Fixed32GreaterThan, LessThan as Fixed32LessThan},
    fixed64_rules::{GreaterThan as Fixed64GreaterThan, LessThan as Fixed64LessThan},
    float_rules::{GreaterThan as FloatGreaterThan, LessThan as FloatLessThan},
    int32_rules::{GreaterThan as I32GreaterThan, LessThan as I32LessThan},
    int64_rules::{GreaterThan as I64GreaterThan, LessThan as I64LessThan},
    s_fixed32_rules::{GreaterThan as SFixed32GreaterThan, LessThan as SFixed32LessThan},
    s_fixed64_rules::{GreaterThan as SFixed64GreaterThan, LessThan as SFixed64LessThan},
    s_int32_rules::{GreaterThan as S32GreaterThan, LessThan as S32LessThan},
    s_int64_rules::{GreaterThan as S64GreaterThan, LessThan as S64LessThan},
    timestamp_rules::{GreaterThan as TimestampGreaterThan, LessThan as TimestampLessThan},
    u_int32_rules::{GreaterThan as U32GreaterThan, LessThan as U32LessThan},
    u_int64_rules::{GreaterThan as U64GreaterThan, LessThan as U64LessThan},
    DurationRules, TimestampRules,
  },
  protovalidate_impls::comparable_rules::{
    ComparableGreaterThan, ComparableLessThan, ComparableRules,
  },
  Duration, Timestamp,
};

pub trait IntoComparable<T> {
  fn into_comparable(self) -> T;
}

impl TimestampRules {
  pub fn comparable_rules(
    &self,
    field_span: Span,
    error_prefix: &str,
  ) -> Result<(Option<TimestampGreaterThan>, Option<TimestampLessThan>), Error> {
    let comparable_rules = ComparableRules {
      greater_than: self.greater_than.map(|gt| gt.into_comparable()),
      less_than: self.less_than.map(|lt| lt.into_comparable()),
    };
    comparable_rules.validate(field_span, error_prefix)?;

    Ok((self.greater_than, self.less_than))
  }
}

impl DurationRules {
  pub fn comparable_rules(
    &self,
    field_span: Span,
    error_prefix: &str,
  ) -> Result<ComparableRules<Duration>, Error> {
    let comparable_rules = ComparableRules {
      greater_than: self.greater_than.map(|gt| gt.into_comparable()),
      less_than: self.less_than.map(|lt| lt.into_comparable()),
    };
    comparable_rules.validate(field_span, error_prefix)
  }
}

impl IntoComparable<ComparableLessThan<Timestamp>> for TimestampLessThan {
  fn into_comparable(self) -> ComparableLessThan<Timestamp> {
    match self {
      Self::Lt(v) => ComparableLessThan::Lt(v),
      Self::Lte(v) => ComparableLessThan::Lte(v),
      Self::LtNow(_) => ComparableLessThan::Lt(Timestamp::now()),
    }
  }
}

impl IntoComparable<ComparableLessThan<Duration>> for DurationLessThan {
  fn into_comparable(self) -> ComparableLessThan<Duration> {
    match self {
      Self::Lt(v) => ComparableLessThan::Lt(v),
      Self::Lte(v) => ComparableLessThan::Lte(v),
    }
  }
}

impl IntoComparable<ComparableLessThan<f32>> for FloatLessThan {
  fn into_comparable(self) -> ComparableLessThan<f32> {
    match self {
      Self::Lt(v) => ComparableLessThan::Lt(v),
      Self::Lte(v) => ComparableLessThan::Lte(v),
    }
  }
}

impl IntoComparable<ComparableLessThan<f64>> for DoubleLessThan {
  fn into_comparable(self) -> ComparableLessThan<f64> {
    match self {
      Self::Lt(v) => ComparableLessThan::Lt(v),
      Self::Lte(v) => ComparableLessThan::Lte(v),
    }
  }
}

impl IntoComparable<ComparableLessThan<i64>> for I64LessThan {
  fn into_comparable(self) -> ComparableLessThan<i64> {
    match self {
      Self::Lt(v) => ComparableLessThan::Lt(v),
      Self::Lte(v) => ComparableLessThan::Lte(v),
    }
  }
}

impl IntoComparable<ComparableLessThan<i64>> for S64LessThan {
  fn into_comparable(self) -> ComparableLessThan<i64> {
    match self {
      Self::Lt(v) => ComparableLessThan::Lt(v),
      Self::Lte(v) => ComparableLessThan::Lte(v),
    }
  }
}

impl IntoComparable<ComparableLessThan<i64>> for SFixed64LessThan {
  fn into_comparable(self) -> ComparableLessThan<i64> {
    match self {
      Self::Lt(v) => ComparableLessThan::Lt(v),
      Self::Lte(v) => ComparableLessThan::Lte(v),
    }
  }
}

impl IntoComparable<ComparableLessThan<i32>> for I32LessThan {
  fn into_comparable(self) -> ComparableLessThan<i32> {
    match self {
      Self::Lt(v) => ComparableLessThan::Lt(v),
      Self::Lte(v) => ComparableLessThan::Lte(v),
    }
  }
}

impl IntoComparable<ComparableLessThan<i32>> for S32LessThan {
  fn into_comparable(self) -> ComparableLessThan<i32> {
    match self {
      Self::Lt(v) => ComparableLessThan::Lt(v),
      Self::Lte(v) => ComparableLessThan::Lte(v),
    }
  }
}

impl IntoComparable<ComparableLessThan<i32>> for SFixed32LessThan {
  fn into_comparable(self) -> ComparableLessThan<i32> {
    match self {
      Self::Lt(v) => ComparableLessThan::Lt(v),
      Self::Lte(v) => ComparableLessThan::Lte(v),
    }
  }
}

impl IntoComparable<ComparableLessThan<u32>> for U32LessThan {
  fn into_comparable(self) -> ComparableLessThan<u32> {
    match self {
      Self::Lt(v) => ComparableLessThan::Lt(v),
      Self::Lte(v) => ComparableLessThan::Lte(v),
    }
  }
}

impl IntoComparable<ComparableLessThan<u32>> for Fixed32LessThan {
  fn into_comparable(self) -> ComparableLessThan<u32> {
    match self {
      Self::Lt(v) => ComparableLessThan::Lt(v),
      Self::Lte(v) => ComparableLessThan::Lte(v),
    }
  }
}

impl IntoComparable<ComparableLessThan<u64>> for U64LessThan {
  fn into_comparable(self) -> ComparableLessThan<u64> {
    match self {
      Self::Lt(v) => ComparableLessThan::Lt(v),
      Self::Lte(v) => ComparableLessThan::Lte(v),
    }
  }
}

impl IntoComparable<ComparableLessThan<u64>> for Fixed64LessThan {
  fn into_comparable(self) -> ComparableLessThan<u64> {
    match self {
      Self::Lt(v) => ComparableLessThan::Lt(v),
      Self::Lte(v) => ComparableLessThan::Lte(v),
    }
  }
}

impl IntoComparable<ComparableGreaterThan<f32>> for FloatGreaterThan {
  fn into_comparable(self) -> ComparableGreaterThan<f32> {
    match self {
      Self::Gt(v) => ComparableGreaterThan::Gt(v),
      Self::Gte(v) => ComparableGreaterThan::Gte(v),
    }
  }
}

impl IntoComparable<ComparableGreaterThan<f64>> for DoubleGreaterThan {
  fn into_comparable(self) -> ComparableGreaterThan<f64> {
    match self {
      Self::Gt(v) => ComparableGreaterThan::Gt(v),
      Self::Gte(v) => ComparableGreaterThan::Gte(v),
    }
  }
}

impl IntoComparable<ComparableGreaterThan<i64>> for I64GreaterThan {
  fn into_comparable(self) -> ComparableGreaterThan<i64> {
    match self {
      Self::Gt(v) => ComparableGreaterThan::Gt(v),
      Self::Gte(v) => ComparableGreaterThan::Gte(v),
    }
  }
}

impl IntoComparable<ComparableGreaterThan<i64>> for S64GreaterThan {
  fn into_comparable(self) -> ComparableGreaterThan<i64> {
    match self {
      Self::Gt(v) => ComparableGreaterThan::Gt(v),
      Self::Gte(v) => ComparableGreaterThan::Gte(v),
    }
  }
}

impl IntoComparable<ComparableGreaterThan<i64>> for SFixed64GreaterThan {
  fn into_comparable(self) -> ComparableGreaterThan<i64> {
    match self {
      Self::Gt(v) => ComparableGreaterThan::Gt(v),
      Self::Gte(v) => ComparableGreaterThan::Gte(v),
    }
  }
}

impl IntoComparable<ComparableGreaterThan<i32>> for I32GreaterThan {
  fn into_comparable(self) -> ComparableGreaterThan<i32> {
    match self {
      Self::Gt(v) => ComparableGreaterThan::Gt(v),
      Self::Gte(v) => ComparableGreaterThan::Gte(v),
    }
  }
}

impl IntoComparable<ComparableGreaterThan<i32>> for S32GreaterThan {
  fn into_comparable(self) -> ComparableGreaterThan<i32> {
    match self {
      Self::Gt(v) => ComparableGreaterThan::Gt(v),
      Self::Gte(v) => ComparableGreaterThan::Gte(v),
    }
  }
}

impl IntoComparable<ComparableGreaterThan<i32>> for SFixed32GreaterThan {
  fn into_comparable(self) -> ComparableGreaterThan<i32> {
    match self {
      Self::Gt(v) => ComparableGreaterThan::Gt(v),
      Self::Gte(v) => ComparableGreaterThan::Gte(v),
    }
  }
}

impl IntoComparable<ComparableGreaterThan<u32>> for U32GreaterThan {
  fn into_comparable(self) -> ComparableGreaterThan<u32> {
    match self {
      Self::Gt(v) => ComparableGreaterThan::Gt(v),
      Self::Gte(v) => ComparableGreaterThan::Gte(v),
    }
  }
}

impl IntoComparable<ComparableGreaterThan<u32>> for Fixed32GreaterThan {
  fn into_comparable(self) -> ComparableGreaterThan<u32> {
    match self {
      Self::Gt(v) => ComparableGreaterThan::Gt(v),
      Self::Gte(v) => ComparableGreaterThan::Gte(v),
    }
  }
}

impl IntoComparable<ComparableGreaterThan<u64>> for U64GreaterThan {
  fn into_comparable(self) -> ComparableGreaterThan<u64> {
    match self {
      Self::Gt(v) => ComparableGreaterThan::Gt(v),
      Self::Gte(v) => ComparableGreaterThan::Gte(v),
    }
  }
}

impl IntoComparable<ComparableGreaterThan<u64>> for Fixed64GreaterThan {
  fn into_comparable(self) -> ComparableGreaterThan<u64> {
    match self {
      Self::Gt(v) => ComparableGreaterThan::Gt(v),
      Self::Gte(v) => ComparableGreaterThan::Gte(v),
    }
  }
}

impl IntoComparable<ComparableGreaterThan<Timestamp>> for TimestampGreaterThan {
  fn into_comparable(self) -> ComparableGreaterThan<Timestamp> {
    match self {
      Self::Gt(v) => ComparableGreaterThan::Gt(v),
      Self::Gte(v) => ComparableGreaterThan::Gte(v),
      Self::GtNow(_) => ComparableGreaterThan::Gt(Timestamp::now()),
    }
  }
}

impl IntoComparable<ComparableGreaterThan<Duration>> for DurationGreaterThan {
  fn into_comparable(self) -> ComparableGreaterThan<Duration> {
    match self {
      Self::Gt(v) => ComparableGreaterThan::Gt(v),
      Self::Gte(v) => ComparableGreaterThan::Gte(v),
    }
  }
}

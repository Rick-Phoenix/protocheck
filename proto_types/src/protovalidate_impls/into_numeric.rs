use crate::protovalidate::{
  fixed32_rules::{GreaterThan as Fixed32GreaterThan, LessThan as Fixed32LessThan},
  fixed64_rules::{GreaterThan as Fixed64GreaterThan, LessThan as Fixed64LessThan},
  int32_rules::{GreaterThan as I32GreaterThan, LessThan as I32LessThan},
  int64_rules::{GreaterThan as I64GreaterThan, LessThan as I64LessThan},
  s_fixed32_rules::{GreaterThan as SFixed32GreaterThan, LessThan as SFixed32LessThan},
  s_fixed64_rules::{GreaterThan as SFixed64GreaterThan, LessThan as SFixed64LessThan},
  s_int32_rules::{GreaterThan as S32GreaterThan, LessThan as S32LessThan},
  s_int64_rules::{GreaterThan as S64GreaterThan, LessThan as S64LessThan},
  u_int32_rules::{GreaterThan as U32GreaterThan, LessThan as U32LessThan},
  u_int64_rules::{GreaterThan as U64GreaterThan, LessThan as U64LessThan},
};

pub enum NumericLessThan<T> {
  Lt(T),
  Lte(T),
}

pub enum NumericGreaterThan<T> {
  Gt(T),
  Gte(T),
}

pub trait IntoNumeric<T> {
  fn into_numeric(self) -> T;
}

impl IntoNumeric<NumericLessThan<i64>> for I64LessThan {
  fn into_numeric(self) -> NumericLessThan<i64> {
    match self {
      Self::Lt(v) => NumericLessThan::Lt(v),
      Self::Lte(v) => NumericLessThan::Lte(v),
    }
  }
}

impl IntoNumeric<NumericLessThan<i64>> for S64LessThan {
  fn into_numeric(self) -> NumericLessThan<i64> {
    match self {
      Self::Lt(v) => NumericLessThan::Lt(v),
      Self::Lte(v) => NumericLessThan::Lte(v),
    }
  }
}

impl IntoNumeric<NumericLessThan<i64>> for SFixed64LessThan {
  fn into_numeric(self) -> NumericLessThan<i64> {
    match self {
      Self::Lt(v) => NumericLessThan::Lt(v),
      Self::Lte(v) => NumericLessThan::Lte(v),
    }
  }
}

impl IntoNumeric<NumericLessThan<i32>> for I32LessThan {
  fn into_numeric(self) -> NumericLessThan<i32> {
    match self {
      Self::Lt(v) => NumericLessThan::Lt(v),
      Self::Lte(v) => NumericLessThan::Lte(v),
    }
  }
}

impl IntoNumeric<NumericLessThan<i32>> for S32LessThan {
  fn into_numeric(self) -> NumericLessThan<i32> {
    match self {
      Self::Lt(v) => NumericLessThan::Lt(v),
      Self::Lte(v) => NumericLessThan::Lte(v),
    }
  }
}

impl IntoNumeric<NumericLessThan<i32>> for SFixed32LessThan {
  fn into_numeric(self) -> NumericLessThan<i32> {
    match self {
      Self::Lt(v) => NumericLessThan::Lt(v),
      Self::Lte(v) => NumericLessThan::Lte(v),
    }
  }
}

impl IntoNumeric<NumericLessThan<u32>> for U32LessThan {
  fn into_numeric(self) -> NumericLessThan<u32> {
    match self {
      Self::Lt(v) => NumericLessThan::Lt(v),
      Self::Lte(v) => NumericLessThan::Lte(v),
    }
  }
}

impl IntoNumeric<NumericLessThan<u32>> for Fixed32LessThan {
  fn into_numeric(self) -> NumericLessThan<u32> {
    match self {
      Self::Lt(v) => NumericLessThan::Lt(v),
      Self::Lte(v) => NumericLessThan::Lte(v),
    }
  }
}

impl IntoNumeric<NumericLessThan<u64>> for U64LessThan {
  fn into_numeric(self) -> NumericLessThan<u64> {
    match self {
      Self::Lt(v) => NumericLessThan::Lt(v),
      Self::Lte(v) => NumericLessThan::Lte(v),
    }
  }
}

impl IntoNumeric<NumericLessThan<u64>> for Fixed64LessThan {
  fn into_numeric(self) -> NumericLessThan<u64> {
    match self {
      Self::Lt(v) => NumericLessThan::Lt(v),
      Self::Lte(v) => NumericLessThan::Lte(v),
    }
  }
}

impl IntoNumeric<NumericGreaterThan<i64>> for I64GreaterThan {
  fn into_numeric(self) -> NumericGreaterThan<i64> {
    match self {
      Self::Gt(v) => NumericGreaterThan::Gt(v),
      Self::Gte(v) => NumericGreaterThan::Gte(v),
    }
  }
}

impl IntoNumeric<NumericGreaterThan<i64>> for S64GreaterThan {
  fn into_numeric(self) -> NumericGreaterThan<i64> {
    match self {
      Self::Gt(v) => NumericGreaterThan::Gt(v),
      Self::Gte(v) => NumericGreaterThan::Gte(v),
    }
  }
}

impl IntoNumeric<NumericGreaterThan<i64>> for SFixed64GreaterThan {
  fn into_numeric(self) -> NumericGreaterThan<i64> {
    match self {
      Self::Gt(v) => NumericGreaterThan::Gt(v),
      Self::Gte(v) => NumericGreaterThan::Gte(v),
    }
  }
}

impl IntoNumeric<NumericGreaterThan<i32>> for I32GreaterThan {
  fn into_numeric(self) -> NumericGreaterThan<i32> {
    match self {
      Self::Gt(v) => NumericGreaterThan::Gt(v),
      Self::Gte(v) => NumericGreaterThan::Gte(v),
    }
  }
}

impl IntoNumeric<NumericGreaterThan<i32>> for S32GreaterThan {
  fn into_numeric(self) -> NumericGreaterThan<i32> {
    match self {
      Self::Gt(v) => NumericGreaterThan::Gt(v),
      Self::Gte(v) => NumericGreaterThan::Gte(v),
    }
  }
}

impl IntoNumeric<NumericGreaterThan<i32>> for SFixed32GreaterThan {
  fn into_numeric(self) -> NumericGreaterThan<i32> {
    match self {
      Self::Gt(v) => NumericGreaterThan::Gt(v),
      Self::Gte(v) => NumericGreaterThan::Gte(v),
    }
  }
}

impl IntoNumeric<NumericGreaterThan<u32>> for U32GreaterThan {
  fn into_numeric(self) -> NumericGreaterThan<u32> {
    match self {
      Self::Gt(v) => NumericGreaterThan::Gt(v),
      Self::Gte(v) => NumericGreaterThan::Gte(v),
    }
  }
}

impl IntoNumeric<NumericGreaterThan<u32>> for Fixed32GreaterThan {
  fn into_numeric(self) -> NumericGreaterThan<u32> {
    match self {
      Self::Gt(v) => NumericGreaterThan::Gt(v),
      Self::Gte(v) => NumericGreaterThan::Gte(v),
    }
  }
}

impl IntoNumeric<NumericGreaterThan<u64>> for U64GreaterThan {
  fn into_numeric(self) -> NumericGreaterThan<u64> {
    match self {
      Self::Gt(v) => NumericGreaterThan::Gt(v),
      Self::Gte(v) => NumericGreaterThan::Gte(v),
    }
  }
}

impl IntoNumeric<NumericGreaterThan<u64>> for Fixed64GreaterThan {
  fn into_numeric(self) -> NumericGreaterThan<u64> {
    match self {
      Self::Gt(v) => NumericGreaterThan::Gt(v),
      Self::Gte(v) => NumericGreaterThan::Gte(v),
    }
  }
}

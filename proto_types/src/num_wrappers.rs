use std::{
  fmt::{Debug, Display},
  hash::Hash,
};

use num_traits::{Num, One, Zero};

pub trait ProtoInt: Num + Clone + Copy + Display + Debug + Eq + Ord + Hash + Default {
  type Target: Num + Clone + Copy + Display + Debug + Eq + Ord + Hash + Default;
}

macro_rules! impl_wrapper {
  ($name:ident, $target:ty) => {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
    pub struct $name(pub $target);

    impl ProtoInt for $name {
      type Target = $target;
    }

    impl Display for $name {
      fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
      }
    }

    impl std::ops::Deref for $name {
      type Target = $target;
      fn deref(&self) -> &Self::Target {
        &self.0
      }
    }

    impl std::cmp::PartialEq<$target> for $name {
      fn eq(&self, other: &$target) -> bool {
        self.0 == *other
      }
    }

    impl core::ops::Add for $name {
      type Output = Self;

      fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
      }
    }

    impl Zero for $name {
      fn zero() -> Self {
        Self(0)
      }

      fn is_zero(&self) -> bool {
        self.0 == 0
      }
    }

    impl One for $name {
      fn one() -> Self {
        Self(1)
      }
    }

    impl core::ops::Mul for $name {
      type Output = Self;

      fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
      }
    }

    impl core::ops::Div for $name {
      type Output = Self;

      fn div(self, rhs: Self) -> Self::Output {
        Self(self.0 / rhs.0)
      }
    }

    impl core::ops::Rem for $name {
      type Output = Self;

      fn rem(self, rhs: Self) -> Self::Output {
        Self(self.0 % rhs.0)
      }
    }

    impl core::ops::Sub for $name {
      type Output = Self;

      fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
      }
    }

    impl core::ops::DerefMut for $name {
      fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
      }
    }

    impl Num for $name {
      type FromStrRadixErr = <$target as num_traits::Num>::FromStrRadixErr;

      fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        Ok(Self(<$target>::from_str_radix(str, radix)?))
      }
    }

    impl From<$name> for $target {
      fn from(value: $name) -> $target {
        value.0
      }
    }
  };
}

impl_wrapper!(Sint64, i64);
impl_wrapper!(Sint32, i32);
impl_wrapper!(Sfixed64, i64);
impl_wrapper!(Sfixed32, i32);
impl_wrapper!(Fixed64, u64);
impl_wrapper!(Fixed32, u32);
impl_wrapper!(EnumVariant, i32);

macro_rules! impl_wrapper {
  ($name:ident, $target:ty) => {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct $name($target);

    impl std::ops::Deref for $name {
      type Target = $target;
      fn deref(&self) -> &Self::Target {
        &self.0
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

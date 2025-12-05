macro_rules! bail {
  ($item:expr, $($tokens:tt)*) => {
    return Err(syn::Error::new_spanned($item, $($tokens)*))
  };
}

macro_rules! error {
  ($item:expr, $($tokens:tt)*) => {
    syn::Error::new_spanned($item, $($tokens)*)
  };
}

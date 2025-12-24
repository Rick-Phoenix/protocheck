use itertools::Itertools;
use ordered_float::OrderedFloat;

use crate::*;

pub enum ListKind {
  Bytes,
  F32,
  F64,
  I32,
  I64,
  U64,
  U32,
  String,
  Any,
  Duration,
}

impl ListKind {
  pub fn list_item_type(&self) -> TokenStream2 {
    match self {
      Self::Bytes => quote! { &'static [u8] },
      Self::F32 => quote! { f32 },
      Self::F64 => quote! { f64 },
      Self::I32 => quote! { i32 },
      Self::I64 => quote! { i64 },
      Self::U64 => quote! { u64 },
      Self::U32 => quote! { u32 },
      Self::String | Self::Any => quote! { &'static str },
      Self::Duration => quote! { ::proto_types::Duration },
    }
  }

  /// Returns `true` if the list kind is [`Bytes`].
  ///
  /// [`Bytes`]: ListKind::Bytes
  #[must_use]
  pub const fn is_bytes(&self) -> bool {
    matches!(self, Self::Bytes)
  }
}

impl ListKind {
  #[must_use]
  pub const fn is_float(&self) -> bool {
    matches!(self, Self::F32 | Self::F64)
  }
}

pub struct List<'a, T: ToTokens + Clone> {
  pub items: Cow<'a, [T]>,
  pub kind: ListKind,
  pub error_message: String,
}

impl<T: ToTokens + Clone> List<'_, T> {
  pub fn output_list(&self) -> TokenStream2 {
    let items = &self.items;

    let items_tokens = if self.kind.is_float() {
      quote! { #(::protocheck::ordered_float::OrderedFloat(#items)),* }
    } else if self.kind.is_bytes() {
      quote! { #(#items as &'static [u8]),* }
    } else {
      quote! { #(#items),* }
    };

    let mut list_item_type = self.kind.list_item_type();

    if self.kind.is_float() {
      list_item_type = quote! { ::protocheck::ordered_float::OrderedFloat<#list_item_type> };
    }

    quote! {
      {
        use std::sync::LazyLock;

        static LIST: LazyLock<Vec<#list_item_type>> = LazyLock::new(|| {
          let mut items: Vec<#list_item_type> = [ #items_tokens ].into_iter().collect();
          items.sort();

          items
        });

        &*LIST
      }
    }
  }
}

pub struct ListsRules<'a, T: ToTokens + Clone> {
  pub in_list_rule: Option<List<'a, T>>,
  pub not_in_list_rule: Option<List<'a, T>>,
}

impl<T: ToTokens + Clone> ListsRules<'_, T> {
  pub const fn is_empty(&self) -> bool {
    self.in_list_rule.is_none() && self.not_in_list_rule.is_none()
  }
}

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct BytesWrapper {
  pub inner: Bytes,
}

impl BytesWrapper {
  pub const fn new(bytes: Bytes) -> Self {
    Self { inner: bytes }
  }
}

pub struct Lists<'a, T: ToTokens + Clone + PartialEq + Eq + Hash + Display> {
  pub in_list: Option<Cow<'a, [T]>>,
  pub not_in_list: Option<Cow<'a, [T]>>,
}

impl<T: ToTokens + Clone + PartialEq + Eq + Hash + Display> Lists<'_, T> {
  pub fn validate(self) -> Result<Self, String> {
    if self.in_list.is_none() || self.not_in_list.is_none() {
      return Ok(self);
    }

    let mut in_list_set: HashSet<&T> = HashSet::new();
    if let Some(in_list) = self.in_list.as_ref() {
      in_list_set.extend(in_list.iter());
    }

    let mut not_in_list_set: HashSet<&T> = HashSet::new();
    if let Some(not_in_list) = self.not_in_list.as_ref() {
      not_in_list_set.extend(not_in_list.iter());
    }

    let invalid_items: Vec<&&T> = in_list_set
      .intersection(&not_in_list_set)
      .collect();

    if !invalid_items.is_empty() {
      return Err(format!(
        "the following items are in the allowed and forbidden lists of items at the same time: {}",
        invalid_items.iter().format(", ")
      ));
    }

    Ok(self)
  }
}

impl Display for BytesWrapper {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.inner.escape_ascii())
  }
}

impl ToTokens for BytesWrapper {
  fn to_tokens(&self, tokens: &mut TokenStream2) {
    let byte_lit = LitByteStr::new(&self.inner, Span::call_site());

    tokens.extend(byte_lit.into_token_stream());
  }
}

pub trait RuleWithLists<T: ToTokens + Display + Hash + Clone + PartialEq + Eq>: Sized {
  const IN_ERROR_MSG: &'static str = "must be one of the following values";
  const NOT_IN_ERROR_MSG: &'static str = "cannot be one of the following values";
  const LIST_KIND: ListKind;

  fn lists(&'_ self) -> Lists<'_, T>;

  fn list_rules(&self) -> Result<ListsRules<'_, T>, String> {
    let Lists {
      in_list,
      not_in_list,
    } = self.lists().validate()?;

    let in_list = in_list.map(|list| {
      let error_prefix = Self::IN_ERROR_MSG;

      let error_message = format!("{error_prefix}: {}", list.iter().format(", "));

      List {
        error_message,
        items: list,
        kind: Self::LIST_KIND,
      }
    });

    let not_in_list = not_in_list.map(|list| {
      let error_prefix = Self::NOT_IN_ERROR_MSG;

      let error_message = format!("{error_prefix}: {}", list.iter().format(", "));

      List {
        error_message,
        items: list,
        kind: Self::LIST_KIND,
      }
    });

    Ok(ListsRules {
      in_list_rule: in_list,
      not_in_list_rule: not_in_list,
    })
  }
}

macro_rules! impl_lists {
  ($rules:ident, $target_ty:ty, $list_kind:ident) => {
    impl RuleWithLists<$target_ty> for $rules {
      const LIST_KIND: ListKind = ListKind::$list_kind;

      fn lists<'a>(&'_ self) -> Lists<'_, $target_ty> {
        let in_list = if !self.r#in.is_empty() {
          Some(Cow::Borrowed(self.r#in.as_slice()))
        } else {
          None
        };

        let not_in_list = if !self.not_in.is_empty() {
          Some(Cow::Borrowed(self.not_in.as_slice()))
        } else {
          None
        };

        Lists {
          in_list,
          not_in_list,
        }
      }
    }
  };
}

impl_lists!(StringRules, String, String);
impl_lists!(EnumRules, i32, I32);
impl_lists!(DurationRules, Duration, Duration);
impl_lists!(Int64Rules, i64, I64);
impl_lists!(SInt64Rules, i64, I64);
impl_lists!(SFixed64Rules, i64, I64);
impl_lists!(Int32Rules, i32, I32);
impl_lists!(SInt32Rules, i32, I32);
impl_lists!(SFixed32Rules, i32, I32);
impl_lists!(UInt64Rules, u64, U64);
impl_lists!(Fixed64Rules, u64, U64);
impl_lists!(UInt32Rules, u32, U32);
impl_lists!(Fixed32Rules, u32, U32);

impl RuleWithLists<String> for AnyRules {
  const LIST_KIND: ListKind = ListKind::Any;
  const IN_ERROR_MSG: &'static str = "must have one of these type URLs";
  const NOT_IN_ERROR_MSG: &'static str = "cannot have one of these type URLs";

  fn lists<'a>(&'_ self) -> Lists<'_, String> {
    let in_list = (!self.r#in.is_empty()).then_some(Cow::Borrowed(self.r#in.as_slice()));

    let not_in_list = (!self.not_in.is_empty()).then_some(Cow::Borrowed(self.not_in.as_slice()));

    Lists {
      in_list,
      not_in_list,
    }
  }
}

impl RuleWithLists<BytesWrapper> for BytesRules {
  const LIST_KIND: ListKind = ListKind::Bytes;

  fn lists<'a>(&'_ self) -> Lists<'_, BytesWrapper> {
    let in_list = (!self.r#in.is_empty()).then(|| {
      let list: Vec<BytesWrapper> = self
        .r#in
        .iter()
        .map(|b| BytesWrapper::new(b.clone()))
        .collect();

      Cow::Owned(list)
    });

    let not_in_list = (!self.not_in.is_empty()).then(|| {
      let list: Vec<BytesWrapper> = self
        .not_in
        .iter()
        .map(|b| BytesWrapper::new(b.clone()))
        .collect();

      Cow::Owned(list)
    });

    Lists {
      in_list,
      not_in_list,
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FloatWrapper {
  pub float: OrderedFloat<f64>,
  pub is_f32: bool,
}

impl Display for FloatWrapper {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    Display::fmt(&self.float, f)
  }
}

impl std::ops::Deref for FloatWrapper {
  type Target = OrderedFloat<f64>;
  fn deref(&self) -> &Self::Target {
    &self.float
  }
}

impl ToTokens for FloatWrapper {
  fn to_tokens(&self, tokens: &mut TokenStream2) {
    let f = self.float;

    let output = if self.is_f32 {
      #[allow(clippy::cast_possible_truncation)]
      proc_macro2::Literal::f32_suffixed(f.into_inner() as f32).to_token_stream()
    } else {
      f.to_token_stream()
    };

    tokens.extend(output);
  }
}

impl RuleWithLists<FloatWrapper> for FloatRules {
  const LIST_KIND: ListKind = ListKind::F32;

  fn lists<'a>(&'_ self) -> Lists<'_, FloatWrapper> {
    let in_list = (!self.r#in.is_empty()).then(|| {
      let list: Vec<FloatWrapper> = self
        .r#in
        .iter()
        .map(|fl| FloatWrapper {
          float: OrderedFloat(f64::from(*fl)),
          is_f32: true,
        })
        .collect();

      Cow::Owned(list)
    });

    let not_in_list = (!self.not_in.is_empty()).then(|| {
      let list: Vec<FloatWrapper> = self
        .not_in
        .iter()
        .map(|fl| FloatWrapper {
          float: OrderedFloat(f64::from(*fl)),
          is_f32: true,
        })
        .collect();

      Cow::Owned(list)
    });

    Lists {
      in_list,
      not_in_list,
    }
  }
}

impl RuleWithLists<FloatWrapper> for DoubleRules {
  const LIST_KIND: ListKind = ListKind::F64;

  fn lists<'a>(&'_ self) -> Lists<'_, FloatWrapper> {
    let in_list = (!self.r#in.is_empty()).then(|| {
      let list: Vec<FloatWrapper> = self
        .r#in
        .iter()
        .map(|fl| FloatWrapper {
          float: OrderedFloat(*fl),
          is_f32: false,
        })
        .collect();

      Cow::Owned(list)
    });

    let not_in_list = (!self.not_in.is_empty()).then(|| {
      let list: Vec<FloatWrapper> = self
        .not_in
        .iter()
        .map(|fl| FloatWrapper {
          float: OrderedFloat(*fl),
          is_f32: false,
        })
        .collect();

      Cow::Owned(list)
    });

    Lists {
      in_list,
      not_in_list,
    }
  }
}

use std::sync::LazyLock;

use regex::Regex;
use syn::{parse::ParseStream, Attribute, Error, LitStr, Token};

use crate::Ident2;

pub fn extract_proto_name_attribute(
  oneof_name: &str,
  attr: &Attribute,
  variant_ident: &Ident2,
  meta: syn::meta::ParseNestedMeta<'_>,
) -> Result<String, Error> {
  let not_found_error = Error::new_spanned(
    attr,
    format!(
      "Could not extract proto_name attribute for variant {} in oneof enum {}",
      variant_ident, oneof_name,
    ),
  );

  if meta.path.is_ident("proto_name") {
    if let Ok(proto_name_tokens) = meta.value() {
      Ok(
        proto_name_tokens
          .parse::<LitStr>()
          .map_err(|e| {
            Error::new_spanned(
              attr,
              format!(
                "Could not extract proto_name attribute for variant {} in oneof enum {}: {}",
                variant_ident, oneof_name, e
              ),
            )
          })?
          .value(),
      )
    } else {
      Err(not_found_error)
    }
  } else {
    Err(not_found_error)
  }
}

pub struct ProstAttrData {
  pub enum_path: Option<String>,
}

static MAP_ENUM_REGEX: LazyLock<Regex> = LazyLock::new(|| {
  Regex::new(r"^[^,]+,\s*enumeration\(([^)]+)\)$").expect("Failed to compile MAP_ENUM_REGEX")
});

impl syn::parse::Parse for ProstAttrData {
  fn parse(input: ParseStream) -> Result<Self, Error> {
    let mut enum_path: Option<String> = None;

    while !input.is_empty() {
      let ident: syn::Ident = input.parse()?;

      if ident == "enumeration" {
        input.parse::<syn::Token![=]>()?;
        let lit_str: LitStr = input.parse()?;
        enum_path = Some(lit_str.value());
      } else if ident == "map" {
        input.parse::<Token![=]>()?;
        let lit_str: LitStr = input.parse()?;
        let content = lit_str.value();
        if let Some(captures) = MAP_ENUM_REGEX.captures(&content)
          && let Some(enum_name_match) = captures.get(1) {
            enum_path = Some(enum_name_match.as_str().to_string());
          }
      } else if input.peek(Token![=]) {
        input.parse::<Token![=]>()?;
        input.parse::<syn::Lit>()?;
      }

      if input.peek(Token![,]) {
        input.parse::<Token![,]>()?;
      }
    }

    Ok(ProstAttrData { enum_path })
  }
}

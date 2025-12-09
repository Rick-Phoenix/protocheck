mod oneof_attrs;

pub use oneof_attrs::*;

use crate::*;

pub fn extract_proto_name_attribute(
  oneof_name: &str,
  attr: &Attribute,
  variant_ident: &Ident,
  meta: syn::meta::ParseNestedMeta<'_>,
) -> Result<String, Error> {
  let not_found_error = Error::new_spanned(
    attr,
    format!(
      "Could not extract name attribute for variant {} in oneof enum {}",
      variant_ident, oneof_name,
    ),
  );

  if meta.path.is_ident("name") {
    if let Ok(proto_name_tokens) = meta.value() {
      Ok(
        proto_name_tokens
          .parse::<LitStr>()
          .map_err(|e| {
            Error::new_spanned(
              attr,
              format!(
                "Could not extract name attribute for variant {} in oneof enum {}: {}",
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
    let metas = Punctuated::<Meta, Token![,]>::parse_terminated(input)?;

    let mut enum_path: Option<String> = None;

    for meta in metas {
      if let Meta::NameValue(nv) = meta {
        let ident = nv.path.require_ident()?.to_string();

        match ident.as_str() {
          "enumeration" => {
            let lit_str: LitStr = syn::parse2(nv.value.into_token_stream())?;

            enum_path = Some(lit_str.value());
          }
          "map" => {
            let lit_str: LitStr = syn::parse2(nv.value.into_token_stream())?;
            let map_attr = lit_str.value();

            if let Some(captures) = MAP_ENUM_REGEX.captures(&map_attr)
            && let Some(enum_name_match) = captures.get(1) {
              enum_path = Some(enum_name_match.as_str().to_string());
            }
          }
          _ => {}
        };
      }
    }

    Ok(ProstAttrData { enum_path })
  }
}

pub fn extract_string_lit(expr: &Expr) -> Result<String, Error> {
  if let Expr::Lit(expr_lit) = expr && let Lit::Str(value) = &expr_lit.lit {
    Ok(value.value())
  } else {
    Err(error!(expr, "Expected a string literal"))
  }
}

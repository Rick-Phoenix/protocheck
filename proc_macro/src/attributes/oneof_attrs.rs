use crate::{attributes::MAP_ENUM_REGEX, *};

pub struct OneofVariantAttrs {
  pub name: String,
  pub enum_path: Option<String>,
}

#[allow(clippy::single_match)]
pub fn extract_oneof_variant_attrs(
  attributes: &[Attribute],
  variant_ident: &Ident,
) -> Result<OneofVariantAttrs, Error> {
  let mut name: Option<String> = None;
  let mut enum_path: Option<String> = None;

  for attr in attributes {
    let attr_ident = if let Some(ident) = attr.path().get_ident() {
      ident.to_string()
    } else {
      continue;
    };

    match attr_ident.as_str() {
      "protocheck" => {
        let args = attr.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)?;

        for arg in args {
          match arg {
            Meta::NameValue(nv) => {
              let ident = nv.path.require_ident()?.to_string();

              match ident.as_str() {
                "name" => name = Some(extract_string_lit(&nv.value)?),
                _ => {}
              };
            }
            Meta::Path(_) => {}
            Meta::List(_) => {}
          };
        }
      }

      "prost" => {
        let args = attr.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)?;

        for arg in args {
          match arg {
            Meta::NameValue(nv) => {
              let ident = nv.path.require_ident()?.to_string();

              match ident.as_str() {
                "enumeration" => enum_path = Some(extract_string_lit(&nv.value)?),
                "map" => {
                  let map_attr = extract_string_lit(&nv.value)?;

                  if let Some(captures) = MAP_ENUM_REGEX.captures(&map_attr)
                    && let Some(enum_name_match) = captures.get(1) {
                      enum_path = Some(enum_name_match.as_str().to_string());
                  }
                }
                _ => {}
              };
            }
            Meta::Path(_) => {}
            Meta::List(_) => {}
          };
        }
      }
      _ => continue,
    };
  }

  let name = name.unwrap_or_else(|| variant_ident.to_string().to_case(Case::Snake));

  Ok(OneofVariantAttrs { name, enum_path })
}

use quote::quote;
use quote::ToTokens;

#[derive(Clone, Debug)]
pub struct FieldData {
  pub name: String,
  pub tag: u32,
  pub is_repeated: bool,
  pub is_map: bool,
  pub is_required: bool,
}

impl ToTokens for FieldData {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    let name = &self.name;
    let tag = self.tag;
    let is_repeated = self.is_repeated;
    let is_map = self.is_map;
    let is_required = self.is_required;

    tokens.extend(quote! {
        proto_types::FieldData {
            name: #name.to_string(),
            tag: #tag,
            is_repeated: #is_repeated,
            is_map: #is_map,
            is_required: #is_required,
        }
    });
  }
}

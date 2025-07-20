use proc_macro2::{Ident, Span, TokenStream};
use proto_types::FieldData;
use quote::quote;

use super::CelRule;
use super::CelRuleValue;
use crate::validator::buf::validate::RepeatedRules;
use crate::validator::get_field_rules;

pub fn get_repeated_rules(
  field_data: FieldData,
  repeated_rules: &RepeatedRules,
) -> Result<Vec<TokenStream>, Box<dyn std::error::Error>> {
  let mut rules: Vec<TokenStream> = Vec::new();
  let field_name = field_data.name.clone();
  let field_name_ident = Ident::new(&field_name, Span::call_site());

  // let index = Ident::new("index", Span::mixed_site());

  if repeated_rules.items.is_some() {
    let items_rules_descriptor = repeated_rules.items.clone().unwrap();
    let items_rules = get_field_rules(field_data, &items_rules_descriptor)?;
    rules.extend(items_rules);
  }

  // if rules.len() > 0 {
  //   let validator = quote! {
  //     for (#index, _) in self.#field_name_ident.iter().enumerate() {
  //       #(#rules)*
  //     }
  //   };
  //
  //   rules.push(validator);
  // }

  // if repeated_rules.min_items.is_some() {
  //   let min_items_value = repeated_rules.min_items.unwrap();
  //
  //   rules.push(CelRule {
  //     id: "repeated.min_items".to_string(),
  //     message: "".to_string(),
  //     expression: "uint(this.size()) < rules.min_items ? 'value must contain at least %d item(s)'.format([rules.min_items]) : ''".to_string(),
  //     value: CelRuleValue::U64(min_items_value),
  //   });
  // }
  //
  // if repeated_rules.max_items.is_some() {
  //   let max_items_value = repeated_rules.max_items.unwrap();
  //
  //   rules.push(CelRule {
  //     id: "repeated.max_items".to_string(),
  //     message: "".to_string(),
  //     expression: "uint(this.size()) > rules.max_items ? 'value must contain no more than %s item(s)'.format([rules.max_items]) : ''".to_string(),
  //     value: CelRuleValue::U64(max_items_value),
  //   });
  // }
  //
  // if repeated_rules.unique.is_some() {
  //   rules.push(CelRule {
  //     id: "repeated.unique".to_string(),
  //     message: "".to_string(),
  //     expression: "!rules.unique || this.unique()".to_string(),
  //     value: CelRuleValue::Bool(true),
  //   });
  // }

  Ok(rules)
}

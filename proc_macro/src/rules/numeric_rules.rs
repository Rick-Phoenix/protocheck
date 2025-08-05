// use proto_types::protovalidate_impls::{
//   ComparableGreaterThan, ComparableLessThan, ContainingRules, NumericRules,
// };
// use quote::{quote, ToTokens};
// use syn::Error;
//
// use super::{ValidatorKind, ValidatorTemplate};
// use crate::{validation_data::ValidationData, validator_template::FieldValidator};
//
// pub fn get_numeric_rules<T: NumericRules>(
//   validation_data: &ValidationData,
//   rules: &T,
// ) -> Result<Vec<ValidatorTemplate>, Error> {
//   let mut templates: Vec<ValidatorTemplate> = Vec::new();
//
//   let field_span = validation_data.field_span;
//
//   let error_prefix = format!(
//     "Error for field {}:",
//     &validation_data.field_data.proto_name
//   );
//
//   if let Some(const_val) = rules.constant() {
//     templates.push(ValidatorTemplate {
//       item_rust_name: validation_data.field_data.rust_name.clone(),
//       kind: ValidatorKind::Field {
//         validation_data: validation_data.clone(),
//         field_validator: FieldValidator::SingleField {
//           validator_path: quote! { protocheck::validators::constants::constant },
//           target_value_tokens: const_val.to_token_stream(),
//         },
//       },
//     });
//     return Ok(templates);
//   }
//
//   let comparable_rules = rules.comparable_rules(field_span, &error_prefix)?;
//   let ContainingRules {
//     in_list,
//     not_in_list,
//   } = rules.containing_rules(field_span, &error_prefix)?;
//
//   if !in_list.is_empty() {
//     templates.push(ValidatorTemplate {
//       item_rust_name: validation_data.field_data.rust_name.clone(),
//       kind: ValidatorKind::Field {
//         validation_data: validation_data.clone(),
//         field_validator: FieldValidator::SingleField {
//           validator_path: quote! { protocheck::validators::containing::in_list },
//           target_value_tokens: quote! { vec![ #(#in_list),* ] },
//         },
//       },
//     });
//   }
//
//   if !not_in_list.is_empty() {
//     templates.push(ValidatorTemplate {
//       item_rust_name: validation_data.field_data.rust_name.clone(),
//       kind: ValidatorKind::Field {
//         validation_data: validation_data.clone(),
//         field_validator: FieldValidator::SingleField {
//           validator_path: quote! { protocheck::validators::containing::not_in_list },
//           target_value_tokens: quote! { vec![ #(#not_in_list),* ] },
//         },
//       },
//     });
//   }
//
//   if let Some(lt_rule) = comparable_rules.less_than {
//     match lt_rule {
//       ComparableLessThan::Lt(val) => {
//         templates.push(ValidatorTemplate {
//           item_rust_name: validation_data.field_data.rust_name.clone(),
//           kind: ValidatorKind::Field {
//             validation_data: validation_data.clone(),
//             field_validator: FieldValidator::SingleField {
//               validator_path: quote! { protocheck::validators::comparables::lt },
//               target_value_tokens: val.to_token_stream(),
//             },
//           },
//         });
//       }
//       ComparableLessThan::Lte(val) => {
//         templates.push(ValidatorTemplate {
//           item_rust_name: validation_data.field_data.rust_name.clone(),
//           kind: ValidatorKind::Field {
//             validation_data: validation_data.clone(),
//             field_validator: FieldValidator::SingleField {
//               validator_path: quote! { protocheck::validators::comparables::lte },
//               target_value_tokens: val.to_token_stream(),
//             },
//           },
//         });
//       }
//     };
//   }
//
//   if let Some(gt_rule) = comparable_rules.greater_than {
//     match gt_rule {
//       ComparableGreaterThan::Gt(val) => {
//         templates.push(ValidatorTemplate {
//           item_rust_name: validation_data.field_data.rust_name.clone(),
//           kind: ValidatorKind::Field {
//             validation_data: validation_data.clone(),
//             field_validator: FieldValidator::SingleField {
//               validator_path: quote! { protocheck::validators::comparables::gt },
//               target_value_tokens: val.to_token_stream(),
//             },
//           },
//         });
//       }
//       ComparableGreaterThan::Gte(val) => {
//         templates.push(ValidatorTemplate {
//           item_rust_name: validation_data.field_data.rust_name.clone(),
//           kind: ValidatorKind::Field {
//             validation_data: validation_data.clone(),
//             field_validator: FieldValidator::SingleField {
//               validator_path: quote! { protocheck::validators::comparables::gte },
//               target_value_tokens: val.to_token_stream(),
//             },
//           },
//         });
//       }
//     };
//   }
//
//   Ok(templates)
// }

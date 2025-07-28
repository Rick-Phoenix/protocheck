// use proc_macro2::Span;
// use prost_reflect::Kind as FieldKind;
// use syn::Error;
//
// use crate::rules::protovalidate::field_rules::Type as RulesType;
//
// pub fn check_rules_compat(
//   rules_type: &RulesType,
//   field_kind: &FieldKind,
//   is_repeated: bool,
//   field_name: &str,
//   span: Span,
// ) -> Result<(), Error> {
//   let rules_name = get_rules_type(rules_type);
//   let type_name = get_proto_type(field_kind);
//   if rules_name == "repeated" && !is_repeated {
//     Err(Error::new(
//       span,
//       format!(
//         "Cannot use repeated rules on non-repeated field {}.",
//         field_name
//       ),
//     ))
//   } else if rules_name != type_name {
//     Err(Error::new(
//       span,
//       format!(
//         "Mismatch in field rules for field {}. Field type: {}, Rules type: {}",
//         field_name, type_name, rules_name
//       ),
//     ))
//   } else {
//     Ok(())
//   }
// }
//
// pub fn get_rules_type(rules_type: &RulesType) -> &str {
//   println!("{:#?}", rules_type);
//   match rules_type {
//     RulesType::Float(_) => "float",
//     RulesType::Double(_) => "double",
//     RulesType::Int32(_) => "int32",
//     RulesType::Int64(_) => "int64",
//     RulesType::Uint32(_) => "uint32",
//     RulesType::Uint64(_) => "uint64",
//     RulesType::Sint32(_) => "sint32",
//     RulesType::Sint64(_) => "sint64",
//     RulesType::Fixed32(_) => "fixed32",
//     RulesType::Fixed64(_) => "fixed64",
//     RulesType::Sfixed32(_) => "sfixed32",
//     RulesType::Sfixed64(_) => "sfixed64",
//     RulesType::Bool(_) => "bool",
//     RulesType::String(_) => "string",
//     RulesType::Bytes(_) => "bytes",
//     RulesType::Enum(_) => "enum",
//     RulesType::Map(_) => "map",
//     RulesType::Any(_) => "Any",
//     RulesType::Timestamp(_) => "Timestamp",
//     RulesType::Duration(_) => "Duration",
//     RulesType::Repeated(_) => "repeated",
//   }
// }
//
// pub fn _get_proto_type(kind: &FieldKind) -> &str {
//   match kind {
//     FieldKind::Float => "float",
//     FieldKind::Double => "double",
//     FieldKind::Int32 => "int32",
//     FieldKind::Int64 => "int64",
//     FieldKind::Uint32 => "uint32",
//     FieldKind::Uint64 => "uint64",
//     FieldKind::Sint32 => "sint32",
//     FieldKind::Sint64 => "sint64",
//     FieldKind::Fixed32 => "fixed32",
//     FieldKind::Fixed64 => "fixed64",
//     FieldKind::Sfixed32 => "sfixed32",
//     FieldKind::Sfixed64 => "sfixed64",
//     FieldKind::Bool => "bool",
//     FieldKind::String => "string",
//     FieldKind::Bytes => "bytes",
//     FieldKind::Enum(_) => "enum",
//     FieldKind::Message(message) => {
//       if message.is_map_entry() {
//         "map"
//       } else {
//         match message.full_name() {
//           "google.protobuf.Any" => "Any",
//           "google.protobuf.Duration" => "Duration",
//           "google.protobuf.Timestamp" => "Timestamp",
//           _ => message.full_name(),
//         }
//       }
//     }
//   }
// }

use super::{
  field_rules, FieldData, FieldRules, GeneratedCodeKind, Type as ProtoType, ValidatorCallTemplate,
};
use crate::{
  pool_loader::DESCRIPTOR_POOL,
  rules::{enum_rules, map_rules, map_rules::get_map_rules, repeated_rules, string_rules},
};

pub fn get_field_rules(
  field_data: &FieldData,
  field_rules: &FieldRules,
) -> Result<Vec<ValidatorCallTemplate>, Box<dyn std::error::Error>> {
  if let Some(rules_type) = field_rules.r#type.clone() {
    match rules_type {
      field_rules::Type::String(string_rules) => {
        string_rules::get_string_rules(field_data, &string_rules)
      }
      // field_rules::Type::Int64(int64_rules) => numeric_rules::get_int64_rules(&int64_rules),
      // field_rules::Type::Int32(int32_rules) => numeric_rules::get_int32_rules(&int32_rules),
      // field_rules::Type::Bytes(bytes_rules) => bytes_rules::get_bytes_rules(&bytes_rules),
      // field_rules::Type::Bool(bool_rules) => bool_rules::get_bool_rules(&bool_rules),
      // field_rules::Type::Enum(enum_rules) => enum_rules::get_enum_rules(field_data, &enum_rules),

      // field_rules::Type::Map(map_rules) => map_rules::get_map_rules(field_data, &map_rules),
      // field_rules::Type::Any(any_rules) => any_rules::get_any_rules(&any_rules),
      // field_rules::Type::Duration(dur_rules) => duration_rules::get_duration_rules(&dur_rules),
      // field_rules::Type::Timestamp(time_rules) => timestamp_rules::get_timestamp_rules(&time_rules),
      _ => Ok(Vec::new()),
    }
  } else {
    Ok(Vec::new())
  }
}

pub fn convert_kind_to_proto_type(kind: Kind) -> ProtoType {
  match kind {
    Kind::Double => ProtoType::Double,
    Kind::Float => ProtoType::Float,
    Kind::Int32 => ProtoType::Int32,
    Kind::Int64 => ProtoType::Int64,
    Kind::Uint32 => ProtoType::Uint32,
    Kind::Uint64 => ProtoType::Uint64,
    Kind::Sint32 => ProtoType::Sint32,
    Kind::Sint64 => ProtoType::Sint64,
    Kind::Fixed32 => ProtoType::Fixed32,
    Kind::Fixed64 => ProtoType::Fixed64,
    Kind::Sfixed32 => ProtoType::Sfixed32,
    Kind::Sfixed64 => ProtoType::Sfixed64,
    Kind::Bool => ProtoType::Bool,
    Kind::String => ProtoType::String,
    Kind::Bytes => ProtoType::Bytes,
    Kind::Message(_) => ProtoType::Message,
    Kind::Enum(_) => ProtoType::Enum,
  }
}

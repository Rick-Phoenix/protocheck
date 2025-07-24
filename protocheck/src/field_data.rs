use proto_types::protovalidate::{field_path_element::Subscript, FieldPathElement, Ignore};

#[derive(Clone, Debug)]
pub struct FieldContext<'a> {
  pub field_data: FieldData,
  pub parent_elements: &'a [FieldPathElement],
  pub subscript: Option<Subscript>,
}

#[derive(Clone, Debug, Default)]
pub struct FieldData {
  pub rust_name: String,
  pub proto_name: String,
  pub tag: u32,
  pub is_repeated: bool,
  pub is_repeated_item: bool,
  pub is_map: bool,
  pub is_map_key: bool,
  pub is_map_value: bool,
  pub is_required: bool,
  pub is_optional: bool,
  pub key_type: Option<ProtoType>,
  pub value_type: Option<ProtoType>,
  pub proto_type: ProtoType,
  pub enum_full_name: Option<String>,
  pub ignore: Ignore,
}

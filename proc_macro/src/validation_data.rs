use crate::{rules::FieldData, Ident2, Span2};

#[derive(Debug, Clone)]
pub(crate) struct ValidationData {
  pub is_required: bool,
  pub is_optional: bool,
  pub is_in_oneof: bool,
  pub field_data_static_ident: Ident2,
  pub field_span: Span2,
  pub field_data: FieldData,
}

use std::collections::HashMap;

use cel::{objects::Key as CelKey, Value as CelValue};

use crate::rpc::ErrorInfo;

impl From<ErrorInfo> for CelValue {
  fn from(value: ErrorInfo) -> Self {
    let mut cel_map: HashMap<CelKey, CelValue> = HashMap::new();
    cel_map.insert(
      "reason".to_string().into(),
      CelValue::String(value.reason.clone().into()),
    );
    cel_map.insert(
      "domain".to_string().into(),
      CelValue::String(value.domain.clone().into()),
    );

    cel_map.insert(
      "metadata".to_string().into(),
      CelValue::Map(value.metadata.clone().into()),
    );

    CelValue::Map(cel_map.into())
  }
}

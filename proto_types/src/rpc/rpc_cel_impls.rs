use std::collections::HashMap;

use cel::{objects::Key as CelKey, Value as CelValue};

use crate::{
  cel::CelConversionError,
  rpc::{
    bad_request::FieldViolation, help::Link, precondition_failure, quota_failure, DebugInfo,
    ErrorInfo, Help, HttpHeader, HttpRequest, HttpResponse, LocalizedMessage, PreconditionFailure,
    QuotaFailure, RequestInfo, ResourceInfo, RetryInfo, Status,
  },
  BadRequest,
};

impl From<ErrorInfo> for CelValue {
  fn from(value: ErrorInfo) -> Self {
    let mut cel_map: HashMap<CelKey, CelValue> = HashMap::new();
    cel_map.insert("reason".into(), CelValue::String(value.reason.into()));
    cel_map.insert("domain".into(), CelValue::String(value.domain.into()));

    cel_map.insert("metadata".into(), CelValue::Map(value.metadata.into()));

    CelValue::Map(cel_map.into())
  }
}

impl TryFrom<RetryInfo> for CelValue {
  type Error = CelConversionError;
  fn try_from(value: RetryInfo) -> Result<CelValue, CelConversionError> {
    let mut cel_map: HashMap<CelKey, CelValue> = HashMap::new();
    let value = match value.retry_delay {
      Some(v) => CelValue::Duration(v.try_into()?),
      None => CelValue::Null,
    };

    cel_map.insert("retry_delay".into(), value);

    Ok(CelValue::Map(cel_map.into()))
  }
}

impl From<DebugInfo> for CelValue {
  fn from(value: DebugInfo) -> Self {
    let mut cel_map: HashMap<CelKey, CelValue> = HashMap::new();
    let stack_entries: Vec<CelValue> = value.stack_entries.into_iter().map(|i| i.into()).collect();

    cel_map.insert("stack_entries".into(), CelValue::List(stack_entries.into()));
    cel_map.insert("detail".into(), CelValue::String(value.detail.into()));

    CelValue::Map(cel_map.into())
  }
}

impl From<quota_failure::Violation> for CelValue {
  fn from(value: quota_failure::Violation) -> Self {
    let mut cel_map: HashMap<CelKey, CelValue> = HashMap::new();
    cel_map.insert("subject".into(), CelValue::String(value.subject.into()));
    cel_map.insert(
      "description".into(),
      CelValue::String(value.description.into()),
    );
    cel_map.insert(
      "api_service".into(),
      CelValue::String(value.api_service.into()),
    );
    cel_map.insert(
      "quota_metric".into(),
      CelValue::String(value.quota_metric.into()),
    );
    cel_map.insert("quota_id".into(), CelValue::String(value.quota_id.into()));

    cel_map.insert(
      "quota_dimensions".into(),
      CelValue::Map(value.quota_dimensions.into()),
    );

    cel_map.insert("quota_value".into(), CelValue::Int(value.quota_value));

    let future_quota_value = value
      .future_quota_value
      .map_or(CelValue::Null, CelValue::Int);
    cel_map.insert("future_quota_value".into(), future_quota_value);

    CelValue::Map(cel_map.into())
  }
}

impl From<QuotaFailure> for CelValue {
  fn from(value: QuotaFailure) -> Self {
    let mut cel_map: HashMap<CelKey, CelValue> = HashMap::new();
    let violations: Vec<CelValue> = value.violations.into_iter().map(|v| v.into()).collect();
    cel_map.insert("violations".into(), CelValue::List(violations.into()));

    CelValue::Map(cel_map.into())
  }
}

impl From<PreconditionFailure> for CelValue {
  fn from(value: PreconditionFailure) -> Self {
    let mut cel_map: HashMap<CelKey, CelValue> = HashMap::new();
    let violations: Vec<CelValue> = value.violations.into_iter().map(|v| v.into()).collect();
    cel_map.insert("violations".into(), CelValue::List(violations.into()));

    CelValue::Map(cel_map.into())
  }
}

impl From<precondition_failure::Violation> for CelValue {
  fn from(value: precondition_failure::Violation) -> Self {
    let mut cel_map: HashMap<CelKey, CelValue> = HashMap::new();
    cel_map.insert("type".into(), CelValue::String(value.r#type.into()));
    cel_map.insert("subject".into(), CelValue::String(value.subject.into()));
    cel_map.insert(
      "description".into(),
      CelValue::String(value.description.into()),
    );

    CelValue::Map(cel_map.into())
  }
}

impl From<LocalizedMessage> for CelValue {
  fn from(value: LocalizedMessage) -> Self {
    let mut cel_map: HashMap<CelKey, CelValue> = HashMap::new();
    cel_map.insert("locale".into(), CelValue::String(value.locale.into()));
    cel_map.insert("message".into(), CelValue::String(value.message.into()));

    CelValue::Map(cel_map.into())
  }
}

impl From<BadRequest> for CelValue {
  fn from(value: BadRequest) -> Self {
    let mut cel_map: HashMap<CelKey, CelValue> = HashMap::new();
    cel_map.insert(
      "field_violations".into(),
      CelValue::List(
        value
          .field_violations
          .into_iter()
          .map(|i| i.into())
          .collect::<Vec<CelValue>>()
          .into(),
      ),
    );

    CelValue::Map(cel_map.into())
  }
}

impl From<FieldViolation> for CelValue {
  fn from(value: FieldViolation) -> Self {
    let mut cel_map: HashMap<CelKey, CelValue> = HashMap::new();
    cel_map.insert("field".into(), CelValue::String(value.field.into()));

    cel_map.insert(
      "description".into(),
      CelValue::String(value.description.into()),
    );

    cel_map.insert("reason".into(), CelValue::String(value.reason.into()));

    cel_map.insert(
      "localized_message".into(),
      value.localized_message.map_or(CelValue::Null, |v| v.into()),
    );

    CelValue::Map(cel_map.into())
  }
}

impl From<RequestInfo> for CelValue {
  fn from(value: RequestInfo) -> Self {
    let mut cel_map: HashMap<CelKey, CelValue> = HashMap::new();

    cel_map.insert(
      "request_id".into(),
      CelValue::String(value.request_id.into()),
    );
    cel_map.insert(
      "serving_data".into(),
      CelValue::String(value.serving_data.into()),
    );

    CelValue::Map(cel_map.into())
  }
}

impl From<ResourceInfo> for CelValue {
  fn from(value: ResourceInfo) -> Self {
    let mut cel_map: HashMap<CelKey, CelValue> = HashMap::new();

    cel_map.insert(
      "resource_type".into(),
      CelValue::String(value.resource_type.into()),
    );
    cel_map.insert(
      "resource_name".into(),
      CelValue::String(value.resource_name.into()),
    );

    cel_map.insert("owner".into(), CelValue::String(value.owner.into()));
    cel_map.insert(
      "description".into(),
      CelValue::String(value.description.into()),
    );

    CelValue::Map(cel_map.into())
  }
}

impl From<Help> for CelValue {
  fn from(value: Help) -> Self {
    let mut cel_map: HashMap<CelKey, CelValue> = HashMap::new();

    cel_map.insert(
      "links".into(),
      CelValue::List(
        value
          .links
          .into_iter()
          .map(|l| l.into())
          .collect::<Vec<CelValue>>()
          .into(),
      ),
    );

    CelValue::Map(cel_map.into())
  }
}

impl From<Link> for CelValue {
  fn from(value: Link) -> Self {
    let mut cel_map: HashMap<CelKey, CelValue> = HashMap::new();

    cel_map.insert(
      "description".into(),
      CelValue::String(value.description.into()),
    );
    cel_map.insert("url".into(), CelValue::String(value.url.into()));

    CelValue::Map(cel_map.into())
  }
}

impl From<HttpHeader> for CelValue {
  fn from(value: HttpHeader) -> Self {
    let mut cel_map: HashMap<CelKey, CelValue> = HashMap::new();

    cel_map.insert("key".into(), CelValue::String(value.key.into()));
    cel_map.insert("value".into(), CelValue::String(value.value.into()));

    CelValue::Map(cel_map.into())
  }
}

impl From<HttpRequest> for CelValue {
  fn from(value: HttpRequest) -> Self {
    let mut cel_map: HashMap<CelKey, CelValue> = HashMap::new();

    cel_map.insert("method".into(), CelValue::String(value.method.into()));
    cel_map.insert("uri".into(), CelValue::String(value.uri.into()));
    cel_map.insert(
      "headers".into(),
      CelValue::List(
        value
          .headers
          .into_iter()
          .map(|h| h.into())
          .collect::<Vec<CelValue>>()
          .into(),
      ),
    );
    cel_map.insert("body".into(), CelValue::Bytes(value.body.to_vec().into()));

    CelValue::Map(cel_map.into())
  }
}

impl From<HttpResponse> for CelValue {
  fn from(value: HttpResponse) -> Self {
    let mut cel_map: HashMap<CelKey, CelValue> = HashMap::new();

    cel_map.insert("status".into(), CelValue::Int(value.status.into()));
    cel_map.insert("reason".into(), CelValue::String(value.reason.into()));
    cel_map.insert(
      "headers".into(),
      CelValue::List(
        value
          .headers
          .into_iter()
          .map(|h| h.into())
          .collect::<Vec<CelValue>>()
          .into(),
      ),
    );
    cel_map.insert("body".into(), CelValue::Bytes(value.body.to_vec().into()));

    CelValue::Map(cel_map.into())
  }
}

impl From<Status> for CelValue {
  fn from(value: Status) -> Self {
    let mut cel_map: HashMap<CelKey, CelValue> = HashMap::new();

    cel_map.insert("code".into(), CelValue::Int(value.code.into()));
    cel_map.insert("message".into(), CelValue::String(value.message.into()));

    cel_map.insert("details".into(), value.details.into());

    CelValue::Map(cel_map.into())
  }
}

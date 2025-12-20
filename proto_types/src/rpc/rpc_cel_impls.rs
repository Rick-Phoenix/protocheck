use std::collections::HashMap;

use cel::{Value as CelValue, objects::Key as CelKey};

use crate::{
  BadRequest,
  cel::CelConversionError,
  rpc::{
    DebugInfo, ErrorInfo, Help, HttpHeader, HttpRequest, HttpResponse, LocalizedMessage,
    PreconditionFailure, QuotaFailure, RequestInfo, ResourceInfo, RetryInfo,
    bad_request::FieldViolation, help::Link, precondition_failure, quota_failure,
  },
};

impl From<ErrorInfo> for CelValue {
  fn from(value: ErrorInfo) -> Self {
    let mut cel_map: HashMap<CelKey, Self> = HashMap::new();
    cel_map.insert("reason".into(), Self::String(value.reason.into()));
    cel_map.insert("domain".into(), Self::String(value.domain.into()));

    cel_map.insert("metadata".into(), Self::Map(value.metadata.into()));

    Self::Map(cel_map.into())
  }
}

impl TryFrom<RetryInfo> for CelValue {
  type Error = CelConversionError;
  fn try_from(value: RetryInfo) -> Result<Self, CelConversionError> {
    let mut cel_map: HashMap<CelKey, Self> = HashMap::new();
    let value = match value.retry_delay {
      Some(v) => Self::Duration(v.try_into()?),
      None => Self::Null,
    };

    cel_map.insert("retry_delay".into(), value);

    Ok(Self::Map(cel_map.into()))
  }
}

impl From<DebugInfo> for CelValue {
  fn from(value: DebugInfo) -> Self {
    let mut cel_map: HashMap<CelKey, Self> = HashMap::new();
    let stack_entries: Vec<Self> = value
      .stack_entries
      .into_iter()
      .map(|i| i.into())
      .collect();

    cel_map.insert("stack_entries".into(), Self::List(stack_entries.into()));
    cel_map.insert("detail".into(), Self::String(value.detail.into()));

    Self::Map(cel_map.into())
  }
}

impl From<quota_failure::Violation> for CelValue {
  fn from(value: quota_failure::Violation) -> Self {
    let mut cel_map: HashMap<CelKey, Self> = HashMap::new();
    cel_map.insert("subject".into(), Self::String(value.subject.into()));
    cel_map.insert("description".into(), Self::String(value.description.into()));
    cel_map.insert("api_service".into(), Self::String(value.api_service.into()));
    cel_map.insert(
      "quota_metric".into(),
      Self::String(value.quota_metric.into()),
    );
    cel_map.insert("quota_id".into(), Self::String(value.quota_id.into()));

    cel_map.insert(
      "quota_dimensions".into(),
      Self::Map(value.quota_dimensions.into()),
    );

    cel_map.insert("quota_value".into(), Self::Int(value.quota_value));

    let future_quota_value = value
      .future_quota_value
      .map_or(Self::Null, Self::Int);
    cel_map.insert("future_quota_value".into(), future_quota_value);

    Self::Map(cel_map.into())
  }
}

impl From<QuotaFailure> for CelValue {
  fn from(value: QuotaFailure) -> Self {
    let mut cel_map: HashMap<CelKey, Self> = HashMap::new();
    let violations: Vec<Self> = value
      .violations
      .into_iter()
      .map(|v| v.into())
      .collect();
    cel_map.insert("violations".into(), Self::List(violations.into()));

    Self::Map(cel_map.into())
  }
}

impl From<PreconditionFailure> for CelValue {
  fn from(value: PreconditionFailure) -> Self {
    let mut cel_map: HashMap<CelKey, Self> = HashMap::new();
    let violations: Vec<Self> = value
      .violations
      .into_iter()
      .map(|v| v.into())
      .collect();
    cel_map.insert("violations".into(), Self::List(violations.into()));

    Self::Map(cel_map.into())
  }
}

impl From<precondition_failure::Violation> for CelValue {
  fn from(value: precondition_failure::Violation) -> Self {
    let mut cel_map: HashMap<CelKey, Self> = HashMap::new();
    cel_map.insert("type".into(), Self::String(value.r#type.into()));
    cel_map.insert("subject".into(), Self::String(value.subject.into()));
    cel_map.insert("description".into(), Self::String(value.description.into()));

    Self::Map(cel_map.into())
  }
}

impl From<LocalizedMessage> for CelValue {
  fn from(value: LocalizedMessage) -> Self {
    let mut cel_map: HashMap<CelKey, Self> = HashMap::new();
    cel_map.insert("locale".into(), Self::String(value.locale.into()));
    cel_map.insert("message".into(), Self::String(value.message.into()));

    Self::Map(cel_map.into())
  }
}

impl From<BadRequest> for CelValue {
  fn from(value: BadRequest) -> Self {
    let mut cel_map: HashMap<CelKey, Self> = HashMap::new();
    cel_map.insert(
      "field_violations".into(),
      Self::List(
        value
          .field_violations
          .into_iter()
          .map(|i| i.into())
          .collect::<Vec<Self>>()
          .into(),
      ),
    );

    Self::Map(cel_map.into())
  }
}

impl From<FieldViolation> for CelValue {
  fn from(value: FieldViolation) -> Self {
    let mut cel_map: HashMap<CelKey, Self> = HashMap::new();
    cel_map.insert("field".into(), Self::String(value.field.into()));

    cel_map.insert("description".into(), Self::String(value.description.into()));

    cel_map.insert("reason".into(), Self::String(value.reason.into()));

    cel_map.insert(
      "localized_message".into(),
      value
        .localized_message
        .map_or(Self::Null, |v| v.into()),
    );

    Self::Map(cel_map.into())
  }
}

impl From<RequestInfo> for CelValue {
  fn from(value: RequestInfo) -> Self {
    let mut cel_map: HashMap<CelKey, Self> = HashMap::new();

    cel_map.insert("request_id".into(), Self::String(value.request_id.into()));
    cel_map.insert(
      "serving_data".into(),
      Self::String(value.serving_data.into()),
    );

    Self::Map(cel_map.into())
  }
}

impl From<ResourceInfo> for CelValue {
  fn from(value: ResourceInfo) -> Self {
    let mut cel_map: HashMap<CelKey, Self> = HashMap::new();

    cel_map.insert(
      "resource_type".into(),
      Self::String(value.resource_type.into()),
    );
    cel_map.insert(
      "resource_name".into(),
      Self::String(value.resource_name.into()),
    );

    cel_map.insert("owner".into(), Self::String(value.owner.into()));
    cel_map.insert("description".into(), Self::String(value.description.into()));

    Self::Map(cel_map.into())
  }
}

impl From<Help> for CelValue {
  fn from(value: Help) -> Self {
    let mut cel_map: HashMap<CelKey, Self> = HashMap::new();

    cel_map.insert(
      "links".into(),
      Self::List(
        value
          .links
          .into_iter()
          .map(|l| l.into())
          .collect::<Vec<Self>>()
          .into(),
      ),
    );

    Self::Map(cel_map.into())
  }
}

impl From<Link> for CelValue {
  fn from(value: Link) -> Self {
    let mut cel_map: HashMap<CelKey, Self> = HashMap::new();

    cel_map.insert("description".into(), Self::String(value.description.into()));
    cel_map.insert("url".into(), Self::String(value.url.into()));

    Self::Map(cel_map.into())
  }
}

impl From<HttpHeader> for CelValue {
  fn from(value: HttpHeader) -> Self {
    let mut cel_map: HashMap<CelKey, Self> = HashMap::new();

    cel_map.insert("key".into(), Self::String(value.key.into()));
    cel_map.insert("value".into(), Self::String(value.value.into()));

    Self::Map(cel_map.into())
  }
}

impl From<HttpRequest> for CelValue {
  fn from(value: HttpRequest) -> Self {
    let mut cel_map: HashMap<CelKey, Self> = HashMap::new();

    cel_map.insert("method".into(), Self::String(value.method.into()));
    cel_map.insert("uri".into(), Self::String(value.uri.into()));
    cel_map.insert(
      "headers".into(),
      Self::List(
        value
          .headers
          .into_iter()
          .map(|h| h.into())
          .collect::<Vec<Self>>()
          .into(),
      ),
    );
    cel_map.insert("body".into(), Self::Bytes(value.body.to_vec().into()));

    Self::Map(cel_map.into())
  }
}

impl From<HttpResponse> for CelValue {
  fn from(value: HttpResponse) -> Self {
    let mut cel_map: HashMap<CelKey, Self> = HashMap::new();

    cel_map.insert("status".into(), Self::Int(value.status.into()));
    cel_map.insert("reason".into(), Self::String(value.reason.into()));
    cel_map.insert(
      "headers".into(),
      Self::List(
        value
          .headers
          .into_iter()
          .map(|h| h.into())
          .collect::<Vec<Self>>()
          .into(),
      ),
    );
    cel_map.insert("body".into(), Self::Bytes(value.body.to_vec().into()));

    Self::Map(cel_map.into())
  }
}

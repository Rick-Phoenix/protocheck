use crate::rpc::{
  bad_request::FieldViolation, ErrorInfo, LocalizedMessage, PreconditionFailureViolation,
  QuotaFailureViolation, RequestInfo, ResourceInfo,
};

impl ErrorInfo {
  has_impl!(reason);
  has_impl!(domain);
}

impl QuotaFailureViolation {
  has_impl!(subject);
  has_impl!(description);
  has_impl!(api_service);
  has_impl!(quota_metric);
  has_impl!(quota_id);
}

impl PreconditionFailureViolation {
  has_impl!(type, r#type);
  has_impl!(subject);
  has_impl!(description);
}

impl FieldViolation {
  has_impl!(field);
  has_impl!(description);
  has_impl!(reason);
}

impl RequestInfo {
  has_impl!(request_id);
}

impl ResourceInfo {
  has_impl!(resource_type);
  has_impl!(resource_name);
  has_impl!(owner);
  has_impl!(description);
}

impl LocalizedMessage {
  has_impl!(locale);
}

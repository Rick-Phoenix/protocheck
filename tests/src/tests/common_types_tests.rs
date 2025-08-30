use proto_types::{
  bad_request::FieldViolation, help::Link, protovalidate::Violations, BadRequest, Color, Date,
  DateTime, Decimal, ErrorInfo, Expr, Fraction, Help, HttpHeader, HttpRequest, HttpResponse,
  Interval, LatLng, LocalizedMessage, LocalizedText, Money, PhoneNumber, PostalAddress,
  PreconditionFailure, Quaternion, QuotaFailure, RequestInfo, ResourceInfo, RetryInfo, Status,
  TimeOfDay, TimeZone,
};

use crate::myapp::v1::{CommonTypesTests, RpcTypesTests};

#[test]
fn rpc_types() {
  let test = RpcTypesTests {
    help: Some(Help {
      ..Default::default()
    }),
    link: Some(Link {
      ..Default::default()
    }),
    resource_info: Some(ResourceInfo {
      ..Default::default()
    }),
    request_info: Some(RequestInfo {
      ..Default::default()
    }),
    field_violation: Some(FieldViolation {
      ..Default::default()
    }),
    localized_message: Some(LocalizedMessage {
      ..Default::default()
    }),
    bad_request: Some(BadRequest {
      ..Default::default()
    }),
    precondition_failure_violation: Some(proto_types::precondition_failure::Violation {
      ..Default::default()
    }),
    precondition_failure: Some(PreconditionFailure {
      ..Default::default()
    }),
    http_request: Some(HttpRequest {
      ..Default::default()
    }),
    http_response: Some(HttpResponse {
      ..Default::default()
    }),
    http_header: Some(HttpHeader {
      ..Default::default()
    }),
    quota_failure: Some(QuotaFailure {
      ..Default::default()
    }),
    quota_failure_violation: Some(proto_types::quota_failure::Violation {
      ..Default::default()
    }),
    retry_info: Some(RetryInfo {
      ..Default::default()
    }),
    error_info: Some(ErrorInfo {
      ..Default::default()
    }),
    status: Some(Status {
      ..Default::default()
    }),
  };

  let Violations { violations } = test.validate().unwrap_err();

  assert_eq!(violations.len(), 17);
}

#[test]
fn common_types() {
  let test = CommonTypesTests {
    quaternion: Some(Quaternion {
      x: 2.0,
      y: 2.5,
      z: 2.5,
      w: 2.19,
    }),
    time_of_day: Some(TimeOfDay {
      ..Default::default()
    }),
    localized_text: Some(LocalizedText {
      ..Default::default()
    }),
    datetime: Some(DateTime {
      ..Default::default()
    }),
    money: Some(Money {
      ..Default::default()
    }),
    expr: Some(Expr {
      ..Default::default()
    }),
    interval: Some(Interval {
      ..Default::default()
    }),
    color: Some(Color {
      ..Default::default()
    }),
    date: Some(Date {
      ..Default::default()
    }),
    fraction: Some(Fraction {
      ..Default::default()
    }),
    postal_address: Some(PostalAddress {
      ..Default::default()
    }),
    decimal: Some(Decimal {
      ..Default::default()
    }),
    lat_lng: Some(LatLng {
      ..Default::default()
    }),
    timezone: Some(TimeZone {
      ..Default::default()
    }),
    phone_number: Some(PhoneNumber {
      ..Default::default()
    }),
  };

  let Violations { violations } = test.validate().unwrap_err();

  assert_eq!(violations.len(), 15);
}

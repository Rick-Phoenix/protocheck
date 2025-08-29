use std::fmt::Display;

use crate::rpc::{Code, HttpHeader, HttpRequest, HttpResponse};

impl HttpRequest {
  has_impl!(method);
  has_impl!(uri);

  /// Returns true if the [`HttpRequest`] contains the given header.
  pub fn has_header(&self, header: &HttpHeader) -> bool {
    self.headers.contains(header)
  }
}

impl HttpResponse {
  /// Returns true if the `status` matches the argument.
  pub fn has_status(&self, status: i32) -> bool {
    self.status == status
  }

  /// Returns true if the [`HttpResponse`] contains the given header.
  pub fn has_header(&self, header: &HttpHeader) -> bool {
    self.headers.contains(header)
  }

  has_impl!(reason);
}

impl Code {
  /// Checks if the code is `Ok`.
  pub fn is_200_ok(&self) -> bool {
    matches!(self, Self::Ok)
  }

  /// Checks if the code is `Cancelled`.
  pub fn is_cancelled(&self) -> bool {
    matches!(self, Self::Cancelled)
  }

  /// Checks if the code is `Unknown`.
  pub fn is_unknown(&self) -> bool {
    matches!(self, Self::Unknown)
  }

  /// Checks if the code is `InvalidArgument`.
  pub fn is_invalid_argument(&self) -> bool {
    matches!(self, Self::InvalidArgument)
  }

  /// Checks if the code is `DeadlineExceeded`.
  pub fn is_deadline_exceeded(&self) -> bool {
    matches!(self, Self::DeadlineExceeded)
  }

  /// Checks if the code is `NotFound`.
  pub fn is_not_found(&self) -> bool {
    matches!(self, Self::NotFound)
  }

  /// Checks if the code is `AlreadyExists`.
  pub fn is_already_exists(&self) -> bool {
    matches!(self, Self::AlreadyExists)
  }

  /// Checks if the code is `PermissionDenied`.
  pub fn is_permission_denied(&self) -> bool {
    matches!(self, Self::PermissionDenied)
  }

  /// Checks if the code is `Unauthenticated`.
  pub fn is_unauthenticated(&self) -> bool {
    matches!(self, Self::Unauthenticated)
  }

  /// Checks if the code is `ResourceExhausted`.
  pub fn is_resource_exhausted(&self) -> bool {
    matches!(self, Self::ResourceExhausted)
  }

  /// Checks if the code is `FailedPrecondition`.
  pub fn is_failed_precondition(&self) -> bool {
    matches!(self, Self::FailedPrecondition)
  }

  /// Checks if the code is `Aborted`.
  pub fn is_aborted(&self) -> bool {
    matches!(self, Self::Aborted)
  }

  /// Checks if the code is `OutOfRange`.
  pub fn is_out_of_range(&self) -> bool {
    matches!(self, Self::OutOfRange)
  }

  /// Checks if the code is `Unimplemented`.
  pub fn is_unimplemented(&self) -> bool {
    matches!(self, Self::Unimplemented)
  }

  /// Checks if the code is `Internal`.
  pub fn is_internal(&self) -> bool {
    matches!(self, Self::Internal)
  }

  /// Checks if the code is `Unavailable`.
  pub fn is_unavailable(&self) -> bool {
    matches!(self, Self::Unavailable)
  }

  /// Checks if the code is `DataLoss`.
  pub fn is_data_loss(&self) -> bool {
    matches!(self, Self::DataLoss)
  }

  /// Returns the name of the code variant in title case.
  pub fn as_title_case(&self) -> &str {
    match self {
      Code::Ok => "Ok",
      Code::Cancelled => "Cancelled",
      Code::Unknown => "Unknown",
      Code::InvalidArgument => "Invalid Argument",
      Code::DeadlineExceeded => "Deadline Exceeded",
      Code::NotFound => "Not Found",
      Code::AlreadyExists => "Already Exists",
      Code::PermissionDenied => "Permission Denied",
      Code::Unauthenticated => "Unauthenticated",
      Code::ResourceExhausted => "Resource Exhausted",
      Code::FailedPrecondition => "Failed Precondition",
      Code::Aborted => "Aborted",
      Code::OutOfRange => "Out Of Range",
      Code::Unimplemented => "Unimplemented",
      Code::Internal => "Internal",
      Code::Unavailable => "Unavailable",
      Code::DataLoss => "Data Loss",
    }
  }

  /// Returns the corresponding HTTP status code mapping.
  pub fn to_http_status(&self) -> u16 {
    match self {
      Code::Ok => 200,
      Code::Cancelled => 499,
      Code::Unknown => 500,
      Code::InvalidArgument => 400,
      Code::DeadlineExceeded => 504,
      Code::NotFound => 404,
      Code::AlreadyExists => 409,
      Code::PermissionDenied => 403,
      Code::Unauthenticated => 401,
      Code::ResourceExhausted => 429,
      Code::FailedPrecondition => 400,
      Code::Aborted => 409,
      Code::OutOfRange => 400,
      Code::Unimplemented => 501,
      Code::Internal => 500,
      Code::Unavailable => 503,
      Code::DataLoss => 500,
    }
  }
}

impl Display for Code {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.as_title_case())
  }
}

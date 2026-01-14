use core::fmt::Display;

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
  pub const fn has_status(&self, status: i32) -> bool {
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
  #[must_use]
  pub const fn is_200_ok(&self) -> bool {
    matches!(self, Self::Ok)
  }

  /// Checks if the code is `Cancelled`.
  #[must_use]
  pub const fn is_cancelled(&self) -> bool {
    matches!(self, Self::Cancelled)
  }

  /// Checks if the code is `Unknown`.
  #[must_use]
  pub const fn is_unknown(&self) -> bool {
    matches!(self, Self::Unknown)
  }

  /// Checks if the code is `InvalidArgument`.
  #[must_use]
  pub const fn is_invalid_argument(&self) -> bool {
    matches!(self, Self::InvalidArgument)
  }

  /// Checks if the code is `DeadlineExceeded`.
  #[must_use]
  pub const fn is_deadline_exceeded(&self) -> bool {
    matches!(self, Self::DeadlineExceeded)
  }

  /// Checks if the code is `NotFound`.
  #[must_use]
  pub const fn is_not_found(&self) -> bool {
    matches!(self, Self::NotFound)
  }

  /// Checks if the code is `AlreadyExists`.
  #[must_use]
  pub const fn is_already_exists(&self) -> bool {
    matches!(self, Self::AlreadyExists)
  }

  /// Checks if the code is `PermissionDenied`.
  #[must_use]
  pub const fn is_permission_denied(&self) -> bool {
    matches!(self, Self::PermissionDenied)
  }

  /// Checks if the code is `Unauthenticated`.
  #[must_use]
  pub const fn is_unauthenticated(&self) -> bool {
    matches!(self, Self::Unauthenticated)
  }

  /// Checks if the code is `ResourceExhausted`.
  #[must_use]
  pub const fn is_resource_exhausted(&self) -> bool {
    matches!(self, Self::ResourceExhausted)
  }

  /// Checks if the code is `FailedPrecondition`.
  #[must_use]
  pub const fn is_failed_precondition(&self) -> bool {
    matches!(self, Self::FailedPrecondition)
  }

  /// Checks if the code is `Aborted`.
  #[must_use]
  pub const fn is_aborted(&self) -> bool {
    matches!(self, Self::Aborted)
  }

  /// Checks if the code is `OutOfRange`.
  #[must_use]
  pub const fn is_out_of_range(&self) -> bool {
    matches!(self, Self::OutOfRange)
  }

  /// Checks if the code is `Unimplemented`.
  #[must_use]
  pub const fn is_unimplemented(&self) -> bool {
    matches!(self, Self::Unimplemented)
  }

  /// Checks if the code is `Internal`.
  #[must_use]
  pub const fn is_internal(&self) -> bool {
    matches!(self, Self::Internal)
  }

  /// Checks if the code is `Unavailable`.
  #[must_use]
  pub const fn is_unavailable(&self) -> bool {
    matches!(self, Self::Unavailable)
  }

  /// Checks if the code is `DataLoss`.
  #[must_use]
  pub const fn is_data_loss(&self) -> bool {
    matches!(self, Self::DataLoss)
  }

  /// Returns the name of the code variant in title case.
  #[must_use]
  pub const fn as_title_case(&self) -> &str {
    match self {
      Self::Ok => "Ok",
      Self::Cancelled => "Cancelled",
      Self::Unknown => "Unknown",
      Self::InvalidArgument => "Invalid Argument",
      Self::DeadlineExceeded => "Deadline Exceeded",
      Self::NotFound => "Not Found",
      Self::AlreadyExists => "Already Exists",
      Self::PermissionDenied => "Permission Denied",
      Self::Unauthenticated => "Unauthenticated",
      Self::ResourceExhausted => "Resource Exhausted",
      Self::FailedPrecondition => "Failed Precondition",
      Self::Aborted => "Aborted",
      Self::OutOfRange => "Out Of Range",
      Self::Unimplemented => "Unimplemented",
      Self::Internal => "Internal",
      Self::Unavailable => "Unavailable",
      Self::DataLoss => "Data Loss",
    }
  }

  /// Returns the corresponding HTTP status code mapping.
  #[must_use]
  pub const fn to_http_status(&self) -> u16 {
    match self {
      Self::Ok => 200,
      Self::Cancelled => 499,
      Self::Unknown | Self::Internal | Self::DataLoss => 500,
      Self::InvalidArgument | Self::FailedPrecondition | Self::OutOfRange => 400,
      Self::DeadlineExceeded => 504,
      Self::NotFound => 404,
      Self::AlreadyExists | Self::Aborted => 409,
      Self::PermissionDenied => 403,
      Self::Unauthenticated => 401,
      Self::ResourceExhausted => 429,
      Self::Unimplemented => 501,
      Self::Unavailable => 503,
    }
  }
}

impl Display for Code {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    write!(f, "{}", self.as_title_case())
  }
}

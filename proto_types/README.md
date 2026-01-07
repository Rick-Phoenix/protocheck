# 🧩 Implementations for common protobuf types

This crate provides implementations for several well known protobuf types, such as those from `google.protobuf`, `google.rpc` and `google.type`, plus some implementations for `buf.validate` which are used by [protocheck](https://github.com/Rick-Phoenix/protocheck). 

You can use feature flags to selectively include the implementations that you're interested in.

# 🗒️ List of implementations

Each struct/enum includes the default impls from [prost](https://crates.io/crates/prost) such as Clone, Debug and so on.

The following features can also enable a variety of automatically derived implementations:
- [`cel`] (`TryInto` [`cel::Value`](https://docs.rs/cel/0.11.0/cel/objects/enum.Value.html))
- [`serde`] (`Serialize`, `Deserialize`)
- [`diesel`] (`FromSql`, `ToSql`, `FromSqlRow`, `QueryId`, `AsExpression`)
    - **The diesel backend must be specified in the feature (i.e. diesel-postgres, diesel-sqlite, diesel-mysql)**

In addition, the types from the google packages come with the implementations listed below, plus a variety of utility methods and helpers that can be found in the documentation.

All of the structs that have fallible methods also come with their own custom error enum.

## 📑 google.protobuf

- [`Duration`]
    - Ord, PartialOrd
    - Display
    - Add, Sub (between durations)
    - Mul, Div (with integers)
    - Conversions from [`Interval`] and to/from [`chrono::TimeDelta`] 
    - Helpers for the extraction of units (seconds, minutes, etc) and formatting
- [`Timestamp`]
    - Ord, PartialOrd
    - Display
    - Add, Sub (with Duration)
    - Custom formatting with chrono syntax (requires the `chrono` feature)
    - Conversions to/from [`std::time::SystemTime`] and [`chrono::DateTime`]
    - ToTokens (optional)
- [`FieldMask`]
    - Display
- [`Empty`]
- [`Any`]

## 📚 google.type

All of these can be included with the `all_common` flag, or selected individually with their respective flag.

- [`TimeOfDay`]
    - Constructor with validation check
    - Display
    - Ord, PartialOrd
    - Conversions to/from [`chrono::NaiveTime`]
- [`Date`] 
    - Constructor with validation check
    - Display
    - PartialOrd
    - Conversions to/from [`chrono::NaiveDate`]
- [`DateTime`] 
    - Constructor with validation check
    - Display
    - PartialOrd
    - Conversions to/from [`chrono::NaiveDateTime`], [`chrono::DateTime`] with [`chrono::FixedOffset`] or [`chrono::Utc`] (and [`chrono::DateTime`] with [`chrono_tz::Tz`] with the `chrono-tz` feature)
- [`TimeZone`]
    - Display
- [`Interval`]
    - Constructor with validation check
    - PartialOrd
    - Conversions to/from [`Duration`]
- [`Money`]
    - Constructor with validation check
    - PartialOrd
    - Formatting helper
    - Checked operations
    - (Lossy) conversions to/from f64
- [`Color`] 
    - Constructor with validation check
    - Conversions to/from [`palette::Srgba`] (with the `palette` feature)
- [`Fraction`]
    - Constructor with validation check (for 0 denominator case)
    - Display
    - PartialOrd
    - Checked operations
    - Conversion to f64
- [`Decimal`]
    - Conversion to [`rust_decimal::Decimal`]
- [`LatLng`]
    - Constructor with validation check
    - Display
- [`PostalAddress`]
    - Validator (checks presence of region_code)
- [`PhoneNumber`]
    - Constructor with validation check (prevents the `kind` field from being `None`)
    - Validator (checks presence of required `kind` field)
- [`Quaternion`]
- [`LocalizedText`]
- [`Expr`]
- [`CalendarPeriod`]
- [`Month`]
- [`DayOfWeek`]

## 💻 google.rpc

Available by default:

- [`Status`] 
- [`Code`]

Enabled with the `rpc` flag:

- [`ErrorInfo`]
- [`DebugInfo`]
- [`RetryInfo`]
- [`QuotaFailure`]
- [`quota_failure::Violation`]
- [`PreconditionFailure`]
- [`precondition_failure::Violation`]
- [`BadRequest`]
- [`bad_request::FieldViolation`]
- [`RequestInfo`]
- [`ResourceInfo`]
- [`Help`]
- [`help::Link`]
- [`LocalizedMessage`]
- [`HttpRequest`]
- [`HttpResponse`]
- [`HttpHeader`]

# 📜 License

Licensed under MPL-2.0.

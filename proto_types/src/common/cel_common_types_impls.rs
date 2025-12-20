#![allow(unused_imports)] // They appear as unused due to the features

use std::collections::HashMap;

use cel::{Value as CelValue, objects::Key as CelKey};

use crate::cel::CelConversionError;

#[cfg(feature = "timeofday")]
impl From<crate::TimeOfDay> for CelValue {
  fn from(value: crate::TimeOfDay) -> Self {
    let mut cel_map: HashMap<CelKey, Self> = HashMap::new();

    cel_map.insert("hours".into(), Self::Int(value.hours.into()));
    cel_map.insert("minutes".into(), Self::Int(value.minutes.into()));
    cel_map.insert("seconds".into(), Self::Int(value.seconds.into()));
    cel_map.insert("nanos".into(), Self::Int(value.nanos.into()));

    Self::Map(cel_map.into())
  }
}

#[cfg(feature = "localized_text")]
impl From<crate::LocalizedText> for CelValue {
  fn from(value: crate::LocalizedText) -> Self {
    let mut cel_map: HashMap<CelKey, Self> = HashMap::new();

    cel_map.insert("text".into(), Self::String(value.text.into()));
    cel_map.insert(
      "language_code".into(),
      Self::String(value.language_code.into()),
    );

    Self::Map(cel_map.into())
  }
}

#[cfg(feature = "quaternion")]
impl From<crate::Quaternion> for CelValue {
  fn from(value: crate::Quaternion) -> Self {
    let mut cel_map: HashMap<CelKey, Self> = HashMap::new();

    cel_map.insert("x".into(), Self::Float(value.x));
    cel_map.insert("y".into(), Self::Float(value.y));
    cel_map.insert("z".into(), Self::Float(value.z));
    cel_map.insert("w".into(), Self::Float(value.w));

    Self::Map(cel_map.into())
  }
}

#[cfg(feature = "money")]
impl From<crate::Money> for CelValue {
  fn from(value: crate::Money) -> Self {
    let mut cel_map: HashMap<CelKey, Self> = HashMap::new();

    cel_map.insert(
      "currency_code".into(),
      Self::String(value.currency_code.into()),
    );
    cel_map.insert("units".into(), Self::Int(value.units));
    cel_map.insert("nanos".into(), Self::Int(value.nanos.into()));

    Self::Map(cel_map.into())
  }
}

#[cfg(feature = "datetime")]
impl From<crate::TimeZone> for CelValue {
  fn from(value: crate::TimeZone) -> Self {
    let mut cel_map: HashMap<CelKey, Self> = HashMap::new();

    cel_map.insert("id".into(), Self::String(value.id.into()));
    cel_map.insert("version".into(), Self::String(value.version.into()));

    Self::Map(cel_map.into())
  }
}

#[cfg(feature = "datetime")]
impl TryFrom<crate::date_time::TimeOffset> for CelValue {
  type Error = CelConversionError;
  fn try_from(value: crate::date_time::TimeOffset) -> Result<Self, Self::Error> {
    match value {
      crate::date_time::TimeOffset::UtcOffset(duration) => duration.try_into(),
      crate::date_time::TimeOffset::TimeZone(tz) => Ok(tz.into()),
    }
  }
}

#[cfg(feature = "datetime")]
impl TryFrom<crate::DateTime> for CelValue {
  type Error = CelConversionError;
  fn try_from(value: crate::DateTime) -> Result<Self, Self::Error> {
    let mut cel_map: HashMap<CelKey, Self> = HashMap::new();

    cel_map.insert("year".into(), Self::Int(value.year.into()));
    cel_map.insert("month".into(), Self::Int(value.month.into()));
    cel_map.insert("day".into(), Self::Int(value.day.into()));
    cel_map.insert("hours".into(), Self::Int(value.hours.into()));
    cel_map.insert("minutes".into(), Self::Int(value.minutes.into()));
    cel_map.insert("seconds".into(), Self::Int(value.seconds.into()));
    cel_map.insert("nanos".into(), Self::Int(value.nanos.into()));

    let time_offset = match value.time_offset {
      Some(v) => v.try_into()?,
      None => Self::Null,
    };

    cel_map.insert("time_offset".into(), time_offset);

    Ok(Self::Map(cel_map.into()))
  }
}

#[cfg(feature = "latlng")]
impl From<crate::LatLng> for CelValue {
  fn from(value: crate::LatLng) -> Self {
    let mut cel_map: HashMap<CelKey, Self> = HashMap::new();

    cel_map.insert("latitude".into(), Self::Float(value.latitude));
    cel_map.insert("longitude".into(), Self::Float(value.longitude));

    Self::Map(cel_map.into())
  }
}

#[cfg(feature = "postal_address")]
impl From<crate::PostalAddress> for CelValue {
  fn from(value: crate::PostalAddress) -> Self {
    let mut cel_map: HashMap<CelKey, Self> = HashMap::new();

    cel_map.insert("revision".into(), Self::Int(value.revision.into()));
    cel_map.insert("region_code".into(), Self::String(value.region_code.into()));
    cel_map.insert(
      "language_code".into(),
      Self::String(value.language_code.into()),
    );
    cel_map.insert("postal_code".into(), Self::String(value.postal_code.into()));
    cel_map.insert(
      "sorting_code".into(),
      Self::String(value.sorting_code.into()),
    );
    cel_map.insert(
      "administrative_area".into(),
      Self::String(value.administrative_area.into()),
    );
    cel_map.insert("locality".into(), Self::String(value.locality.into()));
    cel_map.insert("sublocality".into(), Self::String(value.sublocality.into()));

    cel_map.insert(
      "address_lines".into(),
      Self::List(
        value
          .address_lines
          .into_iter()
          .map(|i| i.into())
          .collect::<Vec<Self>>()
          .into(),
      ),
    );

    cel_map.insert(
      "recipients".into(),
      Self::List(
        value
          .recipients
          .into_iter()
          .map(|i| i.into())
          .collect::<Vec<Self>>()
          .into(),
      ),
    );

    Self::Map(cel_map.into())
  }
}

#[cfg(feature = "date")]
impl From<crate::Date> for CelValue {
  fn from(value: crate::Date) -> Self {
    let mut cel_map: HashMap<CelKey, Self> = HashMap::new();

    cel_map.insert("year".into(), Self::Int(value.year.into()));
    cel_map.insert("month".into(), Self::Int(value.month.into()));
    cel_map.insert("day".into(), Self::Int(value.day.into()));

    Self::Map(cel_map.into())
  }
}

#[cfg(feature = "interval")]
impl TryFrom<crate::Interval> for CelValue {
  type Error = CelConversionError;
  fn try_from(value: crate::Interval) -> Result<Self, Self::Error> {
    let mut cel_map: HashMap<CelKey, Self> = HashMap::new();

    let start_time = match value.start_time {
      Some(v) => v.try_into()?,
      None => Self::Null,
    };

    cel_map.insert("start_time".into(), start_time);

    let end_time = match value.end_time {
      Some(v) => v.try_into()?,
      None => Self::Null,
    };

    cel_map.insert("end_time".into(), end_time);

    Ok(Self::Map(cel_map.into()))
  }
}

#[cfg(feature = "expr")]
impl From<crate::Expr> for CelValue {
  fn from(value: crate::Expr) -> Self {
    let mut cel_map: HashMap<CelKey, Self> = HashMap::new();

    cel_map.insert("expression".into(), Self::String(value.expression.into()));
    cel_map.insert("title".into(), Self::String(value.title.into()));
    cel_map.insert("description".into(), Self::String(value.description.into()));
    cel_map.insert("location".into(), Self::String(value.location.into()));

    Self::Map(cel_map.into())
  }
}

#[cfg(feature = "color")]
impl From<crate::Color> for CelValue {
  fn from(value: crate::Color) -> Self {
    let mut cel_map: HashMap<CelKey, Self> = HashMap::new();

    cel_map.insert("red".into(), Self::Float(f64::from(value.red)));
    cel_map.insert("green".into(), Self::Float(f64::from(value.green)));
    cel_map.insert("blue".into(), Self::Float(f64::from(value.blue)));

    let alpha = match value.alpha {
      Some(v) => Self::Float(f64::from(v.value)),
      None => Self::Null,
    };

    cel_map.insert("alpha".into(), alpha);

    Self::Map(cel_map.into())
  }
}

#[cfg(feature = "fraction")]
impl From<crate::Fraction> for CelValue {
  fn from(value: crate::Fraction) -> Self {
    let mut cel_map: HashMap<CelKey, Self> = HashMap::new();

    cel_map.insert("numerator".into(), Self::Int(value.numerator));
    cel_map.insert("denominator".into(), Self::Int(value.denominator));

    Self::Map(cel_map.into())
  }
}

#[cfg(feature = "decimal")]
impl From<crate::Decimal> for CelValue {
  fn from(value: crate::Decimal) -> Self {
    let mut cel_map: HashMap<CelKey, Self> = HashMap::new();

    cel_map.insert("value".into(), Self::String(value.value.into()));

    Self::Map(cel_map.into())
  }
}

#[cfg(feature = "phone_number")]
impl From<crate::phone_number::ShortCode> for CelValue {
  fn from(value: crate::phone_number::ShortCode) -> Self {
    let mut cel_map: HashMap<CelKey, Self> = HashMap::new();

    cel_map.insert("region_code".into(), Self::String(value.region_code.into()));
    cel_map.insert("number".into(), Self::String(value.number.into()));

    Self::Map(cel_map.into())
  }
}

#[cfg(feature = "phone_number")]
impl From<crate::phone_number::Kind> for CelValue {
  fn from(value: crate::phone_number::Kind) -> Self {
    match value {
      crate::phone_number::Kind::E164Number(v) => Self::String(v.into()),
      crate::phone_number::Kind::ShortCode(sc) => sc.into(),
    }
  }
}

#[cfg(feature = "phone_number")]
impl From<crate::PhoneNumber> for CelValue {
  fn from(value: crate::PhoneNumber) -> Self {
    let mut cel_map: HashMap<CelKey, Self> = HashMap::new();

    cel_map.insert("extension".into(), Self::String(value.extension.into()));
    cel_map.insert("kind".into(), value.kind.into());

    Self::Map(cel_map.into())
  }
}

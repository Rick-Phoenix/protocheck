#![allow(unused_imports)] // They appear as unused due to the features

use std::collections::HashMap;

use cel::{objects::Key as CelKey, Value as CelValue};

use crate::cel::CelConversionError;

#[cfg(feature = "timeofday")]
impl From<crate::TimeOfDay> for CelValue {
  fn from(value: crate::TimeOfDay) -> Self {
    let mut cel_map: HashMap<CelKey, CelValue> = HashMap::new();

    cel_map.insert("hours".into(), CelValue::Int(value.hours.into()));
    cel_map.insert("minutes".into(), CelValue::Int(value.minutes.into()));
    cel_map.insert("seconds".into(), CelValue::Int(value.seconds.into()));
    cel_map.insert("nanos".into(), CelValue::Int(value.nanos.into()));

    CelValue::Map(cel_map.into())
  }
}

#[cfg(feature = "localized_text")]
impl From<crate::LocalizedText> for CelValue {
  fn from(value: crate::LocalizedText) -> Self {
    let mut cel_map: HashMap<CelKey, CelValue> = HashMap::new();

    cel_map.insert("text".into(), CelValue::String(value.text.into()));
    cel_map.insert(
      "language_code".into(),
      CelValue::String(value.language_code.into()),
    );

    CelValue::Map(cel_map.into())
  }
}

#[cfg(feature = "quaternion")]
impl From<crate::Quaternion> for CelValue {
  fn from(value: crate::Quaternion) -> Self {
    let mut cel_map: HashMap<CelKey, CelValue> = HashMap::new();

    cel_map.insert("x".into(), CelValue::Float(value.x));
    cel_map.insert("y".into(), CelValue::Float(value.y));
    cel_map.insert("z".into(), CelValue::Float(value.z));
    cel_map.insert("w".into(), CelValue::Float(value.w));

    CelValue::Map(cel_map.into())
  }
}

#[cfg(feature = "money")]
impl From<crate::Money> for CelValue {
  fn from(value: crate::Money) -> Self {
    let mut cel_map: HashMap<CelKey, CelValue> = HashMap::new();

    cel_map.insert(
      "currency_code".into(),
      CelValue::String(value.currency_code.into()),
    );
    cel_map.insert("units".into(), CelValue::Int(value.units));
    cel_map.insert("nanos".into(), CelValue::Int(value.nanos.into()));

    CelValue::Map(cel_map.into())
  }
}

#[cfg(feature = "datetime")]
impl From<crate::TimeZone> for CelValue {
  fn from(value: crate::TimeZone) -> Self {
    let mut cel_map: HashMap<CelKey, CelValue> = HashMap::new();

    cel_map.insert("id".into(), CelValue::String(value.id.into()));
    cel_map.insert("version".into(), CelValue::String(value.version.into()));

    CelValue::Map(cel_map.into())
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
    let mut cel_map: HashMap<CelKey, CelValue> = HashMap::new();

    cel_map.insert("year".into(), CelValue::Int(value.year.into()));
    cel_map.insert("month".into(), CelValue::Int(value.month.into()));
    cel_map.insert("day".into(), CelValue::Int(value.day.into()));
    cel_map.insert("hours".into(), CelValue::Int(value.hours.into()));
    cel_map.insert("minutes".into(), CelValue::Int(value.minutes.into()));
    cel_map.insert("seconds".into(), CelValue::Int(value.seconds.into()));
    cel_map.insert("nanos".into(), CelValue::Int(value.nanos.into()));

    let time_offset = match value.time_offset {
      Some(v) => v.try_into()?,
      None => CelValue::Null,
    };

    cel_map.insert("time_offset".into(), time_offset);

    Ok(CelValue::Map(cel_map.into()))
  }
}

#[cfg(feature = "latlng")]
impl From<crate::LatLng> for CelValue {
  fn from(value: crate::LatLng) -> Self {
    let mut cel_map: HashMap<CelKey, CelValue> = HashMap::new();

    cel_map.insert("latitude".into(), CelValue::Float(value.latitude));
    cel_map.insert("longitude".into(), CelValue::Float(value.longitude));

    CelValue::Map(cel_map.into())
  }
}

#[cfg(feature = "postal_address")]
impl From<crate::PostalAddress> for CelValue {
  fn from(value: crate::PostalAddress) -> Self {
    let mut cel_map: HashMap<CelKey, CelValue> = HashMap::new();

    cel_map.insert("revision".into(), CelValue::Int(value.revision.into()));
    cel_map.insert(
      "region_code".into(),
      CelValue::String(value.region_code.into()),
    );
    cel_map.insert(
      "language_code".into(),
      CelValue::String(value.language_code.into()),
    );
    cel_map.insert(
      "postal_code".into(),
      CelValue::String(value.postal_code.into()),
    );
    cel_map.insert(
      "sorting_code".into(),
      CelValue::String(value.sorting_code.into()),
    );
    cel_map.insert(
      "administrative_area".into(),
      CelValue::String(value.administrative_area.into()),
    );
    cel_map.insert("locality".into(), CelValue::String(value.locality.into()));
    cel_map.insert(
      "sublocality".into(),
      CelValue::String(value.sublocality.into()),
    );

    cel_map.insert(
      "address_lines".into(),
      CelValue::List(
        value
          .address_lines
          .into_iter()
          .map(|i| i.into())
          .collect::<Vec<CelValue>>()
          .into(),
      ),
    );

    cel_map.insert(
      "recipients".into(),
      CelValue::List(
        value
          .recipients
          .into_iter()
          .map(|i| i.into())
          .collect::<Vec<CelValue>>()
          .into(),
      ),
    );

    CelValue::Map(cel_map.into())
  }
}

#[cfg(feature = "date")]
impl From<crate::Date> for CelValue {
  fn from(value: crate::Date) -> Self {
    let mut cel_map: HashMap<CelKey, CelValue> = HashMap::new();

    cel_map.insert("year".into(), CelValue::Int(value.year.into()));
    cel_map.insert("month".into(), CelValue::Int(value.month.into()));
    cel_map.insert("day".into(), CelValue::Int(value.day.into()));

    CelValue::Map(cel_map.into())
  }
}

#[cfg(feature = "interval")]
impl TryFrom<crate::Interval> for CelValue {
  type Error = CelConversionError;
  fn try_from(value: crate::Interval) -> Result<Self, Self::Error> {
    let mut cel_map: HashMap<CelKey, CelValue> = HashMap::new();

    let start_time = match value.start_time {
      Some(v) => v.try_into()?,
      None => CelValue::Null,
    };

    cel_map.insert("start_time".into(), start_time);

    let end_time = match value.end_time {
      Some(v) => v.try_into()?,
      None => CelValue::Null,
    };

    cel_map.insert("end_time".into(), end_time);

    Ok(CelValue::Map(cel_map.into()))
  }
}

#[cfg(feature = "expr")]
impl From<crate::Expr> for CelValue {
  fn from(value: crate::Expr) -> Self {
    let mut cel_map: HashMap<CelKey, CelValue> = HashMap::new();

    cel_map.insert(
      "expression".into(),
      CelValue::String(value.expression.into()),
    );
    cel_map.insert("title".into(), CelValue::String(value.title.into()));
    cel_map.insert(
      "description".into(),
      CelValue::String(value.description.into()),
    );
    cel_map.insert("location".into(), CelValue::String(value.location.into()));

    CelValue::Map(cel_map.into())
  }
}

#[cfg(feature = "color")]
impl From<crate::Color> for CelValue {
  fn from(value: crate::Color) -> Self {
    let mut cel_map: HashMap<CelKey, CelValue> = HashMap::new();

    cel_map.insert("red".into(), CelValue::Float(value.red as f64));
    cel_map.insert("green".into(), CelValue::Float(value.green as f64));
    cel_map.insert("blue".into(), CelValue::Float(value.blue as f64));

    let alpha = match value.alpha {
      Some(v) => CelValue::Float(v.value as f64),
      None => CelValue::Null,
    };

    cel_map.insert("alpha".into(), alpha);

    CelValue::Map(cel_map.into())
  }
}

#[cfg(feature = "fraction")]
impl From<crate::Fraction> for CelValue {
  fn from(value: crate::Fraction) -> Self {
    let mut cel_map: HashMap<CelKey, CelValue> = HashMap::new();

    cel_map.insert("numerator".into(), CelValue::Int(value.numerator));
    cel_map.insert("denominator".into(), CelValue::Int(value.denominator));

    CelValue::Map(cel_map.into())
  }
}

#[cfg(feature = "decimal")]
impl From<crate::Decimal> for CelValue {
  fn from(value: crate::Decimal) -> Self {
    let mut cel_map: HashMap<CelKey, CelValue> = HashMap::new();

    cel_map.insert("value".into(), CelValue::String(value.value.into()));

    CelValue::Map(cel_map.into())
  }
}

#[cfg(feature = "phone_number")]
impl From<crate::phone_number::ShortCode> for CelValue {
  fn from(value: crate::phone_number::ShortCode) -> Self {
    let mut cel_map: HashMap<CelKey, CelValue> = HashMap::new();

    cel_map.insert(
      "region_code".into(),
      CelValue::String(value.region_code.into()),
    );
    cel_map.insert("number".into(), CelValue::String(value.number.into()));

    CelValue::Map(cel_map.into())
  }
}

#[cfg(feature = "phone_number")]
impl From<crate::phone_number::Kind> for CelValue {
  fn from(value: crate::phone_number::Kind) -> Self {
    match value {
      crate::phone_number::Kind::E164Number(v) => CelValue::String(v.into()),
      crate::phone_number::Kind::ShortCode(sc) => sc.into(),
    }
  }
}

#[cfg(feature = "phone_number")]
impl From<crate::PhoneNumber> for CelValue {
  fn from(value: crate::PhoneNumber) -> Self {
    let mut cel_map: HashMap<CelKey, CelValue> = HashMap::new();

    cel_map.insert("extension".into(), CelValue::String(value.extension.into()));
    cel_map.insert("kind".into(), value.kind.into());

    CelValue::Map(cel_map.into())
  }
}

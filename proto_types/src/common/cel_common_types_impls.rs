use std::collections::HashMap;

use cel::{objects::Key as CelKey, Value as CelValue};

use crate::{
  cel::CelConversionError,
  common::{
    date_time::TimeOffset,
    phone_number::{self, ShortCode},
    Color, Date, DateTime, Decimal, Expr, Fraction, Interval, LatLng, LocalizedText, Money,
    PhoneNumber, PostalAddress, Quaternion, TimeOfDay, TimeZone,
  },
};

impl From<TimeOfDay> for CelValue {
  fn from(value: TimeOfDay) -> Self {
    let mut cel_map: HashMap<CelKey, CelValue> = HashMap::new();

    cel_map.insert("hours".into(), CelValue::Int(value.hours.into()));
    cel_map.insert("minutes".into(), CelValue::Int(value.minutes.into()));
    cel_map.insert("seconds".into(), CelValue::Int(value.seconds.into()));
    cel_map.insert("nanos".into(), CelValue::Int(value.nanos.into()));

    CelValue::Map(cel_map.into())
  }
}

impl From<LocalizedText> for CelValue {
  fn from(value: LocalizedText) -> Self {
    let mut cel_map: HashMap<CelKey, CelValue> = HashMap::new();

    cel_map.insert("text".into(), CelValue::String(value.text.into()));
    cel_map.insert(
      "language_code".into(),
      CelValue::String(value.language_code.into()),
    );

    CelValue::Map(cel_map.into())
  }
}

impl From<Quaternion> for CelValue {
  fn from(value: Quaternion) -> Self {
    let mut cel_map: HashMap<CelKey, CelValue> = HashMap::new();

    cel_map.insert("x".into(), CelValue::Float(value.x));
    cel_map.insert("y".into(), CelValue::Float(value.y));
    cel_map.insert("z".into(), CelValue::Float(value.z));
    cel_map.insert("w".into(), CelValue::Float(value.w));

    CelValue::Map(cel_map.into())
  }
}

impl From<Money> for CelValue {
  fn from(value: Money) -> Self {
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

impl From<TimeZone> for CelValue {
  fn from(value: TimeZone) -> Self {
    let mut cel_map: HashMap<CelKey, CelValue> = HashMap::new();

    cel_map.insert("id".into(), CelValue::String(value.id.into()));
    cel_map.insert("version".into(), CelValue::String(value.version.into()));

    CelValue::Map(cel_map.into())
  }
}

impl TryFrom<TimeOffset> for CelValue {
  type Error = CelConversionError;
  fn try_from(value: TimeOffset) -> Result<Self, Self::Error> {
    match value {
      TimeOffset::UtcOffset(duration) => duration.try_into(),
      TimeOffset::TimeZone(tz) => Ok(tz.into()),
    }
  }
}

impl TryFrom<DateTime> for CelValue {
  type Error = CelConversionError;
  fn try_from(value: DateTime) -> Result<Self, Self::Error> {
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

impl From<LatLng> for CelValue {
  fn from(value: LatLng) -> Self {
    let mut cel_map: HashMap<CelKey, CelValue> = HashMap::new();

    cel_map.insert("latitude".into(), CelValue::Float(value.latitude));
    cel_map.insert("longitude".into(), CelValue::Float(value.longitude));

    CelValue::Map(cel_map.into())
  }
}

impl From<PostalAddress> for CelValue {
  fn from(value: PostalAddress) -> Self {
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

impl From<Date> for CelValue {
  fn from(value: Date) -> Self {
    let mut cel_map: HashMap<CelKey, CelValue> = HashMap::new();

    cel_map.insert("year".into(), CelValue::Int(value.year.into()));
    cel_map.insert("month".into(), CelValue::Int(value.month.into()));
    cel_map.insert("day".into(), CelValue::Int(value.day.into()));

    CelValue::Map(cel_map.into())
  }
}

impl TryFrom<Interval> for CelValue {
  type Error = CelConversionError;
  fn try_from(value: Interval) -> Result<Self, Self::Error> {
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

impl From<Expr> for CelValue {
  fn from(value: Expr) -> Self {
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

impl From<Color> for CelValue {
  fn from(value: Color) -> Self {
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

impl From<Fraction> for CelValue {
  fn from(value: Fraction) -> Self {
    let mut cel_map: HashMap<CelKey, CelValue> = HashMap::new();

    cel_map.insert("numerator".into(), CelValue::Int(value.numerator));
    cel_map.insert("denominator".into(), CelValue::Int(value.denominator));

    CelValue::Map(cel_map.into())
  }
}

impl From<Decimal> for CelValue {
  fn from(value: Decimal) -> Self {
    let mut cel_map: HashMap<CelKey, CelValue> = HashMap::new();

    cel_map.insert("value".into(), CelValue::String(value.value.into()));

    CelValue::Map(cel_map.into())
  }
}

impl From<ShortCode> for CelValue {
  fn from(value: ShortCode) -> Self {
    let mut cel_map: HashMap<CelKey, CelValue> = HashMap::new();

    cel_map.insert(
      "region_code".into(),
      CelValue::String(value.region_code.into()),
    );
    cel_map.insert("number".into(), CelValue::String(value.number.into()));

    CelValue::Map(cel_map.into())
  }
}

impl From<phone_number::Kind> for CelValue {
  fn from(value: phone_number::Kind) -> Self {
    match value {
      phone_number::Kind::E164Number(v) => CelValue::String(v.into()),
      phone_number::Kind::ShortCode(sc) => sc.into(),
    }
  }
}

impl From<PhoneNumber> for CelValue {
  fn from(value: PhoneNumber) -> Self {
    let mut cel_map: HashMap<CelKey, CelValue> = HashMap::new();

    cel_map.insert("extension".into(), CelValue::String(value.extension.into()));
    cel_map.insert("kind".into(), value.kind.into());

    CelValue::Map(cel_map.into())
  }
}

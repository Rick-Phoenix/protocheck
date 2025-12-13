use chrono::{NaiveDate, NaiveDateTime, NaiveTime, TimeDelta};
use diesel::{
  deserialize::{FromSql, Result as DeserializeResult},
  serialize::{IsNull, Output, Result as SerializeResult, ToSql},
  sql_types::{Date as SqlDate, Interval, Time, Timestamp as SqlTimestamp},
};

use crate::{Date, DateTime, Duration, TimeOfDay, Timestamp};

#[cfg(feature = "diesel-postgres")]
mod diesel_postgres {
  use diesel::{
    pg::{Pg, PgValue},
    sql_types::Timestamptz,
  };

  use super::*;

  impl FromSql<Interval, Pg> for Duration {
    fn from_sql(bytes: PgValue<'_>) -> DeserializeResult<Self> {
      let chrono_duration: TimeDelta = FromSql::<Interval, Pg>::from_sql(bytes)?;
      Ok(chrono_duration.into())
    }
  }

  impl FromSql<Time, Pg> for TimeOfDay {
    fn from_sql(bytes: PgValue<'_>) -> DeserializeResult<Self> {
      let chrono_time: NaiveTime = FromSql::<Time, Pg>::from_sql(bytes)?;
      Ok(chrono_time.into())
    }
  }

  impl FromSql<SqlTimestamp, Pg> for Timestamp {
    fn from_sql(bytes: PgValue<'_>) -> DeserializeResult<Self> {
      let chrono_datetime: NaiveDateTime = FromSql::<SqlTimestamp, Pg>::from_sql(bytes)?;
      Ok(chrono_datetime.into())
    }
  }

  impl FromSql<Timestamptz, Pg> for Timestamp {
    fn from_sql(bytes: PgValue<'_>) -> DeserializeResult<Self> {
      let chrono_datetime: NaiveDateTime = FromSql::<Timestamptz, Pg>::from_sql(bytes)?;
      Ok(chrono_datetime.into())
    }
  }

  impl FromSql<SqlTimestamp, Pg> for DateTime {
    fn from_sql(bytes: PgValue<'_>) -> DeserializeResult<Self> {
      let chrono_datetime: NaiveDateTime = FromSql::<SqlTimestamp, Pg>::from_sql(bytes)?;
      Ok(chrono_datetime.into())
    }
  }

  impl FromSql<Timestamptz, Pg> for DateTime {
    fn from_sql(bytes: PgValue<'_>) -> DeserializeResult<Self> {
      let chrono_datetime: NaiveDateTime = FromSql::<Timestamptz, Pg>::from_sql(bytes)?;
      Ok(chrono_datetime.into())
    }
  }

  impl FromSql<SqlDate, Pg> for Date {
    fn from_sql(bytes: PgValue<'_>) -> DeserializeResult<Self> {
      let chrono_date: NaiveDate = FromSql::<SqlDate, Pg>::from_sql(bytes)?;
      Ok(chrono_date.into())
    }
  }

  impl ToSql<Interval, Pg> for Duration {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> SerializeResult {
      let chrono_duration: TimeDelta = (*self).try_into()?;

      ToSql::<Interval, Pg>::to_sql(&chrono_duration, &mut out.reborrow())
    }
  }

  impl ToSql<Time, Pg> for TimeOfDay {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> SerializeResult {
      let chrono_time: NaiveTime = (*self).try_into()?;

      ToSql::<Time, Pg>::to_sql(&chrono_time, &mut out.reborrow())
    }
  }

  impl ToSql<SqlTimestamp, Pg> for Timestamp {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> SerializeResult {
      let chrono_datetime: NaiveDateTime = (*self).try_into()?;

      ToSql::<SqlTimestamp, Pg>::to_sql(&chrono_datetime, &mut out.reborrow())
    }
  }

  impl ToSql<Timestamptz, Pg> for Timestamp {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> SerializeResult {
      ToSql::<SqlTimestamp, Pg>::to_sql(self, out)
    }
  }

  impl ToSql<SqlTimestamp, Pg> for DateTime {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> SerializeResult {
      let chrono_datetime: NaiveDateTime = self.clone().try_into()?;

      ToSql::<SqlTimestamp, Pg>::to_sql(&chrono_datetime, &mut out.reborrow())
    }
  }

  impl ToSql<Timestamptz, Pg> for DateTime {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> SerializeResult {
      ToSql::<SqlTimestamp, Pg>::to_sql(self, out)
    }
  }

  impl ToSql<SqlDate, Pg> for Date {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> SerializeResult {
      let chrono_date: NaiveDate = (*self).try_into()?;

      ToSql::<SqlDate, Pg>::to_sql(&chrono_date, &mut out.reborrow())
    }
  }
}

#[cfg(feature = "diesel-sqlite")]
mod diesel_sqlite {
  use diesel::{backend::Backend, sql_types::TimestamptzSqlite, sqlite::Sqlite};

  use super::*;

  const ENCODE_NAIVE_DATETIME_FORMAT: &str = "%F %T%.f";
  const DATE_FORMAT: &str = "%F";
  const ENCODE_TIME_FORMAT: &str = "%T%.f";

  fn format_naive_datetime(value: NaiveDateTime) -> String {
    value
      .format(ENCODE_NAIVE_DATETIME_FORMAT)
      .to_string()
  }

  impl FromSql<Time, Sqlite> for TimeOfDay {
    fn from_sql(value: <Sqlite as Backend>::RawValue<'_>) -> DeserializeResult<Self> {
      let chrono_time: NaiveTime = FromSql::<Time, Sqlite>::from_sql(value)?;
      Ok(chrono_time.into())
    }
  }

  impl FromSql<SqlTimestamp, Sqlite> for Timestamp {
    fn from_sql(value: <Sqlite as Backend>::RawValue<'_>) -> DeserializeResult<Self> {
      let chrono_datetime: NaiveDateTime = FromSql::<SqlTimestamp, Sqlite>::from_sql(value)?;
      Ok(chrono_datetime.into())
    }
  }

  impl FromSql<TimestamptzSqlite, Sqlite> for Timestamp {
    fn from_sql(value: <Sqlite as Backend>::RawValue<'_>) -> DeserializeResult<Self> {
      let chrono_datetime: NaiveDateTime = FromSql::<TimestamptzSqlite, Sqlite>::from_sql(value)?;
      Ok(chrono_datetime.into())
    }
  }

  impl FromSql<SqlTimestamp, Sqlite> for DateTime {
    fn from_sql(value: <Sqlite as Backend>::RawValue<'_>) -> DeserializeResult<Self> {
      let chrono_datetime: NaiveDateTime = FromSql::<SqlTimestamp, Sqlite>::from_sql(value)?;
      Ok(chrono_datetime.into())
    }
  }

  impl FromSql<TimestamptzSqlite, Sqlite> for DateTime {
    fn from_sql(value: <Sqlite as Backend>::RawValue<'_>) -> DeserializeResult<Self> {
      let chrono_datetime: NaiveDateTime = FromSql::<TimestamptzSqlite, Sqlite>::from_sql(value)?;
      Ok(chrono_datetime.into())
    }
  }

  impl FromSql<SqlDate, Sqlite> for Date {
    fn from_sql(value: <Sqlite as Backend>::RawValue<'_>) -> DeserializeResult<Self> {
      let chrono_date: NaiveDate = FromSql::<SqlDate, Sqlite>::from_sql(value)?;
      Ok(chrono_date.into())
    }
  }

  impl ToSql<Time, Sqlite> for TimeOfDay {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> SerializeResult {
      let chrono_time: NaiveTime = (*self).try_into()?;

      out.set_value(chrono_time.format(ENCODE_TIME_FORMAT).to_string());
      Ok(IsNull::No)
    }
  }

  impl ToSql<TimestamptzSqlite, Sqlite> for Timestamp {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> SerializeResult {
      let chrono_datetime: NaiveDateTime = (*self).try_into()?;

      out.set_value(format_naive_datetime(chrono_datetime));
      Ok(IsNull::No)
    }
  }

  impl ToSql<SqlTimestamp, Sqlite> for Timestamp {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> SerializeResult {
      let chrono_datetime: NaiveDateTime = (*self).try_into()?;

      out.set_value(format_naive_datetime(chrono_datetime));
      Ok(IsNull::No)
    }
  }

  impl ToSql<TimestamptzSqlite, Sqlite> for DateTime {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> SerializeResult {
      let chrono_datetime: NaiveDateTime = self.clone().try_into()?;

      out.set_value(format_naive_datetime(chrono_datetime));
      Ok(IsNull::No)
    }
  }

  impl ToSql<SqlTimestamp, Sqlite> for DateTime {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> SerializeResult {
      let chrono_datetime: NaiveDateTime = self.clone().try_into()?;

      out.set_value(format_naive_datetime(chrono_datetime));
      Ok(IsNull::No)
    }
  }

  impl ToSql<SqlDate, Sqlite> for Date {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> SerializeResult {
      let chrono_date: NaiveDate = (*self).try_into()?;

      out.set_value(chrono_date.format(DATE_FORMAT).to_string());
      Ok(IsNull::No)
    }
  }
}

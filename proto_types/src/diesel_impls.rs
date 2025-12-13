use chrono::NaiveDateTime;
use diesel::{
  deserialize::{FromSql, Result as DeserializeResult},
  serialize::{IsNull, Output, Result as SerializeResult, ToSql},
  sql_types::Timestamp as SqlTimestamp,
};

use crate::Timestamp;

const ENCODE_NAIVE_DATETIME_FORMAT: &str = "%F %T%.f";

#[cfg(feature = "diesel-postgres")]
mod diesel_postgres {
  use diesel::{
    pg::{Pg, PgValue},
    sql_types::Timestamptz,
  };

  use super::*;

  impl FromSql<SqlTimestamp, Pg> for Timestamp {
    fn from_sql(bytes: PgValue<'_>) -> DeserializeResult<Self> {
      let chrono_datetime: NaiveDateTime = FromSql::<SqlTimestamp, Pg>::from_sql(bytes)?;
      Ok(chrono_datetime.into())
    }
  }

  impl ToSql<SqlTimestamp, Pg> for Timestamp {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> SerializeResult {
      let chrono_datetime: NaiveDateTime = (*self).try_into()?;

      ToSql::<SqlTimestamp, Pg>::to_sql(&chrono_datetime, &mut out.reborrow())
    }
  }

  impl FromSql<Timestamptz, Pg> for Timestamp {
    fn from_sql(bytes: PgValue<'_>) -> DeserializeResult<Self> {
      let chrono_datetime: NaiveDateTime = FromSql::<Timestamptz, Pg>::from_sql(bytes)?;
      Ok(chrono_datetime.into())
    }
  }

  impl ToSql<Timestamptz, Pg> for Timestamp {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> SerializeResult {
      ToSql::<SqlTimestamp, Pg>::to_sql(self, out)
    }
  }
}

#[cfg(feature = "diesel-sqlite")]
mod diesel_sqlite {
  use diesel::{backend::Backend, sql_types::TimestamptzSqlite, sqlite::Sqlite};

  use super::*;

  impl FromSql<SqlTimestamp, Sqlite> for Timestamp {
    fn from_sql(value: <Sqlite as Backend>::RawValue<'_>) -> DeserializeResult<Self> {
      let chrono_datetime: NaiveDateTime = FromSql::<SqlTimestamp, Sqlite>::from_sql(value)?;
      Ok(chrono_datetime.into())
    }
  }

  impl ToSql<SqlTimestamp, Sqlite> for Timestamp {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> SerializeResult {
      let chrono_datetime: NaiveDateTime = (*self).try_into()?;

      out.set_value(
        chrono_datetime
          .format(ENCODE_NAIVE_DATETIME_FORMAT)
          .to_string(),
      );
      Ok(IsNull::No)
    }
  }

  impl FromSql<TimestamptzSqlite, Sqlite> for Timestamp {
    fn from_sql(value: <Sqlite as Backend>::RawValue<'_>) -> DeserializeResult<Self> {
      let chrono_datetime: NaiveDateTime = FromSql::<TimestamptzSqlite, Sqlite>::from_sql(value)?;
      Ok(chrono_datetime.into())
    }
  }

  impl ToSql<TimestamptzSqlite, Sqlite> for Timestamp {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> SerializeResult {
      let chrono_datetime: NaiveDateTime = (*self).try_into()?;

      out.set_value(
        chrono_datetime
          .format(ENCODE_NAIVE_DATETIME_FORMAT)
          .to_string(),
      );
      Ok(IsNull::No)
    }
  }
}

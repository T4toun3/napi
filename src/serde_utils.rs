
use serde::de::{Error, Unexpected, Visitor};
use serde::Deserializer;
use chrono::naive::NaiveDateTime;

use std::fmt;

use crate::search::SearchEntry;

pub const fn empty_vec() -> Vec<SearchEntry> {
    Vec::new()
}

pub fn string_to_u32<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
    D: Deserializer<'de>,
{
    struct StringToU32;

    impl<'de> Visitor<'de> for StringToU32 {
        type Value = u32;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a string of integer")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: Error,
        {
            match v.parse::<u32>() {
                Ok(value) => Ok(value),
                Err(_) => Err(Error::invalid_value(Unexpected::Str(&v), &self)),
            }
        }
    }

    let visitor = StringToU32;
    deserializer.deserialize_string(visitor)
}

pub fn unix_to_date<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    struct UnixToDate;

    impl<'de> Visitor<'de> for UnixToDate {
        type Value = NaiveDateTime;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a unsigned interger")
        }

        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(NaiveDateTime::from_timestamp(v as i64, 0))
        }

    }

    let visitor = UnixToDate;
    deserializer.deserialize_u64(visitor)
}
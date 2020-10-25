//! Utilities for transforming data and other misc

#[macro_use]
extern crate lazy_static;

pub use chrono;
pub use reqwest;
pub use serde;
use serde::Serialize;
use std::fmt::Debug;

/// Result type for lta-rs
pub type LTAResult<T> = reqwest::Result<T>;
/// Error type for lta-rs
pub type LTAError = reqwest::Error;

/// Regex patterns
pub mod regex {
    use regex::Regex;

    lazy_static! {
        pub static ref BUS_FREQ_RE: Regex =
            Regex::new(r"^(\d{1,3})?-?(\d{1,3})?$").unwrap();

        pub static ref CARPARK_COORDS_RE: Regex =
            Regex::new(r"^([+-]?([0-9]*[.])?[0-9]+) ([+-]?([0-9]*[.])?[0-9]+)$").unwrap();

        pub static ref SPEED_BAND_RE: Regex =
            Regex::new(r"^([+-]?([0-9]*[.])?[0-9]+) ([+-]?([0-9]*[.])?[0-9]+) ([+-]?([0-9]*[.])?[0-9]+) ([+-]?([0-9]*[.])?[0-9]+)$")
                .unwrap();
    }
}

/// Utils for date types
pub mod serde_date {
    pub mod ymd_hms_option {
        use chrono::{DateTime, TimeZone, Utc};
        use serde::{Deserialize, Deserializer, Serializer};

        const FORMAT: &str = "%Y-%m-%d %H:%M:%S";

        pub fn serialize<S>(date: &Option<DateTime<Utc>>, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match date {
                Some(time) => {
                    let s = format!("{}", time.format(FORMAT));
                    serializer.serialize_str(&s)
                }
                None => serializer.serialize_str("-"),
            }
        }

        pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            Utc.datetime_from_str(&s, FORMAT)
                .map(Some)
                .map_err(serde::de::Error::custom)
        }
    }

    pub mod str_time_option {
        use chrono::{NaiveTime, Timelike};
        use serde::{Deserialize, Deserializer, Serializer};

        pub fn ser_str_time_opt<S>(
            opt_time: &Option<NaiveTime>,
            serializer: S,
        ) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match opt_time {
                Some(time) => {
                    let hr = time.hour();
                    let min = time.minute();
                    let mut sec_str = String::with_capacity(1);
                    sec_str.push_str("0");

                    let s = [hr.to_string(), min.to_string(), sec_str].join(":");

                    serializer.serialize_str(&s)
                }
                None => serializer.serialize_none(),
            }
        }

        pub fn de_str_time_opt_erp<'de, D>(deserializer: D) -> Result<Option<NaiveTime>, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            if s.eq("-") {
                return Ok(None);
            }
            let hr = &mut s[0..1].parse().map_err(serde::de::Error::custom)?;
            let min = &s[3..4].parse().map_err(serde::de::Error::custom)?;
            if *hr == 24 {
                *hr = 0
            }

            let time = NaiveTime::from_hms_opt(*hr, *min, 0);
            Ok(time)
        }

        pub fn de_str_time_opt_br<'de, D>(deserializer: D) -> Result<Option<NaiveTime>, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            if s.eq("-") {
                return Ok(None);
            }
            let hr = &mut s[0..1].parse().map_err(serde::de::Error::custom)?;
            let min = &s[2..3].parse().map_err(serde::de::Error::custom)?;
            if *hr == 24 {
                *hr = 0
            }

            let time = NaiveTime::from_hms_opt(*hr, *min, 0);
            Ok(time)
        }
    }

    pub mod str_date {
        use chrono::NaiveDate;
        use serde::{Deserialize, Deserializer, Serializer};

        const FORMAT: &str = "%Y-%m-%d";

        pub fn serialize<S>(date: &NaiveDate, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let s = format!("{}", date.format(FORMAT));
            serializer.serialize_str(&s)
        }

        pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            NaiveDate::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
        }
    }
}

/// Deserialisation utils
pub mod de {
    use std::fmt;
    use std::fmt::Display;
    use std::iter::FromIterator;
    use std::marker::PhantomData as Phantom;
    use std::str::FromStr;

    use crate::{regex::*, Coordinates, Location};
    use serde::de::{self, Visitor};
    use serde::export::Formatter;
    use serde::{Deserialize, Deserializer};
    use serde_json::Value;

    /// Error for wrapped data
    pub struct WrapErr;

    /// Separator trait
    pub trait Sep {
        fn delimiter() -> &'static str;
    }

    impl Display for WrapErr {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "StringWrapErr")
        }
    }

    /// If error, return None
    pub fn treat_error_as_none<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
    where
        T: Deserialize<'de>,
        D: Deserializer<'de>,
    {
        let value: Value = Deserialize::deserialize(deserializer)?;
        Ok(T::deserialize(value).ok())
    }

    /// Simple conversion of Y and N to boolean
    pub fn from_str_to_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        match s.as_ref() {
            "Y" | "Yes" => Ok(true),
            "N" | "No" => Ok(false),
            _ => Ok(false),
        }
    }

    /// To be used when coordinates are space separated
    /// in a string and you would like to convert them to a Coordinates
    /// structure.
    pub fn from_str_to_coords<'de, D>(deserializer: D) -> Result<Option<Coordinates>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        if s.is_empty() || !CARPARK_COORDS_RE.is_match(s.as_str()) {
            return Ok(None);
        }

        let caps = CARPARK_COORDS_RE.captures(&s).unwrap();
        let lat = caps.get(1).map_or(0.0, |m| m.as_str().parse().unwrap());
        let long = caps.get(3).map_or(0.0, |m| m.as_str().parse().unwrap());

        Ok(Some(Coordinates::new(lat, long)))
    }

    pub fn from_str_loc_to_loc<'de, D>(deserializer: D) -> Result<Option<Location>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        if s.is_empty() || !SPEED_BAND_RE.is_match(s.as_str()) {
            return Ok(None);
        }

        let caps = SPEED_BAND_RE.captures(&s).unwrap();
        let lat_start = caps.get(1).map_or(0.0, |m| m.as_str().parse().unwrap());
        let long_start = caps.get(3).map_or(0.0, |m| m.as_str().parse().unwrap());
        let lat_end = caps.get(5).map_or(0.0, |m| m.as_str().parse().unwrap());
        let long_end = caps.get(7).map_or(0.0, |m| m.as_str().parse().unwrap());

        Ok(Some(Location::new(
            lat_start, long_start, lat_end, long_end,
        )))
    }

    pub fn from_str<'de, T, D>(deserializer: D) -> Result<T, D::Error>
    where
        T: FromStr,
        T::Err: Display,
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        T::from_str(&s).unwrap_or_else(0)
    }

    pub fn delimited<'de, V, T, D>(deserializer: D) -> Result<V, D::Error>
    where
        V: FromIterator<T>,
        T: FromStr + Sep,
        T::Err: Display,
        D: Deserializer<'de>,
    {
        struct DelimitedBy<V, T>(Phantom<V>, Phantom<T>);

        impl<'de, V, T> Visitor<'de> for DelimitedBy<V, T>
        where
            V: FromIterator<T>,
            T: FromStr + Sep,
            T::Err: Display,
        {
            type Value = V;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("string containing / separated elements")
            }

            fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let iter = s.split(T::delimiter()).map(FromStr::from_str);
                Result::from_iter(iter).map_err(de::Error::custom)
            }
        }

        let visitor = DelimitedBy(Phantom, Phantom);
        deserializer.deserialize_str(visitor)
    }
}

/// A `Client` to make requests with
/// The `Client` holds a connection pool internally, so it is advised that you create one and reuse it
pub trait Client<C, RB> {
    /// General constructor
    fn new(api_key: Option<String>, client: C) -> Self;

    /// This method not assign the `api_key` in struct if the provided key is empty or whitespaces
    /// Instead, assign `None`
    fn with_api_key<S>(api_key: S) -> Self
    where
        S: Into<String>;

    /// Make sure that you check that the `api_key` is not `None`!
    fn get_req_builder(&self, url: &str) -> RB;
}

/// Starting and ending location
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Location {
    pub start: Coordinates,
    pub end: Coordinates,
}

impl Location {
    pub fn new(start_lat: f64, start_lang: f64, end_lat: f64, end_lang: f64) -> Self {
        Location {
            start: Coordinates::new(start_lat, start_lang),
            end: Coordinates::new(end_lat, end_lang),
        }
    }

    pub fn from_coords(start: Coordinates, end: Coordinates) -> Self {
        Location { start, end }
    }
}

/// Coordinate on the map
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Coordinates {
    pub lat: f64,
    pub long: f64,
}

impl Coordinates {
    pub fn new(lat: f64, long: f64) -> Self {
        Coordinates { lat, long }
    }
}

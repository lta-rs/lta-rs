//! Utilities for transforming data and other misc

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

        pub static ref DATE_RE: Regex = Regex::new(r"^(\d{4})-(\d{2})-(\d{2})").unwrap();
        pub static ref TIME_RE: Regex = Regex::new(r"^(\d{2}):?(\d{2})").unwrap();
        pub static ref DATETIME_RE: Regex = Regex::new(r"^(\d{4})-(\d{2})-(\d{2}) (\d{2}):?(\d{2}):?(\d{2})").unwrap();
    }
}

pub mod ser {
    use chrono::prelude::*;
    use serde::Serializer;

    const YMD_FORMAT: &str = "%Y-%m-%d";
    const HMS_FORMAT: &str = "%H:%M:%S";
    const YMD_HMS_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

    pub fn from_date_to_str<S>(date: &Date<FixedOffset>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(YMD_FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn from_time_to_str<S>(
        opt_time: &Option<NaiveTime>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match opt_time {
            Some(time) => {
                let s = format!("{}", time.format(HMS_FORMAT));
                serializer.serialize_str(&s)
            }
            None => serializer.serialize_str("-"),
        }
    }

    pub fn from_datetime_to_str<S>(
        opt_time: &Option<DateTime<FixedOffset>>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match opt_time {
            Some(time) => {
                let s = format!("{}", time.format(YMD_HMS_FORMAT));
                serializer.serialize_str(&s)
            }
            None => serializer.serialize_str("-"),
        }
    }
}

pub mod de {
    use std::fmt;
    use std::fmt::Display;
    use std::iter::FromIterator;
    use std::marker::PhantomData as Phantom;
    use std::str::FromStr;

    use chrono::prelude::*;
    use serde::de::{self, Unexpected, Visitor};
    use serde::{Deserialize, Deserializer};

    use crate::bus::bus_services::BusFreq;
    use crate::traffic::est_travel_time::HighwayDirection;
    use crate::train::train_service_alert::TrainStatus;
    use crate::utils::commons::{Coordinates, Location};
    use crate::utils::regex::{
        BUS_FREQ_RE, CARPARK_COORDS_RE, DATETIME_RE, DATE_RE, SPEED_BAND_RE, TIME_RE,
    };

    pub fn from_str_to_datetime<'de, D>(
        deserializer: D,
    ) -> Result<Option<DateTime<FixedOffset>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        if s.is_empty() {
            return Ok(None);
        }

        let parse = |m: regex::Match| m.as_str().parse::<u32>().unwrap();
        let caps = DATETIME_RE.captures(&s).unwrap();
        let year: i32 = caps
            .get(1)
            .map_or(0, |m: regex::Match| m.as_str().parse().unwrap());

        let month: u32 = caps.get(2).map_or(0, parse);

        let day: u32 = caps.get(3).map_or(0, parse);

        let hr: u32 = caps.get(4).map_or(0, parse);

        let min: u32 = caps.get(5).map_or(0, parse);

        let sec: u32 = caps.get(6).map_or(0, parse);

        let dt: DateTime<FixedOffset> = Utc
            .ymd(year, month, day)
            .and_hms(hr, min, sec)
            .with_timezone(&FixedOffset::east(8 * 3600));

        Ok(Some(dt))
    }

    pub fn from_str_to_time<'de, D>(deserializer: D) -> Result<Option<NaiveTime>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;

        if s.eq("-") {
            return Ok(None);
        }

        let caps = TIME_RE.captures(&s).unwrap();
        let mut hr: u32 = caps
            .get(1)
            .map_or(0, |m: regex::Match| m.as_str().parse().unwrap());
        let min: u32 = caps
            .get(2)
            .map_or(0, |m: regex::Match| m.as_str().parse().unwrap());

        if hr == 24 {
            hr = 0
        }

        let time = NaiveTime::from_hms(hr, min, 0);
        Ok(Some(time))
    }

    pub fn from_str_shelter_indicator_to_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;

        match s.as_ref() {
            "Y" => Ok(true),
            "N" => Ok(false),
            _ => Ok(false),
        }
    }

    pub fn from_str_to_date<'de, D>(deserializer: D) -> Result<Date<FixedOffset>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        let caps = DATE_RE.captures(&s).unwrap();

        let year: i32 = caps
            .get(1)
            .map_or(1970, |m: regex::Match| m.as_str().parse().unwrap());
        let month: u32 = caps
            .get(2)
            .map_or(1, |m: regex::Match| m.as_str().parse().unwrap());
        let day: u32 = caps
            .get(3)
            .map_or(1, |m: regex::Match| m.as_str().parse().unwrap());

        let date: Date<FixedOffset> = Utc
            .ymd(year, month, day)
            .with_timezone(&FixedOffset::east(8 * 3600));

        Ok(date)
    }

    /// Converts from eg. 12-15 to `BusFreq::new(12,15)`
    /// There are special cases like `-` and `10`.
    /// In those cases, it will be `BusFreq::default()` and `BusFreq::new(10,10)`
    pub fn from_str_to_bus_freq<'de, D>(deserializer: D) -> Result<BusFreq, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;

        let caps = BUS_FREQ_RE.captures(&s).unwrap();
        let min: u32 = caps.get(1).map_or(0, |m| m.as_str().parse().unwrap());
        let max: u32 = caps.get(2).map_or(0, |m| m.as_str().parse().unwrap());

        let bus_freq = if min == 0 && max == 0 {
            BusFreq::no_timing()
        } else if min != 0 && max == 0 {
            BusFreq::no_max(min)
        } else {
            BusFreq::new(min, max)
        };

        Ok(bus_freq)
    }

    pub fn from_int_to_highway<'de, D>(deserializer: D) -> Result<HighwayDirection, D::Error>
    where
        D: Deserializer<'de>,
    {
        match u8::deserialize(deserializer)? {
            1 => Ok(HighwayDirection::EastToWest),
            2 => Ok(HighwayDirection::WestToEast),
            other => Err(de::Error::invalid_value(
                Unexpected::Unsigned(u64::from(other)),
                &"zero or one",
            )),
        }
    }

    pub fn from_int_to_mrt_status<'de, D>(deserializer: D) -> Result<TrainStatus, D::Error>
    where
        D: Deserializer<'de>,
    {
        match u8::deserialize(deserializer)? {
            1 => Ok(TrainStatus::Normal),
            2 => Ok(TrainStatus::Disrupted),
            other => Err(de::Error::invalid_value(
                Unexpected::Unsigned(u64::from(other)),
                &"one and two",
            )),
        }
    }

    /// To be used when coordinates are space separated
    /// in a string and you would like to convert them to a Coordinates
    /// structure.
    pub fn from_str_to_coords<'de, D>(deserializer: D) -> Result<Option<Coordinates>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;

        if s.is_empty() || !CARPARK_COORDS_RE.is_match(s.as_str()) {
            return Ok(None);
        }

        let caps = CARPARK_COORDS_RE.captures(&s).unwrap();
        let lat: f64 = caps.get(1).map_or(0.0, |m| m.as_str().parse().unwrap());
        let long: f64 = caps.get(3).map_or(0.0, |m| m.as_str().parse().unwrap());

        Ok(Some(Coordinates::new(lat, long)))
    }

    pub fn from_str_loc_to_loc<'de, D>(deserializer: D) -> Result<Option<Location>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;

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
        T::from_str(&s).map_err(de::Error::custom)
    }

    pub fn slash_separated<'de, V, T, D>(deserializer: D) -> Result<V, D::Error>
    where
        V: FromIterator<T>,
        T: FromStr,
        T::Err: Display,
        D: Deserializer<'de>,
    {
        struct SlashSeparated<V, T>(Phantom<V>, Phantom<T>);

        impl<'de, V, T> Visitor<'de> for SlashSeparated<V, T>
        where
            V: FromIterator<T>,
            T: FromStr,
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
                let iter = s.split('/').map(FromStr::from_str);
                Result::from_iter(iter).map_err(de::Error::custom)
            }
        }

        let visitor = SlashSeparated(Phantom, Phantom);
        deserializer.deserialize_str(visitor)
    }

    pub fn dash_separated<'de, V, T, D>(deserializer: D) -> Result<V, D::Error>
    where
        V: FromIterator<T>,
        T: FromStr,
        T::Err: Display,
        D: Deserializer<'de>,
    {
        struct DashSeparated<V, T>(Phantom<V>, Phantom<T>);

        impl<'de, V, T> Visitor<'de> for DashSeparated<V, T>
        where
            V: FromIterator<T>,
            T: FromStr,
            T::Err: Display,
        {
            type Value = V;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("string containing - separated elements")
            }

            fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let iter = s.split('-').map(FromStr::from_str);
                Result::from_iter(iter).map_err(de::Error::custom)
            }
        }

        let visitor = DashSeparated(Phantom, Phantom);
        deserializer.deserialize_str(visitor)
    }
}

pub mod commons {
    use std::fmt::Debug;

    use serde::Serialize;

    use crate::lta_client::LTAClient;

    pub type Result<T> = reqwest::Result<T>;
    pub type Error = reqwest::Error;

    /// A `Client` to make requests with
    /// The `Client` holds a connection pool internally, so it is advised that you create one and reuse it
    pub trait Client<C, RB> {
        type Output;

        /// General constructor
        fn new(api_key: Option<String>, client: C) -> Self::Output;

        /// This method not assign the `api_key` in struct if the provided key is empty or whitespaces
        /// Instead, assign `None`
        fn with_api_key<S>(api_key: S) -> Self::Output
        where
            S: Into<String>;

        /// Make sure that you check that the `api_key` is not `None`!
        fn get_req_builder(&self, url: &str) -> RB;
    }

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
    }

    pub fn build_req<T>(client: &LTAClient, url: &str) -> reqwest::Result<T>
    where
        for<'de> T: serde::Deserialize<'de>,
    {
        let req_builder = client.get_req_builder(url);
        req_builder.send()?.json()
    }

    pub fn build_res_with_query<T, F>(client: &LTAClient, url: &str, query: F) -> reqwest::Result<T>
    where
        F: Fn(reqwest::RequestBuilder) -> reqwest::RequestBuilder,
        for<'de> T: serde::Deserialize<'de> + Debug,
    {
        let req_builder = client.get_req_builder(url);
        query(req_builder).send()?.json()
    }

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub struct Coordinates {
        pub lat: f64,
        pub lang: f64,
    }

    impl Coordinates {
        pub fn new(lat: f64, lang: f64) -> Self {
            Coordinates { lat, lang }
        }
    }
}

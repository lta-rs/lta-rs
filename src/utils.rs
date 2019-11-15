//! Utilities for transforming data and other misc

pub(crate) mod regex {
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

pub(crate) mod serde_date {
    pub mod ymd_hms_option {
        use chrono::{DateTime, FixedOffset, TimeZone, Utc};
        use serde::{Deserialize, Deserializer, Serializer};

        const FORMAT: &str = "%Y-%m-%d %H:%M:%S%z";
        const FORMAT_DE: &str = "%Y-%m-%d %H:%M:%S";

        pub fn serialize<S>(
            date: &Option<DateTime<FixedOffset>>,
            serializer: S,
        ) -> Result<S::Ok, S::Error>
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

        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> Result<Option<DateTime<FixedOffset>>, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s: String = String::deserialize(deserializer)?;
            Utc.datetime_from_str(&s, FORMAT_DE)
                .map(|dt_utc| Some(dt_utc.with_timezone(&FixedOffset::east(8 * 3600))))
                .map_err(serde::de::Error::custom)
        }
    }

    pub mod str_time_option {
        use chrono::{NaiveTime, Timelike};
        use serde::{Deserialize, Deserializer, Serializer};

        const FORMAT: &str = "%H:%M:%S";

        type Hour = u32;
        type Min = u32;

        pub fn serialize<S>(opt_time: &Option<NaiveTime>, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match opt_time {
                Some(time) => {
                    let hr: Hour = time.hour();
                    let min: Min = time.minute();
                    let mut sec_str = String::with_capacity(1);
                    sec_str.push_str("0");

                    let s = [hr.to_string(), min.to_string(), sec_str].join(":");

                    serializer.serialize_str(&s)
                }
                None => serializer.serialize_none(),
            }
        }

        pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<NaiveTime>, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s: String = String::deserialize(deserializer)?;
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
            let s: String = String::deserialize(deserializer)?;
            NaiveDate::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
        }
    }
}

pub(crate) mod de {
    use std::fmt;
    use std::fmt::Display;
    use std::iter::FromIterator;
    use std::marker::PhantomData as Phantom;
    use std::str::FromStr;

    use serde::de::{self, Unexpected, Visitor};
    use serde::{Deserialize, Deserializer};
    use serde_json::Value;

    use crate::bus::bus_services::BusFreq;
    use crate::traffic::est_travel_time::HighwayDirection;
    use crate::train::train_service_alert::TrainStatus;
    use crate::utils::commons::{Coordinates, Location};
    use crate::utils::regex::{BUS_FREQ_RE, CARPARK_COORDS_RE, SPEED_BAND_RE};

    pub fn treat_error_as_none<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
    where
        T: Deserialize<'de>,
        D: Deserializer<'de>,
    {
        let value: Value = Deserialize::deserialize(deserializer)?;
        Ok(T::deserialize(value).ok())
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

    /// Converts from eg. 12-15 to `BusFreq::new(12,15)`
    /// There are special cases like `-` and `10`.
    /// In those cases, it will be `Default::default()` and `BusFreq::new(10,10)`
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

    use futures::Future;
    use reqwest::r#async::RequestBuilder as AsyncRB;
    use serde::Serialize;

    use crate::{lta_client::LTAClient, r#async::lta_client::LTAClient as AsyncLTAClient};

    pub type Result<T> = reqwest::Result<T>;
    pub type Error = reqwest::Error;

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

    pub fn build_req<T, M>(client: &LTAClient, url: &str) -> reqwest::Result<M>
    where
        for<'de> T: serde::Deserialize<'de> + Debug + Into<M>,
    {
        let req_builder = client.get_req_builder(url);
        req_builder.send()?.json().map(|f: T| f.into())
    }

    pub fn build_req_with_query<T, M, F>(
        client: &LTAClient,
        url: &str,
        query: F,
    ) -> reqwest::Result<M>
    where
        F: FnOnce(reqwest::RequestBuilder) -> reqwest::RequestBuilder,
        for<'de> T: serde::Deserialize<'de> + Debug + Into<M>,
    {
        let req_builder = client.get_req_builder(url);
        query(req_builder).send()?.json().map(|f: T| f.into())
    }

    pub fn build_req_async<T, M>(
        client: &AsyncLTAClient,
        url: &str,
    ) -> impl Future<Item = M, Error = Error>
    where
        for<'de> T: serde::Deserialize<'de> + Into<M> + Debug,
    {
        let rb = client.get_req_builder(url);
        rb.send()
            .and_then(|mut f| f.json::<T>())
            .map(|f: T| f.into())
    }

    pub fn build_req_async_with_query<T, M, F>(
        client: &AsyncLTAClient,
        url: &str,
        query: F,
    ) -> impl Future<Item = M, Error = Error>
    where
        F: FnOnce(AsyncRB) -> AsyncRB,
        for<'de> T: serde::Deserialize<'de> + Into<M> + Debug,
    {
        let rb = client.get_req_builder(url);
        query(rb)
            .send()
            .and_then(|mut f| f.json::<T>())
            .map(|f: T| f.into())
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

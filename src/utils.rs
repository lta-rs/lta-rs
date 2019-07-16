pub mod re {
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

pub mod de {
    use std::fmt;
    use std::fmt::Display;
    use std::iter::FromIterator;
    use std::marker::PhantomData as Phantom;
    use std::str::FromStr;

    use regex::Regex;
    use serde::de::{self, Unexpected, Visitor};
    use serde::{Deserialize, Deserializer};

    use crate::bus::bus_services::BusFreq;
    use crate::traffic::est_travel_time::HighwayDirection;
    use crate::train::train_service_alert::TrainStatus;
    use crate::utils::commons::{Coordinates, Location};
    use crate::utils::re::{BUS_FREQ_RE, CARPARK_COORDS_RE, SPEED_BAND_RE};

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
                Unexpected::Unsigned(other as u64),
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
                Unexpected::Unsigned(other as u64),
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
                let iter = s.split("/").map(FromStr::from_str);
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
                let iter = s.split("-").map(FromStr::from_str);
                Result::from_iter(iter).map_err(de::Error::custom)
            }
        }

        let visitor = SlashSeparated(Phantom, Phantom);
        deserializer.deserialize_str(visitor)
    }
}

pub mod commons {
    use std::fmt::Debug;

    use serde::Serialize;

    use crate::lta_client::LTAClient;

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

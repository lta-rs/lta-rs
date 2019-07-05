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
        let s = String::deserialize(deserializer)?;

        let mut bus_freq = None;
        for cap in BUS_FREQ_RE.captures_iter(&s) {
            let min = cap.get(1);
            let max = cap.get(2);

            bus_freq = if min.is_some() && max.is_some() {
                // case where both exist ie. 12-15
                Some(BusFreq::new(
                    min.unwrap().as_str().parse().unwrap(),
                    max.unwrap().as_str().parse().unwrap(),
                ))
            } else if min.is_some() && max.is_none() {
                // case where only the min exist ie. 10
                Some(BusFreq::no_max(min.unwrap().as_str().parse().unwrap()))
            } else {
                Some(BusFreq::no_timing())
            };
        }

        bus_freq.ok_or(de::Error::invalid_value(
            Unexpected::Str(""),
            &"Invalid BusFreq. Please contact crate dev.",
        ))
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
        let s = String::deserialize(deserializer)?;

        let mut coords: Option<Coordinates> = None;

        // split string first, then convert to f64
        for cap in CARPARK_COORDS_RE.captures_iter(&s) {
            let lat_match = cap.get(1);
            let long_match = cap.get(3);

            coords = if lat_match.is_some() && long_match.is_some() {
                let lat: f64 = lat_match.unwrap().as_str().parse().unwrap();
                let long: f64 = long_match.unwrap().as_str().parse().unwrap();

                Some(Coordinates::new(lat, long))
            } else {
                None
            };
        }

        Ok(coords)
    }

    pub fn from_str_loc_to_loc<'de, D>(deserializer: D) -> Result<Option<Location>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        let mut loc: Option<Location> = None;

        // split string first, then convert to f64
        for cap in SPEED_BAND_RE.captures_iter(&s) {
            let lat_match_start = cap.get(1);
            let long_match_start = cap.get(3);
            let lat_match_end = cap.get(5);
            let long_match_end = cap.get(7);

            loc = if lat_match_start.is_some()
                && long_match_start.is_some()
                && lat_match_end.is_some()
                && long_match_end.is_some()
            {
                let lat: f64 = lat_match_start.unwrap().as_str().parse().unwrap();
                let long: f64 = long_match_start.unwrap().as_str().parse().unwrap();
                let lat_end: f64 = lat_match_end.unwrap().as_str().parse().unwrap();
                let long_end: f64 = long_match_end.unwrap().as_str().parse().unwrap();

                Some(Location::new(lat, long, lat_end, long_end))
            } else {
                None
            };
        }

        Ok(loc)
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

    use crate::lta_client::LTAClient;

    #[derive(Debug, Clone, PartialEq)]
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

    #[derive(Debug, Clone, PartialEq)]
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

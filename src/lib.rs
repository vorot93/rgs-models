extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate iso_country;

use std::ops::Deref;
use std::str::FromStr;

use serde::*;
use serde::de::{Deserializer, Visitor};
use serde_json::Value;

use iso_country::Country as CountryBase;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Country(CountryBase);

impl Deref for Country {
    type Target = iso_country::Country;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for Country {
    fn default() -> Country {
        Country(CountryBase::Unspecified)
    }
}

impl Serialize for Country {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}

impl<'de> Deserialize<'de> for Country {
    fn deserialize<D>(deserializer: D) -> Result<Country, D::Error>
        where D: Deserializer<'de>
    {
        struct CountryVisitor;
        impl<'de> Visitor<'de> for CountryVisitor {
            type Value = Country;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ISO country code")
            }

            fn visit_str<E>(self, value: &str) -> Result<Country, E>
                where E: de::Error
            {
                Ok(Country(CountryBase::from_str(value).unwrap_or(CountryBase::Unspecified)))
            }
        }
        deserializer.deserialize_str(CountryVisitor)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Status {
    Unspecified,
    Up,
    Down,
}

impl Default for Status {
    fn default() -> Status {
        Status::Unspecified
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Player {
    pub name: String,
    pub ping: Option<i64>,
    pub info: serde_json::Map<String, Value>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Server {
    // Mandatory parameters
    pub addr: std::net::SocketAddr,

    #[serde(default)]
    pub status: Status,

    #[serde(default)]
    pub country: Country,

    #[serde(default)]
    pub rules: serde_json::Map<String, Value>,

    // Optional fields
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if="Option::is_none")]
    pub need_pass: Option<bool>,

    #[serde(skip_serializing_if="Option::is_none")]
    pub mod_name: Option<String>,

    #[serde(skip_serializing_if="Option::is_none")]
    pub game_type: Option<String>,

    #[serde(skip_serializing_if="Option::is_none")]
    pub terrain: Option<String>,

    #[serde(skip_serializing_if="Option::is_none")]
    pub num_clients: Option<i64>,

    #[serde(skip_serializing_if="Option::is_none")]
    pub max_clients: Option<i64>,

    #[serde(skip_serializing_if="Option::is_none")]
    pub num_bots: Option<i64>,

    #[serde(skip_serializing_if="Option::is_none")]
    pub secure: Option<bool>,

    #[serde(skip_serializing_if="Option::is_none")]
    pub ping: Option<i64>,

    #[serde(skip_serializing_if="Option::is_none")]
    pub players: Option<Vec<Player>>,
}

impl Server {
    pub fn new(addr: std::net::SocketAddr) -> Server {
        Server {
            addr: addr,
            status: Default::default(),
            country: Default::default(),
            rules: Default::default(),
            name: Default::default(),
            need_pass: Default::default(),
            mod_name: Default::default(),
            game_type: Default::default(),
            terrain: Default::default(),
            num_clients: Default::default(),
            max_clients: Default::default(),
            num_bots: Default::default(),
            secure: Default::default(),
            ping: Default::default(),
            players: Default::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate serde_json;

    fn fixtures() -> (Value, Server) {
        let mut srv = Server::new(std::net::SocketAddr::from_str("127.0.0.1:9000").unwrap());
        srv.status = Status::Up;
        srv.country = Country(CountryBase::RU);
        srv.rules.insert("protocol-version".into(), 84.into());

        let ser = json!({
            "addr": "127.0.0.1:9000",
            "status": "Up",
            "country": "RU",
            "rules": {
                "protocol-version": 84,
            },
        });

        (ser, srv)
    }

    #[test]
    fn serialization() {
        let (expectation, fixture) = fixtures();

        let result = serde_json::to_value(&fixture).unwrap();

        assert_eq!(expectation, result);
    }

    #[test]
    fn deserialization() {
        let (fixture, expectation) = fixtures();

        let result = serde_json::from_value(fixture).unwrap();

        assert_eq!(expectation, result);
    }
}

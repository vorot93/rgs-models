extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate iso_country;

use std::ops::Deref;
use std::str::FromStr;

use serde::*;
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

impl Deserialize for Country {
    fn deserialize<D>(deserializer: D) -> Result<Country, D::Error>
        where D: Deserializer
    {
        struct Visitor;
        impl de::Visitor for Visitor {
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
        deserializer.deserialize_str(Visitor)
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

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Server {
    // Mandatory parameters
    pub host: String,

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

#[cfg(test)]
mod tests {
    use super::*;
    extern crate serde_json;

    #[test]
    fn serialization() {
        let mut fixture = Server::default();
        fixture.status = Status::Up;
        fixture.host = "127.0.0.1".to_string();
        fixture.country = Country(CountryBase::RU);
        fixture.rules.insert("protocol-version".into(), 84.into());

        let expectation = json!({
            "host": "127.0.0.1",
            "status": "Up",
            "country": "RU",
            "rules": {
                "protocol-version": 84,
            },
        });

        let result = serde_json::to_value(&fixture).unwrap();

        assert_eq!(expectation, result);
    }

    #[test]
    fn deserialization() {
        let fixture = json!({
            "host": "127.0.0.1",
            "status": "Up",
            "country": "RU",
            "rules": {
                "protocol-version": 84,
            },
        });

        let mut expectation = Server::default();
        expectation.status = Status::Up;
        expectation.host = "127.0.0.1".to_string();
        expectation.country = Country(CountryBase::RU);
        expectation.rules.insert("protocol-version".into(), 84.into());

        let result = serde_json::from_value(fixture).unwrap();

        assert_eq!(expectation, result);
    }
}

use mongodb::bson::{doc, Bson};
use rocket::http;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use std::{
    collections::HashMap,
    fmt,
    sync::{Arc, Mutex, PoisonError},
};
use throttle::Throttle;

use crate::models::ratelimit::RateTime;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Method {
    Any,
    Http(http::Method),
}

impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Any => write!(f, "*"),
            Self::Http(method) => write!(f, "{}", method.as_str()),
        }
    }
}

impl Serialize for Method {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        String::serialize(&self.to_string(), serializer)
    }
}

impl<'de> Deserialize<'de> for Method {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        match String::deserialize(deserializer)?.as_str() {
            "*" => Ok(Self::Any),
            "get" | "GET" => Ok(Self::Http(http::Method::Get)),
            "put" | "PUT" => Ok(Self::Http(http::Method::Put)),
            "post" | "POST" => Ok(Self::Http(http::Method::Post)),
            "delete" | "DELETE" => Ok(Self::Http(http::Method::Delete)),
            "options" | "OPTIONS" => Ok(Self::Http(http::Method::Options)),
            "head" | "HEAD" => Ok(Self::Http(http::Method::Head)),
            "trace" | "TRACE" => Ok(Self::Http(http::Method::Trace)),
            "connect" | "CONNECT" => Ok(Self::Http(http::Method::Connect)),
            "patch" | "PATCH" => Ok(Self::Http(http::Method::Patch)),
            s => Err(de::Error::custom(format!("unknown '{}' method", s))),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Endpoint {
    pub name: String,
    pub method: Method,
    pub throttle: Option<RateTime>,
}

impl From<Endpoint> for Bson {
    fn from(endpoint: Endpoint) -> Self {
        Bson::Document(doc! {
            "name": endpoint.name,
            "method": endpoint.method.to_string(),
            "rate_time": endpoint.throttle.map(|rate_time| Bson::String(rate_time.to_string())).unwrap_or(Bson::Null)
        })
    }
}

#[derive(Default)]
pub struct EndpointHandler {
    throttles: Arc<Mutex<HashMap<(String, String, Method), Option<Throttle>>>>,
}

impl EndpointHandler {
    pub fn can_access(&self, api_key: String, endpoint: Endpoint) -> bool {
        let mut guarded_throttles = self
            .throttles
            .lock()
            .unwrap_or_else(PoisonError::into_inner);

        let throttle = guarded_throttles
            .entry((api_key, endpoint.name.to_string(), endpoint.method))
            .or_insert_with(|| endpoint.throttle.clone().map(|rate| rate.into()));

        if let Some(throttle) = throttle {
            matches!(throttle.accept(), Ok(()))
        } else {
            // No throttle for the given endpoint
            true
        }
    }
}

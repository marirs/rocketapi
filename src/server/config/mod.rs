use serde::Deserialize;
/// Parse Settings from a configuration yaml file
/// ### Default Configuration file:
/// - config.sample.yml
///
use std::{net::IpAddr, path::Path};

const SRV_ADDR: &str = "127.0.0.1";
const SRV_PORT: usize = 8080;
const SRV_KEEP_ALIVE: usize = 60;
const SRV_FORMS_LIMIT: usize = 256 * 1024;
const SRV_JSON_LIMIT: usize = 1024 * 1024;
const SRV_SECRET_KEY: &str = "VTlITUNuM013NnpWWThHV1NJMHlCdDNEcThaaXJOZWc=";
const SSL_ENABLED: bool = false;
const SSL_CN: &str = "localhost";
const SSL_CERT_VALIDITY: u32 = 90;
const MONGO_HOST: &str = "localhost";
const MONGO_PORT: usize = 27017;
const DB_NAME: &str = "rocketapi";
const COLLECTION_NAME: &str = "users";

/// Rocket API Server parameters
#[derive(Deserialize, Clone, Debug)]
pub struct Settings {
    /// Api Server config related parameters
    #[serde(default)]
    pub api_server: ApiServer,
    /// SSL config related parameters
    #[serde(default)]
    pub ssl: SSL,
    /// Database config related parameters
    #[serde(default)]
    pub db: DbConfig,
}

impl Settings {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Self {
        //! Read configuration settings from yaml file
        //!
        //! ## Example usage
        //! ```ignore
        //! Settings::from_file("config.sample.yml");
        //! ```
        //!
        let mut cfg = config::Config::default();
        cfg.merge(config::File::with_name(path.as_ref().to_str().unwrap()))
            .unwrap();
        cfg.try_into().unwrap()
    }

    pub fn default() -> Self {
        Settings {
            api_server: ApiServer {
                ..Default::default()
            },
            ssl: SSL {
                ..Default::default()
            },
            db: DbConfig {
                ..Default::default()
            },
        }
    }
}

/// Rocket API Server params
#[derive(Deserialize, Clone, Debug)]
pub struct ApiServer {
    /// Server Ip Address to start Rocket API Server
    #[serde(default = "default_api_server_host")]
    pub host: IpAddr,
    /// Server port to listen Rocket API Server
    #[serde(default = "default_api_server_port")]
    pub port: usize,
    /// Server Keep Alive
    #[serde(default = "default_api_server_keep_alive")]
    pub keep_alive: usize,
    /// Forms limitation
    #[serde(default = "default_api_server_forms_limit")]
    pub forms_limit: usize,
    /// JSON transfer limitation
    #[serde(default = "default_api_server_json_limit")]
    pub json_limit: usize,
    /// Api Server Secret key
    #[serde(default = "default_api_server_secret_key")]
    pub secret_key: String,
}

impl Default for ApiServer {
    fn default() -> Self {
        ApiServer {
            host: SRV_ADDR.to_string().parse().unwrap(),
            port: SRV_PORT,
            keep_alive: SRV_KEEP_ALIVE,
            forms_limit: SRV_FORMS_LIMIT,
            json_limit: SRV_JSON_LIMIT,
            secret_key: SRV_SECRET_KEY.to_string(),
        }
    }
}

/// SSL params
#[derive(Deserialize, Clone, Debug)]
pub struct SSL {
    /// Enable SSL for Rocket API Server (True/False)
    #[serde(default = "default_ssl_enabled")]
    pub enabled: bool,
    /// Common Name for certificate generation
    #[serde(default = "default_ssl_cn")]
    pub common_name: String,
    /// No. of days the certificate should be valid
    #[serde(default = "default_ssl_cert_valid_days")]
    pub certificate_validity: u32,
}

impl Default for SSL {
    fn default() -> Self {
        SSL {
            enabled: SSL_ENABLED,
            common_name: SSL_CN.to_string(),
            certificate_validity: SSL_CERT_VALIDITY,
        }
    }
}

/// Db configuration parameters
#[derive(Deserialize, Clone, Debug)]
pub struct DbConfig {
    /// Mongo Host to connect to mongo DB
    #[serde(default = "default_mongo_host")]
    pub mongo_host: String,
    /// Mongo Port
    #[serde(default = "default_mongo_port")]
    pub mongo_port: usize,
    /// Mongo URI, if intended to use Mongo URI
    pub mongo_uri: Option<String>,
    /// Mongo Database name to use
    #[serde(default = "default_db_name")]
    pub db_name: String,
    /// Mongo Collection name to use
    #[serde(default = "default_collection_name")]
    pub collection_name: String,
    /// Authentication: Mongo User
    pub auth_user: Option<String>,
    /// Authentication: Mongo Password
    pub auth_pass: Option<String>,
}

impl Default for DbConfig {
    fn default() -> Self {
        DbConfig {
            mongo_host: MONGO_HOST.to_string(),
            mongo_port: MONGO_PORT,
            mongo_uri: None,
            db_name: DB_NAME.to_string(),
            collection_name: COLLECTION_NAME.to_string(),
            auth_user: None,
            auth_pass: None,
        }
    }
}

fn default_api_server_host() -> IpAddr {
    SRV_ADDR.to_string().parse().unwrap()
}
fn default_api_server_port() -> usize {
    SRV_PORT
}
fn default_api_server_keep_alive() -> usize {
    SRV_KEEP_ALIVE
}
fn default_api_server_forms_limit() -> usize {
    SRV_FORMS_LIMIT
}
fn default_api_server_json_limit() -> usize {
    SRV_JSON_LIMIT
}
fn default_api_server_secret_key() -> String {
    SRV_SECRET_KEY.to_string()
}
fn default_ssl_enabled() -> bool {
    SSL_ENABLED
}
fn default_ssl_cn() -> String {
    SSL_CN.to_string()
}
fn default_ssl_cert_valid_days() -> u32 {
    SSL_CERT_VALIDITY
}
fn default_mongo_host() -> String {
    MONGO_HOST.to_string()
}
fn default_mongo_port() -> usize {
    MONGO_PORT
}
fn default_db_name() -> String {
    DB_NAME.to_string()
}
fn default_collection_name() -> String {
    COLLECTION_NAME.to_string()
}

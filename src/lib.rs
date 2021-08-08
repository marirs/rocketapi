#[macro_use]
extern crate rocket;
#[macro_use]
pub(crate) mod macros;

/// All server related
pub mod server;

/// All guards/ssl generation/etc...
pub mod secure;

/// All the Routes/endpoints
mod controllers;

/// Database
mod db;

/// Models
pub mod models;

/// App related Errors
pub mod error;
pub type Result<T> = std::result::Result<T, error::Error>;

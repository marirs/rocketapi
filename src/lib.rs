#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
#[macro_use(bson, doc)]
extern crate bson;
#[macro_use]
pub mod macros;

pub(crate) mod models;

/// db functions
pub(crate) mod db;

/// Server related
pub mod server;

/// all endpoints routing
pub mod controllers;

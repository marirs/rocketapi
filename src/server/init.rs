use rocket::config::{Config, Environment, Limits};

use std::path::Path;

use crate::config::Settings;
use crate::{
    controllers::{others, users},
    db::mongo::init_db_pool,
    server::{catchers, core::cert},
};

#[cfg(debug_assertions)]
static ENV: Environment = Environment::Staging;
#[cfg(not(debug_assertions))]
static ENV: Environment = Environment::Production;

pub fn start(settings: Settings) {
    //! Initialize & Start the Rocket Server
    //!
    //! ## Example usage
    //! ```ignore
    //! start(settings);
    //! ```
    // Create Rocket limits configuration
    let limits = Limits::new()
        .limit("forms", settings.api_server.forms_limit as u64)
        .limit("json", settings.api_server.json_limit as u64);
    // Create configuration from config yaml file
    let rocket_cfg = Config::build(ENV)
        .address(settings.api_server.host.to_string())
        .port(settings.api_server.port as u16)
        .limits(limits)
        .secret_key(settings.api_server.secret_key.as_str())
        .keep_alive(settings.api_server.keep_alive as u32);
    // Configure SSL status for the api server
    let rocket_cfg = if settings.clone().ssl.enabled {
        if !Path::new("private/cert").exists() && !Path::new("private/key").exists() {
            cert::generate_cert(settings.ssl).unwrap();
        }
        rocket_cfg.tls("private/cert", "private/key")
    } else {
        rocket_cfg
    };
    // finalize the configuration to be used with rocket
    let rocket_cfg = rocket_cfg.finalize().unwrap();
    // Create DbPool with DB Options for Mongo
    let db_pool = init_db_pool(settings.db.clone());

    // Launch the Rocket server with configured settings
    let app = rocket::custom(rocket_cfg);
    // Catchers
    let app = app.register(rocket::catchers![
        catchers::bad_request,
        catchers::forbidden,
        catchers::not_authorized,
        catchers::not_found,
        catchers::unprocessed_entity,
        catchers::internal_server_error
    ]);
    let app = app.mount("/api", routes![others::ping,]);
    let app = app.mount(
        "/api/user",
        routes![
            users::routes::list_users,
            users::routes::add_user,
            users::routes::modify_user,
            users::routes::delete_user,
        ],
    );
    // Add Db Pool to the State
    let app = app.manage(db_pool);
    // Add DBConfig to state
    let app = app.manage(settings.db);
    // Start the Rocket server
    app.launch();
}

use crate::config::Settings;
use crate::db::{
    mongo::{init_db_pool, Conn, UserMongoRepository},
    prelude::*,
};
use crate::models::users::{CreateUserParam, User};
use serde_json::to_string_pretty;

fn get_db_conn(path: &str) -> Conn {
    let settings = Settings::from_file(path); // FIXME: will panic..
    let pool = init_db_pool(settings.db); // FIXME: will panic..
    pool.get().expect("Unable to get connection from pool")
}

pub fn create_superuser(path: &str, email: &str, creator: &str) {
    // Initialize DB Connection
    println!("Initializing DB connection...");
    let conn = get_db_conn(path);
    let repo = UserMongoRepository::new(conn);

    // Creates the user.
    let params = CreateUserParam {
        email: email.to_string(),
        created_by: creator.to_string(),
        created_ip: "".to_string(),
    };
    let superuser = User::create_superuser(params);
    let res_str = to_string_pretty(&superuser).unwrap();

    println!("Saving User to database...");
    // Save to DB.
    repo.add_user(superuser).unwrap();
    println!("{}", res_str);
}

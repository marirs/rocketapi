use bson::{doc, ordered::OrderedDocument};
use r2d2_mongodb::mongodb::db::ThreadedDatabase;
use r2d2_mongodb::{ConnectionOptions, MongodbConnectionManager};

use super::errors::RepositoryError;
use super::prelude::UserRepository;
use crate::config::DbConfig;
use crate::models::users::User;

pub type Pool = r2d2::Pool<MongodbConnectionManager>;
pub type Conn = r2d2::PooledConnection<MongodbConnectionManager>;

/// create a connection pool of mongodb connections
/// to allow a lot of users to modify db at same time.
pub fn init_db_pool(db_config: DbConfig) -> Pool {
    //! Initialise a DB Pool for use with endponits & guard
    //! ## Example usage
    //! ```ignore
    //! init_db_pool(db_settings);
    //! ```
    // Get `mongo_host` from config.
    let mongo_addr = &db_config.mongo_host;
    // Get `mongo_port` from config
    let mongo_port = db_config.mongo_port;
    // Get `db_name` from config
    let db_name = &db_config.db_name;
    // Get `auth_user` from config
    let auth_user = db_config.auth_user.unwrap_or("".to_string());
    // Get `auth_pass` from config
    let auth_pass = db_config.auth_pass.unwrap_or("".to_string());

    // Create the MongoPoolManager
    let manager = if auth_user.clone().is_empty() && auth_pass.clone().is_empty() {
        // connect to mongo without auth credentials
        MongodbConnectionManager::new(
            ConnectionOptions::builder()
                .with_host(mongo_addr, mongo_port as u16)
                .with_db(db_name)
                .build(),
        )
    } else {
        // connect to mongo with auth credentials
        MongodbConnectionManager::new(
            ConnectionOptions::builder()
                .with_host(&mongo_addr, mongo_port as u16)
                .with_db(&db_name)
                .with_auth(&auth_user, &auth_pass)
                .build(),
        )
    };
    Pool::builder().max_size(64).build(manager).unwrap()
}

// UserMongoRepository is an implementation of UserRepository
// with MongoDB as database.
pub struct UserMongoRepository {
    client: Conn,
    collection_name: String,
}

impl UserMongoRepository {
    pub fn new(conn: Conn) -> Self {
        Self {
            client: conn,
            collection_name: "users".to_string(),
        }
    }
}

impl UserRepository for UserMongoRepository {
    fn list(&self) -> Result<Vec<User>, RepositoryError> {
        let cursor = self
            .client
            .collection(&self.collection_name)
            .find(None, None)?;
        let mut users = vec![];
        for res in cursor {
            if let Ok(item) = res {
                // Error case
                if item.contains_key("$err") {
                    return Err(RepositoryError::Default(
                        item.get_str("$err").unwrap_or("").to_string(),
                    ));
                }
                let user: User = item.into();
                users.push(user);
            }
        }
        Ok(users)
    }

    fn add_user(&self, new_user: User) -> Result<(), RepositoryError> {
        let res = self
            .client
            .collection(&self.collection_name)
            .insert_one(new_user.into(), None)?;
        if let Some(exception) = res.write_exception {
            return Err(RepositoryError::Default(exception.message));
        }
        Ok(())
    }

    fn get_user_by_api_key(&self, api_key: String) -> Result<User, RepositoryError> {
        let user_res = self.client.collection(&self.collection_name).find_one(
            Some(doc! {
                "api_key": api_key
            }),
            None,
        )?;
        if let Some(user_doc) = user_res {
            return Ok(user_doc.into());
        }
        Err(RepositoryError::NotFound("User is not found".to_string()))
    }

    fn update_user(&self, api_key: String, new_user: User) -> Result<(), RepositoryError> {
        let new_user_doc: OrderedDocument = new_user.into();
        let res = self.client.collection(&self.collection_name).update_one(
            doc! { "api_key": api_key.to_string() },
            doc! {  "$set": new_user_doc },
            None,
        )?;
        if res.matched_count == 0 {
            return Err(RepositoryError::NotFound(format!(
                "User with api_key {:?} is not found",
                api_key
            )));
        }
        if let Some(exception) = res.write_exception {
            return Err(RepositoryError::Default(exception.message));
        }
        Ok(())
    }

    fn delete_user(&self, api_key: String) -> Result<(), RepositoryError> {
        let res = self.client.collection(&self.collection_name).delete_one(
            doc! {
                "api_key": api_key.to_string()
            },
            None,
        )?;
        if res.deleted_count == 0 {
            return Err(RepositoryError::NotFound(format!(
                "User with api_key {:?} is not found",
                api_key
            )));
        }
        Ok(())
    }
}

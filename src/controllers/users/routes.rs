use super::request::AddUserParams;
use crate::db::{mongo::UserMongoRepository, prelude::*};
use crate::models::users::{CreateUserParam, UpdateUserParam, User};
use crate::server::core::guards::{db::MongoConnection, filters::FilterGuard};
use crate::server::errors::ApiError;
use crate::server::response::ApiResponse;
use rocket::{http::Status, response::status};
use rocket_contrib::json::{Json, JsonValue};
use std::net::SocketAddr;

#[get("/")]
pub fn my_info(_a: FilterGuard) -> status::Custom<JsonValue> {
    //! List information about myself
    json_response!(
        "message" => "about myself"
    )
}

#[get("/list")]
pub fn list_users(_guard: FilterGuard, pool: MongoConnection) -> Result<ApiResponse, ApiError> {
    //! List all Users
    let user_repo = UserMongoRepository::new(pool.conn);
    let users = user_repo.list()?;
    Ok(ApiResponse {
        status: Status::Ok,
        data: json!(users),
    })
}

#[post("/add", data = "<params>")]
pub fn add_user(
    guard: FilterGuard,
    client_ip: SocketAddr,
    pool: MongoConnection,
    params: Json<AddUserParams>,
) -> Result<ApiResponse, ApiError> {
    let user_repo = UserMongoRepository::new(pool.conn);
    let creation_obj = CreateUserParam {
        email: params.email.to_string(),
        created_by: guard.creator,
        created_ip: client_ip.ip().to_string(),
    };
    let user = User::create(creation_obj);
    user_repo.add_user(user.clone())?;
    Ok(ApiResponse {
        status: Status::Created,
        data: json!(user),
    })
}

#[post("/modify/<api_key>", data = "<params>")]
pub fn modify_user(
    _guard: FilterGuard,
    pool: MongoConnection,
    api_key: String,
    params: Json<UpdateUserParam>,
) -> Result<ApiResponse, ApiError> {
    let user_repo = UserMongoRepository::new(pool.conn);
    let mut user = user_repo.get_user_by_api_key(api_key.clone())?;
    user.update(params.into_inner());
    user_repo.update_user(api_key, user.clone())?;
    Ok(ApiResponse {
        status: Status::Ok,
        data: json!(user),
    })
}

#[post("/delete/<api_key>")]
pub fn delete_user(
    _guard: FilterGuard,
    pool: MongoConnection,
    api_key: String,
) -> Result<ApiResponse, ApiError> {
    let user_repo = UserMongoRepository::new(pool.conn);
    user_repo.delete_user(api_key)?;
    Ok(ApiResponse {
        status: Status::Ok,
        data: json!("User is deleted".to_string()),
    })
}

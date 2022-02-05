use crate::secure::guards::UserGuard;
use rocket::{http::Status, serde::json::Value};

#[get("/ping")]
pub fn ping(_user_guard: UserGuard) -> (Status, Value) {
    //! Hello World
    //! The First Api; you can add your new
    //! endpoint api's here
    json_response!(
        "message" => "Hello World"
    )
}

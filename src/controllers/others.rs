use rocket::response::status;
use rocket_contrib::json::JsonValue;

use crate::server::core::guards::filters::FilterGuard;

#[get("/ping")]
pub fn ping(_a: FilterGuard) -> status::Custom<JsonValue> {
    //! Hello World
    //! The First Api; you can add your new
    //! endpoint api's here
    json_response!(
        "message" => "Hello World"
    )
}

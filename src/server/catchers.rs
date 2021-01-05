use rocket::{request::Request, response::status};
use rocket_contrib::json::JsonValue;

#[catch(400)]
pub fn bad_request(req: &Request) -> status::Custom<JsonValue> {
    println!("req = {:?}", req);
    json_response!(
        400,
        "message" => "request not understood",
        "request_uri" => req.uri().to_string()
    )
}

#[catch(401)]
pub fn not_authorized(req: &Request) -> status::Custom<JsonValue> {
    json_response!(
        401,
        "message" => "not authorized",
        "request_uri" => req.uri().to_string()
    )
}

#[catch(403)]
pub fn forbidden(req: &Request) -> status::Custom<JsonValue> {
    json_response!(
        403,
        "message" => "forbidden",
        "request_uri" => req.uri().to_string()
    )
}

#[catch(422)]
pub fn unprocessed_entity(req: &Request) -> status::Custom<JsonValue> {
    json_response!(
        422,
        "message" => format!("Check your input data")
    )
}

#[catch(404)]
pub fn not_found(req: &Request) -> status::Custom<JsonValue> {
    json_response!(
        404,
        "message" => "not found",
        "request_uri" => req.uri().to_string()
    )
}

#[catch(500)]
pub fn internal_server_error(req: &Request) -> status::Custom<JsonValue> {
    json_response!(
        500,
        "message" => "internal server error",
        "request_uri" => req.uri().to_string()
    )
}

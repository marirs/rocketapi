use rocket::{
    http::{ContentType, Status},
    request::Request,
    response::{self, Responder, Response},
};
use rocket_contrib::json::JsonValue;

#[derive(Debug)]
pub struct ApiResponse {
    pub data: JsonValue,
    pub status: Status,
}

impl<'r> Responder<'r> for ApiResponse {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        Response::build_from(self.data.respond_to(&req).unwrap())
            .status(self.status)
            .header(ContentType::JSON)
            .ok()
    }
}

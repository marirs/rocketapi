use super::super::db::errors::RepositoryError;
use r2d2::Error as R2d2Error;
use rocket::http::{ContentType, Status};
use rocket::{
    request::Request,
    response::{self, Responder, Response},
};

#[derive(Debug)]
pub enum ApiError {
    BadRequest(String),
    Internal(String),
    NotFound(String),
    Unauthorized(String),
}

impl From<RepositoryError> for ApiError {
    fn from(err: RepositoryError) -> Self {
        match err {
            RepositoryError::Default(reason) => {
                return ApiError::BadRequest(reason);
            }
            RepositoryError::Internal(reason) => {
                return ApiError::Internal(reason);
            }
            RepositoryError::NotFound(reason) => {
                return ApiError::NotFound(reason);
            }
            RepositoryError::Unauthorized(reason) => {
                return ApiError::Unauthorized(reason);
            }
        }
    }
}

impl From<R2d2Error> for ApiError {
    fn from(err: R2d2Error) -> Self {
        ApiError::Internal(err.to_string())
    }
}

// Convert ApiError into Response.
// { "message": "", "status": "" }
impl<'r> Responder<'r> for ApiError {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        let (status_code, message) = match self {
            ApiError::BadRequest(val) => (Status::BadRequest, val),
            ApiError::Internal(val) => (Status::InternalServerError, val),
            ApiError::NotFound(val) => (Status::NotFound, val),
            ApiError::Unauthorized(val) => (Status::Unauthorized, val),
        };
        let data = json!({
            "message": message,
            "status": status_code.to_string(),
        });
        Response::build_from(data.respond_to(&req).unwrap())
            .status(status_code)
            .header(ContentType::JSON)
            .ok()
    }
}

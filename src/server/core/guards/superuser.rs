use crate::db::{
    errors::RepositoryError,
    mongo::{Pool, UserMongoRepository},
    prelude::*,
};

use rocket::{
    http::Status,
    request::{self, FromRequest, Request},
    Outcome, State,
};

pub struct SuperUserGuard {}

impl<'a, 'r> FromRequest<'a, 'r> for SuperUserGuard {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, ()> {
        let pool = request.guard::<State<Pool>>()?;
        let conn_res = pool.get();
        if conn_res.is_err() {
            return Outcome::Failure((Status::ServiceUnavailable, ()));
        }
        let conn = conn_res.unwrap();

        let user_repo = UserMongoRepository::new(conn);
        let api_key: String = request.headers().get("x-api-key").collect();
        let user_res = user_repo.get_user_by_api_key(api_key);
        if user_res.is_err() {
            if let RepositoryError::NotFound(_) = user_res.unwrap_err() {
                return Outcome::Failure((Status::NotFound, ()));
            }
            return Outcome::Failure((Status::ServiceUnavailable, ()));
        }
        let user = user_res.unwrap();
        if !user.super_user {
            return Outcome::Failure((Status::Unauthorized, ()));
        }
        Outcome::Success(SuperUserGuard {})
    }
}

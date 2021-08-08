use derive_more::Deref;
use rocket::{
    http::Status,
    outcome::try_outcome,
    request::{FromRequest, Outcome, Request},
    State,
};

use serde::{Deserialize, Serialize};

use super::GuardedData;
use crate::{
    db::MongodbBackend,
    error::Error,
    models::{endpoint::EndpointHandler, user::User},
};

#[derive(Serialize, Deserialize, Deref)]
pub struct UserGuard(pub GuardedData<User>);

#[derive(Serialize, Deserialize, Deref)]
pub struct AdminGuard(pub GuardedData<User>);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserGuard {
    type Error = Error;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let backend = try_outcome!(request
            .guard::<&State<MongodbBackend>>()
            .await
            .map_failure(|_| (Status::InternalServerError, Error::InternalError)));

        let handler = try_outcome!(request
            .guard::<&State<EndpointHandler>>()
            .await
            .map_failure(|_| (Status::InternalServerError, Error::InternalError)));

        match super::get_user_from_request(request, backend, handler)
            .await
            .map(Self)
        {
            Ok(guard) => Outcome::Success(guard),
            Err(e) => Outcome::Failure((e.to_status(), e)),
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AdminGuard {
    type Error = Error;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let backend = try_outcome!(request
            .guard::<&State<MongodbBackend>>()
            .await
            .map_failure(|_| (Status::InternalServerError, Error::InternalError)));

        let handler = try_outcome!(request
            .guard::<&State<EndpointHandler>>()
            .await
            .map_failure(|_| (Status::InternalServerError, Error::InternalError)));

        match super::get_user_from_request(request, backend, handler)
            .await
            .and_then(|user| {
                if user.is_admin() {
                    Ok(Self(user))
                } else {
                    Err(Error::ForbiddenAccess)
                }
            }) {
            Ok(guard) => Outcome::Success(guard),
            Err(e) => Outcome::Failure((e.to_status(), e)),
        }
    }
}

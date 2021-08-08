pub mod auth;
pub mod client;

pub use auth::{AdminGuard, UserGuard};

use derive_more::Deref;
use rocket::request::Request;
use serde::{Deserialize, Serialize};
use std::net::IpAddr;

use crate::{
    db::MongodbBackend,
    error::Error,
    models::{
        endpoint::{Endpoint, EndpointHandler, Method},
        user::User,
    },
};

#[derive(Serialize, Deserialize, Deref)]
pub struct GuardedData<T> {
    #[deref]
    pub inner: T,
    pub ip: IpAddr,
}

pub(crate) async fn get_user_from_request(
    request: &Request<'_>,
    backend: &MongodbBackend,
    handler: &EndpointHandler,
) -> Result<GuardedData<User>, Error> {
    match request
        .headers()
        .get_one("x-api-key")
        .map(|header| header.trim())
        .ok_or(Error::UnauthenticatedUser)
    {
        Ok(api_key) => backend
            .get_user_from_api_key(api_key)
            .await
            .and_then(|user| {
                let uri = request.uri().path().as_str();
                let method = Method::Http(request.method());

                match (user.get_endpoint_allowed(uri, &method), request.client_ip()) {
                    (Some(endpoint), Some(ip)) if user.is_ip_allowed(&ip.to_string()) => {
                        if handler.can_access(
                            api_key.to_string(),
                            // handle "*" by overriding these value with concrete ones
                            Endpoint {
                                name: uri.to_string(),
                                method,
                                throttle: endpoint.throttle.clone(),
                            },
                        ) {
                            Ok(GuardedData { inner: user, ip })
                        } else {
                            Err(Error::TooManyRequests)
                        }
                    }
                    _ => Err(Error::ForbiddenAccess),
                }
            }),
        Err(e) => Err(e),
    }
}

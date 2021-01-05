use rocket::{
    http::Status,
    request::{self, FromRequest, Request},
    Outcome, State,
};

use bson::{doc, ordered::OrderedDocument};

use r2d2_mongodb::mongodb::db::ThreadedDatabase;

use crate::config::DbConfig;
use crate::db::mongo::Pool;
use crate::models::users::User;
pub struct FilterGuard {
    pub creator: String,
}

/// All Filter Guards Gating
/// every Request to the API Server
impl FilterGuard {
    pub fn new(email: String) -> Self {
        Self { creator: email }
    }
}

#[derive(Debug)]
pub enum FilterGuardError {
    BadCount,
    Missing,
    Invalid,
    NotFound,
    Restricted,
}

impl<'a, 'r> FromRequest<'a, 'r> for FilterGuard {
    type Error = FilterGuardError;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        // Get the database configuration from State
        let db_config = request.guard::<State<DbConfig>>().unwrap();
        // Get `ConnectionPool` for Database from State
        let db_conn = request.guard::<State<Pool>>().unwrap().get().unwrap();

        // Mongo Collection Name
        let collection_name = db_config.collection_name.as_str();

        // Get the Request URL for `acl_deny_endpoints` check
        let request_url = request.route().unwrap().uri.path();
        // Get the client connecting Ip Address for `acl_allow_ips` check
        let ip_address = request.client_ip();
        // Get keys from x-api-key request header for api_key check
        let api_key: Vec<_> = request.headers().get("x-api-key").collect();

        // Gate the requests by checking:
        // api_key, acl_deny_endpoints, acl_allow_ips
        match api_key.len() {
            0 => Outcome::Failure((Status::BadRequest, Self::Error::Missing)),
            1 => {
                // filter
                let query_filter = doc! {"api_key": api_key[0].to_string()};
                let doc = db_conn
                    .collection(collection_name)
                    .find_one(Some(query_filter), None);
                match doc {
                    Ok(x) => match x {
                        Some(d) => {
                            if acl_deny_endpoints_filter(request_url, &d)
                                && acl_allow_ips_filter(
                                    ip_address.unwrap().to_string().as_str(),
                                    &d,
                                )
                            {
                                let user: User = d.into();
                                let state = FilterGuard::new(user.user_email);
                                Outcome::Success(state)
                            } else {
                                Outcome::Failure((Status::BadRequest, Self::Error::Restricted))
                            }
                        }
                        _ => Outcome::Failure((Status::BadRequest, Self::Error::NotFound)),
                    },
                    Err(_) => Outcome::Failure((Status::BadRequest, Self::Error::Invalid)),
                }
            }
            _ => Outcome::Failure((Status::BadRequest, Self::Error::BadCount)),
        }
    }
}

/// Check for endpoints that needs to be denied.
fn acl_deny_endpoints_filter(request_url: &str, doc: &OrderedDocument) -> bool {
    //! Deny Endpoints Guard.
    //! ## Example usage
    //!
    //! ```ignore
    //! acl_deny_endpoints_filter(request, mongo_document);
    //! ```
    let mut result = false;

    let ips = doc.get_array("acl_deny_endpoints").unwrap();
    if let Some(_) = ips.iter().find(|s| s.as_str().unwrap() == request_url) {
        result = true;
    }
    !result
}

/// Check for client Ip's that might not be in ACL for the connecting API key.
fn acl_allow_ips_filter(ip_address: &str, doc: &OrderedDocument) -> bool {
    //! Allow based on client connecting IP
    //! ## Example usage
    //!
    //! ```ignore
    //! acl_allow_ips_filter(client_ip, mongo_doc)
    //! ```
    let mut result = false;

    let ips = doc.get_array("acl_allow_ips").unwrap();
    if let Some(_) = ips.iter().find(|s| s.as_str().unwrap() == "0.0.0.0") {
        result = true;
    } else if let Some(_) = ips.iter().find(|s| s.as_str().unwrap() == ip_address) {
        result = true;
    }
    result
}

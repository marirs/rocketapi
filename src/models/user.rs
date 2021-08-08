use chrono::{offset::Utc, DateTime};
use serde::{Deserialize, Serialize};

use crate::models::endpoint::{Endpoint, Method};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct NewUser {
    pub email: String,
    pub description: String,
    pub is_admin: bool,
    pub acl_allow_ips: Vec<String>,
    pub acl_allow_endpoints: Vec<Endpoint>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub(crate) created_ip: String,
    pub(crate) created_by: String,
    pub(crate) created_at: DateTime<Utc>,
    pub(crate) email: String,
    pub(crate) description: String,
    pub(crate) api_key: String,
    pub(crate) is_admin: bool,
    pub(crate) acl_allow_ips: Vec<String>,
    pub(crate) acl_allow_endpoints: Vec<Endpoint>,
}

impl User {
    /// Check if request ip address is allowed to access from.
    ///
    /// Returns true if accessible ip is one that is stored in user's "acl_allow_ips".
    pub fn is_ip_allowed(&self, origin_ip: &str) -> bool {
        self.acl_allow_ips
            .iter()
            .any(|ip| ip.eq(origin_ip) || ip.eq("*"))
    }

    /// Get all allowed endpoints and their Throttle information.
    ///
    /// Returns Some(Endpoint) or None
    pub fn get_endpoint_allowed(
        &self,
        origin_endpoint: &str,
        origin_method: &Method,
    ) -> Option<&Endpoint> {
        // remove trailing '/'
        let origin_endpoint = origin_endpoint.strip_suffix('/').unwrap_or(origin_endpoint);

        self.acl_allow_endpoints.iter().find(|endpoint| {
            // remove trailing '/'
            let name = endpoint
                .name
                .strip_suffix('/')
                .unwrap_or_else(|| endpoint.name.as_str());

            (origin_endpoint.eq(name) || name.eq("*"))
                && (origin_method.eq(&endpoint.method) || endpoint.method.eq(&Method::Any))
        })
    }

    /// Check if the api key is admin level api key.
    ///
    /// Returns true if admin else false
    pub fn is_admin(&self) -> bool {
        self.is_admin
    }
}

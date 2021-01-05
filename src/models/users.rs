use chrono::prelude::*;
use data_encoding::HEXUPPER;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Deserialize, Debug)]
pub struct CreateUserParam {
    pub email: String,
    pub created_by: String,
    pub created_ip: String,
}

#[derive(Deserialize, Debug)]
pub struct UpdateUserParam {
    pub email: Option<String>,
    pub acl_allow_ips: Option<Vec<String>>,
    pub acl_deny_endpoints: Option<Vec<String>>,
    pub throttle: Option<usize>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub acl_allow_ips: Vec<String>,
    pub acl_deny_endpoints: Vec<String>,
    pub api_key: String,
    pub created_by: String,
    pub created_ip: String,
    pub created_on: chrono::DateTime<Utc>,
    pub super_user: bool,
    pub throttle: usize,
    pub user_email: String,
}

impl User {
    fn generate_api_key(email: &str) -> String {
        // Generate SHA256 hash of email.
        let s = Sha256::digest(email.as_bytes());
        HEXUPPER.encode(s.as_ref())
    }

    pub fn create(param: CreateUserParam) -> Self {
        Self {
            acl_allow_ips: vec![],
            acl_deny_endpoints: vec![],
            api_key: Self::generate_api_key(&param.email),
            created_by: param.created_by,
            created_ip: param.created_ip,
            created_on: Utc::now(),
            super_user: false,
            throttle: 0,
            user_email: param.email,
        }
    }

    fn update_email_if_changed(&mut self, email: &str) {
        if self.user_email != email {
            self.api_key = Self::generate_api_key(email);
            self.user_email = email.to_string();
        }
    }

    pub fn update(&mut self, params: UpdateUserParam) {
        if let Some(ref email) = params.email {
            self.update_email_if_changed(email);
        }
        if let Some(ref allowed_ips) = params.acl_allow_ips {
            self.acl_allow_ips = allowed_ips.clone();
        }
        if let Some(ref acl_deny_endpoints) = params.acl_deny_endpoints {
            self.acl_deny_endpoints = acl_deny_endpoints.clone();
        }
        if let Some(throttle) = params.throttle {
            self.throttle = throttle;
        }
    }
}

impl Eq for User {}

impl PartialEq for User {
    fn eq(&self, other: &User) -> bool {
        // Compare all except for api key
        // and creation attributes
        self.acl_allow_ips == other.acl_allow_ips
            && self.acl_deny_endpoints == other.acl_deny_endpoints
            && self.user_email == other.user_email
            && self.super_user == other.super_user
            && self.throttle == other.throttle
    }
}

#[cfg(test)]
mod tests {
    use super::{CreateUserParam, UpdateUserParam, User};

    #[test]
    fn test_create_user() {
        let param = CreateUserParam {
            email: "user@abc.com".to_string(),
            created_by: "server".to_string(),
            created_ip: "127.0.0.0".to_string(),
        };
        let user = User::create(param);
        let ips: Vec<String> = vec![];
        let deny_endpoints: Vec<String> = vec![];
        assert_eq!(user.acl_allow_ips, ips);
        assert_eq!(user.acl_deny_endpoints, deny_endpoints);
        assert_eq!(user.super_user, false);
        assert_eq!(user.throttle, 0);
        assert_eq!(user.user_email, "user@abc.com".to_string());
        assert_eq!(
            user.api_key,
            "1BED8E26EB4257227A7163089FDDB50949F973E394C4447BDCB15939398982FC".to_string()
        );
    }

    #[test]
    fn test_update_user() {
        let param = CreateUserParam {
            email: "user@abc.com".to_string(),
            created_by: "server".to_string(),
            created_ip: "127.0.0.0".to_string(),
        };
        let mut user = User::create(param);

        let updates = UpdateUserParam {
            email: Some("user123@abc.com".to_string()),
            acl_allow_ips: Some(vec!["127.0.0.1".to_string()]),
            acl_deny_endpoints: None,
            throttle: Some(3usize),
        };
        user.update(updates);
        assert_eq!(user.user_email, "user123@abc.com".to_string());
        assert_eq!(
            user.api_key,
            "7C91FBAAA4C7E56B187CBAAD75FBF9342ED2314CED8E80BB74F85B7E3FD6B967".to_string()
        );
        assert_eq!(user.acl_allow_ips, vec!["127.0.0.1".to_string()]);
        assert_eq!(user.throttle, 3);
    }
}

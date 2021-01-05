use crate::models::users::User;
use bson::{ordered::OrderedDocument, Bson};
use chrono::prelude::*;

impl From<OrderedDocument> for User {
    fn from(doc: OrderedDocument) -> Self {
        let arr: Vec<Bson> = vec![];
        Self {
            acl_allow_ips: doc
                .get_array("acl_allow_ips")
                .unwrap_or(&arr)
                .iter()
                .map(|doc| {
                    if let Bson::String(val) = doc {
                        return val.to_string();
                    } else {
                        return "".to_string();
                    }
                })
                .collect(),
            acl_deny_endpoints: doc
                .get_array("acl_deny_endpoints")
                .unwrap_or(&arr)
                .iter()
                .map(|doc| {
                    if let Bson::String(val) = doc {
                        return val.to_string();
                    } else {
                        return "".to_string();
                    }
                })
                .collect(),
            api_key: doc.get_str("api_key").unwrap_or("").to_string(),
            created_by: doc.get_str("created_by").unwrap_or("").to_string(),
            created_ip: doc.get_str("created_ip").unwrap_or("").to_string(),
            throttle: doc.get_i32("throttle").unwrap_or(0) as usize,
            super_user: doc.get_bool("super_user").unwrap_or(false),
            created_on: *doc
                .get_utc_datetime("created_on")
                .unwrap_or(&Utc.ymd(2020, 1, 1).and_hms(0, 0, 0)),
            user_email: doc.get_str("user_email").unwrap_or("").to_string(),
        }
    }
}

impl From<User> for OrderedDocument {
    fn from(user: User) -> Self {
        let mut doc = OrderedDocument::new();
        doc.insert(
            "acl_allow_ips",
            Bson::Array(
                user.acl_allow_ips
                    .iter()
                    .map(|x| Bson::String(x.to_string()))
                    .collect(),
            ),
        );
        doc.insert(
            "acl_deny_endpoints",
            Bson::Array(
                user.acl_deny_endpoints
                    .iter()
                    .map(|x| Bson::String(x.to_string()))
                    .collect(),
            ),
        );
        doc.insert("api_key", Bson::String(user.api_key));
        doc.insert("created_by", Bson::String(user.created_by));
        doc.insert("created_on", Bson::UtcDatetime(user.created_on));
        doc.insert("created_ip", Bson::String(user.created_ip));
        doc.insert("user_email", Bson::String(user.user_email));
        doc.insert("throttle", Bson::I32(user.throttle as i32));
        doc.insert("super_user", Bson::Boolean(user.super_user));
        doc
    }
}

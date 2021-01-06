use rocket::{
    http::Status,
    request::{self, FromRequest, Request},
    Outcome, State,
};
use std::net::IpAddr;

#[derive(Debug)]
pub struct ClientInfo {
    pub ip: String,
    pub user_agent: String,
}

impl ClientInfo {
    pub fn new(ip: String, user_agent: String) -> Self {
        Self { ip, user_agent }
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for ClientInfo {
    type Error = ();
    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let ua = request.headers().get("user-agent").next().unwrap();
        let browser_info = if ua.is_empty() {
            "".to_string()
        } else {
            ua.to_string()
        };

        match request.real_ip() {
            Some(ip) => {
                let state = ClientInfo::new(ip.to_string(), browser_info);
                Outcome::Success(state)
            }
            None => match request.headers().get("x-forwarded-for").next() {
                Some(forwarded_ip) => {
                    let forwarded_ip = forwarded_ip.split(",").next().unwrap();
                    let state = ClientInfo::new(forwarded_ip.to_string(), browser_info);
                    Outcome::Success(state)
                }
                None => {
                    let ip = request.client_ip().unwrap();
                    let state = ClientInfo::new(ip.to_string(), browser_info);
                    Outcome::Success(state)
                }
            },
        }
    }
}

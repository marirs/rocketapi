use crate::db::mongo::{Conn, Pool};
use rocket::http::Status;
use rocket::{
    request::{self, FromRequest, Request},
    Outcome, State,
};

pub struct MongoConnection {
    pub conn: Conn,
}

impl<'a, 'r> FromRequest<'a, 'r> for MongoConnection {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<MongoConnection, ()> {
        let pool = request.guard::<State<Pool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(MongoConnection { conn }),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

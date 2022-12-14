#[macro_use]
extern crate rocket;

use crate::routes::balance::*;
use crate::routes::send::*;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::Request;
use std::fs::read_to_string;
use std::sync::atomic::AtomicI32;

pub(crate) mod routes;

pub(crate) struct Balances {
    pub(crate) me: AtomicI32,
    pub(crate) bob: AtomicI32,
}

pub(crate) struct Flag {
    pub(crate) flag: String,
}

#[derive(Debug)]
pub(crate) struct Authorization {
    value: String,
}

impl Authorization {
    fn new<S: Into<String>>(authorization: S) -> Self {
        Authorization {
            value: authorization.into(),
        }
    }
}

#[async_trait]
impl<'r> FromRequest<'r> for Authorization {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let authorization: Vec<_> = request.headers().get("Authorization").collect();

        match authorization.len() {
            0 => Outcome::Forward(()),
            1 => Outcome::Success(Authorization::new(authorization[0])),
            _ => Outcome::Failure((Status::BadRequest, ())),
        }
    }
}

#[launch]
fn rocket() -> _ {
    let flag = read_to_string("/tmp/flags/flag.txt").unwrap_or("haxagon{1000000}".into());

    let rocket = rocket::build()
        .manage(Balances {
            me: AtomicI32::new(10),
            bob: AtomicI32::new(1000000),
        })
        .manage(Flag { flag })
        .mount("/", routes![get_balance])
        .mount("/", routes![send]);
    println!("SCENARIO_IS_READY");
    rocket
}

#[cfg(feature = "")]
fn main() {
    println!("Hello, world!");
}

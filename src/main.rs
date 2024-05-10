extern crate futures;
extern crate hyper;

pub mod connect_to_db;
pub mod get_handlers;
pub mod models;
pub mod post_handlers;
pub mod schema;

#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate log;
extern crate env_logger;

use connect_to_db::connect_to_db;
use futures::{future, future::Future, Stream};
use hyper::{
    server::{Request, Response, Service},
    Method::{Get, Post},
    StatusCode,
};

use post_handlers::make_error_response;

use crate::post_handlers::{make_post_response, parse_form, write_to_db};

struct Microservice;
impl Service for Microservice {
    type Response = Response;
    type Request = Request;
    type Error = hyper::Error;
    type Future = Box<dyn Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        let db_conn = match connect_to_db() {
            Some(conn ) =>conn,
            None => return Box::new(future::ok(
                Response::new().with_status(StatusCode::InternalServerError),
            )),
        };

        match (req.method(), req.path()) {
            (&Post, "/") => {
                info!("Received a POST request to '/'");
                let future = req
                    .body()
                    .concat2()
                    .and_then(parse_form)
                    .and_then(move |new_message| write_to_db(new_message, &mut db_conn))
                    .then(make_post_response);
                Box::new(future)
            }
            (&Get, "/") => {
                let time_range = match req.query() {
                    Some(query) => parse_query(query),
                    None => Ok(TimeRanage {
                        before: None,
                        after: None,
                    }),
                };
                let response = match time_range {
                    Ok(time_range) => make_get_response(query_db(time_range)),
                    Err(err) => make_error_response(err),
                };
                Box::new(response)
            }
            _ => Box::new(future::ok(
                Response::new().with_status(StatusCode::NotFound),
            )),
        }
    }
}

fn main() {
    env_logger::init();
    let addr = "127.0.0.1:8080".parse().unwrap();
    let server = hyper::server::Http::new()
        .bind(&addr, || Ok(Microservice {}))
        .unwrap();
    info!("Starting server on http://{}", addr);
    server.run().unwrap();
}

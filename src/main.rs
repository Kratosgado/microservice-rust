extern crate futures;
extern crate hyper;

pub mod request_handlers;

#[macro_use]
extern crate log;
extern crate env_logger;

use futures::{future, future::Future, Stream};
use hyper::{
    server::{Request, Response, Service},
    Method::Post,
    StatusCode,
};

use crate::request_handlers::{make_post_response, parse_form, write_to_db};

struct Microservice;
impl Service for Microservice {
    type Response = Response;
    type Request = Request;
    type Error = hyper::Error;
    type Future = Box<dyn Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        match (req.method(), req.path()) {
            (&Post, "/") => {
                info!("Received a POST request to '/'");
                let future = req
                    .body()
                    .concat2()
                    .and_then(parse_form)
                    .and_then(write_to_db)
                    .then(make_post_response);
                Box::new(future)
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

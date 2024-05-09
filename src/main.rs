extern crate hyper;
extern crate futures;

#[macro_use]
extern crate log;
extern crate env_logger;

use hyper::server::{ Request, Response, Service};
use futures::future::Future;

struct Microservice;
impl Service for Microservice {
    type Response = Response;
    type Request = Request;
    type Error = hyper::Error;
    type Future = Box<dyn Future<Item=Self::Response, Error=Self::Error>>;
    
    fn call(&self, req: Request) -> Self::Future {
        info!("Microservice got a request: {:?}", req);
        Box::new(futures::future::ok(Response::new()))
    }
    
}

fn main() {
    env_logger::init();
    let addr = "127.0.0.1:8080".parse().unwrap();
    let server = hyper::server::Http::new()
        .bind(&addr, || Ok(Microservice {})).unwrap();
    info!("Starting server on http://{}", addr);
    server.run().unwrap();
}

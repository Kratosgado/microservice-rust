use std::collections::HashMap;
use std::io;

use futures::future;
use futures::future::FutureResult;
use hyper::Chunk;

pub struct NewMessage {
    pub username: String,
    pub message: String,
}

pub fn parse_form(form_chunk: Chunk) -> FutureResult<NewMessage, hyper::Error> {
    let mut form = url::form_urlencoded::parse(&form_chunk)
        .into_owned()
        .collect::<HashMap<String, String>>();
    if let Some(message) = form.remove("message") {
        let username = form.remove("username").unwrap_or(String::from("anonymous"));

        future::ok(NewMessage {
            username: String::new(),
            message: String::new(),
        })
    } else {
        future::err(hyper::Error::from(io::Error::new(io::ErrorKind::InvalidInput, "Missing field 'message'")))
    }
}

pub fn write_to_db(_entry: NewMessage) -> FutureResult<i64, hyper::Error> {
    future::ok(0)
}

pub fn make_post_response(
    _result: Result<i64, hyper::Error>,
) -> FutureResult<hyper::Response, hyper::Error> {
    future::ok(hyper::Response::new().with_status(hyper::StatusCode::NotFound))
}

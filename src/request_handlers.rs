use futures::future::FutureResult;
use hyper::Chunk;
use futures::future;



pub struct NewMessage {
    pub username: String,
    pub message: String,
}

pub fn parse_form(_form_chunk: Chunk) -> FutureResult<NewMessage, hyper::Error> {
    future::ok(NewMessage {
        username: String::new(),
        message: String::new(),
    })
}

pub fn write_to_db(_entry: NewMessage) -> FutureResult<i64, hyper::Error> {
    future::ok(0)
}

pub fn make_post_response(_result: Result<i64, hyper::Error>) -> FutureResult<hyper::Response, hyper::Error> {
    future::ok(hyper::Response::new().with_status(hyper::StatusCode::NotFound))
}
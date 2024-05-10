use std::collections::HashMap;
use std::io;

use crate::schema::messages;
use diesel::prelude::*;
use diesel::PgConnection;
use futures::future;
use futures::future::FutureResult;
use hyper::{
    header::{ContentLength, ContentType},
    Chunk, Response,
};

#[derive(Insertable)]
#[diesel(table_name = messages)]
#[diesel(check_for_backend(diesel::pg::Pg))]
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

        future::ok(NewMessage { username, message })
    } else {
        future::err(hyper::Error::from(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Missing field 'message'",
        )))
    }
}

pub fn write_to_db(
    new_message: NewMessage,
    db_conn: &mut PgConnection,
) -> FutureResult<i64, hyper::Error> {
    let timestamp = diesel::insert_into(messages::table)
        .values(&new_message)
        .returning(messages::timestamp)
        .get_result::<i64>(db_conn);
    
    match timestamp {
        Ok(timestamp) => future::ok(timestamp),
        Err(err) => future::err(hyper::Error::from(io::Error::new(
            io::ErrorKind::Other,
            err.to_string(),
        ))),
    }
}

pub fn make_post_response(
    _result: Result<i64, hyper::Error>,
) -> FutureResult<hyper::Response, hyper::Error> {
    match _result {
        Ok(timestamp) => {
            let payload = serde_json::json!({
                "timestamp": timestamp
            })
            .to_string();
            let response = Response::new()
                .with_header(ContentLength(payload.len() as u64))
                .with_header(ContentType::json())
                .with_body(payload);
            debug!("{:?}", response);
            future::ok(response)
        }
        Err(err) => make_error_response(err.to_string().as_str()),
    }
}

pub fn make_error_response(error: &str) -> FutureResult<hyper::Response, hyper::Error> {
    let payload = serde_json::json!({"error": error}).to_string();
    let response = Response::new()
        .with_status(hyper::StatusCode::InternalServerError)
        .with_header(ContentLength(payload.len() as u64))
        .with_header(ContentType::json())
        .with_body(payload);
    debug!("{:?}", response);
    future::ok(response)
}

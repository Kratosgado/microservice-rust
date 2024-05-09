use std::{collections::HashMap};
use futures::future::{self, FutureResult};
use hyper::{header::ContentLength, Response};
use serde::Serialize;

pub struct TimeRange {
    pub before: Option<i64>,
    pub after: Option<i64>,
}
#[derive(Serialize, Debug)]
pub struct  Message {
    pub id: i32,
    pub username: String,
    pub message: String,
    pub timestamp: i64,
}
pub fn parse_query(query: &str) -> Result<TimeRange, String> {
    let args = url::form_urlencoded::parse(&query.as_bytes())
        .into_owned().collect::<HashMap<String, String>>();

    let before = args.get("before").map(|value| value.parse::<i64>());
    if let Some(  result ) = &before {
        if let Err(ref error ) = *result {
            return Err(format!("Error parsing 'before': {}", error));
        }
    }

    let after = args.get("after").map(|value| value.parse::<i64>());
    if let Some( ref result ) = after {
        if let Err(ref error ) = *result {
            return Err(format!("Error parsing 'after': {}", error));
        }
    }
    Ok(TimeRange {
        before: before.map(|b| b.unwrap()),
        after: after.map(|a| a.unwrap()),
    })
}

pub fn make_get_response( messages: Option<Vec<Message>>) -> FutureResult<hyper::Response, hyper::Error> {
    let response = match messages {
        Some(messages ) => {
            let body = render_page(messages);
            Response::new()
                .with_header(ContentLength(body.len() as u64))
                .with_body(body)
        },
        None => Response::new().with_status(hyper::StatusCode::InternalServerError),
    };
    debug!("{:?}", response);
    future::ok(response)
}
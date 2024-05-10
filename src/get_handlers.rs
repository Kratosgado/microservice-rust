use std::{collections::HashMap};
use diesel::prelude::*;
use futures::future::{self, FutureResult};
use hyper::{header::ContentLength, Response};
use serde::Serialize;

use crate::models::Message;

pub struct TimeRange {
    pub before: Option<i64>,
    pub after: Option<i64>,
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

pub fn query_db(time_range: TimeRange, db_conn: &mut PgConnection) -> Option<Vec<Message>> {
    use crate::schema::messages;
    let TimeRange { before, after } = time_range;
    let query_result = match (before, after ) {
      (Some(before), Some(after )) => {
        messages::table
            .filter(messages::timestamp.lt(before as i64))  
            .filter(messages::timestamp.gt(after as i64))
            .load::<Message>(db_conn)
      }
      (Some(before), _) => {
        messages::table
            .filter(messages::timestamp.lt(before as i64))
            .load::<Message>(db_conn)
    }
    (_, Some(after)) => {
        messages::table
            .filter(messages::timestamp.gt(after as i64))
            .load::<Message>(db_conn)
    }
    _ => messages::table.load::<Message>(db_conn),
    };
    match query_result {
        Ok(messages) => Some(messages),
        Err(err) => {
            error!("Error querying database: {}", err);
            None
        }
    }
}
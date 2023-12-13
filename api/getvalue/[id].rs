use serde::Serialize;
use serde_json::{json, Value};
use std::collections::HashMap;
use url::Url;
use listallfrompscale::{getfromquickfetch};

use vercel_runtime::{http::bad_request, run, Body, Error, Request, Response, StatusCode};

#[derive(Serialize)]
pub struct APIError {
    pub message: &'static str,
    pub code: &'static str,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    let parsed_url = Url::parse(&req.uri().to_string()).unwrap();
    let hash_query: HashMap<String, String> = parsed_url.query_pairs().into_owned().collect();
    let uid = hash_query.get("id");

    match uid {
        None => {
            return bad_request(APIError {
                message: "Query string is invalid",
                code: "query_string_invalid",
            });
        }
        Some(id) => {
            let data=getfromquickfetch(id.to_string()).unwrap().value;    
            let jdata:Value=serde_json::from_str(&data).unwrap();
            Ok(Response::builder()
            .status(StatusCode::OK)
            .header("Content-Type", "application/json")
            // .body(Body::Text(id.to_owned()))?),
            .body(
                json!({
                // "got": format!("added {} to {}!", payload.uid, payload.datatoadd),
                "data": serde_json::to_string(&jdata).unwrap(),
                "request":format!("{:?}",req),
                })
                .to_string()
                .into(),
            )?)
        }
    }
}

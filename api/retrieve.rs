use serde::Serialize;
use serde_json::{json, Value};
use std::collections::HashMap;
use url::Url;
use listallfrompscale::getfromquickfetch;
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
    let id = hash_query.get("id");

    match id {
        None => {
            return bad_request(APIError {
                message: "Query string is invalid. 'id' parameter is required.",
                code: "query_string_invalid",
            });
        }
        Some(id) => {
            match getfromquickfetch(id.to_string()) {
                Ok(entry) => {
                    // The value in Redis is stored as a string, which might be a JSON string.
                    // We try to parse it as JSON to return a proper JSON object, 
                    // otherwise we return it as a string.
                    let value_json: Value = match serde_json::from_str(&entry.value) {
                        Ok(v) => v,
                        Err(_) => Value::String(entry.value),
                    };

                    Ok(Response::builder()
                        .status(StatusCode::OK)
                        .header("Content-Type", "application/json")
                        .body(
                            json!({
                                "success": true,
                                "id": id,
                                "data": value_json
                            })
                            .to_string()
                            .into(),
                        )?)
                },
                Err(_) => {
                    Ok(Response::builder()
                        .status(StatusCode::NOT_FOUND)
                        .header("Content-Type", "application/json")
                        .body(
                            json!({
                                "success": false,
                                "message": "Data not found"
                            })
                            .to_string()
                            .into(),
                        )?)
                }
            }
        }
    }
}

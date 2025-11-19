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
    // ────────────────────── CORS Preflight ──────────────────────
    if req.method() == "OPTIONS" {
        return Ok(Response::builder()
            .status(StatusCode::OK)
            .header("Access-Control-Allow-Origin", "*")
            .header("Access-Control-Allow-Methods", "GET, OPTIONS")
            .header("Access-Control-Allow-Headers", "*")
            .header("Access-Control-Max-Age", "86400")
            .body(Body::Empty)?);
    }

    // Only allow GET for this endpoint
    if req.method() != "GET" {
        return Ok(Response::builder()
            .status(StatusCode::METHOD_NOT_ALLOWED)
            .header("Content-Type", "application/json")
            .header("Access-Control-Allow-Origin", "*")
            .header("Access-Control-Allow-Methods", "GET, OPTIONS")
            .header("Access-Control-Allow-Headers", "*")
            .header("Access-Control-Max-Age", "86400")
            .body(json!({ "error": "Method not allowed" }).to_string().into())?);
    }

    // ────────────────────── Normal logic ──────────────────────
    let parsed_url = Url::parse(&req.uri().to_string()).unwrap();
    let hash_query: HashMap<String, String> = parsed_url.query_pairs().into_owned().collect();
    let id = hash_query.get("id");

    match id {
        None => {
            let mut resp = bad_request(APIError {
                message: "Query string is invalid. 'id' parameter is required.",
                code: "query_string_invalid",
            })?;
            resp.headers_mut().insert("Access-Control-Allow-Origin", "*".parse().unwrap());
            resp.headers_mut().insert("Access-Control-Allow-Methods", "GET, OPTIONS".parse().unwrap());
            resp.headers_mut().insert("Access-Control-Allow-Headers", "*".parse().unwrap());
            resp.headers_mut().insert("Access-Control-Max-Age", "86400".parse().unwrap());
            Ok(resp)
        },
        Some(id) => match getfromquickfetch(id.to_string()) {
            Ok(entry) => {
                let value_json: Value = match serde_json::from_str(&entry.value) {
                    Ok(v) => v,
                    Err(_) => Value::String(entry.value),
                };

                Ok(Response::builder()
                    .status(StatusCode::OK)
                    .header("Content-Type", "application/json")
                    .header("Access-Control-Allow-Origin", "*")
                    .header("Access-Control-Allow-Methods", "GET, OPTIONS")
                    .header("Access-Control-Allow-Headers", "*")
                    .header("Access-Control-Max-Age", "86400")
                    .body(
                        json!({
                            "success": true,
                            "id": id,
                            "data": value_json
                        })
                        .to_string()
                        .into(),
                    )?)
            }
            Err(_) => Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .header("Content-Type", "application/json")
                .header("Access-Control-Allow-Origin", "*")
                .header("Access-Control-Allow-Methods", "GET, OPTIONS")
                .header("Access-Control-Allow-Headers", "*")
                .header("Access-Control-Max-Age", "86400")
                .body(
                    json!({
                        "success": false,
                        "message": "Data not found"
                    })
                    .to_string()
                    .into(),
                )?),
        },
    }
}
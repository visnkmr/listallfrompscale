use std::env;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::json;
use listallfrompscale::addtoquickfetch;
use vercel_runtime::{
    http::bad_request, process_request, process_response, run_service, service_fn, Body, Error,
    Request, RequestPayloadExt, Response, ServiceBuilder, StatusCode,
};

#[derive(Debug, Serialize, Deserialize)]
struct Payload {
    id: String,
    value: String,
}

#[derive(Serialize)]
pub struct APIError {
    pub message: &'static str,
    pub code: &'static str,
}

// Helper to add CORS headers to any response
fn with_cors_headers(resp: Response<Body>) -> Response<Body> {
    resp.header("Access-Control-Allow-Origin", "*")
        .header("Access-Control-Allow-Methods", "GET, POST, OPTIONS")
        .header("Access-Control-Allow-Headers", "*")
        .header("Access-Control-Max-Age", "86400")
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::ERROR)
        .with_target(false)
        .init();

    let handler = ServiceBuilder::new()
        .map_request(process_request)
        .map_response(process_response)
        .service(service_fn(handler));

    run_service(handler).await
}

pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    // ────────────────────── CORS Preflight ──────────────────────
    if req.method() == Method::OPTIONS {
        return Ok(Response::builder()
            .status(StatusCode::OK)
            .header("Access-Control-Allow-Origin", "*")
            .header("Access-Control-Allow-Methods", "GET, POST, OPTIONS")
            .header("Access-Control-Allow-Headers", "*")
            .header("Access-Control-Max-Age", "86400")
            .body(Body::Empty)?);
    }

    // ────────────────────── Main logic ──────────────────────
    if req.method() != Method::POST {
        let mut resp = Response::builder()
            .status(StatusCode::METHOD_NOT_ALLOWED)
            .header("Content-Type", "application/json")
            .body(
                json!({ "error": "Method not allowed" }).to_string().into(),
            )?;
        return Ok(with_cors_headers(resp));
    }

    let payload = req.payload::<Payload>();

    let response = match payload {
        Err(_) => bad_request(APIError {
            message: "Invalid payload",
            code: "invalid_payload",
        }),
        Ok(None) => bad_request(APIError {
            message: "No payload",
            code: "no_payload",
        }),
        Ok(Some(payload)) => match addtoquickfetch(payload.id.clone(), payload.value.clone()) {
            Ok(_) => Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "application/json")
                .body(
                    json!({
                        "success": true,
                        "message": "Data saved successfully",
                        "id": payload.id
                    })
                    .to_string()
                    .into(),
                )?,
            Err(_) => Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .header("Content-Type", "application/json")
                .body(
                    json!({
                        "success": false,
                        "message": "Failed to save data"
                    })
                    .to_string()
                    .into(),
                )?,
        },
    };

    // Add CORS headers to every successful/error response
    Ok(with_cors_headers(response))
}
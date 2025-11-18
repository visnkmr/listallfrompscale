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
    if req.method() != Method::POST {
        return Ok(Response::builder()
            .status(StatusCode::METHOD_NOT_ALLOWED)
            .header("Content-Type", "application/json")
            .body(
                json!({
                    "error": "Method not allowed"
                })
                .to_string()
                .into(),
            )?);
    }

    let payload = req.payload::<Payload>();

    match payload {
        Err(..) => bad_request(APIError {
            message: "Invalid payload",
            code: "invalid_payload",
        }),
        Ok(None) => bad_request(APIError {
            message: "No payload",
            code: "no_payload",
        }),
        Ok(Some(payload)) => {
            match addtoquickfetch(payload.id.clone(), payload.value.clone()) {
                Ok(_) => {
                    Ok(Response::builder()
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
                        )?)
                },
                Err(_) => {
                    Ok(Response::builder()
                        .status(StatusCode::INTERNAL_SERVER_ERROR)
                        .header("Content-Type", "application/json")
                        .body(
                            json!({
                                "success": false,
                                "message": "Failed to save data"
                            })
                            .to_string()
                            .into(),
                        )?)
                },
            }
        }
    }
}

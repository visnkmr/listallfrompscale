use serde::{Deserialize, Serialize};
use serde_json::json;
use std::str;
use listallfrompscale::{validate_jwt_token, extract_bearer_token, add_cors_headers, handle_options, adddatatouser};

use vercel_runtime::{http::bad_request, http::unauthorized, run, Body, Error, Request, Response, StatusCode};

#[derive(Debug, Deserialize)]
struct UpdatePayload {
    datatoadd: String,
}

#[derive(Serialize)]
pub struct APIError {
    pub message: &'static str,
    pub code: &'static str,
}

#[derive(Serialize)]
pub struct UpdateResponse {
    pub got: bool,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    // Handle OPTIONS preflight request
    if req.method() == "OPTIONS" {
        return Ok(handle_options());
    }

    // Extract and validate JWT token
    let auth_header = req.headers().get("Authorization");
    let auth_header_str = auth_header.and_then(|h| h.to_str().ok());
    let token = match extract_bearer_token(auth_header_str) {
        Some(token) => token,
        None => {
            let mut response = Response::builder()
                .status(StatusCode::UNAUTHORIZED)
                .header("Content-Type", "application/json")
                .body(Body::Text(json!({
                    "error": {
                        "message": "Missing or invalid Authorization header",
                        "code": "missing_auth"
                    }
                }).to_string()))?;
            
            response.headers_mut().insert("Access-Control-Allow-Origin", "*").unwrap();
            response.headers_mut().insert("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE, PATCH, OPTIONS").unwrap();
            response.headers_mut().insert("Access-Control-Allow-Headers", "Content-Type, Authorization").unwrap();
            
            return Ok(response);
        }
    };

    // Validate JWT token
    let claims = match validate_jwt_token(&token) {
        Ok(claims) => claims,
        Err(_) => {
            let mut response = Response::builder()
                .status(StatusCode::UNAUTHORIZED)
                .header("Content-Type", "application/json")
                .body(Body::Text(json!({
                    "error": {
                        "message": "Invalid or expired JWT token",
                        "code": "invalid_token"
                    }
                }).to_string()))?;
            
            response.headers_mut().insert("Access-Control-Allow-Origin", "*").unwrap();
            response.headers_mut().insert("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE, PATCH, OPTIONS").unwrap();
            response.headers_mut().insert("Access-Control-Allow-Headers", "Content-Type, Authorization").unwrap();
            
            return Ok(response);
        }
    };

    match req.method() {
        "POST" => {
            // Read request body
            let body_str = match &req.into_body() {
                Body::Text(text) => text.clone(),
                Body::Empty => String::new(),
                _ => {
                    let mut response = Response::builder()
                        .status(StatusCode::BAD_REQUEST)
                        .header("Content-Type", "application/json")
                        .body(Body::Text(json!({
                            "error": {
                                "message": "Failed to read request body",
                                "code": "body_read_error"
                            }
                        }).to_string()))?;
                    
                    response.headers_mut().insert("Access-Control-Allow-Origin", "*").unwrap();
                    response.headers_mut().insert("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE, PATCH, OPTIONS").unwrap();
                    response.headers_mut().insert("Access-Control-Allow-Headers", "Content-Type, Authorization").unwrap();
                    
                    return Ok(response);
                }
            };
            
            // Parse form data
            let mut datatoadd = String::new();
            for pair in body_str.split('&') {
                let mut parts = pair.splitn(2, '=');
                if let (Some(key), Some(value)) = (parts.next(), parts.next()) {
                    if key == "datatoadd" {
                        match urlencoding::decode(value) {
                            Ok(decoded) => datatoadd = decoded,
                            Err(_) => {
                                let mut response = Response::builder()
                                    .status(StatusCode::BAD_REQUEST)
                                    .header("Content-Type", "application/json")
                                    .body(Body::Text(json!({
                                        "error": {
                                            "message": "Failed to decode URL",
                                            "code": "url_decode_error"
                                        }
                                    }).to_string()))?;
                                
                                response.headers_mut().insert("Access-Control-Allow-Origin", "*").unwrap();
                                response.headers_mut().insert("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE, PATCH, OPTIONS").unwrap();
                                response.headers_mut().insert("Access-Control-Allow-Headers", "Content-Type, Authorization").unwrap();
                                
                                return Ok(response);
                            }
                        }
                    }
                }
            }

            if datatoadd.is_empty() {
                let mut response = Response::builder()
                    .status(StatusCode::BAD_REQUEST)
                    .header("Content-Type", "application/json")
                    .body(Body::Text(json!({
                        "error": {
                            "message": "Missing datatoadd field",
                            "code": "missing_data"
                        }
                    }).to_string()))?;
                
                response.headers_mut().insert("Access-Control-Allow-Origin", "*").unwrap();
                response.headers_mut().insert("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE, PATCH, OPTIONS").unwrap();
                response.headers_mut().insert("Access-Control-Allow-Headers", "Content-Type, Authorization").unwrap();
                
                return Ok(response);
            }

            // Use user ID from JWT token
            let user_id = claims.sub;
            
            // Add data to user
            match adddatatouser(&user_id, &datatoadd) {
                Ok(_) => {
                    let response = json!(UpdateResponse { got: true });
                    
                    let mut response = Response::builder()
                        .status(StatusCode::OK)
                        .header("Content-Type", "application/json")
                        .body(Body::Text(response.to_string()))?;
                    
                    response.headers_mut().insert("Access-Control-Allow-Origin", "*").unwrap();
                    response.headers_mut().insert("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE, PATCH, OPTIONS").unwrap();
                    response.headers_mut().insert("Access-Control-Allow-Headers", "Content-Type, Authorization").unwrap();
                    
                    Ok(response)
                }
                Err(_) => {
                    let response = json!(UpdateResponse { got: false });
                    
                    let mut response = Response::builder()
                        .status(StatusCode::INTERNAL_SERVER_ERROR)
                        .header("Content-Type", "application/json")
                        .body(Body::Text(response.to_string()))?;
                    
                    response.headers_mut().insert("Access-Control-Allow-Origin", "*").unwrap();
                    response.headers_mut().insert("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE, PATCH, OPTIONS").unwrap();
                    response.headers_mut().insert("Access-Control-Allow-Headers", "Content-Type, Authorization").unwrap();
                    
                    Ok(response)
                }
            }
        }
        _ => {
            let mut response = Response::builder()
                .status(StatusCode::METHOD_NOT_ALLOWED)
                .header("Content-Type", "application/json")
                .body(Body::Text(json!({
                    "error": {
                        "message": "Method not allowed",
                        "code": "method_not_allowed"
                    }
                }).to_string()))?;
            
            response.headers_mut().insert("Access-Control-Allow-Origin", "*").unwrap();
            response.headers_mut().insert("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE, PATCH, OPTIONS").unwrap();
            response.headers_mut().insert("Access-Control-Allow-Headers", "Content-Type, Authorization").unwrap();
            
            Ok(response)
        }
    }
}
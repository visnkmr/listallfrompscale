use serde::{Serialize, Deserialize};
use serde_json::json;
use listallfrompscale::{validate_jwt_token, extract_bearer_token, add_cors_headers, handle_options, printdata};

use vercel_runtime::{http::bad_request, http::unauthorized, run, Body, Error, Request, Response, StatusCode};

#[derive(Serialize)]
pub struct APIError {
    pub message: &'static str,
    pub code: &'static str,
}

#[derive(Serialize)]
pub struct UserResponse {
    pub data: String, // Frontend expects stringified JSON
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

    let parsed_url = url::Url::parse(&req.uri().to_string()).unwrap();
    let path_segments: Vec<&str> = parsed_url.path_segments().unwrap_or_default().collect();
    
    // Extract username from URL path /api/user/{username}
    let username = if path_segments.len() >= 3 && path_segments[0] == "api" && path_segments[1] == "user" {
        path_segments[2]
    } else {
        let error_response = json!({
            "error": {
                "message": "Invalid URL path",
                "code": "invalid_path"
            }
        });
        let mut response = Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .header("Content-Type", "application/json")
            .body(Body::Text(error_response.to_string()))?;
        
        // Add CORS headers manually
        response.headers_mut().insert("Access-Control-Allow-Origin", "*").unwrap();
        response.headers_mut().insert("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE, PATCH, OPTIONS").unwrap();
        response.headers_mut().insert("Access-Control-Allow-Headers", "Content-Type, Authorization").unwrap();
        
        return Ok(response);
    };

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
    let _claims = match validate_jwt_token(&token) {
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
            // Get user data for the requested username
            match printdata() {
                Ok(data_string) => {
                    // Frontend expects { "data": "stringified_json_array" }
                    let response = json!({
                        "data": data_string
                    });
                    
                    let mut response = Response::builder()
                        .status(StatusCode::OK)
                        .header("Content-Type", "application/json")
                        .body(Body::Text(response.to_string()))?;
                    
                    // Add CORS headers manually
                    response.headers_mut().insert("Access-Control-Allow-Origin", "*").unwrap();
                    response.headers_mut().insert("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE, PATCH, OPTIONS").unwrap();
                    response.headers_mut().insert("Access-Control-Allow-Headers", "Content-Type, Authorization").unwrap();
                    
                    Ok(response)
                }
                Err(_) => {
                    let error_response = json!({
                        "error": {
                            "message": "Failed to retrieve user data",
                            "code": "data_retrieval_error"
                        }
                    });
                    
                    let mut response = Response::builder()
                        .status(StatusCode::INTERNAL_SERVER_ERROR)
                        .header("Content-Type", "application/json")
                        .body(Body::Text(error_response.to_string()))?;
                    
                    response.headers_mut().insert("Access-Control-Allow-Origin", "*").unwrap();
                    response.headers_mut().insert("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE, PATCH, OPTIONS").unwrap();
                    response.headers_mut().insert("Access-Control-Allow-Headers", "Content-Type, Authorization").unwrap();
                    
                    Ok(response)
                }
            }
        }
        _ => {
            let error_response = json!({
                "error": {
                    "message": "Method not allowed",
                    "code": "method_not_allowed"
                }
            });
            
            let mut response = Response::builder()
                .status(StatusCode::METHOD_NOT_ALLOWED)
                .header("Content-Type", "application/json")
                .body(Body::Text(error_response.to_string()))?;
            
            response.headers_mut().insert("Access-Control-Allow-Origin", "*").unwrap();
            response.headers_mut().insert("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE, PATCH, OPTIONS").unwrap();
            response.headers_mut().insert("Access-Control-Allow-Headers", "Content-Type, Authorization").unwrap();
            
            Ok(response)
        }
    }
}
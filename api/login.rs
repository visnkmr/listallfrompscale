use std::{env, time::Duration};

use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::json;
use listallfrompscale::{ createuser, checklogin};
// use listallfrompscale::choose_starter;
use vercel_runtime::{
    http::bad_request, process_request, process_response, run_service, service_fn, Body, Error,
    Request, RequestPayloadExt, Response, ServiceBuilder, StatusCode,
};

#[derive(Debug, Serialize, Deserialize)]
struct Payload {
    uid: String,
    // datatoadd: String,
    pswd: String,
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
        // disable printing the name of the module in every log line.
        .with_target(false)
        .init();

    // This allows to extend the tower service with more layers
    let handler = ServiceBuilder::new()
        .map_request(process_request)
        .map_response(process_response)
        .service(service_fn(handler));

    run_service(handler).await
}
use upstash_ratelimit::{Limiter, RateLimit, Response as rsp};

pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    tracing::info!("Recieved a req");
    let ip=req.headers().get("x-vercel-forwarded-for").unwrap().to_str().unwrap().to_string();
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
            let usak=env::var("USAK").unwrap();
            let redis = redis::Client::open(usak)?;

            let ratelimit = RateLimit::builder()
                .redis(redis.clone())
                .limiter(Limiter::FixedWindow {
                    tokens: 1,
                    window: Duration::from_millis(5000),
                })
                .build()?;

            // let response = ;

            match ratelimit.limit(ip).unwrap() {
                rsp::Success { .. } => {
                    // let starter = choose_starter();
                    // let data=printeuser(payload.uid.clone(), payload.datatoadd.clone()).unwrap().url;
                    // let jdata:Vec<String>=serde_json::from_str(&data).unwrap();
                    if(req.method()==Method::POST){
                        match(checklogin(payload.uid.clone(), payload.pswd.clone())){
                            Ok(_) => {
                                Ok(Response::builder()
                                .status(StatusCode::ACCEPTED)
                                // .header("Access-Control-Allow-Origin", "*")
                                // .header("Access-Control-Allow-Headers", "Origin, X-Requested-With, Content-Type, Accept")
                                .header("Content-Type", "application/json")
                                .body(
                                    json!({
                                    "got": format!("LOGIN SUCCESSFUL!"),
                                    // "data": serde_json::to_string(&jdata).unwrap(),
                                    "request":format!("{:?}",req),
                                    })
                                    .to_string()
                                    .into(),
                                    )?)
                            },
                            Err(_) => {
                                Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .header("Content-Type", "application/json")
                                .body(
                                    json!({
                                        "FAILED": "YES",
                                        "request":format!("{:?}",req),
                                        })
                                        .to_string()
                                        .into(),
                                    )?)
                            },
                        }
                        
                    }
                    else{
                        Ok(Response::builder()
                    .status(StatusCode::BAD_REQUEST)
                    .header("Content-Type", "application/json")
                    // .header("Access-Control-Allow-Origin", "*")
                    // .header("Access-Control-Allow-Headers", "Origin, X-Requested-With, Content-Type, Accept")
                    .body(
                        json!({
                        "FAILED": "YES",
                        "request":format!("{:?}",req),
                        })
                        .to_string()
                        .into(),
                    )?)
                    }
                    // let jdata=serde_json::to_value(&data).unwrap();
                    
                },
                rsp::Failure { .. } => {
                    Ok(Response::builder()
                    .status(StatusCode::TOO_MANY_REQUESTS)
                    // .header("Access-Control-Allow-Origin", "*")
                    // .header("Access-Control-Allow-Headers", "Origin, X-Requested-With, Content-Type, Accept")
                    .header("Content-Type", "application/json")
                    .body(
                        json!({
                        "FAILED": "YES"
                        })
                        .to_string()
                        .into(),
                    )?)
                }
            }
            
        }
    }
}

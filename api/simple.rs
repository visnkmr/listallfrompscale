use std::{time::Duration, env};

use serde_json::json;
use listallfrompscale::printdata;
// mod pscale;
use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};
// use upstash_ratelimit::{Limiter, RateLimit, Response as rsp};

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(_req: Request) -> Result<Response<Body>, Error> {
//     let usak=env::var("USAK").unwrap();
//     let redis = redis::Client::open(usak)?;

// let ratelimit = RateLimit::builder()
//     .redis(redis.clone())
//     .limiter(Limiter::FixedWindow {
//         tokens: 1,
//         window: Duration::from_millis(1000),
//     })
//     .build()?;

// // let response = ;

// match ratelimit.limit("rlimit").unwrap() {
//     rsp::Success { .. } => {
        Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(
            json!({
              "data": printdata().unwrap(),
              "request":format!("{:?}",_req),
            })
            .to_string()
            .into(),
        )?)
//     },
//     rsp::Failure { .. } => {
//         Ok(Response::builder()
//         .status(StatusCode::TOO_MANY_REQUESTS)
//         .header("Content-Type", "application/json")
//         .body(
//             json!({
//               "FAILED": "YES"
//             })
//             .to_string()
//             .into(),
//         )?)
//     }
// }

    // let jh=_req.body().
    // let starter = choose_starter();
   
}

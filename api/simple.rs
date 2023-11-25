use serde_json::json;
use listallfrompscale::printdata;
// mod pscale;
use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(_req: Request) -> Result<Response<Body>, Error> {
    // let jh=_req.body().
    // let starter = choose_starter();
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(
            json!({
              "data": printdata(String::new(),String::new()).unwrap(),
              "request":format!("{:?}",_req),
            })
            .to_string()
            .into(),
        )?)
}

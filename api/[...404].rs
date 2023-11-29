use serde_json::json;
use vercel_runtime::{run, Body, Error, Request, Response,StatusCode};


#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(_req: Request) -> Result<Response<Body>, Error> {

        Ok(Response::builder()
        .status(StatusCode::NOT_FOUND)
        .header("Content-Type", "application/json")
        .body(
            json!({
              "data": "NOT FOUND",
              "request":format!("{:?}",_req),
            })
            .to_string()
            .into(),
        )?)

    // let jh=_req.body().
    // let starter = choose_starter();
   
}
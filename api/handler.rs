use serde_json::json;
use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};

use v4v::pc20::calc;

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(_req: Request) -> Result<Response<Body>, Error> {
    let splits = vec![60, 40];
    let total_sats = 1000;

    let testing = calc::compute_sat_recipients(&splits, total_sats);

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(
            json!({
              "message": format!("{:#?}", testing)
            })
            .to_string()
            .into(),
        )?)
}

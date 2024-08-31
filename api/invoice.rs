use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};

use serde_json;
use serde_json::json;
use serde_json::Value;

use std::env;

// use v4v::pc20::forwarding::CreateInvoiceMetadataForwardingStruct;
use v4v::alby::api::invoices::{CreateInvoiceArgs, create_invoice};

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    let token = env::var("ALBY_TOKEN").unwrap();

    let body_bytes = req.body();
    let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();

    let content_type = req.headers().get("Content-Type").unwrap().to_str().unwrap();

    if content_type.contains("application/json") {
        // Parse JSON data
        let form_data: Value = serde_json::from_str(&body_str).unwrap();

        let invoice = CreateInvoiceArgs{
            user_agent: "V4V Boost Split",
            token: &token,
            metadata: form_data["metadata"].clone(),
            num_sats: form_data["num_sats"].as_u64().unwrap_or_default(),
            description: Some(form_data["description"].as_str().unwrap_or_default().to_string()),
            payer_name: Some(form_data["payer_name"].as_str().unwrap_or_default().to_string()),
        };

        let response = create_invoice(invoice).await?;

        // Do something with form_data
        // let response_body = format!("Received JSON data: {:?}", form_data);
        // Ok(Response::new(Body::from(response_body)))

        return Ok(Response::builder()
            .status(StatusCode::OK)
            .header("Content-Type", "application/json")
            .body(
                json!({
                    "payment_request": response.payment_request,
                })
                .to_string()
                .into(),
            )?);
    }

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(
            json!({
            })
            .to_string()
            .into(),
        )?)
}
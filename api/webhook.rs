// use serde_json::json;
use serde_json::Value;
use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};

use std::str;

use v4v::alby;

// const ALBY_TOKEN: &str = "my_secret_alby_token";
const USER_AGENT: &str = "V4V Boost/1.0";

// #[derive(serde::Deserialize)]
// struct GenerateForwardingInvoiceRequest {
//     pub payment_info: v4v::pc20::payments::PaymentInfo,
//     pub recipients: Vec<v4v::pc20::payments::PaymentRecipientInfo>,
// }

// #[derive(serde::Serialize)]
// struct GenerateForwardingInvoiceResponse {
//     pub invoice: String,
// }

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(wrapper).await
}

pub async fn wrapper(req: Request) -> Result<Response<Body>, Error> {
    let status = handler(req).await?;

    Ok(Response::builder()
        .status(status)
        .header("Content-Type", "application/json")
        .body(
            ""
            .to_string()
            .into(),
        )?
    )
}

pub async fn handler(req: Request) -> Result<StatusCode, Error> {
    let secret = std::env::var("ALBY_WEBHOOK_SECRET")?;
    let token = std::env::var("ALBY_TOKEN")?;

    let headers = req.headers();

    let mut strbody = "".to_string();
    let mut body = strbody.as_bytes();

    match req.body() {
        Body::Text(val) => {
            strbody = val.to_string();
            body = strbody.as_bytes();
        },
        Body::Binary(val) => {
            body = val;
        },
        Body::Empty => {
            eprintln!("Body is empty!");
        },
    };

    let valid = alby::webhooks::verify_signature(&secret, body, headers);

    if let Err(_) = valid {
        eprintln!("Unable to verify signature: {:#?}", valid);
        return Ok(StatusCode::BAD_REQUEST);
    }

    let strbody = str::from_utf8(&body).unwrap();
    let json: Value = serde_json::from_str(strbody)?;
    let invoice = alby::webhooks::extract_alby_invoice(&json)?;

    println!("invoice: {:#?}", invoice);

    if invoice.state != "SETTLED".to_string() {
        println!("Invoice not settled yet");
        return Ok(StatusCode::OK);
    }

    let metadata = match v4v::pc20::forwarding::CreateInvoiceMetadata::try_from(
        invoice.clone(),
    ) {
        Ok(metadata) => metadata,
        Err(e) => {
            eprintln!("Unable to deserialize invoice metadata: {:#?}", e);
            return Ok(StatusCode::OK)
        }
    };

    println!("metadata: {:#?}", metadata);

    let payment_info = metadata.forwarding_data.payment_info;
    let recipients = metadata.forwarding_data.recipients;

    // Trim if the sum of payments somehow exceeds the total amount received.
    let recipients = v4v::pc20::forwarding::clip_recipients_at_amount(invoice.clone().num_sats, &recipients);

    println!("clipped recipients: {:#?}", recipients);

    // match v4v::pc20::forwarding::forward_payments(v4v::pc20::forwarding::ForwardPaymentArgs {
    //     user_agent: USER_AGENT,
    //     token: &token,
    //     payment_info,
    //     recipients,
    // }).await {
    //     Ok(_) => Ok(StatusCode::NO_CONTENT),
    //     Err(e) => {
    //         eprintln!("Failed to forward payments: {}", e);
    //         Ok(StatusCode::INTERNAL_SERVER_ERROR)
    //     },
    // }

    Ok(StatusCode::NO_CONTENT)
}

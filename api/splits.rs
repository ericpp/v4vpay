use serde_json::json;
use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};

use reqwest;

use serde::{Deserialize, Serialize};
use serde_xml_rs::from_str;

#[derive(Debug, Deserialize, Serialize)]
struct Rss {
    channel: RssChannel,
}

#[derive(Debug, Deserialize, Serialize)]
struct RssChannel {
    title: String,
    #[serde(rename = "guid")]
    guid: String,
    #[serde(rename = "value")]
    value: RssValue,
    #[serde(rename = "liveItem")]
    live_items: Vec<RssLiveItem>,
}

#[derive(Debug, Deserialize, Serialize)]
struct RssLiveItem {
    status: String,
    start: String,
    end: String,
    title: String,
    guid: String,
    #[serde(rename = "value")]
    value: RssValue,
}

#[derive(Debug, Deserialize, Serialize)]
struct RssValue {
    #[serde(rename = "valueRecipient")]
    recipients: Vec<RssValueRecipient>,
}

#[derive(Debug, Deserialize, Serialize)]
struct RssValueRecipient {
    name: String,
    r#type: String,
    address: String,
    #[serde(rename = "customKey")]
    custom_key: Option<String>,
    #[serde(rename = "customValue")]
    custom_value: Option<String>,
    split: u32,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(_req: Request) -> Result<Response<Body>, Error> {
    let url = "https://music.behindthesch3m3s.com/wp-content/uploads/cultconv/feed.xml";
    let xml = reqwest::get(url).await?.text().await?;

    let rss: Rss = from_str(&xml).expect("Failed to parse XML");


    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(
            json!({
              "title": rss.channel.title,
              "guid": rss.channel.guid,
              "recipients": rss.channel.value.recipients,
              "live_items": rss.channel.live_items,
            })
            .to_string()
            .into(),
        )?)
}

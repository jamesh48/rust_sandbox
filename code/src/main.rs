// https://www.youtube.com/watch?v=EqV5wKD233c
// https://github.com/awslabs/aws-lambda-rust-runtime
mod models;
mod post_event;
use crate::models::custom_event::CustomEvent;
use crate::models::single_event::SingleEvent;
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_dynamodb::Client;
use lambda_runtime::{service_fn, Error as LambdaError, LambdaEvent};
use serde::Serialize;
use serde_dynamo::aws_sdk_dynamodb_0_0_25_alpha::from_items;
use serde_json::{json, Value as JsonValue};
mod utilities;

// type Result<T, E = Box<dyn std::error::Error + Send + Sync>> = std::result::Result<T, E>;

#[derive(Debug, Serialize)]
struct FailureResponse {
    pub body: String,
}

#[tokio::main]
async fn main() -> Result<(), LambdaError> {
    let func = service_fn(handler);
    lambda_runtime::run(func).await?;
    Ok(())
}

async fn handler(event: LambdaEvent<CustomEvent>) -> Result<JsonValue, LambdaError> {
    println!("{:#?}", event);
    let region_provider = RegionProviderChain::default_provider().or_else("us-west-1");
    let config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&config);

    let (event, _context) = event.into_parts();

    if event.command == "getAllEvents" {
        let resp = client.scan().table_name("sitrep-events").send().await?;

        // And deserialize them as strongly-typed data structures
        if let Some(items) = resp.items {
            let scan_items: Vec<SingleEvent> = from_items(items)?;
            let all_events = scan_items
                .clone()
                .into_iter()
                .filter(|x| x.pk.to_string().starts_with("Event"))
                .collect::<Vec<SingleEvent>>();

            let stations_list = scan_items
                .clone()
                .into_iter()
                .filter(|x| x.pk.to_string().starts_with("Station"))
                .collect::<Vec<SingleEvent>>();

            // println!("{:#?}", json!(all_events));
            println!("{:#?}", json!(stations_list));
            println!("Got {} sitrep-events", all_events.len());
            return Ok(json!(all_events));
        }
    } else if event.command == "postEvent" {
        let response_json = post_event::post_event(event, client, _context).await;
        return response_json;
    }

    return Ok(json!({ "error": "there was an error" }));
}

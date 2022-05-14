// https://www.youtube.com/watch?v=EqV5wKD233c
// https://github.com/awslabs/aws-lambda-rust-runtime
// use aws_sdk_dynamodb::error::ScanError;
mod models;
// use crate::models::single_event::StationEntry;
use crate::models::single_event::SingleEvent;
use crate::models::custom_event::CustomEvent;
use aws_sdk_dynamodb::model::AttributeValue;
use serde_dynamo::aws_sdk_dynamodb_0_0_25_alpha::from_items;
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_dynamodb::{Client};
use serde::{Serialize};
use lambda_runtime::{service_fn, LambdaEvent, Error as LambdaError};
use serde_json::{json, Value as JsonValue};
use uuid::Uuid;
mod utilities;

// type Result<T, E = Box<dyn std::error::Error + Send + Sync>> = std::result::Result<T, E>;

#[derive(Debug, Serialize)]
struct FailureResponse {
    pub body: String
}

#[tokio::main]
async fn main() -> Result<(), LambdaError> {
    let func = service_fn(handler);
    lambda_runtime::run(func).await?;
    Ok(())
}

async fn handler(event: LambdaEvent<CustomEvent>) -> Result<JsonValue, LambdaError> {
    let region_provider = RegionProviderChain::default_provider()
        .or_else("us-west-1");
    let config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&config);


    let ( event, _context) = event.into_parts();

    if event.command == "getAllEvents" {
        let resp = client
                    .scan()
                    .table_name("sitrep-events")
                    .send()
                    .await?;

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
        let sk = utilities::time_utils::handle_time();
        let uuid = Uuid::new_v4().to_string();
        let pk = utilities::string_utils::handle_pk(event.event_type.clone(), uuid);
        let request = client.put_item()
            .table_name("sitrep-events")
            .item("PK", AttributeValue::S(pk.clone()))
            .item("SK", AttributeValue::S(sk.clone()))
            .item("scope", AttributeValue::S(String::from(event.scope)))
            .item("severity", AttributeValue::S(String::from(event.severity)))
            .item("status", AttributeValue::S(String::from(event.status)))
            .item("step", AttributeValue::S(String::from(event.step)))
            .item("eventType", AttributeValue::S(String::from(event.event_type.clone())))
            .item("headline", AttributeValue::S(String::from(event.headline)));

        request.send().await?;
        return  Ok(json!({
                            "message": "Record written!".to_string(),
                            "request_id": _context.request_id,
                            "SK": sk.clone()
                        }))
    }

    return Ok(json!({ "error": "there was an error" }));
}


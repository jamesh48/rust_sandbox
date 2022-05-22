use crate::models::single_event::SingleEvent;
use aws_sdk_dynamodb::Client;
use lambda_runtime::Error as LambdaError;
use serde_dynamo::aws_sdk_dynamodb_0_0_25_alpha::from_items;
use serde_json::{json, Value as JsonValue};

pub async fn get_all_events(client: Client) -> Result<JsonValue, LambdaError> {
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
  return Ok(json!({ "error": "there was an error" }));
}

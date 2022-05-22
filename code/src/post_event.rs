use crate::utilities::string_utils::handle_pk;
use crate::utilities::time_utils::handle_time;
use crate::CustomEvent;
use aws_sdk_dynamodb::model::AttributeValue;
use aws_sdk_dynamodb::Client;
use lambda_runtime::{Context, Error as LambdaError};
use serde_json::{json, Value as JsonValue};
use uuid::Uuid;

pub async fn post_event(
  event: CustomEvent,
  client: Client,
  _context: Context,
) -> Result<JsonValue, LambdaError> {
  let sk = handle_time();
  let uuid = Uuid::new_v4().to_string();
  let pk = handle_pk(event.event_type.clone(), uuid);
  let request = client
    .put_item()
    .table_name("sitrep-events")
    .item("PK", AttributeValue::S(pk.clone()))
    .item("SK", AttributeValue::S(sk.clone()))
    .item(
      "eventCategory",
      AttributeValue::S(String::from(event.event_category)),
    )
    .item("scope", AttributeValue::S(String::from(event.scope)))
    .item("severity", AttributeValue::S(String::from(event.severity)))
    .item("status", AttributeValue::S(String::from(event.status)))
    .item("step", AttributeValue::S(String::from(event.step)))
    .item(
      "eventType",
      AttributeValue::S(String::from(event.event_type.clone())),
    )
    .item(
      "readBy",
      AttributeValue::L([AttributeValue::S(String::from(event.read_by))].to_vec()),
    )
    .item("carriers", AttributeValue::Ss(Vec::from(event.carriers)))
    .item("headline", AttributeValue::S(String::from(event.headline)));

  request.send().await?;
  return Ok(json!({
      "message": "Record written!".to_string(),
      "request_id": _context.request_id,
      "SK": sk.clone()
  }));
}

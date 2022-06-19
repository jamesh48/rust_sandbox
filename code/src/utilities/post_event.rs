use crate::models::input_params::PostEventParams;
use crate::utilities::{string_utils::handle_pk, time_utils::handle_time};
use aws_sdk_dynamodb::{model::AttributeValue, Client};
use lambda_runtime::{Context, Error as LambdaError};
use serde_json::{json, Value as JsonValue};
use uuid::Uuid;

pub async fn post_event(
  post_event_params: PostEventParams,
  client: Client,
  context: Context,
) -> Result<JsonValue, LambdaError> {
  let sk = handle_time();
  let uuid = Uuid::new_v4().to_string();
  let pk = handle_pk(post_event_params.event_type.clone(), uuid);
  let request = client
    .put_item()
    .table_name("sitrep-events")
    .item("PK", AttributeValue::S(pk.clone()))
    .item("SK", AttributeValue::S(sk.clone()))
    .item(
      "dates",
      AttributeValue::S(String::from(post_event_params.dates)),
    )
    .item(
      "eventCategory",
      AttributeValue::S(String::from(post_event_params.event_category)),
    )
    .item(
      "scope",
      AttributeValue::S(String::from(post_event_params.scope)),
    )
    .item(
      "severity",
      AttributeValue::S(String::from(post_event_params.severity)),
    )
    .item(
      "status",
      AttributeValue::S(String::from(post_event_params.status)),
    )
    .item(
      "step",
      AttributeValue::S(String::from(post_event_params.step)),
    )
    .item(
      "eventType",
      AttributeValue::S(String::from(post_event_params.event_type.clone())),
    )
    .item(
      "readBy",
      AttributeValue::L([AttributeValue::S(String::from(post_event_params.read_by))].to_vec()),
    )
    .item(
      "carriers",
      AttributeValue::Ss(Vec::from(post_event_params.carriers)),
    )
    .item(
      "headline",
      AttributeValue::S(String::from(post_event_params.headline)),
    );

  request.send().await?;
  return Ok(json!({
      "message": "Record written!".to_string(),
      "request_id": context.request_id,
      "SK": sk.clone()
  }));
}

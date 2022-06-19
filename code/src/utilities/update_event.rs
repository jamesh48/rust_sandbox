use crate::models::input_params::{Description, PostEventParams};
use crate::models::single_event::SingleEvent;
use crate::utilities::get_descriptions_for_event::get_descriptions_for_event;
use crate::LambdaError;
use aws_sdk_dynamodb::{model::AttributeValue, Client};
use lambda_runtime::Context;
use serde_dynamo::aws_sdk_dynamodb_0_0_25_alpha::{from_items, to_item};
use serde_json::{json, Value as JsonValue};

pub async fn update_event(
  update_event_params: PostEventParams,
  client: Client,
  context: Context,
) -> Result<JsonValue, LambdaError> {
  if let Some(existing_pk) = update_event_params.pk {
    if let Some(existing_sk) = update_event_params.sk {
      let resp = client.scan().table_name("sitrep-events").send().await?;
      if let Some(items) = resp.items {
        let scan_items: Vec<SingleEvent> = from_items(items)?;
        let specific_event = scan_items
          .clone()
          .into_iter()
          .filter(|y| y.pk.to_string().starts_with(&existing_pk))
          .into_iter()
          .nth(0);

        let existing_descriptions = get_descriptions_for_event(specific_event);

        if let Some(description_version) = u8::try_from(existing_descriptions.len()).ok() {
          let description = Description {
            text: update_event_params.description,
            version: description_version + 1,
            user: "v859656".to_string(),
          };

          let description_item = to_item(description)?;

          let request = client
            .update_item()
            .table_name("sitrep-events")
            .key("PK", AttributeValue::S(existing_pk.clone()))
            .key("SK", AttributeValue::S(existing_sk.clone()))
            // Update Expression
            .update_expression(
              "set
               #descriptions = list_append(#descriptions, :descriptions),
               headline = :headline,
               eventCategory = :eventCategory,
               eventType = :eventType",
            )
            // Attribute Names
            .expression_attribute_names("#descriptions", "descriptions")
            // Attribute Values
            .expression_attribute_values(
              ":descriptions",
              AttributeValue::L([AttributeValue::M(description_item)].to_vec()),
            )
            .expression_attribute_values(
              ":headline",
              AttributeValue::S(update_event_params.headline),
            )
            .expression_attribute_values(
              ":eventCategory",
              AttributeValue::S(update_event_params.event_category),
            )
            .expression_attribute_values(
              ":eventType",
              AttributeValue::S(update_event_params.event_type),
            );

          request.send().await?;
        }
      }
      return Ok(json!({
          "message": "Record written!".to_string(),
          "request_id": context.request_id,
          "PK": existing_pk,
          "SK": existing_sk
      }));
    }
  }

  return Ok(json!({
    "message": "Request Failed"
  }));
}

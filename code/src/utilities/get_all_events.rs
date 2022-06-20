use crate::models::single_event::AirportEvent;
use aws_sdk_dynamodb::Client;
use lambda_runtime::Error as LambdaError;
use serde_dynamo::aws_sdk_dynamodb_0_0_25_alpha::from_items;
use serde_json::{json, Value as JsonValue};

pub async fn get_all_events(client: Client) -> Result<JsonValue, LambdaError> {
  let resp = client.scan().table_name("sitrep-events").send().await?;
  if let Some(items) = resp.items {
    let scan_items: Vec<AirportEvent> = from_items(items)?;
    let all_events = scan_items
      .clone()
      .into_iter()
      .filter(|x| x.pk.to_string().starts_with("Event"))
      .map(|airport_event| {
        return json!({
          "id": airport_event.pk,
          "sk": airport_event.sk,
          "readBy": airport_event.read_by,
          "event": {
            "cards": airport_event.cards,
            "carriers": airport_event.carriers,
            "customerImpact": airport_event.customer_impact,
            "dates": airport_event.dates,
            "descriptions": airport_event.descriptions,
            "employeeImpact": airport_event.employee_impact,
            "eventCategory": airport_event.event_category,
            "eventId": airport_event.event_id,
            "eventType": airport_event.event_type,
            "facilitiesImpact": airport_event.facilities_impact,
            "files": airport_event.files,
            "headline": airport_event.headline,
            "impactKeys": airport_event.impact_keys,
            "impacts": airport_event.impacts,
            "linkedEvents": airport_event.linked_events,
            "operationalImpact": airport_event.operational_impact,
            "phase": airport_event.phase,
            "scope": airport_event.scope,
            "severity": airport_event.severity,
            "stations": airport_event.stations,
            "status": airport_event.status,
            "timeEnd": airport_event.time_end,
            "timeStart": airport_event.time_start,
            "weatherUpdates": airport_event.weather_updates
          }
        });
      })
      .collect::<Vec<JsonValue>>();

    let stations_list = scan_items
      .clone()
      .into_iter()
      .filter(|x| x.pk.to_string().starts_with("Station"))
      .collect::<Vec<AirportEvent>>();

    // println!("{:#?}", json!(all_events));
    println!("{:#?}", json!(stations_list));
    println!("Got {} sitrep-events", all_events.len());
    return Ok(json!(all_events));
  }
  return Ok(json!({ "error": "there was an error" }));
}

use crate::models::single_event::AirportEvent;
use serde_json::Value as JsonValue;

pub fn get_descriptions_for_event(specific_event: Option<AirportEvent>) -> Vec<JsonValue> {
  if let Some(x) = specific_event {
    return x.descriptions;
  }
  return [].to_vec();
}

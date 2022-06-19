use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

// #[derive(Deserialize, Serialize, Debug)]
// pub struct StationEntry {
//   #[serde(rename = "PK")]
//   pub pk: String,
//   #[serde(rename = "SK")]
//   pub sk: String,
//   #[serde(rename = "cards")]
//   pub cards: Option<Vec<String>>,
//   #[serde(rename = "carriers")]
//   pub carriers: Option<Vec<String>>,
//   #[serde(rename = "customerImpact")]
//   pub customer_impact: Vec<String>,
// }

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SingleEvent {
  #[serde(rename = "PK")]
  pub pk: String,
  #[serde(rename = "SK")]
  pub sk: String,
  #[serde(rename = "cards")]
  pub cards: Option<Vec<String>>,
  #[serde(rename = "carriers")]
  pub carriers: Option<Vec<String>>,
  #[serde(rename = "dates")]
  pub dates: Option<String>,
  #[serde(rename = "eventCategory")]
  pub event_category: Option<String>,
  #[serde(rename = "eventType")]
  pub event_type: Option<String>,
  #[serde(rename = "timeEnd")]
  pub time_end: Option<String>,
  #[serde(rename = "files")]
  pub files: Option<Vec<String>>,
  #[serde(rename = "descriptions")]
  pub descriptions: Vec<JsonValue>,
  #[serde(rename = "headline")]
  pub headline: Option<String>,
  #[serde(rename = "impacts")]
  pub impacts: Option<Vec<String>>,
  #[serde(rename = "linkedEvents")]
  pub linked_events: Option<Vec<String>>,
  #[serde(rename = "phase")]
  pub phase: Option<String>,
  #[serde(rename = "readBy")]
  pub read_by: Option<Vec<String>>,
  #[serde(rename = "scope")]
  pub scope: Option<String>,
  #[serde(rename = "severity")]
  pub severity: Option<String>,
  #[serde(rename = "stations")]
  pub stations: Option<JsonValue>,
  #[serde(rename = "status")]
  pub status: Option<String>,
  #[serde(rename = "timeStart")]
  pub time_start: Option<String>,
  // Station Type Entries
  #[serde(rename = "eventId")]
  pub event_id: Option<String>,
  #[serde(rename = "customerImpact")]
  pub customer_impact: Option<Vec<String>>,
  #[serde(rename = "employeeImpact")]
  pub employee_impact: Option<Vec<String>>,
  #[serde(rename = "facilitiesImpact")]
  pub facilities_impact: Option<Vec<String>>,
  #[serde(rename = "operationalImpact")]
  pub operational_impact: Option<Vec<String>>,
  #[serde(rename = "weatherUpdates")]
  pub weather_updates: Option<Vec<String>>,
  #[serde(rename = "impactKeys")]
  pub impact_keys: Option<Vec<String>>,
}

// #[derive(Deserialize, Serialize, Debug, Clone)]
// pub struct StationData {
//   pub cards: JsonValue,
//   pub files: JsonValue,
//   pub impacts: JsonValue,
//   pub label: String,
// }

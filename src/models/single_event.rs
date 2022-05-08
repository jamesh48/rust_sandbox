use serde::{Serialize, Deserialize};

// #[derive(Deserialize, Serialize)]
// pub struct PKString { pub S: String }

#[derive(Deserialize, Serialize, Debug)]
pub struct SingleEvent {
  #[serde(rename = "PK")]
  pub pk: String,
  #[serde(rename = "SK")]
  pub sk: String,
  #[serde(rename = "cards")]
  pub cards: Vec<String>,
  #[serde(rename = "carriers")]
  pub carriers: Vec<String>,
  #[serde(rename = "dates")]
  pub dates: String,
  #[serde(rename = "eventCategory")]
  pub event_category: String,
  #[serde(rename = "eventType")]
  pub event_type: String,
  #[serde(rename = "timeEnd")]
  pub time_end: String,
  #[serde(rename = "files")]
  pub files: Vec<String>,
  #[serde(rename = "descriptions")]
  pub descriptions: Vec<String>,
  #[serde(rename = "headline")]
  pub headline: String,
  #[serde(rename = "impacts")]
  pub impacts: Vec<String>,
  #[serde(rename = "linkedEvents")]
  pub linked_events: Vec<String>,
  #[serde(rename = "phase")]
  pub phase: String,
  #[serde(rename = "readBy")]
  pub read_by: Vec<String>,
  #[serde(rename = "scope")]
  pub scope: String,
  #[serde(rename = "severity")]
  pub severity: String,
  #[serde(rename = "stations")]
  pub stations: Vec<String>,
  #[serde(rename = "status")]
  pub status: String,
  #[serde(rename = "timeStart")]
  pub time_start: String
}


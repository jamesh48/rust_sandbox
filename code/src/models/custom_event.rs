
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct CustomEvent {
    pub carriers: Vec<String>,
    pub event_category: String,
    pub headline: String,
    pub scope: String,
    pub severity: String,
    pub status: String,
    pub step: String,
    pub event_type: String,
    pub command: String,
    pub read_by: String
}


use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct CustomEvent {
    pub headline: String,
    pub scope: String,
    pub severity: String,
    pub status: String,
    pub step: String,
    pub event_type: String,
    pub command: String
}

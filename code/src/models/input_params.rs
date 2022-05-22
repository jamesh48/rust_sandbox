use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PostEventParams {
    pub carriers: Vec<String>,
    pub event_category: String,
    pub headline: String,
    pub scope: String,
    pub severity: String,
    pub status: String,
    pub step: String,
    pub event_type: String,
    pub read_by: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetObjParams {
    pub key: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InputParams {
    pub command: String,
    pub post_event_params: Option<PostEventParams>,
    pub get_object_params: Option<GetObjParams>,
}

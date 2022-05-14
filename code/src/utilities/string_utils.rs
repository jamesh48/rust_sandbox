pub fn handle_pk (event_type: String, uuid: String) -> String {
  return format!("Event#{}#{}", event_type, uuid);
}
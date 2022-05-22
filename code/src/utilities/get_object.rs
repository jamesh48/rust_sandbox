// used to get user data from s3
use lambda_runtime::Error as LambdaError;
use s3::Client;
use serde_json::json;
use serde_json::Value as JsonValue;

pub async fn get_object(key: String) -> Result<JsonValue, LambdaError> {
  let s3 = Client::from_env();
  let result = s3
    .get_object()
    .bucket("sitrep-event-files")
    .key(key)
    .response_content_type("application/json")
    .send()
    .await?;
  let data = result.body.collect().await?;
  let response = data.into_bytes().to_vec();
  return Ok(json!({ "body": response }));
}

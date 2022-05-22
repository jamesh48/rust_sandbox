// https://www.youtube.com/watch?v=EqV5wKD233c
// https://github.com/awslabs/aws-lambda-rust-runtime
mod models;
mod utilities;
use crate::models::input_params::InputParams;
use crate::utilities::{
    get_all_events::get_all_events, get_object::get_object, post_event::post_event,
};
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_dynamodb::Client;
use lambda_runtime::{service_fn, Error as LambdaError, LambdaEvent};
use simple_error::SimpleError;
extern crate simple_error;
use serde_json::Value as JsonValue;

#[tokio::main]
async fn main() -> Result<(), LambdaError> {
    let func = service_fn(handler);
    lambda_runtime::run(func).await?;
    Ok(())
}

async fn handler(event: LambdaEvent<InputParams>) -> Result<JsonValue, LambdaError> {
    println!("{:#?}", event);
    let region_provider = RegionProviderChain::default_provider().or_else("us-west-1");
    let config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&config);

    let (event, context) = event.into_parts();

    if event.command == "getAllEvents" {
        let response_json = get_all_events(client).await;
        return response_json;
    } else if event.command == "postEvent" {
        if let Some(post_event_params) = event.post_event_params {
            let response_json = post_event(post_event_params, client, context).await;
            return response_json;
        }
    } else if event.command == "getObject" {
        if let Some(get_object_params) = event.get_object_params {
            let response_json = get_object(get_object_params.key).await;
            return response_json;
        }
    }
    return Err(Box::new(SimpleError::new("[404] COMMAND NOT FOUND!")));
}

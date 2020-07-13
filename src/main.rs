use std::error::Error;

use lambda_runtime::{error::HandlerError, lambda, Context};
use log::{self};
use simple_logger;

use aws_lambda_events::event::apigw;

fn main() -> Result<(), Box<dyn Error>> {
    simple_logger::init_with_level(log::Level::Debug)?;
    lambda!(my_handler);

    Ok(())
}

fn my_handler(e: apigw::ApiGatewayProxyRequest, _c: Context) -> Result<apigw::ApiGatewayProxyResponse, HandlerError> {
    log::info!("{:?}", e);

    let mut message = "Hello Rust!".to_string();

    match e.body {
        Some(body) => {
            log::info!("Body: {}", body);
            message = body;
        }
        None => log::info!("Body is empty.")
    }

    Ok(apigw::ApiGatewayProxyResponse {
        status_code: 200,
        headers: Default::default(),
        multi_value_headers: Default::default(),
        body: Some(format!("{{\"message\": \"{}\"}}", message)),
        is_base64_encoded: None,
    })
}
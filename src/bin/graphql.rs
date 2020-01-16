use lambda_runtime::{error::HandlerError, lambda, Context};
use log::{self, error};
use serde_derive::{Deserialize, Serialize};
use simple_error::bail;
use simple_logger;
use std::error::Error;

#[derive(Serialize, Deserialize)]
struct GraphqlEvent {
    body: String,
}

#[derive(Serialize, Deserialize)]
struct GraphqlOutput {
    body: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    simple_logger::init_with_level(log::Level::Info)?;
    lambda!(execute);

    Ok(())
}


fn execute(e: GraphqlEvent, c: Context) -> Result<GraphqlOutput, HandlerError> {
    if e.body == "" {
        error!("Empty body in request {}", c.aws_request_id);
        bail!("Empty Body");
    }

    Ok(GraphqlOutput {
        body: format!("Hello, {}!", e.body),
    })
}

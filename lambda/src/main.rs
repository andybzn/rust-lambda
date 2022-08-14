use lambda_runtime::{Context, Error};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let handler = lambda_runtime::handler_fn(handler);
    lambda_runtime::run(handler).await?;
    Ok(())
}

#[derive(Deserialize)]
struct Event{
    elem_1: String,
    elem_2: String,
}

#[derive(Serialize)]
struct Output {
    message: String,
    request_id: String,
}

async fn handler(event: Event, context: Context) -> Result<Output, Error> {
    let message: String = format!("{} {}", event.elem_1, event.elem_2);
    Ok(Output{
        message,
        request_id: context.request_id,
    })
}

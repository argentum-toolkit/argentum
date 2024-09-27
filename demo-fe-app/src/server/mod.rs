use dioxus::prelude::*;
use dioxus::prelude::{server, ServerFnError};
use dioxus_logger::tracing;

#[server(PostServerData)]
pub async fn post_server_data(data: String) -> Result<(), ServerFnError> {
    tracing::info!("Server received: {}", data);
    Ok(())
}

#[server(GetServerData)]
pub async fn get_server_data() -> Result<String, ServerFnError> {
    Ok("Hello from the server!".to_string())
}

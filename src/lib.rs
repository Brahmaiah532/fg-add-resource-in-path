mod generated;

use anyhow::{anyhow, Result};

use pdk::hl::*;
use pdk::logger::info;

use crate::generated::config::Config;


async fn request_filter(request_state: RequestState, config: &Config) {

    let headers_state = request_state.into_headers_state().await;
    let headers_handler = headers_state.handler();
    let path = headers_state.path();
    // info!("headers_handler information: {:?}", headers_handler); 
    info!("Current path is {:?}", path);
    let new_path = format!("{}{}", path, &config.base_resource);
    info!("New path is {:?}", new_path);
    headers_handler.set_header(":path", &new_path);
}

#[entrypoint]
async fn configure(launcher: Launcher, Configuration(bytes): Configuration) -> Result<()> {
    let config: Config = serde_json::from_slice(&bytes).map_err(|err| {
        anyhow!(
            "Failed to parse configuration '{}'. Cause: {}",
            String::from_utf8_lossy(&bytes),
            err
        )
    })?;
    let filter = on_request(|rs| request_filter(rs, &config));
    launcher.launch(filter).await?;
    Ok(())
}

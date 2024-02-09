mod generated;

use anyhow::{anyhow, Result};
use pdk::hl::*;
use pdk::logger;

use crate::generated::config::Config;

//CENTRALIZED KEY MANAGEMENT - Retrieve the Open API Key and add to header
async fn request_filter(request_state: RequestState, config: &Config) {
    logger::info!("==================>OPENAI API KEY MANAGEMENT FLEX GATEWAY POLICY<=====================>");

    let openai_api_key = &config.openai_api_key;
    let redacted_key = format!("{}{}", &openai_api_key[..6], "****************");
    
    let headers = request_state.into_headers_state().await;

    logger::info!("==>ADDING KEY TO HEADER: {redacted_key}",);
    headers.handler().add_header("Authorization", format!("Bearer {}", openai_api_key).as_str());
    
    Flow::Continue(());
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

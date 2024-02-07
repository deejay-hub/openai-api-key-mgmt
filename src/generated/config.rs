use serde::Deserialize;
#[derive(Deserialize, Clone, Debug)]
pub struct Config {
    #[serde(alias = "openai-api-key")]
    pub openai_api_key: String,
}

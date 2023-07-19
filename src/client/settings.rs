use std::collections::HashMap;

use tracing::debug;

use crate::Client;

/// Represents the settings of a GoTrue instances.
#[derive(Debug, Default, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Settings {
    /// The map shows which external providers are enabled.
    pub external: HashMap<String, bool>,
    /// Whether signup is disabled.
    pub disable_signup: bool,
    /// Whether autoconfirm is enabled.
    pub autoconfirm: bool,
}

impl Client {
    /// Get the publicly available settings for the GoTrue instance.
    pub async fn get_settings(&self) -> anyhow::Result<Settings> {
        let endpoint = format!("{}/settings", self.url);
        debug!("calling {}", endpoint);
        let response: Settings = self
            .client
            .get(endpoint)
            .headers(self.headers.clone())
            .send()
            .await?
            .error_for_status()?
            .json::<Settings>()
            .await?;

        Ok(response)
    }
}

use std::collections::HashMap;

use crate::Client;

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct Settings {
    pub external: HashMap<String, String>,
    pub disable_sigup: bool,
    pub autoconfirm: bool,
}

impl Client {
    /// Get the publicly available settings for the gotrue instance.
    pub async fn settings(&self) -> anyhow::Result<Settings> {
        Ok(Settings::default())
    }
}

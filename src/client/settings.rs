use std::collections::HashMap;

use tracing::{debug, error};

use crate::{client::handle_gotrue_resp_status, Client, Error};

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
    pub async fn get_settings(&self) -> Result<Settings, Error> {
        let endpoint = format!("{}/settings", self.url);

        debug!("calling {}", endpoint);
        let resp = match self
            .client
            .get(endpoint)
            .headers(self.headers.clone())
            .send()
            .await
        {
            Ok(resp) => resp,
            Err(e) => {
                error!("could not make request to gotrue: {}", e);
                return Err(Error::InternalError);
            }
        };

        if let Err(e) = handle_gotrue_resp_status(resp.status()) {
            error!("gotrue returned an error status: {}", resp.status());
            return Err(e);
        }

        let settings = match resp.json::<Settings>().await {
            Ok(settings) => settings,
            Err(e) => {
                error!("could not deserialize the response into settings: {}", e);
                return Err(Error::InternalError);
            }
        };

        Ok(settings)
    }
}

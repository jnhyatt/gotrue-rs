use tracing::{debug, error};

use crate::{user::User, Client, Error};

use super::handle_gotrue_resp_status;

impl Client {
    /// Creates a user
    ///
    /// # Example
    ///
    /// ```
    /// use go_true::{Api};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let url = "http://localhost:9998".to_string();
    ///     let mut client = Api::new(url);
    ///
    ///     let user = AdminUserAttributes {
    ///         email: "createemail@example.com",
    ///         password: Some(String::from("Abcd1234!")),
    ///         data: None,
    ///         email_confirmed: None,
    ///         phone_confirmed: None,
    ///     };

    ///     client.create_user(user).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn create_user(&self, user: User) -> Result<User, Error> {
        let endpoint = format!("{}/admin/users", self.url);

        let json = match serde_json::to_value(&user) {
            Ok(value) => value,
            Err(e) => {
                error!("could not serialize the user: {}", e);
                return Err(Error::InternalError);
            }
        };

        debug!("calling {}", endpoint);
        let resp = match self
            .client
            .post(endpoint)
            .headers(self.headers.clone())
            .json(&json)
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

        let user = match resp.json::<User>().await {
            Ok(user) => user,
            Err(e) => {
                error!("could not deserialize the response into a user: {}", e);
                return Err(Error::InternalError);
            }
        };

        Ok(user)
    }
}

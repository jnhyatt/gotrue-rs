use crate::{
    api::{Api, EmailOrPhone},
    error::Error,
    session::Session,
    user_attributes::UserAttributes,
    user_update::UserUpdate,
};

///
/// Represents a client used to interact with a Gotrue Server.
///
#[derive(Debug, Clone)]
pub struct Client {
    api: Api,
}

impl Client {
    /// Creates a GoTrue Client.
    ///
    /// # Example
    ///
    /// ```
    /// use go_true::Client;
    ///
    /// let client = Client::new("http://your.gotrue.endpoint");
    /// ```
    pub fn new(url: &str) -> Client {
        Client { api: Api::new(url) }
    }

    /// Signs up a new user.
    ///
    /// # Example
    ///
    /// ```
    /// use go_true::{Client, EmailOrPhone};
    ///
    /// #[tokio::main]
    ///     async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let mut client = Client::new("http://your.gotrue.endpoint");
    ///     let email = "some_email".to_string();
    ///     let password = "some_password".to_string();
    ///     let res = client
    ///         .sign_up(EmailOrPhone::Email(email), &password)
    ///         .await?;
    ///     Ok(())
    /// }
    pub async fn sign_up(
        &mut self,
        email_or_phone: EmailOrPhone,
        password: &str,
    ) -> Result<Session, Error> {
        let result = self.api.sign_up(email_or_phone, password).await;

        match result {
            Ok(session) => Ok(session),
            Err(e) => {
                if e.is_status() && e.status().unwrap().as_str() == "400" {
                    return Err(Error::AlreadySignedUp);
                }
                Err(Error::InternalError)
            }
        }
    }

    /// Signs in a user.
    ///
    /// # Example
    ///
    /// ```
    /// use go_true::{Client, EmailOrPhone};
    ///
    /// #[tokio::main]
    ///     async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::new("http://your.gotrue.endpoint".to_string());
    ///     let email = "some_email".to_string();
    ///     let password = "some_password".to_string();
    ///     let res = client
    ///         .sign_in(EmailOrPhone::Email(email), &password)
    ///         .await?;
    ///     Ok(())
    /// }
    pub async fn sign_in(
        &mut self,
        email_or_phone: EmailOrPhone,
        password: &str,
    ) -> Result<Session, Error> {
        let result = self.api.sign_in(email_or_phone, password).await;

        match result {
            Ok(session) => Ok(session),
            Err(e) => {
                if e.is_status() && e.status().unwrap().as_str() == "400" {
                    Err(Error::WrongCredentials)
                } else {
                    Err(Error::InternalError)
                }
            }
        }
    }

    /// Sends an OTP
    ///
    /// # Example
    ///
    /// ```
    /// use go_true::{Client, EmailOrPhone};
    ///
    /// #[tokio::main]
    ///     async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::new("http://your.gotrue.endpoint".to_string());
    ///     let email = "some_email".to_string();
    ///
    ///     let res = client
    ///         .send_otp(EmailOrPhone::Email(email), None)
    ///         .await?;
    ///     Ok(())
    /// }
    pub async fn send_otp(
        &self,
        email_or_phone: EmailOrPhone,
        should_create_user: Option<bool>,
    ) -> Result<bool, Error> {
        let result = self.api.send_otp(email_or_phone, should_create_user).await;

        match result {
            Ok(_) => Ok(true),
            Err(e) => {
                if e.is_status() && e.status().unwrap().as_str() == "422" {
                    Err(Error::UserNotFound)
                } else {
                    Err(Error::InternalError)
                }
            }
        }
    }

    /// Verifies an OTP request.
    pub async fn verify_otp<T: serde::Serialize>(&mut self, params: T) -> Result<bool, Error> {
        let result = self.api.verify_otp(params).await;

        match result {
            Ok(_) => Ok(true),
            Err(e) => {
                if e.is_status() && e.status().unwrap().as_str() == "400" {
                    Err(Error::WrongToken)
                } else {
                    Err(Error::InternalError)
                }
            }
        }
    }

    /// Sign out the current user
    ///
    /// # Example
    ///
    /// ```
    /// use go_true::{Client};
    ///
    /// #[tokio::main]
    ///     async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::new("http://your.gotrue.endpoint".to_string());
    ///
    ///     // Sign in first
    ///
    ///     let res = client.sign_out().await?;
    ///     Ok(())
    /// }
    pub async fn sign_out(&self, access_token: &str) -> Result<bool, Error> {
        match self.api.sign_out(access_token).await {
            Ok(_) => Ok(true),
            Err(_) => Err(Error::InternalError),
        }
    }

    /// Reset a user's password for an email address
    ///
    /// # Example
    ///
    /// ```
    /// use go_true::{Client};
    ///
    /// #[tokio::main]
    ///     async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::new("http://your.gotrue.endpoint".to_string());
    ///     let email = "some_email".to_string()
    ///
    ///     let res = client.reset_password_for_email(&email).await?;
    ///     Ok(())
    /// }
    pub async fn reset_password_for_email(&self, email: &str) -> Result<bool, Error> {
        let result = self.api.reset_password_for_email(email).await;

        match result {
            Ok(_) => Ok(true),
            Err(_) => Err(Error::UserNotFound),
        }
    }

    /// Update a user.
    pub async fn update_user(
        &self,
        access_token: &str,
        user: UserAttributes,
    ) -> Result<UserUpdate, Error> {
        let result = self.api.update_user(user, access_token).await;

        match result {
            Ok(user) => Ok(user),
            Err(e) => {
                if e.is_status() && e.status().unwrap().as_str() == "400" {
                    Err(Error::UserNotFound)
                } else {
                    Err(Error::InternalError)
                }
            }
        }
    }

    /// Refreshes the current session
    ///
    /// # Example
    ///
    /// ```
    /// use go_true::{Client};
    ///
    /// #[tokio::main]
    ///     async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = Client::new("http://your.gotrue.endpoint".to_string());
    ///
    ///     // sign in first
    ///
    ///     client.refresh_session().await?:
    ///     Ok(())
    /// }
    pub async fn refresh_session(&mut self, refresh_token: &str) -> Result<Session, Error> {
        if refresh_token.is_empty() {
            return Err(Error::NotAuthenticated);
        }

        let result = self.api.refresh_access_token(refresh_token).await;

        let session = match result {
            Ok(session) => session,
            Err(_) => return Err(Error::InternalError),
        };

        Ok(session)
    }
}

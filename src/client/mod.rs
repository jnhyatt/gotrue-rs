pub mod create_user;
pub mod settings;

use reqwest::{
    header::{HeaderMap, HeaderValue, IntoHeaderName},
    StatusCode,
};
use serde_json::json;

use crate::{
    session::Session, user::User, user_attributes::UserAttributes, user_list::UserList,
    user_update::UserUpdate, Error,
};

///
/// Represents the API of Gotrue.
///
#[derive(Debug, Clone)]
pub struct Client {
    url: String,
    headers: HeaderMap,
    client: reqwest::Client,
}

/// Represent either an email or phone number for OTP.
#[derive(Debug, Clone)]
pub enum EmailOrPhone {
    /// Email
    Email(String),
    /// Phone number
    Phone(String),
}

impl Client {
    /// Creates a GoTrue API client.
    ///
    /// # Example
    ///
    /// ```
    /// use go_true::Api;
    ///
    /// let client = Api::new("http://your.gotrue.endpoint");
    /// ```
    pub fn new(url: &str) -> Client {
        Client {
            url: url.to_owned(),
            headers: HeaderMap::new(),
            client: reqwest::Client::new(),
        }
    }

    /*
    NEW API
    */

    /// Creates a user based on the user_id specified.
    pub async fn admin_create_user(&self) -> anyhow::Result<()> {
        Ok(())
    }

    /// Updates a user based on the user_id specified.
    pub async fn admin_update_user(&self) -> anyhow::Result<()> {
        Ok(())
    }

    /// Generates a corresponding email action link based on the type specified.
    pub async fn generate_email_action_link(&self) -> anyhow::Result<()> {
        Ok(())
    }

    /// Register a new user with an email and password.
    pub async fn signup(&self) -> anyhow::Result<()> {
        Ok(())
    }

    /// Invites a new user with an email.
    ///
    /// This method requires the service_role or supabase_admin JWT set as an Auth Bearer header:

    pub async fn invite(&self) -> anyhow::Result<()> {
        Ok(())
    }

    /// Verify a registration or a password recovery.
    pub async fn verify_registration_or_recovery(&self) -> anyhow::Result<()> {
        Ok(())
    }

    ///  Delivers a magiclink or sms otp to the user depending on whether the request body contains an "email" or "phone" key.
    pub async fn send_new_otp(&self) -> anyhow::Result<()> {
        Ok(())
    }

    /// Send a password recovery mail to the user based on email address.
    pub async fn send_password_recovery(&self) -> anyhow::Result<()> {
        Ok(())
    }

    /// This is an OAuth2 endpoint that currently implements the password and refresh_token grant types.
    pub async fn oauth2_token(&self) -> anyhow::Result<()> {
        Ok(())
    }

    /// Get the JSON object for the logged in user.
    pub async fn get_new_user(&self) -> anyhow::Result<()> {
        Ok(())
    }

    /// Sends a nonce to the user's email (preferred) or phone.
    /// This endpoint requires the user to be logged in / authenticated first.
    /// The user needs to have either an email or phone number for the nonce to be sent successfully.
    pub async fn reauth(&self) -> anyhow::Result<()> {
        Ok(())
    }

    /// Logout a user (Requires authentication).
    /// This will revoke all refresh tokens for the user.
    /// Remember that the JWT tokens will still be valid for stateless auth until they expires.
    pub async fn sign_out_new(&self) -> anyhow::Result<()> {
        Ok(())
    }

    /// Get access_token from external oauth provider
    pub async fn external_authorize(&self) -> anyhow::Result<()> {
        Ok(())
    }

    /// Recieve a external authorize callback.
    pub async fn external_callback(&self) -> anyhow::Result<()> {
        Ok(())
    }

    /*
    Old API
     */
    /// Add arbitrary headers to the request. For instance when you may want to connect
    /// through an API gateway that needs an API key header.
    ///
    /// # Example
    ///
    /// ```
    /// use go_true::Api;
    ///
    /// let client = Api::new("https://your.gotrue.endpoint")
    ///     .insert_header("apikey", "super.secret.key");
    /// ```
    pub fn with_header(
        mut self,
        header_name: impl IntoHeaderName,
        header_value: impl AsRef<str>,
    ) -> Self {
        self.headers.insert(
            header_name,
            HeaderValue::from_str(header_value.as_ref()).expect("Invalid header value."),
        );
        self
    }

    /// Signs up for a new account
    ///
    /// # Example
    ///
    /// ```
    /// use go_true::{Api, EmailOrPhone};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let url = "http://localhost:9998";
    ///     let mut client = Api::new(url);
    ///
    ///     let email = "email@example.com";
    ///     let password = "Abcd1234!";
    ///
    ///     let result = client.sign_up(EmailOrPhone::Email(email.to_string()), &password).await;
    ///     Ok(())
    /// }
    /// ```
    pub async fn sign_up(
        &self,
        email_or_phone: EmailOrPhone,
        password: &str,
    ) -> Result<Session, reqwest::Error> {
        let endpoint = format!("{}/signup", self.url);

        let body = match email_or_phone {
            EmailOrPhone::Email(email) => json!({
                "email": email,
                "password": &password,
            }),
            EmailOrPhone::Phone(phone) => json!({
                "phone": phone,
                "password": &password
            }),
        };

        let response: Session = self
            .client
            .post(endpoint)
            .headers(self.headers.clone())
            .json(&body)
            .send()
            .await?
            .error_for_status()?
            .json::<Session>()
            .await?;

        Ok(response)
    }

    /// Signs into an existing account
    ///
    /// # Example
    ///
    /// ```
    /// use go_true::{Api, EmailOrPhone};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let url = "http://localhost:9998";
    ///     let mut client = Api::new(url);
    ///
    ///     let email = "email@example.com".to_string();
    ///     let password = "Abcd1234!";
    ///
    ///     let result = client.sign_in(EmailOrPhone::Email(email), password).await;
    ///     
    ///     Ok(())
    /// }
    /// ```
    pub async fn sign_in(
        &self,
        email_or_phone: EmailOrPhone,
        password: &str,
    ) -> Result<Session, reqwest::Error> {
        let query_string = String::from("?grant_type=password");

        let endpoint = format!("{}/token{}", self.url, query_string);

        let body = match email_or_phone {
            EmailOrPhone::Email(email) => json!({
                "email": email,
                "password": &password,
            }),
            EmailOrPhone::Phone(phone) => json!({
                "phone": phone,
                "password": &password
            }),
        };

        let response: Session = self
            .client
            .post(endpoint)
            .headers(self.headers.clone())
            .json(&body)
            .send()
            .await?
            .error_for_status()?
            .json::<Session>()
            .await?;

        Ok(response)
    }

    /// Sends an OTP Code and creates user if it does not exist
    ///
    /// # Example
    ///
    /// ```
    /// use go_true::{Api, EmailOrPhone};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let url = "http://localhost:9998";
    ///     let mut client = Api::new(url);
    ///
    ///     let email = "email@example.com".to_string();
    ///
    ///     let result = client.send_otp(EmailOrPhone::Email(email), None).await;
    ///     Ok(())
    /// }
    /// ```
    pub async fn send_otp(
        &self,
        email_or_phone: EmailOrPhone,
        should_create_user: Option<bool>,
    ) -> Result<bool, reqwest::Error> {
        let endpoint = format!("{}/otp", self.url);

        let body = match email_or_phone {
            EmailOrPhone::Email(email) => json!({
                "email": email,
                "should_create_user": Some(should_create_user)
            }),
            EmailOrPhone::Phone(phone) => json!({
                "phone": phone,
                "should_create_user": Some(should_create_user)
            }),
        };

        self.client
            .post(endpoint)
            .headers(self.headers.clone())
            .json(&body)
            .send()
            .await?
            .error_for_status()?;

        Ok(true)
    }

    /// Verifies a phone signup or sms otp.
    ///
    /// # Example
    ///
    /// ```
    /// // TODO
    /// ```
    pub async fn verify_otp<T: serde::Serialize>(&self, params: T) -> Result<bool, reqwest::Error> {
        let endpoint = format!("{}/verify", self.url);

        let body = serde_json::to_value(&params).unwrap();

        self.client
            .post(endpoint)
            .headers(self.headers.clone())
            .json(&body)
            .send()
            .await?
            .error_for_status()?;

        Ok(true)
    }

    /// Signs the current user out
    ///
    /// # Example
    ///
    /// ```no_run
    /// use go_true::{Api, EmailOrPhone};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let url = "http://localhost:9998";
    ///     let mut client = Api::new(url);
    ///
    ///
    ///     let email = "email@example.com".to_string();
    ///     let password = "Abcd1234!".to_string();
    ///
    ///     let session = client.sign_in(EmailOrPhone::Email(email), &password).await?;
    ///     client.sign_out(&session.access_token);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn sign_out(&self, access_token: &str) -> Result<bool, reqwest::Error> {
        let endpoint = format!("{}/logout", self.url);

        let mut headers: HeaderMap = self.headers.clone();
        let bearer = format!("Bearer {access_token}");
        headers.insert(
            "Authorization",
            HeaderValue::from_str(bearer.as_ref()).expect("Invalid header value."),
        );

        self.client
            .post(endpoint)
            .headers(headers)
            .send()
            .await?
            .error_for_status()?;

        Ok(true)
    }

    /// Sends password recovery email
    ///
    /// # Example
    ///
    /// ```
    /// use go_true::{Api, EmailOrPhone};
    ///
    /// let url = "http://localhost:9998".to_string();
    /// let mut client = Api::new(url);
    /// let email = "random@mail.com".to_string();
    ///
    /// client.reset_password_for_email(&email);
    /// ```
    pub async fn reset_password_for_email(&self, email: &str) -> Result<bool, reqwest::Error> {
        let endpoint = format!("{}/recover", self.url);

        let body = json!({
            "email": &email,
        });

        self.client
            .post(endpoint)
            .headers(self.headers.clone())
            .json(&body)
            .send()
            .await?
            .error_for_status()?;

        Ok(true)
    }

    /// Returns the url for a given provider.
    pub fn get_url_for_provider(&self, provider: &str) -> String {
        format!("{}/authorize?provider={}", self.url, provider)
    }

    /// Refreshes the current session by refresh token
    ///
    /// # Example
    ///
    /// ```
    /// use go_true::{Api, EmailOrPhone};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let url = "http://localhost:9998".to_string();
    ///     let mut client = Api::new(url);
    ///
    ///
    ///     let email = "email@example.com".to_string();
    ///     let password = "Abcd1234!".to_string();
    ///
    ///     let session = client.sign_in(EmailOrPhone::Email(email), &password).await?;
    ///     client.refresh_access_token(&session.refresh_token);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn refresh_access_token(
        &self,
        refresh_token: &str,
    ) -> Result<Session, reqwest::Error> {
        let endpoint = format!("{}/token?grant_type=refresh_token", self.url);
        let body = json!({ "refresh_token": refresh_token });

        let session: Session = self
            .client
            .post(endpoint)
            .headers(self.headers.clone())
            .json(&body)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        Ok(session)
    }

    /// Gets a user by access token
    ///
    /// # Example
    ///
    /// ```
    /// use go_true::{Api, EmailOrPhone};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let url = "http://localhost:9998".to_string();
    ///     let mut client = Api::new(url);
    ///
    ///
    ///     let email = "email@example.com".to_string();
    ///     let password = "Abcd1234!".to_string();
    ///
    ///     let session = client.sign_in(EmailOrPhone::Email(email), &password).await?;
    ///     let user = client.get_user(&session.access_token);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_user(&self, jwt: &str) -> Result<User, reqwest::Error> {
        let endpoint = format!("{}/user", self.url);

        let mut headers: HeaderMap = self.headers.clone();
        let bearer = format!("Bearer {jwt}");
        headers.insert(
            "Authorization",
            HeaderValue::from_str(bearer.as_ref()).expect("Invalid header value."),
        );

        let user: User = self
            .client
            .get(endpoint)
            .headers(headers)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        Ok(user)
    }

    /// Updates a user
    ///
    /// # Example
    ///
    /// ```
    /// use go_true::{Api, EmailOrPhone};
    /// use serde_json::json;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let url = "http://localhost:9998".to_string();
    ///     let mut client = Api::new(url);
    ///
    ///     let email = "email@example.com".to_string();
    ///     let password = "Abcd1234!".to_string();
    ///
    ///     client.sign_up(EmailOrPhone::Email(email.clone()), &password)
    ///         .await?;
    ///     let session = client.sign_in(EmailOrPhone::Email(email), &password).await?;
    ///
    ///     let new_email = "otheremail@example.com";
    ///     let attributes = UserAttributes {
    ///         email: new_email.clone(),
    ///         password: "Abcd12345!".to_string(),
    ///         data: json!({ "test": "test" }),
    ///     };
    ///
    ///     let updatedUser = client.update_user(attributes, &session.access_token).await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn update_user(
        &self,
        user: UserAttributes,
        jwt: &str,
    ) -> Result<UserUpdate, reqwest::Error> {
        let endpoint = format!("{}/user", self.url);

        let mut headers: HeaderMap = self.headers.clone();
        let bearer = format!("Bearer {jwt}");
        headers.insert(
            "Authorization",
            HeaderValue::from_str(bearer.as_ref()).expect("Invalid header value."),
        );

        let body = json!({"email": user.email, "password": user.password, "data": user.data});

        let user: UserUpdate = self
            .client
            .put(endpoint)
            .headers(headers)
            .json(&body)
            .send()
            .await?
            .error_for_status()?
            .json::<UserUpdate>()
            .await?;

        Ok(user)
    }

    /// Invites a user via email
    ///
    /// # Example
    ///
    /// ```
    /// use go_true::{Api, EmailOrPhone};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let url = "http://localhost:9998".to_string();
    ///     let mut client = Api::new(url);
    ///
    ///     let email = "email@example.com".to_string();
    ///
    ///     let user = client.invite_user_by_email(&email).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn invite_user_by_email(&self, email: &str) -> Result<User, reqwest::Error> {
        let endpoint = format!("{}/invite", self.url);

        let body = json!({
            "email": &email,
        });

        let user: User = self
            .client
            .post(endpoint)
            .headers(self.headers.clone())
            .json(&body)
            .send()
            .await?
            .error_for_status()?
            .json::<User>()
            .await?;

        Ok(user)
    }

    /// Lists all users based on a query string
    ///
    /// # Example
    ///
    /// ```
    /// use go_true::{Api, EmailOrPhone};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let url = "http://localhost:9998".to_string();
    ///     let mut client = Api::new(url);
    ///
    ///     let email = "email@example.com".to_string();
    ///     let password = "Abcd1234!".to_string();
    ///
    ///     client
    ///         .sign_up(EmailOrPhone::Email(email), &password)
    ///         .await?;
    ///
    ///     let users = client.list_users(None).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn list_users(
        &self,
        query_string: Option<String>,
    ) -> Result<UserList, reqwest::Error> {
        let endpoint = match query_string {
            Some(query) => format!("{}/admin/users{}", self.url, query),
            None => format!("{}/admin/users", self.url),
        };

        let users: UserList = self
            .client
            .get(endpoint)
            .headers(self.headers.clone())
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        Ok(users)
    }

    /// Gets a user by id
    ///
    /// # Example
    ///
    /// ```
    /// use go_true::{Api, EmailOrPhone};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let url = "http://localhost:9998".to_string();
    ///     let mut client = Api::new(url);
    ///
    ///     let email = "email@example.com".to_string();
    ///     let password = "Abcd1234!".to_string();
    ///
    ///     let session = client
    ///         .sign_up(EmailOrPhone::Email(email), &password)
    ///         .await?;
    ///
    ///     let user = client.get_user_by_id(&session.user.id).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_user_by_id(&self, user_id: &str) -> Result<User, reqwest::Error> {
        let endpoint = format!("{}/admin/users/{}", self.url, user_id);

        let user: User = self
            .client
            .get(endpoint)
            .headers(self.headers.clone())
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        Ok(user)
    }

    /// Updates a user by id
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
    ///         email: "oldemail@example.com",
    ///         password: Some(String::from("Abcd1234!")),
    ///         data: None,
    ///         email_confirmed: None,
    ///         phone_confirmed: None,
    ///     };
    ///
    ///     let create_response = client.create_user(user).await?;
    ///     let user = AdminUserAttributes {
    ///         email: "newemail@example.com".to_string(),
    ///         password: None,
    ///         data: None,
    ///         email_confirmed: None,
    ///         phone_confirmed: None,
    ///     };
    ///
    ///     let update_response = client
    ///         .update_user_by_id(&create_response.id, user.clone())
    ///         .await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn update_user_by_id<T: serde::Serialize>(
        &self,
        id: &str,
        user: T,
    ) -> Result<User, reqwest::Error> {
        let endpoint = format!("{}/admin/users/{}", self.url, id);

        let json = serde_json::to_value(&user).unwrap();

        let client = reqwest::Client::new();
        let user: User = client
            .put(endpoint)
            .headers(self.headers.clone())
            .json(&json)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        Ok(user)
    }

    /// Deletes a user by id
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
    ///         email: "delete@example.com",
    ///         password: Some(String::from("Abcd1234!")),
    ///         data: None,
    ///         email_confirmed: None,
    ///         phone_confirmed: None,
    ///     };

    ///     let user = client.create_user(user).await?;
    ///     client.delete_user(&user.id).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn delete_user(&self, user_id: &str) -> Result<bool, reqwest::Error> {
        let endpoint = format!("{}/admin/users/{}", self.url, user_id);

        self.client
            .delete(endpoint)
            .headers(self.headers.clone())
            .send()
            .await?
            .error_for_status()?;

        Ok(true)
    }
}

fn handle_gotrue_resp_status(status_code: StatusCode) -> Result<(), Error> {
    if status_code.is_success() {
        return Ok(());
    }
    Ok(())
}

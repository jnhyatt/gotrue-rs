use std::{collections::HashMap, default};

use serde::{Deserialize, Serialize};

/// Represents a User.
#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize, Serialize)]
pub struct User {
    /// The User's id.
    pub id: String,
    /// The User's email.
    pub email: String,
    /// The User's password.
    pub password: Option<String>,
    /// Additional data for the user.
    pub data: Option<HashMap<String, String>>,
    /// The User's audience.
    pub aud: String,
    /// The User's role.
    pub role: String,
    /// The date the User's email was confirmed, if confirmed.
    pub email_confirmed_at: Option<String>,
    /// The User's phone number.
    pub phone: String,
    /// The date the User's phone was confirmed, if confirmed.
    pub phone_confirmed: Option<bool>,
    /// The User's last login, if the user has logged in.
    pub last_sign_in_at: Option<String>,
    /// The date the User was created.
    pub created_at: String,
    /// The date the User was last updated.
    pub updated_at: String,
}

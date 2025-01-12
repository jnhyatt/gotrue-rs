#![deny(missing_docs)]
//! # go_true
//!
//! [GoTrue][gotrue] client-side library.
//!
//! ## Usage
//! Add the following line to your `Cargo.toml`:
//!
//! ```toml
//! go_true = "0.1.0"
//! ```
//!
//! ## Examples
//!
//! To create an account, create a new client and execute the `sign_up` function with email and password:
//!
//! ```rust
//! use go_true::{Client, EmailOrPhone};
//!
//! #[tokio::main]
//! async fn main() {
//!     let url = "http://localhost:9998".to_string();
//!     let mut client = Client::new(url);
//!
//!     let email = "email@example.com".to_string();
//!     let password = "Abcd1234!".to_string();
//!
//!     let session = client.sign_up(EmailOrPhone::Email(email), &password).await;
//!
//!     println!("{:?}", session);
//! }
//! ```
//!
//! Check out the [README][readme] for more info.
//!
//! [gotrue]: https://github.com/supabase/gotrue
//! [readme]: https://github.com/fubinator/gotrue-rs

mod client;
mod error;
mod session;
mod user;
mod user_attributes;
mod user_list;
mod user_update;

pub use client::settings::Settings;
pub use client::Client;
pub use client::EmailOrPhone;
pub use error::Error;
pub use user::User;
pub use user_attributes::UserAttributes;

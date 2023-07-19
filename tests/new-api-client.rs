use std::{collections::BTreeMap, error::Error};

use go_true_redux::{Client, Settings, User};

use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use rand::{distributions::Alphanumeric, Rng};
use sha2::Sha256;

use tracing::info;

fn get_service_api_client() -> Client {
    let key: Hmac<Sha256> = Hmac::new_from_slice(b"37c304f8-51aa-419a-a1af-06154e63707a").unwrap();
    let mut claims = BTreeMap::new();
    claims.insert("sub", "1234567890");
    claims.insert("role", "supabase_admin");

    let token_str = claims.sign_with_key(&key).unwrap();
    let api: Client = Client::new("http://localhost:9998")
        .with_header("Authorization", format!("Bearer {token_str}"));

    api
}

#[tokio::test]
async fn test_get_settings() -> anyhow::Result<()> {
    let client = get_service_api_client();
    let settings = client.get_settings().await?;
    info!("the settings returned were: {:?}", settings);
    assert_ne!(settings, Settings::default());

    Ok(())
}

#[tokio::test]
async fn it_should_create_user() -> anyhow::Result<()> {
    let client: Client = get_service_api_client();
    let email = get_random_email();
    let user = User {
        email: email.clone(),
        password: Some("Abcd1234!".to_owned()),
        data: None,
        email_confirmed_at: None,
        phone_confirmed: None,
        ..User::default()
    };

    let response = client.create_user(user).await?;

    assert_eq!(response.email, email);

    Ok(())
}

fn get_random_email() -> String {
    let random_string: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(|c| c.to_ascii_lowercase())
        .map(char::from)
        .collect();

    format!("{random_string}@example.com")
}

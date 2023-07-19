use std::collections::BTreeMap;

use go_true_redux::{Client, Settings};

use hmac::{Hmac, Mac};
use jwt::SignWithKey;
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
async fn test_get_settings() {
    tracing_subscriber::fmt::init();

    let client = get_service_api_client();
    let settings = client.get_settings().await.unwrap();
    info!("the settings returned were: {:?}", settings);
    assert_ne!(settings, Settings::default());
}

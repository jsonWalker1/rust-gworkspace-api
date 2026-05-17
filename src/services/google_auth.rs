use crate::config::Config;
use crate::models::payload::{Claims, OdooUser, OnboardingPayload};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use reqwest::Client;
use serde::Deserialize;

pub async fn get_user_info(config: &Config, id: i32) -> OdooUser {
    let client = Client::new();

    let url = format!(
        "{}/hr.employee/{}?fields=name,work_email,id",
        config.odoo_base_url.trim_end_matches('/'),
        id
    );

    let response = client
        .get(&url)
        .basic_auth(&config.odoo_username, Some(&config.odoo_password))
        .send()
        .await
        .unwrap();

    println!("STATUS: {:?}", response.status());

    response.json().await.unwrap()
}

pub fn map_to_onboarding(config: &Config, user: OdooUser) -> OnboardingPayload {
    OnboardingPayload {
        name: user.name,
        email: user.work_email,
        role: config.default_user_role.clone(),
        start_date: config.default_start_date.clone(),
    }
}

pub fn create_jwt(config: &Config) -> String {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize;

    let claims = Claims {
        iss: config.google_client_email.clone(),
        scope: "https://www.googleapis.com/auth/admin.directory.user".to_string(),
        aud: "https://oauth2.googleapis.com/token".to_string(),
        iat: now,
        exp: now + 3600,
        sub: config.google_delegated_user.clone(),
    };

    let header = Header::new(Algorithm::RS256);

    encode(
        &header,
        &claims,
        &EncodingKey::from_rsa_pem(config.google_private_key.as_bytes()).unwrap(),
    )
    .unwrap()
}

#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
}

pub async fn get_access_token(jwt: String) -> String {
    let client = Client::new();

    let params = [
        ("grant_type", "urn:ietf:params:oauth:grant-type:jwt-bearer"),
        ("assertion", &jwt),
    ];

    let response = client
        .post("https://oauth2.googleapis.com/token")
        .form(&params)
        .send()
        .await
        .unwrap();

    let status = response.status();
    let text = response.text().await.unwrap();

    println!("STATUS: {}", status);
    println!("RAW RESPONSE: {}", text);

    let res: TokenResponse = serde_json::from_str(&text).unwrap();
    res.access_token
}

pub async fn create_user(config: &Config, token: String, payload: OnboardingPayload) {
    let client = reqwest::Client::new();

    let parts: Vec<&str> = payload.name.split(' ').collect();

    let given = parts.first().copied().unwrap_or("User");
    let family = parts.get(1).copied().unwrap_or("");
    let local_part = payload.email.split('@').next().unwrap_or("user");
    let email = format!("{}@{}", local_part, config.google_user_domain);

    let body = serde_json::json!({
        "primaryEmail": email,
        "name": {
            "givenName": given,
            "familyName": family
        },
        "password": config.google_default_password,
        "changePasswordAtNextLogin": true
    });

    let res = client
        .post("https://admin.googleapis.com/admin/directory/v1/users")
        .bearer_auth(token)
        .json(&body)
        .send()
        .await
        .unwrap();

    let status = res.status();
    let text = res.text().await.unwrap();

    println!("STATUS: {:?}", status);
    println!("BODY: {}", text);
}

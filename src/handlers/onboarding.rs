use crate::config::Config;
use crate::models::payload::IdPayload;
use crate::services::google_auth::{
    create_jwt, create_user, get_access_token, get_user_info, map_to_onboarding,
};
use axum::extract::State;
use axum::Json;
use serde::Serialize;
use std::sync::Arc;

#[derive(Serialize)]
pub struct ApiResponse {
    pub status: String,
    pub token: String,
}

pub async fn handle_onboarding(
    State(config): State<Arc<Config>>,
    Json(payload): Json<IdPayload>,
) -> Json<ApiResponse> {
    println!("START handler");
    println!("Received payload: {:?}", payload);

    println!("Creating JWT...");
    let jwt = create_jwt(&config);
    println!("JWT created");

    println!("Getting access token...");
    let token = get_access_token(jwt).await;
    println!("Got access token");

    println!("Fetching user from Odoo with ID: {}", payload.id);
    let user_info = get_user_info(&config, payload.id).await;
    println!("Odoo user: {:?}", user_info);

    println!("Mapping to onboarding payload...");
    let onboarding_payload = map_to_onboarding(&config, user_info);
    println!("ONBOARDING: {:?}", onboarding_payload);

    println!("Creating Google user...");
    create_user(&config, token.clone(), onboarding_payload).await;
    println!("Google user created");

    println!("END handler");

    Json(ApiResponse {
        status: "ok".to_string(),
        token,
    })
}

use crate::config::Config;
use crate::handlers::onboarding::handle_onboarding;
use axum::{routing::post, Router};
use std::sync::Arc;

pub fn webhook_routes() -> Router<Arc<Config>> {
    Router::new().route("/webhook/odoo", post(handle_onboarding))
}

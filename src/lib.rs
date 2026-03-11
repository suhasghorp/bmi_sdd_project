// Library interface for testing
mod api;
mod domain;
mod ui;

use axum::{
    Router,
    routing::{get, post},
};

// Export create_app for integration tests
pub fn create_app() -> Router {
    Router::new()
        .route("/", get(ui::serve_homepage))
        .route("/api/bmi", post(api::calculate_bmi_handler))
        .route("/health", get(api::health_check))
}

// API layer: Axum handlers, JSON types, error mapping
use axum::{
    extract::Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};

use crate::domain::{BmiError, calculate_bmi};

// T025: BmiRequest struct with Deserialize
#[derive(Debug, Deserialize)]
pub struct BmiRequest {
    pub weight_kg: f64,
    pub height_m: f64,
}

// T026: BmiResponse struct with Serialize
#[derive(Debug, Serialize)]
pub struct BmiResponse {
    pub bmi: f64,
    pub category: String,
}

// T027: ErrorResponse struct with Serialize
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

// T028: Implement IntoResponse trait for BmiError
impl IntoResponse for BmiError {
    fn into_response(self) -> Response {
        let error_message = self.to_string();
        let body = Json(ErrorResponse {
            error: error_message,
        });
        (StatusCode::UNPROCESSABLE_ENTITY, body).into_response()
    }
}

// T029: POST /api/bmi handler with Json extractor
// T048: Add tracing spans for request handling
#[tracing::instrument(
    name = "calculate_bmi_request",
    skip(payload),
    fields(
        weight_kg = %payload.weight_kg,
        height_m = %payload.height_m
    )
)]
pub async fn calculate_bmi_handler(
    Json(payload): Json<BmiRequest>,
) -> Result<Json<BmiResponse>, BmiError> {
    tracing::debug!("Processing BMI calculation request");

    let result = calculate_bmi(payload.weight_kg, payload.height_m)?;

    tracing::debug!("BMI calculation completed successfully");

    tracing::info!(
        bmi = %result.bmi,
        category = %result.category,
        "BMI calculation successful"
    );

    Ok(Json(BmiResponse {
        bmi: result.bmi,
        category: result.category.to_string(),
    }))
}

// T030: GET /health handler returning 200 OK
#[tracing::instrument(name = "health_check")]
pub async fn health_check() -> impl IntoResponse {
    tracing::debug!("Health check request");
    StatusCode::OK
}

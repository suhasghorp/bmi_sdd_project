// Integration tests for BMI API endpoints
use bmi_sdd::create_app;
use reqwest;
use serde_json::json;

// Helper function to start test server on random port
async fn spawn_app() -> String {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();

    // Create the router
    let app = create_app();

    // Spawn the server in a background task
    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    // Give the server a moment to start
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    format!("http://127.0.0.1:{}", port)
}

// T020: Integration test for POST /api/bmi with valid inputs
#[tokio::test]
async fn test_api_bmi_valid_request() {
    let address = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .post(&format!("{}/api/bmi", address))
        .json(&json!({
            "weight_kg": 70.0,
            "height_m": 1.75
        }))
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(response.status(), 200);

    let body: serde_json::Value = response.json().await.expect("Failed to parse JSON");
    assert_eq!(body["bmi"], 22.9);
    assert_eq!(body["category"], "Normal");
}

// T021: Integration test for POST /api/bmi with zero weight returning 422
#[tokio::test]
async fn test_api_bmi_zero_weight() {
    let address = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .post(&format!("{}/api/bmi", address))
        .json(&json!({
            "weight_kg": 0.0,
            "height_m": 1.75
        }))
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(response.status(), 422);

    let body: serde_json::Value = response.json().await.expect("Failed to parse JSON");
    assert_eq!(body["error"], "weight_kg must be positive");
}

// T022: Integration test for POST /api/bmi with negative height returning 422
#[tokio::test]
async fn test_api_bmi_negative_height() {
    let address = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .post(&format!("{}/api/bmi", address))
        .json(&json!({
            "weight_kg": 70.0,
            "height_m": -1.75
        }))
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(response.status(), 422);

    let body: serde_json::Value = response.json().await.expect("Failed to parse JSON");
    assert_eq!(body["error"], "height_m must be positive");
}

// T023: Integration test for GET /health returning 200
#[tokio::test]
async fn test_health_check() {
    let address = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health", address))
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(response.status(), 200);
}

// T024: Integration test for malformed JSON request
#[tokio::test]
async fn test_api_bmi_malformed_json() {
    let address = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .post(&format!("{}/api/bmi", address))
        .header("Content-Type", "application/json")
        .body("{invalid json}")
        .send()
        .await
        .expect("Failed to execute request");

    // Axum's Json extractor returns 400 for malformed JSON
    assert_eq!(response.status(), 400);
}

// T039: Integration test for multiple validation scenarios
#[tokio::test]
async fn test_multiple_validation_scenarios() {
    let address = spawn_app().await;
    let client = reqwest::Client::new();

    // Test zero height
    let response = client
        .post(&format!("{}/api/bmi", address))
        .json(&json!({"weight_kg": 70.0, "height_m": 0.0}))
        .send()
        .await
        .expect("Failed to execute request");
    assert_eq!(response.status(), 422);
    let body: serde_json::Value = response.json().await.unwrap();
    assert_eq!(body["error"], "height_m must be positive");

    // Test negative weight
    let response = client
        .post(&format!("{}/api/bmi", address))
        .json(&json!({"weight_kg": -10.0, "height_m": 1.75}))
        .send()
        .await
        .expect("Failed to execute request");
    assert_eq!(response.status(), 422);
    let body: serde_json::Value = response.json().await.unwrap();
    assert_eq!(body["error"], "weight_kg must be positive");

    // Test both zero
    let response = client
        .post(&format!("{}/api/bmi", address))
        .json(&json!({"weight_kg": 0.0, "height_m": 0.0}))
        .send()
        .await
        .expect("Failed to execute request");
    assert_eq!(response.status(), 422);
    // Should fail on weight first
    let body: serde_json::Value = response.json().await.unwrap();
    assert_eq!(body["error"], "weight_kg must be positive");
}

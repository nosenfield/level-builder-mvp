//! Backend API server for Roblox Level Builder.
//!
//! Axum HTTP server providing the `/api/export` endpoint.
//! Accepts Space JSON from frontend, validates it, generates `.rbxlx` file, and returns it.

use axum::{
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use serde_json::json;
use std::env;
use tower_http::cors::{Any, CorsLayer};

use backend::models::SpaceJSON;
use backend::rbxlx::generate_rbxlx;
use backend::validation::validate_space_json;

/// API error response containing an error code and human-readable message.
#[derive(Debug)]
struct ApiError {
    error: String,
    message: String,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let body = json!({
            "error": self.error,
            "message": self.message
        });
        (StatusCode::BAD_REQUEST, Json(body)).into_response()
    }
}

/// Handles POST `/api/export` requests.
///
/// Validates the incoming Space JSON, generates a `.rbxlx` file, and returns it
/// as a downloadable attachment.
async fn export_handler(Json(payload): Json<SpaceJSON>) -> Result<Response, ApiError> {
    // Validate Space JSON before processing
    if let Err(validation_error) = validate_space_json(&payload) {
        return Err(ApiError {
            error: validation_error.error_code().to_string(),
            message: validation_error.message(),
        });
    }

    // Generate .rbxlx file from validated Space JSON
    let rbxlx_content = match generate_rbxlx(&payload) {
        Ok(content) => content,
        Err(e) => {
            return Err(ApiError {
                error: "RBXLX_GENERATION_FAILED".to_string(),
                message: format!("Failed to generate .rbxlx file: {}", e),
            });
        }
    };

    // Return file with proper headers
    let response = Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/octet-stream")
        .header(
            header::CONTENT_DISPOSITION,
            format!(r#"attachment; filename="level.rbxlx""#),
        )
        .body(rbxlx_content.into())
        .map_err(|e| ApiError {
            error: "RESPONSE_BUILD_FAILED".to_string(),
            message: format!("Failed to build response: {}", e),
        })?;

    Ok(response)
}

/// Handles GET `/health` requests.
///
/// Returns a simple health check response for deployment platforms (e.g., Railway).
async fn health_handler() -> impl IntoResponse {
    Json(json!({
        "status": "ok",
        "service": "roblox-level-builder-backend"
    }))
}

/// Initializes and runs the HTTP server.
///
/// Configures CORS for cross-origin requests and binds to the port specified
/// by the `PORT` environment variable (defaults to 4000).
#[tokio::main]
async fn main() {
    let port = env::var("PORT")
        .unwrap_or_else(|_| "4000".to_string())
        .parse::<u16>()
        .expect("PORT must be a valid u16");

    // CORS: allow all origins for development (restrict in production)
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/health", get(health_handler))
        .route("/api/health", get(health_handler))
        .route("/api/export", post(export_handler))
        .layer(cors);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .expect("Failed to bind to address");

    println!("🚀 Backend server running on http://0.0.0.0:{}", port);
    println!("📡 Export endpoint: POST http://localhost:{}/api/export", port);
    println!("❤️  Health endpoint: GET http://localhost:{}/health", port);

    axum::serve(listener, app)
        .await
        .expect("Server failed to start");
}

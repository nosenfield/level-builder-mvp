/**
 * Phase 5: Backend API Setup
 * 
 * Axum HTTP server with CORS and /api/export endpoint.
 * Accepts Space JSON from frontend and returns .rbxlx file.
 */

use axum::{
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    routing::post,
    Json, Router,
};
use serde_json::json;
use std::env;
use tower_http::cors::{Any, CorsLayer};

mod models;

use models::SpaceJSON;

/// Error response structure
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

/// Export endpoint handler
async fn export_handler(Json(payload): Json<SpaceJSON>) -> Result<Response, ApiError> {
    // For Phase 5, we return a placeholder .rbxlx file
    // Actual RBXLX generation will be implemented in Phase 7
    
    // Validate schema version (must be 1 for MVP)
    if payload.schema_version != 1 {
        return Err(ApiError {
            error: "INVALID_SCHEMA_VERSION".to_string(),
            message: format!("Unsupported schema version: {}. Only version 1 is supported.", payload.schema_version),
        });
    }

    // Create placeholder .rbxlx content
    // This is a minimal valid XML structure that Roblox Studio can recognize
    // Full generation will be in Phase 7
    let rbxlx_content = format!(
        r#"<?xml version="1.0" encoding="utf-8"?>
<roblox xmlns:xmime="http://www.w3.org/2005/05/xmlmime" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:noNamespaceSchemaLocation="http://www.roblox.com/roblox.xsd" version="4">
  <Meta name="ExplicitAutoJoints">true</Meta>
  <External>null</External>
  <External>nil</External>
  <Item class="DataModel" referent="RBX0">
    <Properties>
      <string name="Name">Level</string>
    </Properties>
  </Item>
</roblox>"#,
    );

    // Return file with proper headers
    let response = Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/octet-stream")
        .header(
            header::CONTENT_DISPOSITION,
            format!(r#"attachment; filename="level.rbxlx""#),
        )
        .body(rbxlx_content.into())
        .unwrap();

    Ok(response)
}

/// Main server setup
#[tokio::main]
async fn main() {
    // Get port from environment or default to 4000
    let port = env::var("PORT")
        .unwrap_or_else(|_| "4000".to_string())
        .parse::<u16>()
        .expect("PORT must be a valid u16");

    // Configure CORS - allow all origins for MVP development
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Build router
    let app = Router::new()
        .route("/api/export", post(export_handler))
        .layer(cors);

    // Bind and serve
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .expect("Failed to bind to address");

    println!("🚀 Backend server running on http://0.0.0.0:{}", port);
    println!("📡 Export endpoint: POST http://localhost:{}/api/export", port);

    axum::serve(listener, app)
        .await
        .expect("Server failed to start");
}

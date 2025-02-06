use std::ops::Deref;

use crate::app_register::{register, unregister};
use crate::beam_app::{BeamAppDelete, BeamAppPost};
use crate::security::validate_api_key;
use axum::routing::{get, post};
use axum::{http::StatusCode, middleware, response::IntoResponse, Json, Router};
use tracing::{info, error};

// Function to set up routes
pub fn create_router() -> Router {
    let api_routes = Router::new()
        .route("/beam-app", post(post_beam_app).delete(delete_beam_app))
        .layer(middleware::from_fn(validate_api_key)); // Middleware applied only to these routes

    let public_routes = Router::new()
        .route("/info", get(get_info)); // No middleware applied

    api_routes.merge(public_routes) // Combine both sets of routes
}

// POST /beam-app handler
async fn post_beam_app(
    Json(payload): Json<BeamAppPost>,
) -> Result<String, axum::response::Response> {
    let beam_id = payload.beam_id;
    match register(beam_id.clone(), payload.beam_secret).await {
        Ok(_) => {
            let message = format!("Beam ID {} registered", beam_id.deref());
            info!(message);
            Ok(message)
        }
        Err(e) => {
            let message = format!("Error registering app: {e}");
            error!(message);
            Err(send_error_response(StatusCode::INTERNAL_SERVER_ERROR, message))
        }
    }
}

// DELETE /beam-app handler
async fn delete_beam_app(
    Json(payload): Json<BeamAppDelete>,
) -> Result<String, axum::response::Response> {
    match unregister(payload.beam_id.clone()).await {
        Ok(_) => {
            let message = format!("Beam ID {} unregistered", payload.beam_id.deref());
            info!(message);
            Ok(message)
        }
        Err(e) => {
            let message = format!("Error unregistering app: {e}");
            error!(message);
            Err(send_error_response(StatusCode::INTERNAL_SERVER_ERROR, message))
        }
    }
}

// New GET /info handler
async fn get_info() -> Json<serde_json::Value> {
    let version = env!("CARGO_PKG_VERSION"); // Gets the version from Cargo.toml
    Json(serde_json::json!({
        "version": version,
        "description": "The Beam Register service enables other components to seamlessly register and unregister app IDs within the Beam ecosystem."
    }))
}

// Helper function to create a response with both status and error message
fn send_error_response(status: StatusCode, message: String) -> axum::response::Response {
    (status, Json(message)).into_response() // Return the response with the status and the message in JSON format
}

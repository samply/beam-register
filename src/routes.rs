use std::ops::Deref;

use crate::app_register::{register, unregister};
use crate::beam_app::{BeamAppDelete, BeamAppPost};
use crate::security::validate_api_key;
use axum::routing::post;
use axum::{http::StatusCode, middleware, response::IntoResponse, Json, Router};

// Function to set up routes
pub fn create_router() -> Router {
    Router::new()
        .route("/beam-app", post(post_beam_app).delete(delete_beam_app)) // No need for middleware
        .layer(middleware::from_fn(validate_api_key))
}

// POST /beam-app handler
async fn post_beam_app(
    Json(payload): Json<BeamAppPost>,
) -> Result<String, axum::response::Response> {
    let beam_id = payload.beam_id;
    match register(beam_id.clone(), payload.beam_secret).await {
        Ok(_) => Ok(format!("Received POST with beam_id: {}", beam_id.deref())),
        Err(e) => Err(send_error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error registering app: {e}"),
        )),
    }
}

// DELETE /beam-app handler
async fn delete_beam_app(
    Json(payload): Json<BeamAppDelete>,
) -> Result<String, axum::response::Response> {
    match unregister(payload.beam_id.clone()).await {
        Ok(_) => Ok(format!(
            "Received DELETE with beam_id: {}",
            payload.beam_id.deref()
        )),
        Err(e) => Err(send_error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error unregistering app: {e}"),
        )),
    }
}

// Helper function to create a response with both status and error message
fn send_error_response(status: StatusCode, message: String) -> axum::response::Response {
    (status, Json(message)).into_response() // Return the response with the status and the message in JSON format
}

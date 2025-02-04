use axum::{http::StatusCode, Json, Router, response::IntoResponse, middleware};
use crate::beam_app::{BeamApp, BeamAppError};
use crate::security::validate_api_key;
use crate::app_register::{register, unregister};

// Function to set up routes
pub fn create_router() -> Router {
    Router::new()
        .route("/beam-app", axum::routing::post(post_beam_app))  // No need for middleware
        .route("/beam-app", axum::routing::delete(delete_beam_app))  // No need for middleware
        .layer(middleware::from_fn(validate_api_key))
}

// POST /beam-app handler
async fn post_beam_app(Json(payload): Json<BeamApp>) -> Result<String, axum::response::Response> {
    match payload.validate() {
        Ok(_) => {
            let beam_id = payload.beam_id.unwrap();
            match register(beam_id.clone(), payload.beam_secret.unwrap()).await {
                Ok(_) => Ok(format!("Received POST with beam_id: {}", beam_id)),
                Err(e) => {
                    let (status, message) = handle_error_from_register(e);
                    Err(send_error_response(status, message))
                }
            }
        },
        Err(e) => {
            let (status, message) = handle_input_error(e);
            Err(send_error_response(status, message)) // Return error response
        }
    }
}

// DELETE /beam-app handler
async fn delete_beam_app(Json(payload): Json<BeamApp>) -> Result<String, axum::response::Response> {
    match payload.is_valid_beam_id() {
        Ok(_) => {
            let beam_id = payload.beam_id.unwrap();
            match unregister(beam_id.clone()).await {
                Ok(_) => Ok(format!("Received DELETE with beam_id: {}", beam_id)),
                Err(e) => {
                    let (status, message) = handle_error_from_unregister(e);
                    Err(send_error_response(status, message))
                }
            }
        },
        Err(e) => {
            let (status, message) = handle_input_error(e);
            Err(send_error_response(status, message)) // Return error response
        }
    }
}

// Helper function to handle errors from the register function
fn handle_error_from_register(error: std::io::Error) -> (StatusCode, String) {
    // Customize this mapping to the specific errors you expect from the register function
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("Error registering app: {}", error),
    )
}

// Helper function to handle errors from the unregister function
fn handle_error_from_unregister(error: std::io::Error) -> (StatusCode, String) {
    // Customize this mapping to the specific errors you expect from the unregister function
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("Error unregistering app: {}", error),
    )
}

// Helper function to map BeamAppError to StatusCode and message
fn handle_input_error(error: BeamAppError) -> (StatusCode, String) {
    match error {
        BeamAppError::MissingBeamId => (
            StatusCode::BAD_REQUEST,
            "beam_id is missing".to_string(),
        ),
        BeamAppError::MissingBeamSecret => (
            StatusCode::BAD_REQUEST,
            "beam_secret is missing or empty".to_string(),
        ),
        BeamAppError::InvalidBeamIdFormat => (
            StatusCode::BAD_REQUEST,
            "beam_id contains invalid characters (only alphanumeric allowed)".to_string(),
        ),
    }
}

// Helper function to create a response with both status and error message
fn send_error_response(status: StatusCode, message: String) -> axum::response::Response {
    (status, Json(message)).into_response() // Return the response with the status and the message in JSON format
}

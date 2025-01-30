use axum::{routing::get, Router, Json};
use serde::{Deserialize, Serialize};

// Define a simple data structure
#[derive(Serialize, Deserialize)]
pub struct Item {
    pub id: u32,
    pub name: String,
}

// Handler for a GET request
pub async fn get_items() -> Json<Vec<Item>> {
    let items = vec![
        Item { id: 1, name: "Item 1".to_string() },
        Item { id: 2, name: "Item 2".to_string() },
    ];
    Json(items)
}

// Handler for a POST request
pub async fn create_item(Json(payload): Json<Item>) -> Json<Item> {
    Json(payload)
}

// Function to set up routes
pub fn create_router() -> Router {
    Router::new()
        .route("/items", get(get_items))
        .route("/items", axum::routing::post(create_item))
}

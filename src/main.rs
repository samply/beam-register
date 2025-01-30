use tokio::net::TcpListener;

mod routes;

#[tokio::main]
async fn main() {
    // Build the application using routes from the `routes` module
    let app = routes::create_router();

    // Bind to a specific address and port
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server running at http://0.0.0.0:3000");

    // Use `axum::serve` to run the server
    axum::serve(listener, app).await.unwrap();
}

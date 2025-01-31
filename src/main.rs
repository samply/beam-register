use tokio::net::TcpListener;
use crate::environment_variables::EnvironmentVariable;

mod routes;
mod environment_variables;
mod beam_app;

#[tokio::main]
async fn main() {
    // Build the application using routes from the `routes` module
    let app = routes::create_router();
    let host = EnvironmentVariable::Host.get_env_var();
    // Ensure no protocol is included in the host
    if host.starts_with("http://") || host.starts_with("https://") {
        panic!("Host should not include the protocol (e.g., 'http://').");
    }
    let port = EnvironmentVariable::Port.get_env_var();
    let address = format!("{}:{}", host, port);
    // Bind to a specific address and port
    let listener = TcpListener::bind(address.clone()).await.unwrap();
    println!("Server running at http://{}", address );

    // Use `axum::serve` to run the server
    axum::serve(listener, app).await.unwrap();
}

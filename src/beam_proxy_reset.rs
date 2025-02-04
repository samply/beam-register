use crate::environment_variables::EnvironmentVariable;
use bollard::Docker;
use log::{error, info, debug};
use once_cell::sync::Lazy;
use std::sync::Mutex;
use bollard::container::{ListContainersOptions};


// This will hold the Docker client in a lazy manner
static DOCKER_CLIENT: Lazy<Mutex<Option<Docker>>> = Lazy::new(|| Mutex::new(None));

pub async fn create_docker_client() -> Result<Docker, Box<dyn std::error::Error>> {
    let mut docker_client = DOCKER_CLIENT.lock().unwrap();

    // If Docker client has not been initialized yet, initialize it
    if let Some(ref docker) = *docker_client {
        return Ok(docker.clone());
    }

    // Attempt to get Docker host from the environment variable
    let docker_host = match EnvironmentVariable::DockerHost.catch_env_var() {
        Some(host) if !host.is_empty() => host,
        _ => {
            // Fallback to Unix socket if Docker host is not set or is empty
            info!("Docker host not set or empty, falling back to Unix socket.");
            let docker_unix_socket = EnvironmentVariable::DockerUnixSocket.get_env_var();
            info!("Using Docker Unix socket: {}", docker_unix_socket);
            let client = Docker::connect_with_unix(&docker_unix_socket, 180, bollard::API_DEFAULT_VERSION)?;
            *docker_client = Some(client.clone());
            return Ok(client);
        }
    };

    info!("Using Docker host: {}", docker_host);

    // If docker_host is provided and starts with "tcp://", connect to Docker via TCP
    if docker_host.starts_with("tcp://") {
        let client = Docker::connect_with_http(&docker_host, 180, bollard::API_DEFAULT_VERSION)?;
        *docker_client = Some(client.clone());
        Ok(client)
    } else {
        error!("Invalid Docker host format: {}", docker_host);
        Err("Invalid Docker host format. Expected 'tcp://...'.".into())
    }
}


// Function to list Docker containers and print their names
async fn list_docker_containers(docker_client: &Docker) -> Result<(), Box<dyn std::error::Error>> {
    let options = ListContainersOptions::<String> {
        all: true, // Optional: Set to false to list only running containers
        ..Default::default()
    };

    // List containers
    let containers = docker_client.list_containers(Some(options)).await?;

    // Print the names of all containers
    debug!("List of containers:");
    for container in containers {
        debug!("Container-id: {}", container.id.unwrap());
        debug!("Container Name: {}", container.names.unwrap()[0]);
    }

    Ok(())
}

// Your existing reset_beam_proxy function calling the list_docker_containers function
pub async fn reset_beam_proxy() -> Result<(), Box<dyn std::error::Error>> {
    // Get the container name from the environment variable using the `get_env_var` function
    let container_name = EnvironmentVariable::BeamProxyContainerName.get_env_var();

    if container_name.is_empty() {
        error!("Beam Proxy container name is not specified!");
        return Err("Beam Proxy container name is not specified!".into());
    }

    // Get the Docker client
    let docker_client = create_docker_client().await?;

    // Call the function to list containers for debugging (without handling its result in this function)
    list_docker_containers(&docker_client).await?;


    // Restart the container
    match docker_client.restart_container(&container_name, None).await {
        Ok(_) => {
            info!("Successfully restarted container: {}", container_name);
            Ok(())
        }
        Err(err) => {
            error!("Failed to restart container {}: {:?}", container_name, err);
            Err(Box::new(err))
        }
    }

}

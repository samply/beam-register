use std::process::{Command, Stdio};
use tokio::process::Command as TokioCommand;
use tokio::io::{self, AsyncReadExt};
use tracing::{error, info, instrument};
use tracing_subscriber;
use crate::CONFIG;

#[instrument]
pub async fn reset_beam_proxy() -> Result<(), String> {
    let service_name = &CONFIG.beam_proxy_container_name;
    info!("Starting docker-compose up for service: {}", service_name);

    // Use tokio's async Command to run docker-compose asynchronously
    let mut command = TokioCommand::new("docker-compose")
        .args(&["up", "--force-recreate", service_name])
        .stdout(Stdio::piped()) // Capture stdout
        .stderr(Stdio::piped()) // Capture stderr
        .spawn();

    match command {
        Ok(mut child) => {
            let mut stdout = child.stdout.take().expect("Failed to capture stdout");
            let mut stderr = child.stderr.take().expect("Failed to capture stderr");

            // Buffers to capture the output
            let mut stdout_output = Vec::new();
            let mut stderr_output = Vec::new();

            // Asynchronously read stdout
            let stdout_task = tokio::spawn(async move {
                if let Err(e) = io::copy(&mut stdout, &mut stdout_output).await {
                    error!("Error reading stdout: {}", e);
                }
            });

            // Asynchronously read stderr
            let stderr_task = tokio::spawn(async move {
                if let Err(e) = io::copy(&mut stderr, &mut stderr_output).await {
                    error!("Error reading stderr: {}", e);
                }
            });

            // Wait for the process to finish
            let status = child.wait().await;

            // Wait for the reading tasks to finish
            stdout_task.await.unwrap();
            stderr_task.await.unwrap();

            match status {
                Ok(status) if status.success() => {
                    //let stdout_str = String::from_utf8_lossy(&stdout_output);
                    //info!("Successfully recreated service '{}':\n{}", service_name, stdout_str);
                    info!("Successfully recreated service '{}':\n", service_name);
                    Ok(())
                }
                Ok(status) => {
                    // let stderr_str = String::from_utf8_lossy(&stderr_output);
                    // error!(
                    //     "Failed to recreate service '{}'. Process exited with status: {:?}\n{}",
                    //     service_name, status, stderr_str
                    // );
                    // Err(stderr_str.to_string())
                    error!(
                        "Failed to recreate service '{}'. Process exited with status: {:?}\n",
                        service_name, status
                    );
                    Err("Failed to recreate service".into())
                }
                Err(err) => {
                    error!("Failed to execute docker-compose: {}", err);
                    Err(err.to_string())
                }
            }
        }
        Err(err) => {
            error!("Failed to spawn docker-compose process: {}", err);
            Err(err.to_string())
        }
    }
}

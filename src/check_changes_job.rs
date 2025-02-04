use crate::environment_variables::EnvironmentVariable;
use log::{debug};
use std::collections::hash_map::DefaultHasher;
use std::fs;
use std::hash::{Hash, Hasher};
use tokio_cron_scheduler::{Job, JobScheduler};

static mut LAST_CHECKSUM: Option<u64> = None;

pub async fn check_beam_file_changes_job() {
    // Create the job scheduler
    let scheduler = JobScheduler::new().await.unwrap();

    // Get the cron expression from the environment
    let cron_expression = EnvironmentVariable::BeamFileChangeCheckCronExpression.get_env_var();

    // Add a job that runs based on the cron expression
    scheduler.add(
        Job::new_async(cron_expression, |uuid, mut l| {
            Box::pin(async move {
                debug!("Cron job triggered, checking file changes...");

                // Trigger the job (e.g., check the file and possibly restart Docker)
                monitor_and_check().await;

                // Query the next execution time for this job
                let next_tick = l.next_tick_for_job(uuid).await;
                match next_tick {
                    Ok(Some(ts)) => debug!("Next time for job is {:?}", ts),
                    _ => debug!("Could not get next tick for the job"),
                }
            })
        }).unwrap()
    ).await.unwrap();

    // Start the scheduler
    scheduler.start().await.unwrap();

    // Keep the application running to allow the scheduled jobs to execute
    tokio::signal::ctrl_c().await.unwrap();
}

async fn monitor_and_check() {
    let new_checksum = calculate_checksum();

    // Check for changes in checksum
    unsafe {
        if let Some(last_checksum) = LAST_CHECKSUM {
            if new_checksum != last_checksum {
                // File has changed, trigger Docker container restart
                restart_beam_proxy().await;
            }
        }

        // Update global checksum
        LAST_CHECKSUM = Some(new_checksum);
    }
}

fn calculate_checksum() -> u64 {
    let content = fs::read_to_string(EnvironmentVariable::BeamFilePath.get_env_var()).expect("Unable to read file");
    let mut hasher = DefaultHasher::new();
    content.hash(&mut hasher);
    hasher.finish()
}

async fn restart_beam_proxy() {
    println!("Restarting beam proxy ...");
}

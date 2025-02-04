use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::fs::{self, OpenOptions};
use tokio::sync::Mutex;
use tokio_stream::wrappers::LinesStream; // Converts Lines into a Stream
use futures::StreamExt; // For using `next` with Streams
use lazy_static::lazy_static;
use std::path::Path;
use crate::environment_variables::EnvironmentVariable;

lazy_static! {
    static ref FILE_LOCK: Mutex<()> = Mutex::new(());
    static ref BEAM_API_KEY_FORMAT: String = EnvironmentVariable::BeamAppKeyFormat.get_env_var();
    static ref FILE_PATH: String = EnvironmentVariable::BeamFilePath.get_env_var();
}

/// Registers an app by adding a line `BEAM_API_KEY_FORMAT` to the file.
/// Returns `Ok(())` if successful, or an `Err` if an I/O error occurs.
pub async fn register(app_id: String, app_secret: String) -> io::Result<()> {
    let line_to_add = format!("{}='{}'\n",
                              BEAM_API_KEY_FORMAT.replace("{}", &app_id),
                              app_secret
    );

    // Lock to ensure exclusive access to the file
    let _lock = FILE_LOCK.lock().await;

    // Check if the app_id already exists
    if contains_app_id(&app_id).await? {
        return Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            "App ID already registered",
        ));
    }

    // Open the file in append mode (creates the file if it doesn't exist)
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(Path::new(&*FILE_PATH))
        .await?;

    // Check if the file is empty
    let metadata = fs::metadata(Path::new(&*FILE_PATH)).await?;
    let file_is_empty = metadata.len() == 0;

    // If the file is not empty, we check for the last line
    if !file_is_empty {
        let mut file_reader = BufReader::new(file.try_clone().await?);
        let mut last_line = String::new();
        if let Ok(_) = file_reader.read_line(&mut last_line).await {
            // If the last line doesn't end with a newline, we write one before appending
            if !last_line.ends_with('\n') {
                file.write_all(b"\n").await?;
            }
        }
    }

    // Now write the new line to the file
    file.write_all(line_to_add.as_bytes()).await?;
    Ok(())
}





/// Unregisters an app by removing the line `BEAM_API_KEY_FORMAT` from the file.
/// Returns `Ok(())` if successful, or an `Err` if an I/O error occurs.
pub async fn unregister(app_id: String) -> io::Result<()> {
    let _lock = FILE_LOCK.lock().await;

    let file_exists = Path::new(&*FILE_PATH).exists();
    if !file_exists {
        return Err(io::Error::new(io::ErrorKind::NotFound, "File not found"));
    }

    let file = OpenOptions::new().read(true).open(Path::new(&*FILE_PATH)).await?;
    let reader = BufReader::new(file);
    let lines = reader.lines();

    // Convert `lines` to a Stream using `LinesStream`
    let mut stream = LinesStream::new(lines);

    // Collect all lines except the ones matching the `app_id`
    let mut filtered_lines = Vec::new();
    while let Some(line_result) = stream.next().await {
        let line = line_result?;
        let key_to_match = BEAM_API_KEY_FORMAT.replace("{}", &app_id);
        if !line.starts_with(&key_to_match) {
            filtered_lines.push(line);
        }
    }

    // Write the filtered lines back to the file
    fs::write(Path::new(&*FILE_PATH), filtered_lines.join("\n") + "\n").await?;
    Ok(())
}

/// Helper function to check if the given app_id already exists in the file.
/// Returns `true` if the app_id exists, otherwise `false`.
async fn contains_app_id(app_id: &str) -> io::Result<bool> {
    let file_exists = Path::new(&*FILE_PATH).exists();
    if !file_exists {
        return Ok(false);
    }

    let file = OpenOptions::new().read(true).open(Path::new(&*FILE_PATH)).await?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    while let Some(line) = lines.next_line().await? {
        let key_to_match = BEAM_API_KEY_FORMAT.replace("{}", app_id);
        if line.starts_with(&key_to_match) {
            return Ok(true);
        }
    }
    Ok(false)
}

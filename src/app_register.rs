use tokio::fs::{File, OpenOptions};
use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader};

use crate::CONFIG;

/// Registers an app by adding a line `BEAM_API_KEY_FORMAT` to the file.
/// Returns `Ok(())` if successful, or an `Err` if an I/O error occurs.
pub async fn register(app_id: String, app_secret: String) -> io::Result<()> {
    let line_to_add = format!("{}='{}'\n",
                              CONFIG.beam_app_key_format.replace("{}", &app_id),
                              app_secret
    );

    let mut file = OpenOptions::new()
        .append(true)
        .read(true)
        .open(&CONFIG.beam_file_path)
        .await?;

    // Check if the app_id already exists
    if contains_app_id(&app_id, &mut file).await? {
        return Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            "App ID already registered",
        ));
    }

    // Now write the new line to the file
    file.write_all(line_to_add.as_bytes()).await?;
    file.flush().await?;
    Ok(())
}

/// Unregisters an app by removing the line `BEAM_API_KEY_FORMAT` from the file.
/// Returns `Ok(())` if successful, or an `Err` if an I/O error occurs.
pub async fn unregister(app_id: String) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .read(true)
        .open(&CONFIG.beam_file_path)
        .await?;

    let reader = BufReader::new(&mut file);
    // Collect all lines except the ones matching the `app_id`
    let key_to_match = CONFIG.beam_app_key_format.replace("{}", &app_id);
    let mut filtered_lines = Vec::new();
    let mut lines = reader.lines();
    while let Some(line) = lines.next_line().await? {
        if !line.starts_with(&key_to_match) {
            filtered_lines.push(line);
        }
    }

    // Write the filtered lines back to the file
    file.set_len(0).await?;
    file.write_all((filtered_lines.join("\n") + "\n").as_bytes()).await?;
    Ok(())
}

/// Helper function to check if the given app_id already exists in the file.
/// Returns `true` if the app_id exists, otherwise `false`.
async fn contains_app_id(app_id: &str, file: &mut File) -> io::Result<bool> {
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let key_to_match = CONFIG.beam_app_key_format.replace("{}", app_id);
    while let Some(line) = lines.next_line().await? {
        if line.starts_with(&key_to_match) {
            return Ok(true);
        }
    }
    Ok(false)
}

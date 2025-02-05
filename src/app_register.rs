use std::fs::{self, File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::ops::DerefMut;
use tokio::sync::Mutex;
use std::sync::LazyLock;

use crate::CONFIG;

static FILE_LOCK: LazyLock<Mutex<fs::File>> = LazyLock::new(|| {
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(CONFIG.beam_file_path.clone())
        .expect("Failed to open file");
    Mutex::new(file)
});

/// Registers an app by adding a line `BEAM_API_KEY_FORMAT` to the file.
/// Returns `Ok(())` if successful, or an `Err` if an I/O error occurs.
pub async fn register(app_id: String, app_secret: String) -> io::Result<()> {
    let line_to_add = format!("{}='{}'\n",
                              CONFIG.beam_app_key_format.replace("{}", &app_id),
                              app_secret
    );

    // Lock to ensure exclusive access to the file
    let mut file = FILE_LOCK.lock().await;

    // Check if the app_id already exists
    if contains_app_id(&app_id, &mut file)? {
        return Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            "App ID already registered",
        ));
    }

    // Now write the new line to the file
    file.write_all(line_to_add.as_bytes())?;
    file.write(b"\n")?;
    file.flush()?;
    Ok(())
}





/// Unregisters an app by removing the line `BEAM_API_KEY_FORMAT` from the file.
/// Returns `Ok(())` if successful, or an `Err` if an I/O error occurs.
pub async fn unregister(app_id: String) -> io::Result<()> {
    let mut file = FILE_LOCK.lock().await;

    let reader = BufReader::new(file.deref_mut());
    // Collect all lines except the ones matching the `app_id`
    let key_to_match = CONFIG.beam_app_key_format.replace("{}", &app_id);
    let mut filtered_lines = Vec::new();
    for line_result in reader.lines() {
        let line = line_result?;
        if !line.starts_with(&key_to_match) {
            filtered_lines.push(line);
        }
    }

    // Write the filtered lines back to the file
    file.set_len(0)?;
    file.write_all((filtered_lines.join("\n") + "\n").as_bytes())?;
    Ok(())
}

/// Helper function to check if the given app_id already exists in the file.
/// Returns `true` if the app_id exists, otherwise `false`.
fn contains_app_id(app_id: &str, mut lock: &mut File) -> io::Result<bool> {
    let reader = BufReader::new(lock.deref_mut());
    let mut lines = reader.lines();

    let key_to_match = CONFIG.beam_app_key_format.replace("{}", app_id);
    while let Some(line) = lines.next() {
        if line?.starts_with(&key_to_match) {
            return Ok(true);
        }
    }
    Ok(false)
}

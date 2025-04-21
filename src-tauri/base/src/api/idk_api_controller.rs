use anyhow::Error;

#[tauri::command]
pub fn commit_edit_changes() -> Result<(), Error> {
    Ok(())
}

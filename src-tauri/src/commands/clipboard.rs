#[tauri::command]
pub async fn copy_to_clipboard(text: String) -> Result<(), String> {
    let mut ctx = arboard::Clipboard::new().map_err(|e| e.to_string())?;
    ctx.set_text(text).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn paste_text(text: String) -> Result<(), String> {
    let mut ctx = arboard::Clipboard::new().map_err(|e| e.to_string())?;
    let previous = ctx.get_text().unwrap_or_default();

    ctx.set_text(&text).map_err(|e| e.to_string())?;
    ctx.set_text(previous).map_err(|e| e.to_string())?;
    Ok(())
}

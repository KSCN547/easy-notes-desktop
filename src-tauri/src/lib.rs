use std::fs;
use std::path::Path;
use tauri::Manager;

#[tauri::command]
fn get_default_save_path(app: tauri::AppHandle) -> String {
    let path = app.path().document_dir().unwrap_or_else(|_| std::path::PathBuf::from("C:/"));
    let full_path = path.join("EasyNotes_Data");
    if !full_path.exists() {
        fs::create_dir_all(&full_path).unwrap_or_default();
    }
    full_path.to_string_lossy().into_owned()
}

#[tauri::command]
fn save_note(path: String, title: String, content: String) -> Result<String, String> {
    let directory = Path::new(&path);
    if !directory.exists() {
        fs::create_dir_all(directory).map_err(|e| e.to_string())?;
    }
    
    let file_path = directory.join(format!("{}.md", title));
    fs::write(&file_path, content).map_err(|e| e.to_string())?;
    Ok(format!("成功保存: {}", title))
}

#[tauri::command]
fn list_notes(path: String) -> Vec<String> {
    let mut notes = Vec::new();
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() && path.extension().map_or(false, |ext| ext == "md") {
                if let Some(file_name) = path.file_stem().and_then(|s| s.to_str()) {
                    notes.push(file_name.to_string());
                }
            }
        }
    }
    notes.sort();
    notes
}

#[tauri::command]
fn read_note(path: String, title: String) -> Result<String, String> {
    let file_path = Path::new(&path).join(format!("{}.md", title));
    fs::read_to_string(file_path).map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            get_default_save_path,
            save_note, 
            list_notes, 
            read_note
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use tauri_plugin_http::reqwest;

#[tauri::command]
async fn fetch(url: &str) -> Result<String, String> {
    let response = reqwest::get(url).await.map_err(|err| err.to_string())?;
    let text = response.text().await.map_err(|err| err.to_string())?;
    Ok(text)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![fetch])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

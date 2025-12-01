
#[tauri::command]
pub fn print_json(json: serde_json::Value) -> String {
    println!("Received JSON: {:#?}", json);
    format!("JSON received: {}", json)
}
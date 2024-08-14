// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use image::GenericImageView;
use tauri::Manager;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct ConversionResult {
    success: bool,
    message: String,
    path: String,
    output_path: Option<String>,
    dimensions: Option<(u32, u32)>,
}

#[tauri::command]
fn convert_image(input_path: String, output_path: String, format: String) -> Result<serde_json::Value, String> {
    let img = image::open(&input_path).map_err(|e| e.to_string())?;
    
    let format = image::ImageFormat::from_extension(format)
        .ok_or_else(|| "Format non pris en charge".to_string())?;
    
    img.save_with_format(&output_path, format).map_err(|e| e.to_string())?;
    
    let (width, height) = img.dimensions();
    
    Ok(serde_json::json!({
        "success": true,
        "message": "Conversion réussie",
        "path": input_path,
        "output_path": output_path,
        "dimensions": [width, height],
    }))
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(debug_assertions)]
            {
                let window = app.get_window("main").unwrap();
                window.open_devtools();
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![convert_image])
        .run(tauri::generate_context!())
        .expect("Erreur lors de l'exécution de l'application");
}
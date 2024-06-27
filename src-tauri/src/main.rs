// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod evaluator;

#[tauri::command]
fn compute(content: &str) -> String
{
    evaluator::compute(content)
}

fn main()
{
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![compute])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

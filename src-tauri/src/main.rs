#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

use opener;

mod downloader;
mod proxy;

fn main() {
  tauri::Builder::default()
      .invoke_handler(tauri::generate_handler![downloader::download_file])
      .invoke_handler(tauri::generate_handler![run_program])
      .invoke_handler(tauri::generate_handler![connect, disconnect])
      .run(tauri::generate_context!())
      .expect("error while running tauri application");
}

#[tauri::command]
async fn connect() {
  // Log message to console.
  println!("Connecting to proxy...");

  // Create and start a proxy.
  proxy::create_proxy().await;

  // Change proxy settings.
  proxy::connect_to_proxy();
}

#[tauri::command]
fn disconnect() {
  // Change proxy settings.
  proxy::disconnect_from_proxy();
}

#[tauri::command]
fn run_program(path: String) {
  // Open the program from the specified path.
  opener::open(path.clone()).expect("Failed to open program");
}
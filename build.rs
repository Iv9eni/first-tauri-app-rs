use std::thread::Builder;

#[tauri::command]
fn my_custom_command() {
  println!("I was invoked from JS!");
}

// Also in main.rs
fn main() {
  tauri::Builder::default()
    // This is where you pass in your commands
    .invoke_handler(tauri::generate_handler![my_custom_command])
    .run(tauri::generate_context!())
    .expect("failed to run app");
}
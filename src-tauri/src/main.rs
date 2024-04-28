// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Window;
use time::macros::format_description;
use time::OffsetDateTime;


#[tauri::command]
fn init_process(window: Window) {
    let format_description = format_description!(version = 2, "[hour]:[minute]:[second]");

  std::thread::spawn(move || {
    loop {
        std::thread::sleep(std::time::Duration::from_millis(300));

        let now = OffsetDateTime::now_local().unwrap();
        let _ = window.emit("clock", now.format(&format_description).unwrap());
      }
  });
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![init_process])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

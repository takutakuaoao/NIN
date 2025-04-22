// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

use device_query::{DeviceQuery, DeviceState, Keycode};
use external::EnigoMouseController;
use nin::command::{EmitCommand, NinCursorExecuter};
use nin::core::NinCore;
use std::thread;
use std::time::Duration;
mod nin;
mod external;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .setup(|app| {
            #[cfg(desktop)]
            {
                // キーボードイベントを監視するスレッドを開始
                let app_handle = app.handle().clone();
                let device_state = DeviceState::new();

                thread::spawn(move || {
                    let mut emmiter_command = EmitCommand::new(
                        NinCore::new(), 
                        Box::new(external::FrontEndEmitter::new(app_handle))
                    );

                    let mut cursor_command = NinCursorExecuter::new(
                        NinCore::new(), 
                        Box::new(EnigoMouseController::new())
                    );

                    loop {
                        let keys: Vec<Keycode> = device_state.get_keys();
                        if !keys.is_empty() {
                            println!("押されたキー: {:?}", keys);
                            let cursor_command_keys = keys.clone();

                            emmiter_command.execute(keys);
                            cursor_command.execute(cursor_command_keys);
                        }
                        thread::sleep(Duration::from_millis(10));
                    }
                });
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

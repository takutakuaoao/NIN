// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

mod nin_core {
    #[allow(dead_code)]
    pub const CONTROL_KEY: &str = "ctrl";
    #[allow(dead_code)]
    pub const SPACE_KEY: &str = "space";

    #[allow(dead_code)]
    pub struct NinCore {
        is_idle: bool,
        is_cursor: bool,
    }

    impl NinCore {
        #[allow(dead_code)]
        pub fn new() -> Self {
            Self {
                is_idle: true,
                is_cursor: false,
            }
        }

        #[allow(dead_code)]
        pub fn is_idle(&self) -> bool {
            self.is_idle
        }

        #[allow(dead_code)]
        pub fn fire_key_event(&mut self, key1: &str, key2: &str) {
            if key1 == "ctrl" && key2 == "space" {
                self.is_cursor = true;
            }
        }

        #[allow(dead_code)]
        pub fn is_cursor(&self) -> bool {
            self.is_cursor
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::nin_core::{CONTROL_KEY, SPACE_KEY};

    use super::*;

    #[test]
    fn nin_coreの起動時はアイドルモードになっている() {
        let sut = nin_core::NinCore::new();

        assert_eq!(sut.is_idle(), true);
    }

    #[test]
    fn nin_coreはctrlとspaceを受け取ったらカーソルモードになる() {
        let mut sut = nin_core::NinCore::new();

        sut.fire_key_event(CONTROL_KEY, SPACE_KEY);

        assert_eq!(sut.is_cursor(), true);
    }
}

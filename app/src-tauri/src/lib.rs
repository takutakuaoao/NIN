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
    pub struct NinCore {
        is_idle: bool,
    }

    impl NinCore {
        pub fn new() -> Self {
            Self {
                is_idle: true,
            }
        }

        pub fn is_idle(&self) -> bool {
            self.is_idle
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nin_coreの起動時はアイドルモードになっている() {
        let sut = nin_core::NinCore::new();

        assert_eq!(sut.is_idle(), true);
    }
}

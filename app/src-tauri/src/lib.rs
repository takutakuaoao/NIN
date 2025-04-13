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
    #[derive(PartialEq, Copy, Clone)]
    enum MODE {
        IDLE,
        CURSOR,
    }

    #[derive(PartialEq)]
    pub enum Key {
        Control,
        Space,
        J,
        Empty,
    }

    #[allow(dead_code)]
    pub struct NinCore {
        mode: MODE,
    }

    impl NinCore {
        #[allow(dead_code)]
        pub fn new() -> Self {
            Self {
                mode: MODE::IDLE,
            }
        }

        #[allow(dead_code)]
        pub fn is_idle(&self) -> bool {
            self.mode == MODE::IDLE
        }

        #[allow(dead_code)]
        pub fn pass_key(&mut self, key1: Key, key2: Key) -> String {
            match self.mode {
                MODE::IDLE => {
                    if key1 == Key::Control && key2 == Key::Space {
                        self.mode = MODE::CURSOR;
                        "Mode: Cursor, Event: Change to Cursor".to_string()
                    } else {
                        "Mode: Idel, Event: None".to_string()
                    }
                },
                MODE::CURSOR => {
                    if key1 == Key::J && key2 == Key::Empty {
                        "Mode: Cursor, Event: Move Cursor [0, 10]".to_string()
                    } else {
                        "Mode: Cursor, Event: None".to_string()
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::nin_core::Key;

    use super::*;

    #[test]
    fn nin_coreの起動時はアイドルモードになっている() {
        let sut = nin_core::NinCore::new();

        assert_eq!(sut.is_idle(), true);
    }

    #[test]
    fn nin_corはアイドルモードでjを入力しても何もしない() {
        let mut sut = nin_core::NinCore::new();

        let result = sut.pass_key(Key::J, Key::Empty);

        assert_eq!(result, "Mode: Idel, Event: None");
    }

    #[test]
    fn nin_coreはアイドルモードでctrlとspaceを入力するとカーソルモードに移行する() {
        let mut sut = nin_core::NinCore::new();

        let result = sut.pass_key(Key::Control, Key::Space);

        assert_eq!(result, "Mode: Cursor, Event: Change to Cursor");
    }

    #[test]
    fn nin_coreはカーソルモードでjを入力するとカーソルを下に10移動するイベントを発行する() {
        let mut sut = nin_core::NinCore::new();
        sut.pass_key(Key::Control, Key::Space);

        let result = sut.pass_key(Key::J, Key::Empty);

        assert_eq!(result, "Mode: Cursor, Event: Move Cursor [0, 10]");
    }

    #[test]
    fn nin_coreはカーソルモードでspaceを入力しても何もしない() {
        let mut sut = nin_core::NinCore::new();
        sut.pass_key(Key::Control, Key::Space);

        let result = sut.pass_key(Key::Space, Key::Empty);

        assert_eq!(result, "Mode: Cursor, Event: None");
    }
}

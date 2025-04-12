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
        Other,
        Empty,
    }

    #[allow(dead_code)]
    pub struct NinCore {
        mouse_controller: Box<dyn MouseController>,
        mode: MODE,
    }

    impl NinCore {
        #[allow(dead_code)]
        pub fn new(mouse_controller: impl MouseController + 'static) -> Self {
            Self {
                mouse_controller: Box::new(mouse_controller),
                mode: MODE::IDLE,
            }
        }

        #[allow(dead_code)]
        pub fn is_idle(&self) -> bool {
            self.mode == MODE::IDLE
        }

        #[allow(dead_code)]
        pub fn is_cursor(&self) -> bool {
            self.mode == MODE::CURSOR
        }

        #[allow(dead_code)]
        pub fn fire_key_event(&mut self, key1: Key, key2: Key) {
            if key1 == Key::Control && key2 == Key::Space {
                self.mode = MODE::CURSOR;
            }

            if self.mode == MODE::CURSOR && key1 == Key::J && key2 == Key::Empty {
                self.mouse_controller.move_cursor(0, 10);
            }
        }
    }

    pub trait MouseController {
        fn move_cursor(&mut self, x: i32, y: i32);
    }
}

#[cfg(test)]
mod tests {
    use crate::nin_core::{MouseController, NinCore, Key};
    use mockall::{mock, predicate};

    use super::*;

    mock! {
        pub MockMouseController {}
        impl MouseController for MockMouseController {
            fn move_cursor(&mut self, x: i32, y: i32);
        }
    }

    #[test]
    fn nin_coreの起動時はアイドルモードになっている() {
        let mock = MockMockMouseController::new();
        let sut = nin_core::NinCore::new(mock);

        assert_eq!(sut.is_idle(), true);
    }

    #[test]
    fn nin_coreはctrlとspaceを受け取ったらカーソルモードになる() {
        let mock = MockMockMouseController::new();
        let mut sut = nin_core::NinCore::new(mock);

        sut.fire_key_event(Key::Control, Key::Space);

        assert_eq!(sut.is_cursor(), true);
    }

    #[test]
    fn nin_coreはカーソルモードでjを入力するとカーソル位置を10下に下げる指令を出す() {
        let mut mock = MockMockMouseController::new();
        mock.expect_move_cursor()
            .with(predicate::eq(0), predicate::eq(10))
            .times(1).returning(|_, _| ());

        let mut nin = NinCore::new(mock);

        nin.fire_key_event(Key::Control, Key::Space);
        nin.fire_key_event(Key::J, Key::Empty);
    }

    #[test]
    fn nin_coreはカーソルモードでpを入力しても何もしない() {
        let mut mock = MockMockMouseController::new();
        mock.expect_move_cursor()
            .times(0);

        let mut sut = nin_core::NinCore::new(mock);

        sut.fire_key_event(Key::Control, Key::Space);
        sut.fire_key_event(Key::Other, Key::Empty);
    }
}

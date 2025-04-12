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
    pub const J_KEY: &str = "j";

    const CURSOR_MODE_EVENT: &str = "cursor_mode";

    const CURSOR_DOWN_EVENT: &str = "cursor_down";

    #[derive(PartialEq, Copy, Clone)]
    enum MODE {
        IDLE,
        CURSOR,
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
        pub fn fire_key_event(&mut self, key1: &str, key2: &str) {
            let event = self.to_event_from_keys(key1, key2, self.mode);

            if event == CURSOR_MODE_EVENT {
                self.mode = MODE::CURSOR;
            }

            if event == CURSOR_DOWN_EVENT {
                self.mouse_controller.move_cursor(0, 10);
            }
        }

        // @TODO これを別structにして、NinCoreがショートカットキーの詳細を知らなくてもよくする
        fn to_event_from_keys(&self, key1: &str, key2: &str, mode: MODE) -> String {
            if key1 == CONTROL_KEY && key2 == SPACE_KEY {
                return CURSOR_MODE_EVENT.to_string()
            }

            if mode == MODE::CURSOR && key1 == J_KEY {
                return CURSOR_DOWN_EVENT.to_string()
            }

            "".to_string()
        }
    }

    pub trait MouseController {
        fn move_cursor(&mut self, x: i32, y: i32);
    }
}

#[cfg(test)]
mod tests {
    use crate::nin_core::{MouseController, NinCore, CONTROL_KEY, J_KEY, SPACE_KEY};
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

        sut.fire_key_event(CONTROL_KEY, SPACE_KEY);

        assert_eq!(sut.is_cursor(), true);
    }

    #[test]
    fn nin_coreはカーソルモードでjを入力するとカーソル位置を10下に下げる指令を出す() {
        let mut mock = MockMockMouseController::new();
        mock.expect_move_cursor()
            .with(predicate::eq(0), predicate::eq(10))
            .times(1).returning(|_, _| ());

        let mut nin = NinCore::new(mock);
        nin.fire_key_event(CONTROL_KEY, SPACE_KEY);
        nin.fire_key_event(J_KEY, "");
    }

    #[test]
    fn nin_coreはカーソルモードでpを入力しても何もしない() {
        let mut mock = MockMockMouseController::new();
        mock.expect_move_cursor()
            .times(0);

        let mut sut = nin_core::NinCore::new(mock);

        sut.fire_key_event(CONTROL_KEY, SPACE_KEY);
        sut.fire_key_event("p", "");
    }
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

use device_query::{DeviceQuery, DeviceState, Keycode};
use nin_core::{EnigoMouseController, FrontEndEmitter, NinCursorExecuter, NinEmitExecuter};
use std::thread;
use std::time::Duration;

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
                    let mut emmiter_command = NinEmitExecuter::new(
                        nin_core::NinCore::new(), 
                        Box::new(FrontEndEmitter::new(app_handle))
                    );

                    let mut cursor_command = NinCursorExecuter::new(
                        nin_core::NinCore::new(), 
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

mod nin_core {
    use device_query::Keycode;
    use enigo::{Coordinate, Enigo, Mouse, Settings};
    use tauri::{AppHandle, Emitter, Wry};
    #[cfg(test)]
    use mockall::{automock, predicate::*};

    #[derive(PartialEq, Debug)]
    pub enum Event {
        None,
        ChangedMode(MODE),
        MovedCursor(i32, i32),
    }

    #[derive(PartialEq, Copy, Clone, Debug)]
    pub enum MODE {
        IDLE,
        CURSOR,
    }

    impl MODE {
        pub fn pass_key(&self, keys: Vec<Key>) -> Event {
            let mut sorted_keys = keys.clone();
            sorted_keys.sort();

            match self {
                MODE::IDLE => {
                    if sorted_keys == vec![Key::Control, Key::Space] {
                        Event::ChangedMode(MODE::CURSOR)
                    } else {
                        Event::None
                    }
                },
                MODE::CURSOR => {
                    if sorted_keys == vec![Key::J] {
                        Event::MovedCursor(0, 10)
                    } else if sorted_keys == vec![Key::K] {
                        Event::MovedCursor(0, -10)
                    } else if sorted_keys == vec![Key::Escape] {
                        Event::ChangedMode(MODE::IDLE)
                    } else {
                        Event::None
                    }
                }
            }
        }
    }

    #[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
    pub enum Key {
        Control,
        Space,
        J,
        K,
        Escape,
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
        pub fn pass_key(&mut self, keys: Vec<Key>) -> Event {
            let event = self.mode.pass_key(keys);
            
            match event {
                Event::ChangedMode(mode) => {
                    self.mode = mode;
                },
                _ => {}
            }

            return event
        }
    }

    pub struct NinEmitExecuter {
        nin: NinCore,
        emitter: Box<dyn NinEmitter>,
    }

    impl NinEmitExecuter {
        pub fn new(nin: NinCore, emitter: Box<dyn NinEmitter>) -> Self {
            Self { nin, emitter}
        }

        pub fn execute(&mut self, keys: Vec<Keycode>) {
            self.emitter.change_mode("Cursor".to_string());
        }
    }

    pub struct NinCursorExecuter {
        nin: NinCore,
        mouse_controller: Box<dyn MouseController>,
    }

    impl NinCursorExecuter {
        pub fn new(nin: NinCore, mouse_controller: Box<dyn MouseController>) -> Self {
            Self { nin, mouse_controller}
        }

        pub fn execute(&mut self, keys: Vec<Keycode>) {
            let inputs = self.convert_keycode_to_key(keys);

            let event = self.nin.pass_key(inputs);

            match event {
                Event::MovedCursor(x, y) => {
                    self.mouse_controller.move_cursor(x, y);
                },
                _ => {}
            }
        }

        fn convert_keycode_to_key(&self, keys: Vec<Keycode>) -> Vec<Key> {
            let truncated_keys: Vec<Keycode> = keys.into_iter().take(2).collect();

            let mut inputs = vec![];

            for (_, key) in truncated_keys.iter().enumerate() {
                match key {
                    Keycode::Space => {
                        inputs.push(Key::Space);
                    },
                    Keycode::LControl => {
                        inputs.push(Key::Control);
                    },
                    Keycode::J => {
                        inputs.push(Key::J);
                    },
                    _ => ()
                }
            }

            inputs
        }
    }

    #[cfg_attr(test, automock)]
    pub trait MouseController {
        fn move_cursor(&mut self, x: i32, y: i32);
    }

    pub struct EnigoMouseController {
        enigo: Enigo,
    }

    impl EnigoMouseController {
        pub fn new() -> Self {
            Self { enigo: Enigo::new(&Settings::default()).unwrap() }
        }
    }

    impl MouseController for EnigoMouseController {
        fn move_cursor(&mut self, x: i32, y: i32) {
            self.enigo.move_mouse(x, y, Coordinate::Rel);
        }
    }

    #[cfg_attr(test, automock)]
    pub trait NinEmitter: Send {
        fn change_mode(&self, mode: String);
    }

    pub struct FrontEndEmitter {
        emitter: AppHandle<Wry>,
    }

    impl FrontEndEmitter {
        pub fn new(emitter: AppHandle<Wry>) -> Self {
            Self { emitter }
        }
    }

    impl NinEmitter for FrontEndEmitter {
        fn change_mode(&self, mode: String) {
            self.emitter.emit("changed_mode", mode).unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;
    use crate::nin_core::{Event, Key, MockMouseController, MockNinEmitter, MODE};
    use rstest::rstest;

    #[test]
    fn nin_coreの起動時はアイドルモードになっている() {
        let sut = nin_core::NinCore::new();

        assert_eq!(sut.is_idle(), true);
    }

    #[test]
    fn nin_corはアイドルモードでjを入力しても何もしない() {
        let mut sut = nin_core::NinCore::new();

        let event = sut.pass_key(vec![Key::J]);

        assert_eq!(event, Event::None);
    }

    #[test]
    fn nin_coreはアイドルモードでctrlとspaceを入力するとカーソルモードに移行する() {
        let mut sut = nin_core::NinCore::new();

        let event = sut.pass_key(vec![Key::Space, Key::Control]);

        assert_eq!(event, Event::ChangedMode(MODE::CURSOR));
    }

    #[test]
    fn nin_coreはカーソルモードでjを入力するとカーソルを下に10移動するイベントを発行する() {
        let mut sut = nin_coreをカーソルモードとして生成する();

        let event = sut.pass_key(vec![Key::J]);

        assert_eq!(event, Event::MovedCursor(0, 10));
    }

    #[test]
    fn nin_coreはカーソルモードでkを入力するとカーソルを上に10移動するイベントを発行する() {
        let mut sut = nin_coreをカーソルモードとして生成する();

        let event = sut.pass_key(vec![Key::K]);

        assert_eq!(event, Event::MovedCursor(0, -10));
    }

    #[test]
    fn nin_coreはカーソルモードでspaceを入力しても何もしない() {
        let mut sut = nin_coreをカーソルモードとして生成する();

        let event = sut.pass_key(vec![Key::Space]);

        assert_eq!(event, Event::None);
    }

    #[test]
    fn nin_coreはカーソルモードでescを入力するとアイドルモードに戻る() {
        let mut sut = nin_coreをカーソルモードとして生成する();

        let event = sut.pass_key(vec![Key::Escape]);

        assert_eq!(event, Event::ChangedMode(MODE::IDLE));
    }

    fn nin_coreをカーソルモードとして生成する() -> nin_core::NinCore {
        let mut sut = nin_core::NinCore::new();
        sut.pass_key(vec![Key::Space, Key::Control]);

        sut
    }

    #[test]
    fn アイドルモード中にctrlとspaceを入力するとchange_modeが発火する() {
        let mut emitter = MockNinEmitter::new();
        emitter.expect_change_mode()
            .with(eq("Cursor".to_string()))
            .times(1)
            .returning(|_| ());

        let nin = nin_core::NinCore::new();

        let mut sut = nin_core::NinEmitExecuter::new(nin, Box::new(emitter));

        sut.execute(vec![Keycode::Space, Keycode::LControl]);
    }

    #[test]
    fn カーソルモード中にjを入力するとカーソルを下に10移動するオペレーションを実行する() {
        let mut mouse_controller = MockMouseController::new();
        mouse_controller.expect_move_cursor()
            .with(eq(0), eq(10))
            .times(1)
            .returning(|_, _| ());

        let nin = nin_core::NinCore::new();

        let mut sut = nin_core::NinCursorExecuter::new(nin, Box::new(mouse_controller));

        sut.execute(vec![Keycode::Space, Keycode::LControl]);
        sut.execute(vec![Keycode::J]);
    }

    #[test]
    fn アイドルモード中にjを押しても何もしない() {
        let mut mouse_controller = MockMouseController::new();
        mouse_controller.expect_move_cursor()
            .never();

        let nin = nin_core::NinCore::new();

        let mut sut = nin_core::NinCursorExecuter::new(nin, Box::new(mouse_controller));

        sut.execute(vec![Keycode::J]);
    }

    #[rstest(name, input, expected,
        case("カーソルを下に移動", vec![Key::J], Event::MovedCursor(0, 10)),
        case("カーソルを上に移動", vec![Key::K], Event::MovedCursor(0, -10)),
        case("アイドルモードになる", vec![Key::Escape], Event::ChangedMode(MODE::IDLE)),
    )]
    fn カーソルモードのテスト(name: &str, input: Vec<Key>, expected: Event) {
        let sut = nin_core::MODE::CURSOR;

        let result = sut.pass_key(input);

        assert_eq!(result, expected, "テストケース: {}", name);
    }

    #[rstest(name, input, expected,
        case("カーソルモードに移行する", vec![Key::Space, Key::Control], Event::ChangedMode(MODE::CURSOR)),
    )]
    fn アイドルモードのテスト(name: &str, input: Vec<Key>, expected: Event) {
        let sut = nin_core::MODE::IDLE;

        let result = sut.pass_key(input);

        assert_eq!(result, expected, "テストケース: {}", name);
    }
}

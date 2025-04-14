// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

use device_query::{DeviceQuery, DeviceState, Keycode};
use tauri::Emitter;
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
                    loop {
                        let keys: Vec<Keycode> = device_state.get_keys();
                        if !keys.is_empty() {
                            println!("押されたキー: {:?}", keys);
                            app_handle.emit("key-pressed", "キーが入力されました").unwrap();
                        }
                        thread::sleep(Duration::from_millis(50));
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

    #[derive(PartialEq)]
    pub enum Key {
        Control,
        Space,
        J,
        Escape,
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
        pub fn pass_key(&mut self, key1: Key, key2: Key) -> Event {
            match self.mode {
                MODE::IDLE => {
                    if key1 == Key::Control && key2 == Key::Space {
                        self.mode = MODE::CURSOR;
                        Event::ChangedMode(MODE::CURSOR)
                    } else {
                        Event::None
                    }
                },
                MODE::CURSOR => {
                    if key1 == Key::J && key2 == Key::Empty {
                        Event::MovedCursor(0, 10)
                    } else if key1 == Key::Escape && key2 == Key::Empty {
                        self.mode = MODE::IDLE;
                        Event::ChangedMode(MODE::IDLE)
                    } else {
                        Event::None
                    }
                }
            }
        }
    }

    pub struct NinExecuter {
        nin: NinCore,
        emitter: Box<dyn Emitter>,
    }

    impl NinExecuter {
        pub fn new(nin: NinCore, emitter: Box<dyn Emitter>) -> Self {
            Self { nin, emitter }
        }

        pub fn execute(&mut self, keys: Vec<Keycode>) {
            self.emitter.change_mode("Cursor".to_string());
        }
    }

    #[cfg_attr(test, automock)]
    pub trait Emitter {
        fn change_mode(&self, mode: String);
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;
    use crate::nin_core::{Event, Key, MODE, MockEmitter};

    #[test]
    fn nin_coreの起動時はアイドルモードになっている() {
        let sut = nin_core::NinCore::new();

        assert_eq!(sut.is_idle(), true);
    }

    #[test]
    fn nin_corはアイドルモードでjを入力しても何もしない() {
        let mut sut = nin_core::NinCore::new();

        let event = sut.pass_key(Key::J, Key::Empty);

        assert_eq!(event, Event::None);
    }

    #[test]
    fn nin_coreはアイドルモードでctrlとspaceを入力するとカーソルモードに移行する() {
        let mut sut = nin_core::NinCore::new();

        let event = sut.pass_key(Key::Control, Key::Space);

        assert_eq!(event, Event::ChangedMode(MODE::CURSOR));
    }

    #[test]
    fn nin_coreはカーソルモードでjを入力するとカーソルを下に10移動するイベントを発行する() {
        let mut sut = nin_coreをカーソルモードとして生成する();

        let event = sut.pass_key(Key::J, Key::Empty);

        assert_eq!(event, Event::MovedCursor(0, 10));
    }

    #[test]
    fn nin_coreはカーソルモードでspaceを入力しても何もしない() {
        let mut sut = nin_coreをカーソルモードとして生成する();

        let event = sut.pass_key(Key::Space, Key::Empty);

        assert_eq!(event, Event::None);
    }

    #[test]
    fn nin_coreはカーソルモードでescを入力するとアイドルモードに戻る() {
        let mut sut = nin_coreをカーソルモードとして生成する();

        let event = sut.pass_key(Key::Escape, Key::Empty);

        assert_eq!(event, Event::ChangedMode(MODE::IDLE));
    }

    fn nin_coreをカーソルモードとして生成する() -> nin_core::NinCore {
        let mut sut = nin_core::NinCore::new();
        sut.pass_key(Key::Control, Key::Space);

        sut
    }

    #[test]
    fn アイドルモード中にctrlとspaceを入力するとchange_modeが発火する() {
        let mut emitter = MockEmitter::new();
        emitter.expect_change_mode()
            .with(eq("Cursor".to_string()))
            .times(1)
            .returning(|_| ());

        let nin = nin_core::NinCore::new();

        let mut sut = nin_core::NinExecuter::new(nin, Box::new(emitter));

        sut.execute(vec![Keycode::Space, Keycode::LControl]);
    }
}

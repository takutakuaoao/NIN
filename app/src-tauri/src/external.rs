use enigo::{Button, Coordinate, Enigo, Mouse, Settings};
use tauri::{AppHandle, Emitter, Wry};

use crate::nin::command::{EmitExecuter, MouseController};

pub struct FrontEndEmitter {
    emitter: AppHandle<Wry>,
}

impl FrontEndEmitter {
    pub fn new(emitter: AppHandle<Wry>) -> Self {
        Self { emitter }
    }
}

impl EmitExecuter for FrontEndEmitter {
    fn change_mode(&self, mode: String) {
        self.emitter.emit("changed_mode", mode).unwrap();
    }
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

    fn click(&mut self) {
        self.enigo.button(Button::Left, enigo::Direction::Click);
    }
}
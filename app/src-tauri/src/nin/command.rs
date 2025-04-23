use device_query::Keycode;

#[cfg(test)]
use mockall::{automock, predicate::*};

use super::core::{Event, Key, NinCore};

pub struct EmitCommand {
    nin: NinCore,
    emitter: Box<dyn EmitExecuter>,
}

impl EmitCommand {
    pub fn new(nin: NinCore, emitter: Box<dyn EmitExecuter>) -> Self {
        EmitCommand { nin: nin, emitter: emitter }
    }

    pub fn execute(&mut self, keys: Vec<Keycode>) {
        self.emitter.change_mode("Cursor".to_string());
    }
}

#[cfg_attr(test, automock)]
pub trait EmitExecuter: Send {
    fn change_mode(&self, mode: String);
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
        let inputs = InputParser::new(keys).parse();

        let event = self.nin.pass_key(inputs);

        match event {
            Event::MovedCursor(x, y) => {
                self.mouse_controller.move_cursor(x, y);
            },
            _ => {}
        }
    }
}

struct InputParser {
    inputs: Vec<Keycode>
}

impl InputParser {
    fn new(inputs: Vec<Keycode>) -> Self {
        Self { inputs }
    }

    fn truncate(&self) -> Vec<Keycode> {
        let inputs = self.inputs.clone();

        inputs.into_iter().take(2).collect()
    }

    fn parse(&self) -> Vec<Key> {
        let inputs = self.truncate();

        let mut result = vec![];

        for (_, key) in inputs.iter().enumerate() {
            match key {
                Keycode::Space => {
                    result.push(Key::Space);
                },
                Keycode::LControl => {
                    result.push(Key::Control);
                },
                Keycode::J => {
                    result.push(Key::J);
                },
                Keycode::K => {
                    result.push(Key::K);
                },
                Keycode::H => {
                    result.push(Key::H);
                },
                _ => ()
            }
        }

        result
    }
}

#[cfg_attr(test, automock)]
pub trait MouseController {
    fn move_cursor(&mut self, x: i32, y: i32);
}

#[cfg(test)]
mod tests {
    use device_query::Keycode;
    use mockall::predicate::eq;
    use rstest::rstest;

    use crate::nin::{command::InputParser, core::{Key, NinCore}};

    use super::{EmitCommand, MockEmitExecuter, MockMouseController, NinCursorExecuter};

    #[test]
    fn アイドルモード中にctrlとspaceを入力するとchange_modeが発火する() {
        let mut emitter = MockEmitExecuter::new();
        emitter
            .expect_change_mode()
            .with(eq("Cursor".to_string()))
            .times(1)
            .returning(|_| ());

        let nin = NinCore::new();

        let mut sut = EmitCommand::new(nin, Box::new(emitter));

        sut.execute(vec![Keycode::Space, Keycode::LControl]);
    }

    #[rstest(name, expected_x, expected_y, input,
        case("カーソルを下に移動", 0, 10, vec![Keycode::J]),
    )]
    fn カーソル操作(name: &str, expected_x: i32, expected_y: i32, input: Vec<Keycode>) {
        let mut mouse_controller = MockMouseController::new();
        mouse_controller.expect_move_cursor()
            .with(eq(expected_x), eq(expected_y))
            .times(1)
            .returning(|_, _| ());

        let nin = NinCore::new();

        let mut sut = NinCursorExecuter::new(nin, Box::new(mouse_controller));

        sut.execute(vec![Keycode::Space, Keycode::LControl]);
        sut.execute(input);
    }

    #[rstest(inputs, expected,
        case(vec![Keycode::Space], vec![Key::Space]),
        case(vec![Keycode::LControl], vec![Key::Control]),
        case(vec![Keycode::J], vec![Key::J]),
        case(vec![Keycode::K], vec![Key::K]),
        case(vec![Keycode::H], vec![Key::H]),
    )]
    fn インプットパーサーのテスト(inputs: Vec<Keycode>, expected: Vec<Key>) {
        let sut = InputParser::new(inputs);

        let result = sut.parse();

        assert_eq!(result, expected);
    }
}

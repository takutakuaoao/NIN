use std::collections::HashMap;

pub struct NinCore {
    mode: MODE,
}

impl NinCore {
    pub fn new() -> Self {
        Self {
            mode: MODE::IDLE,
        }
    }

    pub fn pass_key(&mut self, keys: Vec<Key>) -> Event {
        let event = self.mode.fire_event(Keys::new(keys));
        
        match event {
            Event::ChangedMode(mode) => {
                self.mode = mode;
            },
            _ => {}
        }

        return event
    }
}

#[derive(Clone)]
struct Keys {
    keys: Vec<Key>
}

impl Keys {
    fn new(keys: Vec<Key>) -> Self {
        Self { keys }
    }

    fn get(&self) -> Vec<Key> {
        let mut sorting = self.keys.clone();
        sorting.sort();

        sorting
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug, Copy, Hash)]
pub enum Key {
    Control,
    Space,
    J,
    K,
    H,
    L,
    I,
    Escape,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Event {
    None,
    ChangedMode(MODE),
    MovedCursor(i32, i32),
    Clicked,
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum MODE {
    IDLE,
    CURSOR,
}

impl MODE {
    fn fire_event(&self, keys: Keys) -> Event {
        match self {
            MODE::IDLE => {
                if keys.get() == vec![Key::Control, Key::Space] {
                    Event::ChangedMode(MODE::CURSOR)
                } else {
                    Event::None
                }
            }
            MODE::CURSOR => {
                CursorEventDispatcher::new().handle(keys)
            }
        }
    }
}

struct CursorEventDispatcher {
    events: HashMap<Vec<Key>, Event>
}

impl CursorEventDispatcher {
    fn new() -> Self {
        let normal_movement_width = 10;
        let speed_up_movement_width = 30;

        let cursor_move_events = HashMap::from([
            ("down"          , Event::MovedCursor(0, normal_movement_width)),
            ("down_speed_up" , Event::MovedCursor(0, speed_up_movement_width)),
            ("up"            , Event::MovedCursor(0, - normal_movement_width)),
            ("up_speed_up"   , Event::MovedCursor(0, - speed_up_movement_width)),
            ("left"          , Event::MovedCursor(- normal_movement_width, 0)),
            ("left_speed_up" , Event::MovedCursor(- speed_up_movement_width, 0)),
            ("right"         , Event::MovedCursor(normal_movement_width, 0)),
            ("right_speed_up", Event::MovedCursor(speed_up_movement_width, 0)),
        ]);

        let events = HashMap::from([
            (vec![Key::J]              , *cursor_move_events.get("down").unwrap()),
            (vec![Key::Control, Key::J], *cursor_move_events.get("down_speed_up").unwrap()),
            (vec![Key::K]              , *cursor_move_events.get("up").unwrap()),
            (vec![Key::Control, Key::K], *cursor_move_events.get("up_speed_up").unwrap()),
            (vec![Key::H]              , *cursor_move_events.get("left").unwrap()),
            (vec![Key::Control, Key::H], *cursor_move_events.get("left_speed_up").unwrap()),
            (vec![Key::L]              , *cursor_move_events.get("right").unwrap()),
            (vec![Key::Control, Key::L], *cursor_move_events.get("right_speed_up").unwrap()),
            (vec![Key::Escape]         , Event::ChangedMode(MODE::IDLE)),
            (vec![Key::I]              , Event::Clicked),
        ]);

        Self { events }
    }

    fn handle(&self, keys: Keys) -> Event {
        if let Some(e) = self.events.get(&keys.get()) {
            *e
        } else {
            Event::None
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use crate::nin::core::{Event, Key, NinCore, MODE};

    #[rstest(name, input, expected,
        case("ctrlとspaceを入力するとカーソルモードに移行する", vec![Key::Space, Key::Control], Event::ChangedMode(MODE::CURSOR)),
        case("関係ないキーを入力しても何もしない", vec![Key::J], Event::None),
    )]
    fn アイドルモード時のイベント発行(name: &str, input: Vec<Key>, expected: Event) {
        let mut sut = NinCore::new();

        let event = sut.pass_key(input);

        assert_eq!(event, expected, "テストケース: {}", name);
    }

    #[rstest(name, input, expected,
        case("jを入力するとカーソルを下に10移動する", vec![Key::J], Event::MovedCursor(0, 10)),
        case("ctrl jを入力するとカーソルを下に30移動する", vec![Key::Control, Key::J], Event::MovedCursor(0, 30)),
        case("kを入力するとカーソルを上に10移動する", vec![Key::K], Event::MovedCursor(0, -10)),
        case("ctrl kを入力するとカーソルを上に30移動する", vec![Key::Control, Key::K], Event::MovedCursor(0, -30)),
        case("hを入力するとカーソルを左に10移動する", vec![Key::H], Event::MovedCursor(-10, 0)),
        case("ctrl hを入力するとカーソルを左に30移動する", vec![Key::Control, Key::H], Event::MovedCursor(-30, 0)),
        case("lを入力するとカーソルを右に10移動する", vec![Key::L], Event::MovedCursor(10, 0)),
        case("ctrl lを入力するとカーソルを右に30移動する", vec![Key::Control, Key::L], Event::MovedCursor(30, 0)),
        case("escを入力するとアイドルモードに戻る", vec![Key::Escape], Event::ChangedMode(MODE::IDLE)),
        case("iを入力するとクリックイベントを発行する", vec![Key::I], Event::Clicked),
        case("関係ないキーを入力しても何もしない", vec![Key::Space], Event::None),
    )]
    fn カーソルモード時のイベント発行(name: &str, input: Vec<Key>, expected: Event) {
        let mut sut = nin_coreをカーソルモードとして生成する();

        let event = sut.pass_key(input);

        assert_eq!(event, expected, "テストケース: {}", name);
    }

    fn nin_coreをカーソルモードとして生成する() -> NinCore {
        let mut sut = NinCore::new();
        sut.pass_key(vec![Key::Space, Key::Control]);

        sut
    }
}

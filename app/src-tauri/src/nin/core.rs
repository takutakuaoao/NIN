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

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub enum Key {
    Control,
    Space,
    J,
    K,
    H,
    L,
    Escape,
}

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
            }
            MODE::CURSOR => {
                if sorted_keys == vec![Key::J] {
                    Event::MovedCursor(0, 10)
                } else if sorted_keys == vec![Key::K] {
                    Event::MovedCursor(0, -10)
                } else if sorted_keys == vec![Key::H] {
                    Event::MovedCursor(-10, 0)
                } else if sorted_keys == vec![Key::L] {
                    Event::MovedCursor(10, 0)
                } else if sorted_keys == vec![Key::Escape] {
                    Event::ChangedMode(MODE::IDLE)
                } else {
                    Event::None
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use crate::nin::core::{Event, Key, NinCore, MODE};

    #[test]
    fn 起動時はアイドルモードになっている() {
        let sut = NinCore::new();

        assert_eq!(sut.is_idle(), true);
    }

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
        case("kを入力するとカーソルを上に10移動する", vec![Key::K], Event::MovedCursor(0, -10)),
        case("hを入力するとカーソルを左に10移動する", vec![Key::H], Event::MovedCursor(-10, 0)),
        case("lを入力するとカーソルを右に10移動する", vec![Key::L], Event::MovedCursor(10, 0)),
        case("escを入力するとアイドルモードに戻る", vec![Key::Escape], Event::ChangedMode(MODE::IDLE)),
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

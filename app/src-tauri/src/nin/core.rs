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

    #[rstest(name, input, expected,
        case("カーソルモードに移行する", vec![Key::Space, Key::Control], Event::ChangedMode(MODE::CURSOR)),
    )]
    fn アイドルモードのテスト(name: &str, input: Vec<Key>, expected: Event) {
        let sut = MODE::IDLE;

        let result = sut.pass_key(input);

        assert_eq!(result, expected, "テストケース: {}", name);
    }

    #[rstest(name, input, expected,
        case("カーソルを下に移動", vec![Key::J], Event::MovedCursor(0, 10)),
        case("カーソルを上に移動", vec![Key::K], Event::MovedCursor(0, -10)),
        case("アイドルモードになる", vec![Key::Escape], Event::ChangedMode(MODE::IDLE)),
    )]
    fn カーソルモードのテスト(name: &str, input: Vec<Key>, expected: Event) {
        let sut = MODE::CURSOR;

        let result = sut.pass_key(input);

        assert_eq!(result, expected, "テストケース: {}", name);
    }

    #[test]
    fn nin_coreの起動時はアイドルモードになっている() {
        let sut = NinCore::new();

        assert_eq!(sut.is_idle(), true);
    }

    #[test]
    fn nin_corはアイドルモードでjを入力しても何もしない() {
        let mut sut = NinCore::new();

        let event = sut.pass_key(vec![Key::J]);

        assert_eq!(event, Event::None);
    }

    #[test]
    fn nin_coreはカーソルモードでescを入力するとアイドルモードに戻る() {
        let mut sut = nin_coreをカーソルモードとして生成する();

        let event = sut.pass_key(vec![Key::Escape]);

        assert_eq!(event, Event::ChangedMode(MODE::IDLE));
    }

    #[test]
    fn nin_coreはアイドルモードでctrlとspaceを入力するとカーソルモードに移行する() {
        let mut sut = NinCore::new();

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

    fn nin_coreをカーソルモードとして生成する() -> NinCore {
        let mut sut = NinCore::new();
        sut.pass_key(vec![Key::Space, Key::Control]);

        sut
    }
}

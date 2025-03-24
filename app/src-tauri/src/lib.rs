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

struct Cursor {
    x: f64,
    y: f64,
}

impl Cursor {
    fn down(&mut self) -> Cursor {
        return Cursor {
            x: self.x,
            y: self.y + 1.0,
        };
    }

    fn same(&self, other: Cursor) -> bool {
        return self.x == other.x && self.y == other.y;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[test]
    fn カーソルを下にずらす() {
        let mut cursor = Cursor { x: 0.0, y: 0.0 };

        let moved = cursor.down();

        assert_eq!(true, moved.same(Cursor { x: 0.0, y: 1.0 }))
    }

    #[rstest]
    #[case::same(vec![0.0, 0.0], vec![0.0, 0.0], true)]
    #[case::not_same(vec![0.0, 0.0], vec![0.1, 0.0], false)]
    #[test]
    fn カーソルの等価性のテスト(
        #[case] my: Vec<f64>,
        #[case] other: Vec<f64>,
        #[case] want: bool,
    ) {
        let cursor_1 = Cursor { x: my[0], y: my[1] };
        let cursor_2 = Cursor {
            x: other[0],
            y: other[1],
        };

        assert_eq!(want, cursor_1.same(cursor_2))
    }
}

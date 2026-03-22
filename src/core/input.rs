use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

pub struct InputHandler{
    pub text: String,
    character_index: usize
}

impl InputHandler {
    pub const fn new() -> Self {
        Self {
            text: String::new(),
            character_index: 0
        }
    }

    pub fn event_handle(&mut self, key: KeyEvent) {
        if key.kind == KeyEventKind::Press {
            match (key.code, key.modifiers) {
                (KeyCode::Enter, KeyModifiers::NONE) => self.submit_message(),
                (KeyCode::Char(to_insert), KeyModifiers::NONE) => self.enter_char(to_insert),
                (KeyCode::Backspace, KeyModifiers::NONE) => self.delete_char(),
                (KeyCode::Left, KeyModifiers::NONE) => self.move_cursor_left(),
                (KeyCode::Right, KeyModifiers::NONE) => self.move_cursor_right(),
                _ => {}
            }
        }
    }

    fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.character_index.saturating_sub(1);
        self.character_index = self.clamp_cursor(cursor_moved_left);
    }

    fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.character_index.saturating_add(1);
        self.character_index = self.clamp_cursor(cursor_moved_right);
    }

    fn enter_char(&mut self, new_char: char) {
        let index = self.byte_index();
        self.text.insert(index, new_char);
        self.move_cursor_right();
    }

    fn byte_index(&self) -> usize {
        self.text
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.character_index)
            .unwrap_or(self.text.len())
    }

    fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.character_index != 0;
        if is_not_cursor_leftmost {
            let current_index = self.character_index;
            let from_left_to_current_index = current_index - 1;

            let before_char_to_delete = self.text.chars().take(from_left_to_current_index);
            let after_char_to_delete = self.text.chars().skip(current_index);

            self.text = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }

    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.text.chars().count())
    }

    const fn reset_cursor(&mut self) {
        self.character_index = 0;
    }

    fn submit_message(&mut self) {
        self.text.clear();
        self.reset_cursor();
    }
}
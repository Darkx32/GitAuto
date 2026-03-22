use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyEventKind};

pub struct TabHandler {
    pub titles: Vec<String>,
    pub tab_index: usize
}

impl TabHandler {
    pub fn new<S: Into<String>>(titles: Vec<S>) -> Self {
        Self {
            titles: titles.into_iter().map(|s| s.into()).collect(),
            tab_index: 0
        }
    }

    pub fn event_handle(&mut self, key: KeyEvent) {
        if key.kind == KeyEventKind::Press {
            match key.code {
                KeyCode::Left => {
                    self.tab_index -= 1;
                    self.tab_index = self.tab_index.clamp(0, self.titles.len() - 1);
                },
                KeyCode::Right => {
                    self.tab_index += 1;
                    self.tab_index = self.tab_index.clamp(0, self.titles.len() - 1)
                },
                _ => {}
            }
        }
    }
}
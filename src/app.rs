pub struct App {
    pub should_quit: bool,
    pub show_preview: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            should_quit: false,
            show_preview: true,
        }
    }

    pub fn handle_key(&mut self, key: crossterm::event::KeyEvent) {
        match key.code {
            crossterm::event::KeyCode::Char('q') => self.should_quit = true,
            crossterm::event::KeyCode::Char('z') => self.show_preview = !self.show_preview,
            _ => {}
        }
    }
}
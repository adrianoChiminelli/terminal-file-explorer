mod app;
mod ui;

use crate::app::App;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

fn main() -> std::io::Result<()> {
    // setup
    crossterm::terminal::enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    crossterm::execute!(stdout, crossterm::terminal::EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();

    // loop
    while !app.should_quit {
        terminal.draw(|frame| ui::draw(frame, &app))?;

        if crossterm::event::poll(std::time::Duration::from_millis(100))? {
            if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
                app.handle_key(key);
            }
        }
    }

    // restore
    crossterm::terminal::disable_raw_mode()?;
    crossterm::execute!(terminal.backend_mut(), crossterm::terminal::LeaveAlternateScreen)?;
    Ok(())
}

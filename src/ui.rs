use ratatui::layout::{Layout, Constraint, Rect};
use ratatui::widgets::{Block, Borders};
use ratatui::Frame;
use crate::app::App;

fn build_layout(area: Rect, show_preview: bool) -> (Rect, Rect, Option<Rect>, Rect) {
    let [top, bottom] = Layout::vertical([
        Constraint::Min(10),
        Constraint::Length(10),
    ]).areas(area);

    if show_preview {
        let [dirs, files, preview] = Layout::horizontal([
            Constraint::Percentage(20),
            Constraint::Percentage(40),
            Constraint::Percentage(40),
        ]).areas(top);
        (dirs, files, Some(preview), bottom)
    } else {
        let [dirs, files] = Layout::horizontal([
            Constraint::Percentage(20),
            Constraint::Percentage(80),
        ]).areas(top);
        (dirs, files, None, bottom)
    }
}

pub fn draw(frame: &mut Frame, app: &App) {
    let (dirs, files, preview, terminal) = build_layout(frame.area(), app.show_preview);

    frame.render_widget(Block::default().borders(Borders::ALL).title("Diretórios"), dirs);
    frame.render_widget(Block::default().borders(Borders::ALL).title("Arquivos"), files);

    if let Some(preview_area) = preview {
        frame.render_widget(Block::default().borders(Borders::ALL).title("Preview"), preview_area);
    }

    frame.render_widget(Block::default().borders(Borders::ALL).title("Terminal"), terminal);
}
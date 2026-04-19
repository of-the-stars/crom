use ratatui::{layout, widgets};
use std::io;

fn main() -> io::Result<()> {
    ratatui::run(app)?;
    Ok(())
}

fn app(terminal: &mut ratatui::DefaultTerminal) -> io::Result<()> {
    loop {
        terminal.draw(render)?;
        if crossterm::event::read()?.is_key_press() {
            break Ok(());
        }
    }
}

fn render(frame: &mut ratatui::Frame) {
    let layout = layout::Layout::default()
        .direction(layout::Direction::Vertical)
        .constraints(vec![
            layout::Constraint::Percentage(50),
            layout::Constraint::Percentage(50),
        ])
        .split(frame.area());

    frame.render_widget(
        widgets::Paragraph::new("Top").block(widgets::Block::new().borders(widgets::Borders::ALL)),
        layout[0],
    );
    frame.render_widget(
        widgets::Paragraph::new("Bottom")
            .block(widgets::Block::new().borders(widgets::Borders::ALL)),
        layout[1],
    );
}

pub mod app_history;
pub mod chrome;
pub mod details;
pub mod dialog;
pub mod format;
pub mod graph;
pub mod performance;
pub mod processes;
pub mod services;
pub mod startup;
pub mod users;

use crate::app::App;
use ratatui::Frame;

pub fn draw(frame: &mut Frame, app: &App) {
    chrome::draw(frame, app);
}

pub mod test_util {
    use crate::app::App;
    use crate::ui;
    use ratatui::Terminal;
    use ratatui::backend::TestBackend;

    /// Render a full frame and return it as text, one line per row.
    pub fn render(app: &App, width: u16, height: u16) -> String {
        let mut terminal = Terminal::new(TestBackend::new(width, height)).unwrap();
        terminal.draw(|frame| ui::draw(frame, app)).unwrap();
        let buffer = terminal.backend().buffer().clone();
        let mut out = String::new();
        for y in 0..height {
            for x in 0..width {
                out.push_str(buffer[(x, y)].symbol());
            }
            out.push('\n');
        }
        out
    }
}

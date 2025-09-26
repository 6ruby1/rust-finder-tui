use std::io;

use ratatui::crossterm::event::{Event, read};
use ratatui::{DefaultTerminal, Frame};

fn main() {
    let _guard = sentry::init((
        "https://62cf52ffca04346f14507c4a08db9b2c@o4510067097665536.ingest.de.sentry.io/4510081161756752",
        sentry::ClientOptions {
            release: sentry::release_name!(),
            // Capture user IPs and potentially sensitive headers when using HTTP server integrations
            // see https://docs.sentry.io/platforms/rust/data-management/data-collected for more info
            send_default_pii: true,
            ..Default::default()
        },
    ));

    let mut terminal = ratatui::init();
    let res = App::default().run(&mut terminal);
    ratatui::restore();
}

#[derive(Debug)]
pub struct App {
    query: String,
    running: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            query: String::new(),
            running: true,
        }
    }
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while self.running {
            terminal.draw(|frame| self.draw(frame))?;
            if matches!(read()?, Event::Key(_)) {
                self.running = false
            }
        }
        Ok(())
    }

    pub fn draw(&self, frame: &mut Frame) {
        frame.render_widget("hello world", frame.area());
    }
}

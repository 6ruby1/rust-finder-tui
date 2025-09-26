use color_eyre::eyre::WrapErr;
use ratatui::crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, read};
use ratatui::{DefaultTerminal, Frame};

fn main() -> color_eyre::Result<()> {
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
    color_eyre::install()?;
    let mut terminal = ratatui::init();
    let result = App::default().run(&mut terminal);
    ratatui::restore();
    result
}

#[derive(Debug)]
pub struct App {
    // TODO: collect text input for search
    // query: String,
    running: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            // TODO: collect text input for search
            // query: String::new(),
            running: true,
        }
    }
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> color_eyre::Result<()> {
        while self.running {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events().wrap_err("handle_events failed")?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget("hello world", frame.area());
    }

    fn handle_events(&mut self) -> color_eyre::Result<()> {
        match read()? {
            Event::Key(key_ev) if key_ev.kind == KeyEventKind::Press => self
                .handle_key_event(key_ev)
                .wrap_err_with(|| format!("handle_key_event failed with:\n{key_ev:#?}")),
            _ => Ok(()),
        }
    }

    fn handle_key_event(&mut self, ev: KeyEvent) -> color_eyre::Result<()> {
        match ev.code {
            KeyCode::Char('q') => self.quit(),
            //TODO: add other keybinds
            _ => {}
        }
        Ok(())
    }

    fn quit(&mut self) {
        self.running = false;
    }
}

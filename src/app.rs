use color_eyre::eyre::WrapErr;
use ratatui::crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, read};
use ratatui::{DefaultTerminal, Frame};

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
            self.process_events().wrap_err("process_events failed")?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget("hello world", frame.area());
    }

    fn process_events(&mut self) -> color_eyre::Result<()> {
        match read()? {
            Event::Key(key_ev) if key_ev.kind == KeyEventKind::Press => self
                .process_key_event(key_ev)
                .wrap_err_with(|| format!("process_key_event failed with:\n{key_ev:#?}")),
            _ => Ok(()),
        }
    }

    fn process_key_event(&mut self, ev: KeyEvent) -> color_eyre::Result<()> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn app_quits_on_key_q() {
        let mut app = App::default();
        app.process_key_event(KeyCode::Char('q').into()).unwrap();
        assert!(!app.running)
    }
}

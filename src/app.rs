use std::io;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::prelude::*;

use crate::state::State;
use crate::tui;

#[derive(Debug, Default)]
pub struct App {
    pub state: State,
    exit: bool,
}

impl App {
    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn render_frame(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.size());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Esc => self.exit(),
            KeyCode::Char('q') => self.exit(),

            KeyCode::Char('i') => { self.state = State::Inbox; },
            KeyCode::Char('s') => { self.state = State::Sent; },
            KeyCode::Char('d') => { self.state = State::Drafts; },
            KeyCode::Char('c') => { self.state = State::Compose; },

            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}
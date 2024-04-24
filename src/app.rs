use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::prelude::*;
use std::io;

use crate::state::*;
use crate::tui;

#[derive(Debug, Default)]
pub struct App {
    pub inbox: InboxState,
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

            KeyCode::Down => self.scroll(true),
            KeyCode::Up => self.scroll(false),

            KeyCode::Char('i') => {
                self.state = State::Inbox;
            }
            KeyCode::Char('s') => {
                self.state = State::Sent;
            }
            KeyCode::Char('d') => {
                self.state = State::Drafts;
            }
            KeyCode::Char('c') => {
                self.state = State::Compose;
            }

            _ => {}
        }
    }

    fn scroll(&mut self, next: bool) {
        if next {
            self.inbox.scroll += 1;
        } else if self.inbox.scroll != 0 {
            self.inbox.scroll -= 1;
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

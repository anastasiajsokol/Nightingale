use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

use ratatui::{
    prelude::*,
    symbols::border,
    widgets::{block::*, *},
};

use crate::tui;

#[derive(Debug, Default)]
pub struct App {
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
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Title::from(Line::from(vec![
            " Nightingale ".green().bold(),
            " Inbox ".bold(),
            "<I>".cyan().bold(),
            " Sent ".into(),
            "<S>".cyan().bold(),
            " Drafts ".into(),
            "<D>".cyan().bold(),
            " Compose ".into(),
            "<C>".cyan().bold(),
            " ".into(),
        ]));
        let instructions = Title::from(Line::from(vec![
            " Previous ".into(),
            "<Up>".cyan().bold(),
            " Next ".into(),
            "<Down>".cyan().bold(),
            " Quit ".into(),
            "<Q>".cyan().bold(),
            " Exit ".into(),
            "<Esc> ".cyan().bold(),
        ]));
        let block = Block::default()
            .title(title.alignment(Alignment::Left))
            .title(
                instructions
                    .alignment(Alignment::Center)
                    .position(Position::Bottom),
            )
            .borders(Borders::ALL)
            .border_set(border::THICK);

        let body = Text::from(vec![Line::from(vec![
            "Inbox [or at least what will be the inbox eventually, maybe]".into(),
        ])]);

        Paragraph::new(body)
            .centered()
            .block(block)
            .render(area, buf);
    }
}
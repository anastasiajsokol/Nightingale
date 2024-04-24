use ratatui::{
    prelude::*,
    symbols::border,
    widgets::{block::*, *},
};

use crate::app;
use crate::state::State;

fn generate_title(state: &State) -> Title<'_> {
    let mut inbox = " Inbox ".white();
    let mut sent = " Sent ".white();
    let mut drafts = " Drafts ".white();
    let mut compose = " Compose ".white();

    match state {
        State::Compose => {
            compose = compose.bold();
        }
        State::Drafts => {
            drafts = drafts.bold();
        }
        State::Inbox => {
            inbox = inbox.bold();
        }
        State::Sent => sent = sent.bold(),
    }

    Title::from(Line::from(vec![
        " Nightingale ".green().bold(),
        inbox,
        "<I>".cyan().bold(),
        sent,
        "<S>".cyan().bold(),
        drafts,
        "<D>".cyan().bold(),
        compose,
        "<C>".cyan().bold(),
        " ".into(),
    ]))
}

fn generate_instructions(state: &State) -> Title<'_> {
    let mut commands = match state {
        State::Inbox | State::Sent | State::Drafts => vec![
            " Previous ".into(),
            "<Up>".cyan().bold(),
            " Next ".into(),
            "<Down>".cyan().bold(),
            " Quit ".into(),
            "<Q>".cyan().bold(),
        ],
        _ => vec![],
    };

    let mut general = vec![" Exit ".into(), "<Esc> ".cyan().bold()];

    commands.append(&mut general);

    Title::from(Line::from(commands))
}

fn generate_body(state: &State) -> impl Widget {
    Paragraph::new(Text::from(vec![Line::from(vec![match state {
        State::Inbox => "Inbox".into(),
        State::Sent => "Sent".into(),
        State::Drafts => "Drafts".into(),
        State::Compose => "Compose".into(),
    }])]))
    .centered()
}

fn generate_shell(state: &State) -> Block<'_> {
    let title = generate_title(&state);
    let instructions = generate_instructions(&state);

    Block::default()
        .title(title.alignment(Alignment::Left))
        .title(
            instructions
                .alignment(Alignment::Center)
                .position(Position::Bottom),
        )
        .borders(Borders::ALL)
        .border_set(border::THICK)
}

impl Widget for &app::App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        generate_shell(&self.state).render(area, buf);

        let inner = Rect {
            x: area.x + 1,
            y: area.y + 1,
            width: area.width - 2,
            height: area.height - 2,
        };

        generate_body(&self.state).render(inner, buf);
    }
}

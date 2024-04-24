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

fn render_shell(state: &State, area: Rect, buf: &mut Buffer) {
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
        .render(area, buf);
}

fn render_body(app: &app::App, area: Rect, buf: &mut Buffer) {
    let mut items = vec![];

    for i in 0..20 {
        items.push(format!("Message {}", i));
    }

    let list = List::new(items);

    let mut state = ListState::default().with_offset(app.inbox.scroll);

    StatefulWidget::render(list, area, buf, &mut state);
}

impl Widget for &app::App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        render_shell(&self.state, area, buf);

        let margin = 1;
        let inner = Rect {
            x: area.x + margin,
            y: area.y + margin,
            width: area.width - 2 * margin,
            height: area.height - 2 * margin,
        };

        render_body(&self, inner, buf);
    }
}

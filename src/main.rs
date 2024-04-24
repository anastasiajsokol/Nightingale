use nightingale::{app::App, tui};
use std::io;

fn main() -> io::Result<()> {
    let mut terminal = tui::init()?;
    let result = App::default().run(&mut terminal);
    tui::restore()?;
    result
}
mod event;
mod ui;

use std::io;

use app_core::{AppState, update};
use ratatui::DefaultTerminal;

use crate::event::Event;

fn main() -> io::Result<()> {
    let result = ratatui::try_init().and_then(|mut terminal| run(&mut terminal));
    let restore = ratatui::try_restore();
    result.and(restore)
}

fn run(terminal: &mut DefaultTerminal) -> io::Result<()> {
    let mut state = AppState::default();

    loop {
        terminal.draw(|frame| ui::render(frame, &state))?;

        match event::read()? {
            Event::Action(action) => update(&mut state, action),
            Event::Redraw => {}
            Event::Quit => return Ok(()),
        }
    }
}

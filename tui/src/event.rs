use std::io;

use app_core::Action;
use crossterm::event::{self, Event as TerminalEvent, KeyCode, KeyEventKind, KeyModifiers};

pub enum Event {
    Action(Action),
    Redraw,
    Quit,
}

pub fn read() -> io::Result<Event> {
    loop {
        match event::read()? {
            TerminalEvent::Resize(..) => return Ok(Event::Redraw),
            TerminalEvent::Key(key) if key.kind == KeyEventKind::Press => {
                match (key.code, key.modifiers) {
                    (KeyCode::Esc, _)
                    | (KeyCode::Char('q' | 'Q'), KeyModifiers::NONE | KeyModifiers::SHIFT)
                    | (KeyCode::Char('c' | 'C'), KeyModifiers::CONTROL) => return Ok(Event::Quit),
                    (KeyCode::Char(key), KeyModifiers::NONE | KeyModifiers::SHIFT) => {
                        if let Some(action) = Action::from_shortcut(key) {
                            return Ok(Event::Action(action));
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}

use app_core::{AppState, Screen};
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, List, Paragraph},
};

const ACCENT: Color = Color::Cyan;

pub fn render(frame: &mut Frame, state: &AppState) {
    let area = frame.area();

    if area.width < 16 || area.height < 6 {
        frame.render_widget(Paragraph::new("1-3  I  Q"), area);
        return;
    }

    let wide = area.width >= 38;
    let nav_height = if wide || area.height < 9 { 1 } else { 3 };
    let [header, navigation, panel, help] = Layout::vertical([
        Constraint::Length(1),
        Constraint::Length(nav_height),
        Constraint::Min(3),
        Constraint::Length(1),
    ])
    .areas(area);

    frame.render_widget(Paragraph::new("Rust Universal Starter").bold(), header);
    render_navigation(frame, navigation, state.screen(), wide);
    render_panel(frame, panel, state);
    let shortcuts = match area.width {
        38.. => "1-3 Navigate   I Increment   Q Quit",
        21.. => "1-3 Nav  I Inc  Q Quit",
        _ => "1-3  I  Q Quit",
    };
    frame.render_widget(Paragraph::new(shortcuts), help);
}

fn render_navigation(frame: &mut Frame, area: Rect, active: Screen, wide: bool) {
    let item = |screen: Screen| {
        let marker = if screen == active { '>' } else { ' ' };
        let label = format!("{marker}{} {} ", screen.shortcut(), screen.label());
        if screen == active {
            Span::styled(label, Style::new().fg(Color::Black).bg(ACCENT).bold())
        } else {
            Span::raw(label)
        }
    };

    let lines = if wide {
        vec![Line::from(Screen::ALL.map(item).to_vec())]
    } else if area.height >= 3 {
        Screen::ALL
            .into_iter()
            .map(|screen| Line::from(item(screen)))
            .collect()
    } else {
        vec![Line::from(item(active))]
    };

    frame.render_widget(Paragraph::new(lines), area);
}

fn render_panel(frame: &mut Frame, area: Rect, state: &AppState) {
    let screen = state.screen();
    let panel = Block::bordered().title(screen.label());

    match screen {
        Screen::Home => {
            let lines = [
                Line::raw(format!("Counter: {}", state.count())),
                Line::styled("[I] Increment counter", Style::new().fg(ACCENT)),
            ];
            frame.render_widget(Paragraph::new(lines.to_vec()).block(panel), area);
        }
        Screen::Activity => {
            let label = if area.width >= 30 {
                "Increments this session"
            } else {
                "Increments"
            };
            let list = List::new([format!("{label}: {}", state.count())]);
            frame.render_widget(list.block(panel), area);
        }
        Screen::Settings => {
            let list = List::new(["1-3 Navigate screens", "I Increment counter"]);
            frame.render_widget(list.block(panel), area);
        }
    }
}

use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, Gauge, Paragraph, Tabs, Wrap};
use tui::Frame;

use crate::app::App;

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Length(3),
                Constraint::Min(0),
            ]
            .as_ref(),
        )
        .split(f.size());
    let titles = ["Overview", "Vehicles", "Events"]
        .iter()
        .map(|t| Spans::from(Span::styled(*t, Style::default().fg(Color::Cyan))))
        .collect();
    let tabs = Tabs::new(titles)
        .block(Block::default().title("Tabs").borders(Borders::ALL))
        .highlight_style(Style::default().fg(Color::Green))
        .select(0);
    f.render_widget(tabs, chunks[0]);
    draw_overview(f, app, chunks[1]);
}

fn draw_overview<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref())
        .split(area);
    draw_overview_gauges(f, app, chunks[0]);
    draw_overview_text(f, app, chunks[1]);
}

fn draw_overview_gauges<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    let mut constraint = Vec::new();
    for _ in 0..=8 {
        constraint.push(Constraint::Ratio(1, 8));
    }

    let chunks = Layout::default().constraints(constraint).split(area);
    let mut vehicles = Vec::new();
    for i in 0..=8 {
        vehicles.push(format!("Vehicle {} progress:", i));
    }

    for i in 0..=8 {
        let gauge = Gauge::default()
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(vehicles[i].as_ref()),
            )
            .gauge_style(Style::default().fg(Color::Magenta).bg(Color::Black))
            .label(if i % 3 == 0 { "66.6%" } else { "33.3%" })
            .ratio(if i % 3 == 0 { 2.0 / 3.0 } else { 1.0 / 3.0 });
        f.render_widget(gauge, chunks[i]);
    }
}

fn draw_overview_text<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    let text = vec![
        Spans::from(
            "Registered Vehicles: udp://:14540 udp://:14541 udp://:14542 udp://:14543 udp://:14544 udp://:14545 udp://:14546 udp://:14548"
        ),
        Spans::from(
            "Registered Plans: rect0.plan rect1.plan rect2.plan rect3.plan rect4.plan rect5.plan rect6.plan rect7.plan"
        )
    ];
    let block = Block::default().borders(Borders::ALL).title(Span::styled(
        "Notes",
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    ));
    let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: true });
    f.render_widget(paragraph, area)
}

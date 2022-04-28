use std::error::Error;
use std::io;
use std::time::{Duration, Instant};

use crossterm::event::{DisableMouseCapture, Event, KeyCode};
use crossterm::terminal::{disable_raw_mode, LeaveAlternateScreen};
use crossterm::{event, execute, terminal};
use tui::backend::{Backend, CrosstermBackend};
use tui::Terminal;

use crate::ui;

pub struct TabState<'a> {
    pub titles: Vec<&'a str>,
    pub index: usize,
}

impl<'a> TabState<'a> {
    pub fn new(titles: Vec<&'a str>) -> TabState {
        TabState { titles, index: 0 }
    }
    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.titles.len();
    }
    pub fn prev(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.titles.len() - 1;
        }
    }
}

// App handles the state of the manager/data and state of the UI
pub struct App<'a> {
    pub title: &'a str,
    pub should_quit: bool,
    pub tabs: TabState<'a>,
    pub enhanced_graphics: bool,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str, enhanced_graphics: bool) -> App<'a> {
        App {
            title,
            should_quit: false,
            tabs: TabState::new(vec!["Tab0", "Tab1", "Tab2"]),
            enhanced_graphics,
        }
    }

    pub fn on_up(&mut self) {
        self.tabs.prev();
    }

    pub fn on_down(&mut self) {
        self.tabs.next();
    }

    pub fn on_left(&mut self) {
        self.tabs.prev();
    }

    pub fn on_right(&mut self) {
        self.tabs.next();
    }

    pub fn on_key(&mut self, c: char) {
        match c {
            'q' => {
                self.should_quit = true;
            }
            _ => {}
        }
    }

    pub fn on_tick(&mut self) {}
}

pub fn run(tick_rate: Duration) -> Result<(), Box<dyn Error>> {
    // prepare terminal
    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(
        stdout,
        terminal::EnterAlternateScreen,
        event::EnableMouseCapture,
    )?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let app = App::new("FlightCTL", true);
    let res = run_app(&mut terminal, app, tick_rate);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture,
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    tick_rate: Duration,
) -> io::Result<()> {
    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|f| ui::draw(f, &mut app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char(c) => app.on_key(c),
                    KeyCode::Up => app.on_up(),
                    KeyCode::Down => app.on_down(),
                    KeyCode::Left => app.on_left(),
                    KeyCode::Right => app.on_right(),
                    _ => {}
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }
        if app.should_quit {
            return Ok(());
        }
    }
}

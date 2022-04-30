use std::error::Error;
use std::io;
use std::time::{Duration, Instant};

use crate::run_mode::RunApp;
use crossterm::event::{DisableMouseCapture, Event, KeyCode};
use crossterm::terminal::{disable_raw_mode, LeaveAlternateScreen};
use crossterm::{event, execute, terminal};
use tui::backend::{Backend, CrosstermBackend};
use tui::Terminal;

use crate::{run_mode, ui};

// TabState handles the state of tabs that the UI uses to draw
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

enum InputMode {
    Normal,
    Editing,
}

// App handles the state of the manager/data and state of the UI
pub struct App {
    input: String,
    input_mode: InputMode,
    messages: Vec<String>,
}

impl Default for App {
    fn default() -> Self {
        App {
            input: String::new(),
            input_mode: InputMode::Normal,
            messages: Vec::new(),
        }
    }
}

pub async fn run(
    tick_rate: Duration,
    data: Option<Vec<(String, String)>>,
    is_run_mode: bool,
) -> Result<(), Box<dyn Error>> {
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

    if is_run_mode {
        let mut app = RunApp::new("FlightCTL",  true);
        run_mode::run_app(&mut terminal, &mut app, data.unwrap(), tick_rate);
    } else {
        let app = App::default();
        // run app with UI still in progress
        run_app(&mut terminal, app);
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture,
    )?;
    terminal.show_cursor()?;

    Ok(())
}

async fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
) -> io::Result<()> {

    loop {
        terminal.draw(|f| ui::draw(f, &app))?;

            if let Event::Key(key) = event::read()? {
                match app.input_mode {
                    InputMode::Normal => match key.code {
                        KeyCode::Char('i') => {
                            app.input_mode = InputMode::Editing;
                        },
                        KeyCode::Char('q') => {
                            return Ok(());
                        }
                        _ => {}
                    },
                    InputMode::Editing => match key.code {
                        KeyCode::Enter => {
                            app.messages.push(app.input.drain(..).collect());
                        }
                        KeyCode::Char(c) => {
                            app.input.push(c);
                        }
                        KeyCode::Backspace => {
                            app.input.pop();
                        }
                        KeyCode::Esc => {
                            app.input_mode = InputMode::Normal;
                        }
                        _ => {}
                    },
                }

        }
    }
}

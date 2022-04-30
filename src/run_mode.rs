use std::ffi::CString;
use std::future::Future;
use std::io;
use std::path::Path;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

use crossterm::event;
use crossterm::event::{Event, KeyCode};
use futures::future::{join_all, JoinAll};
use tokio::task::JoinHandle;
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, LineGauge, List, ListItem, Tabs};
use tui::Frame;
use tui::Terminal;

use crate::app::TabState;

fn draw<B: Backend>(f: &mut Frame<'_, B>, run_app: &RunApp<'_>) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(f.size());
    let run_titles = run_app
        .tabs
        .titles
        .iter()
        .map(|t| Spans::from(Span::styled(*t, Style::default().fg(Color::Blue))))
        .collect();
    let run_tabs = Tabs::new(run_titles)
        .block(Block::default().title("Tabs").borders(Borders::ALL))
        .highlight_style(Style::default().fg(Color::Red))
        .select(0);
    f.render_widget(run_tabs, chunks[0]);
    draw_overview(f, run_app, chunks[1]);
}

fn draw_overview<B: Backend>(f: &mut Frame<'_, B>, run_app: &RunApp<'_>, area: Rect) {
    let chunks = Layout::default()
        .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref())
        .split(area);
    draw_overview_gauges(f, run_app, chunks[0]);
    draw_overview_logs(f, run_app, chunks[1]);
}

fn draw_overview_gauges<B: Backend>(f: &mut Frame<'_, B>, run_app: &RunApp<'_>, area: Rect) {
    let mut constraint = Vec::new();
    let data: Vec<f64>;
    {
        let state = run_app.state.read().unwrap();
        data = state.progress.clone();
    }
    let max = data.len();
    for _ in 0..max {
        constraint.push(Constraint::Ratio(1, max as u32));
    }

    let chunks = Layout::default().constraints(constraint).split(area);
    let mut vehicles = Vec::new();
    for i in 0..=8 {
        vehicles.push(format!("Vehicle {} progress:", i));
    }

    let mut i = 0;

    for ratio in data {
        let label = format!("{:.2}%", ratio * 100.0);
        let gauge = LineGauge::default()
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(vehicles[i].as_ref()),
            )
            .gauge_style(Style::default().fg(Color::Magenta))
            .label(label)
            .ratio(ratio);
        f.render_widget(gauge, chunks[i]);
        i += 1;
    }
}

fn draw_overview_logs<B: Backend>(f: &mut Frame<B>, run_app: &RunApp, area: Rect) {
    let state = run_app.state.read().unwrap();
    let logs: Vec<ListItem> = state
        .logs
        .iter()
        .map(|(id, event)| {
            let content = vec![Spans::from(vec![
                Span::styled(
                    format!("[{:<2}]LOG  ", id),
                    Style::default().fg(Color::Blue),
                ),
                Span::raw(event),
            ])];
            ListItem::new(content)
        })
        .collect();
    let logs = List::new(logs).block(Block::default().borders(Borders::ALL).title("Logs"));
    f.render_widget(logs, area)
}

// RunApp handles the state of the data in run mode
pub struct RunApp<'a> {
    pub title: &'a str,
    pub should_quit: bool,
    pub tabs: TabState<'a>,
    pub state: RwLock<RunAppState>,
    pub enhanced_graphics: bool,
}

struct RunAppState {
    progress: Vec<f64>,
    logs: Vec<(usize, String)>,
}

impl<'a> RunApp<'a> {
    pub(crate) fn new(title: &'a str, enhanced_graphics: bool) -> RunApp {
        RunApp {
            title,
            should_quit: false,
            tabs: TabState::new(vec!["Overview", "Connections"]),
            state: RwLock::new(RunAppState {
                progress: vec![],
                logs: vec![(0, String::from("Loading Mavsdk"))],
            }),
            enhanced_graphics,
        }
    }

    fn on_key(&mut self, c: char) {
        match c {
            'q' => {
                self.should_quit = true;
            }
            _ => {}
        }
    }

    pub fn on_left(&mut self) {
        self.tabs.prev();
    }

    pub fn on_right(&mut self) {
        self.tabs.next();
    }

    fn on_tick(&mut self) {}
}

pub async fn start_workers<'a>(
    arc: Arc<&'a mut RunApp<'a>>,
    sets: Vec<(String, String)>,
) -> JoinAll<JoinHandle<Result<(), ()>>> {
    let mut futures = vec![];
    let id = 0;
    for (uri, plan) in sets {
        let f = tokio::spawn(async move {
            let arc = Arc::clone(&arc);
            let path = Path::new(plan.as_str());
            if !(path.exists() && path.is_file()) {
                panic!(
                    "Error, path file {} does not exist or is not a file",
                    path.display()
                )
            }
            let sdk = unsafe { mavsdk::new_mavsdk() };
            let system =
                unsafe { mavsdk::connect(sdk, CString::new(uri.clone()).unwrap().as_ptr()) };
            let worker = RunWorker::new(arc, id);
            let ptr: Box<Box<dyn mavsdk::RunCallBackObject>> = Box::new(Box::new(worker));

            unsafe {
                mavsdk::run_qgc_plan(
                    system,
                    CString::new(uri).unwrap().as_ptr(),
                    Box::into_raw(ptr) as *mut Box<dyn mavsdk::RunCallBackObject>,
                    mavsdk::run_callback_position,
                    mavsdk::run_callback_progress,
                    mavsdk::run_callback_complete,
                    mavsdk::run_callback_log,
                );
            };
            Ok(())
        });
        futures.push(f);
    }
    join_all(futures)
}

struct RunWorker<'a> {
    app: Arc<&'a mut RunApp<'a>>,
    id: usize,
}

impl<'a> RunWorker<'a> {
    fn new(app: Arc<&'a mut RunApp<'a>>, id: usize) -> RunWorker<'a> {
        RunWorker { app, id }
    }
}

impl<'a> mavsdk::RunCallBackObject for RunWorker<'a> {
    fn save_position(&self, lat: f64, lon: f64, alt: f32) {
        todo!()
    }

    fn save_progress(&self, current: i32, total: i32) {
        let val = current as f64 / total as f64;
        {
            let mut data = self.app.state.write().unwrap();
            data.progress[self.id] = val;
        }
    }

    fn log(&self, msg: &str) {
        let mut data = self.app.state.write().unwrap();
        data.logs.push((self.id, msg.to_string()));
    }

    fn complete(&self) {
        println!("Worker {} done, Exiting", self.id)
    }
}

pub async fn run_app<'a, B: Backend>(
    terminal: &mut Terminal<B>,
    app: &'a mut RunApp<'a>,
    sets: Vec<(String, String)>,
    tick_rate: Duration,
) -> io::Result<()> {
    let arc = Arc::new(app);
    let ret = start_workers(arc, sets);
    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|f| draw(f, &app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char(c) => app.on_key(c),
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
        if arc.should_quit {
            ret.await;
            return Ok(());
        }
    }
}

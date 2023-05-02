mod azdevops;
mod utils;

use azdevops::get_repo_list;
use azure_devops_rust_api::git::models::GitRepository;
use tokio::runtime::Runtime;

use std::{io, time::{Duration, Instant}};
use std::error::Error;
use tui::{
    backend::{CrosstermBackend, Backend},
    widgets::{Block, Borders, List, ListState, ListItem},
    layout::{Layout, Constraint, Direction},
    Terminal, Frame, style::{Color, Style, Modifier}
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

struct StatefulList<T> {
    state: ListState,
    items: Vec<T>,
}

impl<T> StatefulList<T> {
    fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn unselect(&mut self) {
        self.state.select(None);
    }
}

struct App<'a> {
    items: StatefulList<&'a str>,
}

impl<'a> App<'a> {
	async fn new(data: Vec<&'a str>) -> App<'a> {
		App {
			items: StatefulList::with_items(data)
		}
	}
	
	fn on_tick(&mut self) {
	}
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    tick_rate: Duration,
) -> io::Result<()> {
    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Left => app.items.unselect(),
                    KeyCode::Down => app.items.next(),
                    KeyCode::Up => app.items.previous(),
                    _ => {}
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
   let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ].as_ref()
        )
        .split(f.size());
    let block = Block::default()
         .title("Block")
         .borders(Borders::ALL);
    f.render_widget(block, chunks[0]);

    let block = Block::default()
         .title("Block 2")
         .borders(Borders::ALL);

	let items: Vec<ListItem> = app.items.items.iter().map(|i| {
		// ListItem::new(i).style(Style::default().fg(Color::Black).bg(Color::White))
		ListItem::new(*i)
	}).collect();

    let items = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Repos"))
        .highlight_style(
            Style::default()
                .bg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    f.render_stateful_widget(items, chunks[1], &mut app.items.state);
}

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let tick_rate = Duration::from_millis(500);

    let repo_list = get_repo_list().await.expect("Failed to fetch repos");
    let repo_name_list = repo_list.iter().map(|repo| repo.name.as_str()).collect();
    let app = App::new(repo_name_list).await;

    let res = run_app(&mut terminal, app, tick_rate);

    // restore terminal
    disable_raw_mode()?;
    execute!(
       terminal.backend_mut(),
       LeaveAlternateScreen,
       DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
       println!("{:?}", err)
    }

    Ok(())
}

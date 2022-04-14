use crossterm::{
    event::{self, Event as CEvent, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use thiserror::Error;
use std::io;
use std::fs;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};
use serde::{Deserialize, Serialize};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span},
    widgets::{
        Block, BorderType, Borders, Cell, ListState, Row, Table,
    },
    Terminal,
};

const TX_PATH: &str = "./src/data/tx.json";


#[derive(Serialize, Deserialize, Clone)]
struct Transaction {
    tx_hash: String,
    from: String,
    to: String,
    value: usize,
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("error reading the DB file: {0}")]
    ReadDBError(#[from] io::Error),
    #[error("error parsing the DB file: {0}")]
    ParseDBError(#[from] serde_json::Error),
}

enum Event<I> {
    Input(I),
    Tick,
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode().expect("can run in raw mode");

    let (tx, rx) = mpsc::channel();
    let tick_rate = Duration::from_millis(200);
    thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if event::poll(timeout).expect("pool works") {
                if let CEvent::Key(key) = event::read().expect("can read events") {
                    tx.send(Event::Input(key)).expect("can send events");
                }
            }

            if last_tick.elapsed() >= tick_rate {
                if let Ok(_) = tx.send(Event::Tick) {
                    last_tick = Instant::now();
                }
            }
        }
    });

    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let mut tx_list_state = ListState::default();
    tx_list_state.select(Some(0));

    loop {
        terminal.draw(|rect| {
            let size = rect.size();
            let chuncks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(100)].as_ref())
                .split(size);
            rect.render_widget(render_home(&tx_list_state), chuncks[0]);
        })?;

        match rx.recv()? {
            Event::Input(event) => match event.code {
                KeyCode::Char('q') => {
                    disable_raw_mode()?;
                    terminal.show_cursor()?;
                    break;
                }
                _ => {}
            },
            Event::Tick => {}
        }
    }

    Ok(())
}

fn render_home<'a>(tx_list_state: &ListState) -> Table<'a> {
    let tx_list = read_db().expect("can fetch tx list");

    let selected_tx = tx_list
        .get(
            tx_list_state
                .selected()
                .expect("there is always a selected tx"),
        )
        .expect("exists")
        .clone();

    let tx_detail = Table::new(vec![Row::new(vec![
        Cell::from(Span::raw(selected_tx.tx_hash)),
        Cell::from(Span::raw(selected_tx.from)),
        Cell::from(Span::raw(selected_tx.to)),
        Cell::from(Span::raw(selected_tx.value.to_string())),
    ])])
    .header(Row::new(vec![
        Cell::from(Span::styled(
            "TX HASH",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "FROM",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "TO",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Cell::from(Span::styled(
            "VALUE",
            Style::default().add_modifier(Modifier::BOLD),
        )),
    ]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title("Mempool")
            .border_type(BorderType::Plain),
    )
    .widths(&[
        Constraint::Percentage(35),
        Constraint::Percentage(25),
        Constraint::Percentage(25),
        Constraint::Percentage(25),
    ]);

    tx_detail
}


fn read_db() -> Result<Vec<Transaction>, Error> {
    let db_content = fs::read_to_string(TX_PATH)?;
    let parsed: Vec<Transaction> = serde_json::from_str(&db_content)?;
    Ok(parsed)
}

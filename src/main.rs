use ethers_providers::*;
use dotenv::dotenv;
use dotenv::var;

use std::io;
use tui::{
    backend::{Backend, CrosstermBackend},
    widgets::{Block, Borders, Cell, Row, Table, TableState},
    style::{Color, Modifier, Style},
    layout::{Layout, Alignment, Constraint},
    Frame, Terminal
};
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};


struct App<'a> {
    state: TableState,
    items: Vec<Vec<&'a str>>,
}


impl<'a> App<'a> {
    fn new() -> App<'a> {
        App {
            state: TableState::default(),
            items: vec![],
        }
    }

    pub fn push(&mut self) {
        self.items.push(vec!["test3", "test2", "test1"]);
    }
}



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // setup terminal
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    
    // 1. Load files
    dotenv().ok();
    let node_url = var("NODE_URL").unwrap();

    // 2. Connect to WebSocket endpoint
    let provider = Provider::connect(node_url).await?;

    let mut stream = provider.subscribe_pending_txs().await?;

    let mut app = App::new();

    while let Some(tx) = stream.next().await {
        terminal.draw(|f| ui(f, &mut app))?;
        
        let new_tx = provider.get_transaction(tx).await?;

        if !new_tx.is_none() {
            //let pending_tx = new_tx.unwrap();
            app.push();
        }
    }
    
    // restore terminal
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    
    Ok(())
}


fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {

    let rects = Layout::default()
    .constraints([Constraint::Percentage(100)].as_ref())
    .split(f.size());

    let selected_style = Style::default().add_modifier(Modifier::REVERSED);
    let normal_style = Style::default().bg(Color::Blue);
    
    let header_cells = ["TX HASH", "FROM", "TO"]
        .iter()
        .map(|h| Cell::from(*h));
    let header = Row::new(header_cells)
        .style(normal_style)
        .height(1)
        .bottom_margin(1);
    let rows = app.items.iter().map(|item| {
        let height = item
            .iter()
            .map(|content| content.chars().filter(|c| *c == '\n').count())
            .max()
            .unwrap_or(0)
            + 1;
        let cells = item.iter().map(|c| Cell::from(*c));
        Row::new(cells).height(height as u16).bottom_margin(1)
    });
    let t = Table::new(rows)
        .header(header)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Memwatch")
                .title_alignment(Alignment::Center)
        )
        .highlight_style(selected_style)
        .highlight_symbol(">> ")
        .widths(&[
            Constraint::Percentage(20),
            Constraint::Length(20),
            Constraint::Min(20),
        ]);

    f.render_stateful_widget(t, rects[0], &mut app.state);
}

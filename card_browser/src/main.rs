use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};
use rusqlite::Connection;
use std::io;
use std::error::Error;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

const DB_PATH: &str = "./scryfall_cards.db";

struct Card {
    name: String,
    set_code: String,
    collector_number: String,
    type_line: String,
    oracle_text: Option<String>,
    mana_cost: Option<String>,
}

struct App {
    sets: Vec<String>,
    sets_state: ListState,
    cards: Vec<Card>,
    selected_card: usize,
}

impl App {
    fn new() -> Result<Self, Box<dyn Error>> {
        let conn = Connection::open(DB_PATH)?;
        let mut stmt = conn.prepare("SELECT DISTINCT set_code FROM cards ORDER BY set_code")?;
        let sets = stmt.query_map([], |row| row.get(0))?
            .collect::<Result<Vec<String>, _>>()?;

        let mut sets_state = ListState::default();
        sets_state.select(Some(0));

        Ok(App {
            sets,
            sets_state,
            cards: Vec::new(),
            selected_card: 0,
        })
    }

    fn load_cards(&mut self) -> Result<(), Box<dyn Error>> {
        if let Some(selected) = self.sets_state.selected() {
            let conn = Connection::open(DB_PATH)?;
            let mut stmt = conn.prepare("SELECT name, set_code, collector_number, type_line, oracle_text, mana_cost FROM cards WHERE set_code = ? ORDER BY collector_number")?;
            let cards = stmt.query_map([&self.sets[selected]], |row| {
                Ok(Card {
                    name: row.get(0)?,
                    set_code: row.get(1)?,
                    collector_number: row.get(2)?,
                    type_line: row.get(3)?,
                    oracle_text: row.get(4)?,
                    mana_cost: row.get(5)?,
                })
            })?
            .collect::<Result<Vec<Card>, _>>()?;

            self.cards = cards;
            self.selected_card = 0;
        }
        Ok(())
    }

    fn next_set(&mut self) {
        let i = match self.sets_state.selected() {
            Some(i) => {
                if i >= self.sets.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.sets_state.select(Some(i));
    }

    fn previous_set(&mut self) {
        let i = match self.sets_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.sets.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.sets_state.select(Some(i));
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new()?;
    app.load_cards()?;

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(20), Constraint::Percentage(80)])
                .split(f.size());

            let sets: Vec<ListItem> = app.sets
                .iter()
                .map(|m| {
                    ListItem::new(Line::from(vec![Span::styled(m, Style::default())]))
                })
                .collect();

            let sets = List::new(sets)
                .block(Block::default().borders(Borders::ALL).title("Sets"))
                .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));

            f.render_stateful_widget(sets, chunks[0], &mut app.sets_state);

            if let Some(card) = app.cards.get(app.selected_card) {
                let mut card_info = vec![
                    Line::from(vec![
                        Span::raw("Name: "),
                        Span::styled(&card.name, Style::default().fg(Color::Yellow)),
                    ]),
                    Line::from(vec![
                        Span::raw("Set: "),
                        Span::styled(&card.set_code, Style::default().fg(Color::Cyan)),
                    ]),
                    Line::from(vec![
                        Span::raw("Collector Number: "),
                        Span::styled(&card.collector_number, Style::default().fg(Color::Green)),
                    ]),
                    Line::from(vec![
                        Span::raw("Type: "),
                        Span::styled(&card.type_line, Style::default().fg(Color::Magenta)),
                    ]),
                ];

                if let Some(mana_cost) = &card.mana_cost {
                    card_info.push(Line::from(vec![
                        Span::raw("Mana Cost: "),
                        Span::styled(mana_cost, Style::default().fg(Color::Red)),
                    ]));
                }

                card_info.push(Line::from(""));
                card_info.push(Line::from(Span::styled("Oracle Text:", Style::default().add_modifier(Modifier::UNDERLINED))));

                if let Some(oracle_text) = &card.oracle_text {
                    card_info.push(Line::from(oracle_text.to_string()));
                } else {
                    card_info.push(Line::from("(No oracle text available)"));
                }

                let card_info = Paragraph::new(card_info)
                    .block(Block::default().borders(Borders::ALL).title("Card Info"));
                f.render_widget(card_info, chunks[1]);
            }
        })?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => break,
                KeyCode::Up => {
                    app.previous_set();
                    app.load_cards()?;
                }
                KeyCode::Down => {
                    app.next_set();
                    app.load_cards()?;
                }
                KeyCode::Left => {
                    app.selected_card = app.selected_card.saturating_sub(1);
                }
                KeyCode::Right => {
                    if app.selected_card < app.cards.len() - 1 {
                        app.selected_card += 1;
                    }
                }
                _ => {}
            }
        }
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
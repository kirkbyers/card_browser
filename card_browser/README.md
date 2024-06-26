# Magic: The Gathering Card Browser

This is a command-line interface (CLI) application that allows you to browse Magic: The Gathering cards by set. It uses a local SQLite database containing card information from Scryfall.

## Features

- Browse Magic: The Gathering sets
- View cards within each set
- Display detailed card information including name, set code, collector number, type line, mana cost, and oracle text

## Prerequisites

Before you begin, ensure you have met the following requirements:

- Rust programming language (latest stable version)
- SQLite database file `scryfall_cards.db`

## Installation

1. Clone this repository:

```
git clone https://github.com/your-username/mtg-card-browser.git
cd mtg-card-browser
```

2. Build the program:

```
cargo build --release
```

### Controls

- Use Up/Down arrow keys to navigate through sets
- Use Left/Right arrow keys to navigate through cards within a set
- Press 'q' to quit the program

## Dependencies

This project uses the following Rust crates:

- `ratatui`: For creating the terminal user interface
- `rusqlite`: For interacting with the SQLite database
- `crossterm`: For handling terminal input and output

## Database Structure

The program expects a SQLite database with the following table structure:

```sql
CREATE TABLE IF NOT EXISTS cards (
 id TEXT PRIMARY KEY,
 name TEXT,
 set_code TEXT,
 collector_number TEXT,
 type_line TEXT,
 oracle_text TEXT,
 mana_cost TEXT,
 cmc REAL,
 colors TEXT,
 rarity TEXT,
 image_uris TEXT
)

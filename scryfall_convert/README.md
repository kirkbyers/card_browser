# Scryfall Card Data to SQLite Converter

This Rust program converts Scryfall's bulk card data from JSON format to a SQLite database. It provides an efficient way to store and query Magic: The Gathering card information locally.

## Features

- Reads Scryfall's JSON bulk data file
- Creates a SQLite database with a structured table for card information
- Efficiently inserts card data using a database transaction
- Handles common error cases

## Prerequisites

- Rust programming language (https://www.rust-lang.org/tools/install)
- Cargo (Rust's package manager, typically installed with Rust)
- Scryfall bulk data file (`scryfall-default-cards.json`)

## Dependencies

This project uses the following external crates:

- `rusqlite`: For SQLite database operations
- `serde_json`: For parsing JSON data

## Setup

1. Clone this repository or create a new Rust project.
2. Ensure you have the Scryfall bulk data file (`scryfall-default-cards.json`) in the project root directory.

## Usage

1. Place the scryfall-default-cards.json file in the same directory as your Rust project.
2. Run the program using Cargo:

```bash
cargo run
```

The program will create a SQLite database file named scryfall_cards.db in the project directory and populate it with the card data from the JSON file.
Database Schema
The created SQLite database contains a single table named cards with the following columns:

```
id (TEXT, PRIMARY KEY): Unique identifier for the card
name (TEXT): Name of the card
set_code (TEXT): Set code the card belongs to
collector_number (TEXT): Collector number of the card
type_line (TEXT): Type line of the card
oracle_text (TEXT): Oracle text of the card
mana_cost (TEXT): Mana cost of the card
cmc (REAL): Converted mana cost of the card
colors (TEXT): Colors of the card (stored as a JSON array string)
rarity (TEXT): Rarity of the card
image_uris (TEXT): Image URIs for the card (stored as a JSON object string)
```

### Error Handling
The program uses Rust's Result type with Box<dyn std::error::Error> for error handling. It will print any errors to the console if they occur during execution.

## Contributing
Contributions to improve the program are welcome. Please feel free to submit issues or pull requests.
# studibudget — Student Budget CLI

**Created by Brian Doctor**

A tiny Rust CLI that helps students track income & expenses, export CSV, and estimate how many months of tuition your current balance can cover ("runway").

## Features
- `add` income/expense quickly
- `summary` shows balance and recent transactions
- `export` to CSV for spreadsheets
- `runway` computes months of tuition you can pay from your balance
- Stores data locally under your OS app data directory

## Quick Start
```bash
# Build
cargo build --release

# Add entries
./target/release/studibudget add income 450 "Kaufland shift" --date 2025-10-20
./target/release/studibudget add expense 320 "Rent part"

# See summary
./target/release/studibudget summary --last 20

# Tuition runway (e.g., €250/month)
./target/release/studibudget runway 250

# Export CSV
./target/release/studibudget export out.csv
```

## Install
- From source: `cargo install --path .`
- From releases: download a binary for your platform and place it on your PATH.

## Data Location
Defaults to an OS-specific app data folder, e.g. `~/.local/share/studibudget/ledger.json` on Linux.

## License
MIT

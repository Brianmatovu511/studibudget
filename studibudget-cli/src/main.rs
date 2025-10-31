use clap::{Parser, Subcommand, ValueEnum};
use chrono::{DateTime, Local, NaiveDate};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{BufReader, Write};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name="studibudget", version, about="Track income, expenses & tuition runway from your terminal.")]
struct Cli {
    /// Use a custom data file (defaults to OS app data dir)
    #[arg(long)]
    data: Option<PathBuf>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Add a transaction
    Add {
        /// income or expense
        #[arg(value_enum)]
        kind: Kind,
        /// amount in EUR (e.g. 25.50)
        amount: f64,
        /// short description
        #[arg(short, long, default_value="")]
        note: String,
        /// YYYY-MM-DD (defaults to today)
        #[arg(short, long)]
        date: Option<NaiveDate>,
    },
    /// Show current balance and recent items
    Summary {
        /// show N most recent items
        #[arg(short, long, default_value_t=10)]
        last: usize,
    },
    /// Export to CSV
    Export {
        /// output file path (e.g. out.csv)
        out: PathBuf,
    },
    /// Estimate months of tuition runway given monthly tuition in EUR
    Runway {
        /// monthly tuition fee in EUR
        tuition_monthly: f64,
    },
    /// Reset all data (asks for confirmation)
    Reset
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Tx {
    ts: DateTime<Local>,
    date: String,
    kind: Kind,
    amount: f64,
    note: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, ValueEnum, PartialEq, Eq)]
enum Kind {
    Income,
    Expense,
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct Ledger {
    items: Vec<Tx>,
}

impl Ledger {
    fn balance(&self) -> f64 {
        self.items.iter().map(|t| match t.kind {
            Kind::Income =>  t.amount,
            Kind::Expense => -t.amount,
        }).sum()
    }
}

fn default_data_file() -> PathBuf {
    if let Some(proj) = ProjectDirs::from("de", "DIT", "studibudget") {
        let dir = proj.data_dir().to_path_buf();
        fs::create_dir_all(&dir).ok();
        return dir.join("ledger.json");
    }
    PathBuf::from("ledger.json")
}

fn load_ledger(path: &PathBuf) -> Ledger {
    if path.exists() {
        let f = File::open(path).expect("cannot open data file");
        let reader = BufReader::new(f);
        serde_json::from_reader(reader).unwrap_or_default()
    } else {
        Ledger::default()
    }
}

fn save_ledger(path: &PathBuf, lg: &Ledger) {
    let mut f = File::create(path).expect("cannot create data file");
    f.write_all(serde_json::to_string_pretty(lg).unwrap().as_bytes()).unwrap();
}

fn main() {
    let cli = Cli::parse();

    let data_path = cli.data.unwrap_or_else(|| default_data_file());
    let mut ledger = load_ledger(&data_path);

    match cli.command {
        Commands::Add { kind, amount, note, date } => {
            let today = Local::now().date_naive();
            let d = date.unwrap_or(today);
            let tx = Tx {
                ts: Local::now(),
                date: d.to_string(),
                kind,
                amount,
                note,
            };
            ledger.items.push(tx);
            save_ledger(&data_path, &ledger);
            println!("Added {kind:?} {:.2} EUR. Balance now: {:.2} EUR", amount, ledger.balance());
        }
        Commands::Summary { last } => {
            println!("Current balance: {:.2} EUR", ledger.balance());
            let n = ledger.items.len().saturating_sub(last);
            for tx in ledger.items.iter().skip(n) {
                let sign = match tx.kind { Kind::Income => "+", Kind::Expense => "-" };
                println!("{} {}{:.2}  | {}  {}", tx.date, sign, tx.amount, tx.note, if tx.kind==Kind::Income {"(in)"} else {"(out)"});
            }
        }
        Commands::Export { out } => {
            let mut wtr = csv::Writer::from_path(&out).expect("cannot open output");
            wtr.write_record(&["timestamp","date","kind","amount","note"]).unwrap();
            for t in &ledger.items {
                wtr.write_record(&[
                    t.ts.to_rfc3339(),
                    t.date.clone(),
                    format!("{:?}", t.kind),
                    format!("{:.2}", t.amount),
                    t.note.clone()
                ]).unwrap();
            }
            wtr.flush().unwrap();
            println!("Exported {} items to {}", ledger.items.len(), out.display());
        }
        Commands::Runway { tuition_monthly } => {
            let bal = ledger.balance();
            if tuition_monthly <= 0.0 {
                eprintln!("tuition_monthly must be > 0");
                std::process::exit(1);
            }
            let months = bal / tuition_monthly;
            println!("Balance: {:.2} EUR | Tuition/month: {:.2} EUR | Runway: {:.2} months", bal, tuition_monthly, months);
        }
        Commands::Reset => {
            println!("Type 'YES' to confirm reset:");
            let mut s = String::new();
            std::io::stdin().read_line(&mut s).unwrap();
            if s.trim() == "YES" {
                ledger = Ledger::default();
                save_ledger(&data_path, &ledger);
                println!("All data cleared.");
            } else {
                println!("Aborted.");
            }
        }
    }
}

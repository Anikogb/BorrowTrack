use clap::{Parser, Subcommand};
use borrowtrack::BorrowTrack;
use std::path::PathBuf;
use chrono::{Utc, Duration};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new user
    AddUser { name: String },
    /// Add a new item
    AddItem { name: String, description: String },
    /// Create a new loan
    Loan { owner_id: u32, borrower_id: u32, item_id: u32, days: i64 },
    /// Mark an item as returned
    Return { loan_id: u32 },
    /// List all active loans
    List,
    /// List overdue loans
    Overdue,
    /// Search for users or items
    Search { 
        #[arg(short, long)]
        user: Option<String>,
        #[arg(short, long)]
        item: Option<String>,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let mut bt = BorrowTrack::new(PathBuf::from("data.json"))?;

    match cli.command {
        Commands::AddUser { name } => {
            bt.add_user(name.clone());
            println!("User '{}' added.", name);
        }
        Commands::AddItem { name, description } => {
            bt.add_item(name.clone(), description);
            println!("Item '{}' added.", name);
        }
        Commands::Loan { owner_id, borrower_id, item_id, days } => {
            let now = Utc::now();
            let due = now + Duration::days(days);
            bt.create_loan(owner_id, borrower_id, item_id, now, due);
            println!("Loan created. Due in {} days.", days);
        }
        Commands::Return { loan_id } => {
            if bt.mark_returned(loan_id) {
                println!("Loan {} marked as returned.", loan_id);
            } else {
                println!("Loan {} not found.", loan_id);
            }
        }
        Commands::List => {
            let active = bt.list_active_loans();
            if active.is_empty() {
                println!("No active loans.");
            } else {
                for loan in active {
                    println!("{:?}", loan);
                }
            }
        }
        Commands::Overdue => {
            let overdue = bt.overdue_loans();
            if overdue.is_empty() {
                println!("No overdue loans.");
            } else {
                for loan in overdue {
                    println!("{:?}", loan);
                }
            }
        }
        Commands::Search { user, item } => {
            if let Some(name) = user {
                let found = bt.search_by_user(&name);
                println!("Users found: {:?}", found);
            }
            if let Some(name) = item {
                let found = bt.search_by_item(&name);
                println!("Items found: {:?}", found);
            }
        }
    }

    bt.save()?;
    Ok(())
}

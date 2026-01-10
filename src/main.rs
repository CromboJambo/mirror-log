mod db;
mod log;
mod view;

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "mirror-log")]
#[command(about = "Append-only event log with SQLite", long_about = None)]
struct Cli {
    #[arg(short, long, default_value = "mirror.db")]
    db: PathBuf,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add an event to the log
    Add {
        /// The content to log
        content: String,

        #[arg(short, long, default_value = "cli")]
        source: String,

        #[arg(short, long)]
        meta: Option<String>,
    },

    /// Add events from stdin (one per line)
    Stdin {
        #[arg(short, long, default_value = "stdin")]
        source: String,
    },

    /// Show recent events
    Show {
        #[arg(short, long, default_value_t = 20)]
        last: i64,

        #[arg(short, long)]
        source: Option<String>,
    },

    /// Search events by content
    Search {
        /// Search term
        term: String,
    },

    /// Get a specific event by ID
    Get {
        /// Event ID
        id: String,
    },

    /// Show database info
    Info,
}

fn main() {
    let cli = Cli::parse();

    let db_path = cli.db.to_str().unwrap_or("mirror.db");
    let conn = db::init_db(db_path).expect("Failed to open database");

    match cli.command {
        Commands::Add {
            content,
            source,
            meta,
        } => {
            let id = log::append(&conn, &source, &content, meta.as_deref())
                .expect("Failed to append event");
            println!("Added: {}", id);
        }

        Commands::Stdin { source } => {
            let ids = log::append_stdin(&conn, &source).expect("Failed to read from stdin");
            println!("Added {} events", ids.len());
            for id in ids {
                println!("  {}", id);
            }
        }

        Commands::Show { last, source } => {
            let events = if let Some(src) = source {
                view::by_source(&conn, &src, Some(last)).expect("Failed to query events")
            } else {
                view::recent(&conn, last).expect("Failed to query events")
            };

            if events.is_empty() {
                println!("No events found");
            } else {
                for event in events {
                    println!("\n[{}] {}", event.format_time(), event.source);
                    println!("ID: {}", event.id);
                    println!("{}", event.content);
                    if let Some(meta) = event.meta {
                        println!("Meta: {}", meta);
                    }
                }
            }
        }

        Commands::Search { term } => {
            let events = view::search(&conn, &term).expect("Failed to search events");

            if events.is_empty() {
                println!("No events found matching '{}'", term);
            } else {
                println!("Found {} events:\n", events.len());
                for event in events {
                    println!("[{}] {}", event.format_time(), event.source);
                    println!("ID: {}", event.id);
                    println!("{}", event.content);
                    if let Some(meta) = event.meta {
                        println!("Meta: {}", meta);
                    }
                    println!();
                }
            }
        }

        Commands::Get { id } => match view::get_by_id(&conn, &id) {
            Ok(event) => {
                println!("\n[{}] {}", event.format_time(), event.source);
                println!("ID: {}", event.id);
                println!("{}", event.content);
                if let Some(meta) = event.meta {
                    println!("Meta: {}", meta);
                }
            }
            Err(_) => {
                eprintln!("Event not found: {}", id);
                std::process::exit(1);
            }
        },

        Commands::Info => {
            let (count, oldest, newest) = db::db_info(&conn).expect("Failed to get database info");

            println!("Database: {}", db_path);
            println!("Total events: {}", count);

            if count > 0 {
                use chrono::{DateTime, TimeZone, Utc};
                let oldest_dt: DateTime<Utc> = Utc.timestamp_opt(oldest, 0).unwrap();
                let newest_dt: DateTime<Utc> = Utc.timestamp_opt(newest, 0).unwrap();

                println!("Oldest: {}", oldest_dt.format("%Y-%m-%d %H:%M:%S UTC"));
                println!("Newest: {}", newest_dt.format("%Y-%m-%d %H:%M:%S UTC"));
            }
        }
    }
}

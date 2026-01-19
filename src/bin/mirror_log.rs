use ::mirror_log::chunk;
use ::mirror_log::db;
use ::mirror_log::log;
use ::mirror_log::view;

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

    /// Add a file's contents as a single event
    AddFile {
        /// Path to the file
        path: std::path::PathBuf,

        #[arg(short, long, default_value = "file")]
        source: String,

        #[arg(short, long)]
        meta: Option<String>,
    },

    /// Add events from stdin (one per line)
    Stdin {
        #[arg(short, long, default_value = "stdin")]
        source: String,

        #[arg(short, long)]
        meta: Option<String>,
    },

    /// Show recent events
    Show {
        #[arg(short, long, default_value_t = 20)]
        last: i64,

        #[arg(short, long)]
        source: Option<String>,

        #[arg(short, long)]
        preview: Option<usize>,
    },

    /// Search events by content
    Search {
        /// Search term
        term: String,

        #[arg(short, long)]
        preview: Option<usize>,

        #[arg(long)]
        chunks: bool,
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

        Commands::AddFile { path, source, meta } => {
            let content = std::fs::read_to_string(&path).expect("Failed to read file");

            let timestamp = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64;

            let id = log::append(&conn, &source, &content, meta.as_deref())
                .expect("Failed to append event");

            // Auto-chunk large files (> 2000 chars)
            if content.len() > 2000 {
                let chunk_count = chunk::create_chunks(&conn, &id, &content, timestamp, 1500)
                    .expect("Failed to create chunks");
                println!(
                    "Added file: {} ({}) - created {} chunks",
                    path.display(),
                    id,
                    chunk_count
                );
            } else {
                println!("Added file: {} ({})", path.display(), id);
            }
        }

        Commands::Stdin { source, meta } => {
            match log::append_stdin(&conn, &source, meta.as_deref()) {
                Ok(ids) => {
                    println!("Added {} events", ids.len());
                }
                Err(e) => {
                    eprintln!("Failed to read from stdin: {}", e);
                    std::process::exit(1);
                }
            }
        }

        Commands::Show {
            last,
            source,
            preview,
        } => {
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

                    if let Some(max_chars) = preview {
                        println!("{}", event.preview_content(max_chars));
                    } else {
                        println!("{}", event.content);
                    }

                    if let Some(meta) = event.meta {
                        println!("Meta: {}", meta);
                    }
                }
            }
        }

        Commands::Search {
            term,
            preview,
            chunks,
        } => {
            if chunks {
                // Search chunks instead of full events
                let found_chunks =
                    chunk::search_chunks(&conn, &term, Some(20)).expect("Failed to search chunks");

                if found_chunks.is_empty() {
                    println!("No chunks found matching '{}'", term);
                } else {
                    println!("Found {} chunks:\n", found_chunks.len());
                    for chunk in found_chunks {
                        // Get parent event for context
                        let event = view::get_by_id(&conn, &chunk.event_id)
                            .expect("Failed to get parent event");

                        println!(
                            "[{}] {} (chunk {}/...)",
                            event.format_time(),
                            event.source,
                            chunk.chunk_index + 1
                        );
                        println!("Event ID: {}", event.id);
                        println!("Chunk ID: {}", chunk.id);

                        if let Some(max_chars) = preview {
                            if chunk.content.len() > max_chars {
                                println!(
                                    "{}...\n[{} of {} chars]",
                                    &chunk.content[..max_chars],
                                    max_chars,
                                    chunk.content.len()
                                );
                            } else {
                                println!("{}", chunk.content);
                            }
                        } else {
                            println!("{}", chunk.content);
                        }

                        if let Some(meta) = event.meta {
                            println!("Meta: {}", meta);
                        }
                        println!();
                    }
                }
            } else {
                // Original full-event search
                let events = view::search(&conn, &term).expect("Failed to search events");

                if events.is_empty() {
                    println!("No events found matching '{}'", term);
                } else {
                    println!("Found {} events:\n", events.len());
                    for event in events {
                        println!("[{}] {}", event.format_time(), event.source);
                        println!("ID: {}", event.id);

                        if let Some(max_chars) = preview {
                            println!("{}", event.preview_content(max_chars));
                        } else {
                            println!("{}", event.content);
                        }

                        if let Some(meta) = event.meta {
                            println!("Meta: {}", meta);
                        }
                        println!();
                    }
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

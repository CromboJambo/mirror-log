use std::path::PathBuf;

use chrono::{DateTime, TimeZone, Utc};
use clap::{Parser, Subcommand};
use mirror_log::{chunk, db, log, pipeline, view};

#[derive(Parser)]
#[command(name = "mirror-log")]
#[command(about = "Append-only event log with SQLite", long_about = None)]
struct Cli {
    #[arg(short, long, default_value = "mirror.db")]
    db: PathBuf,

    #[arg(short, long, default_value_t = 1000)]
    batch_size: usize,

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
        path: PathBuf,

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

    /// Show ingestion statistics
    Stats,

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

    /// Verify database integrity invariants
    Verify,

    /// Generate embeddings for events in a source
    Embed {
        #[arg(short, long, default_value = "cli")]
        source: String,

        #[arg(long)]
        model: Option<String>,
    },

    /// Search similar events using embeddings
    SearchSimilar {
        /// Search term (used to generate query vector)
        term: String,

        #[arg(long, default_value_t = 10)]
        limit: usize,
    },
}

fn main() {
    let cli = Cli::parse();
    let db_path = cli.db;
    let conn = db::init_db(&db_path).expect("Failed to open database");

    match cli.command {
        Commands::Add {
            content,
            source,
            meta,
        } => {
            let result = pipeline::ingest_single(
                &conn,
                pipeline::IngestRequest::new(&source, &content, meta.as_deref()),
            )
            .expect("Failed to append event");

            if result.chunk_count > 0 {
                println!(
                    "Added: {} (created {} chunks)",
                    result.event_id, result.chunk_count
                );
            } else {
                println!("Added: {}", result.event_id);
            }
        }

        Commands::AddFile { path, source, meta } => {
            let content = std::fs::read_to_string(&path).expect("Failed to read file");
            let result = pipeline::ingest_single(
                &conn,
                pipeline::IngestRequest::new(&source, &content, meta.as_deref()),
            )
            .expect("Failed to append event");

            if result.chunk_count > 0 {
                println!(
                    "Added file: {} ({}) - created {} chunks",
                    path.display(),
                    result.event_id,
                    result.chunk_count
                );
            } else {
                println!("Added file: {} ({})", path.display(), result.event_id);
            }
        }

        Commands::Stdin { source, meta } => {
            match pipeline::ingest_stdin(&conn, &source, meta.as_deref(), cli.batch_size) {
                Ok(result) => {
                    println!("Added {} events", result.event_ids.len());
                    if result.total_chunks > 0 {
                        println!(
                            "Structured {} events into {} chunks",
                            result.chunked_events, result.total_chunks
                        );
                    }
                }
                Err(e) => {
                    eprintln!("Failed to read from stdin: {}", e);
                    std::process::exit(1);
                }
            }
        }

        Commands::Stats => {
            let (total, unique, oldest, newest) = log::stats(&conn).expect("Failed to get stats");

            println!("Ingestion Statistics:");
            println!("  Total events: {}", total);
            println!("  Unique events: {}", unique);
            println!("  Duplicate events: {}", total - unique);

            if total > 0 {
                let oldest_dt: DateTime<Utc> = Utc.timestamp_opt(oldest, 0).unwrap();
                let newest_dt: DateTime<Utc> = Utc.timestamp_opt(newest, 0).unwrap();

                println!("  Oldest: {}", oldest_dt.format("%Y-%m-%d %H:%M:%S UTC"));
                println!("  Newest: {}", newest_dt.format("%Y-%m-%d %H:%M:%S UTC"));
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
                let found_chunks =
                    chunk::search_chunks(&conn, &term, Some(20)).expect("Failed to search chunks");

                if found_chunks.is_empty() {
                    println!("No chunks found matching '{}'", term);
                } else {
                    println!("Found {} chunks:\n", found_chunks.len());
                    for chunk in found_chunks {
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
                            let total_chars = chunk.content.chars().count();
                            if total_chars > max_chars {
                                let preview_text: String =
                                    chunk.content.chars().take(max_chars).collect();
                                println!(
                                    "{}...\n[{} of {} chars]",
                                    preview_text, max_chars, total_chars
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

            println!("Database: {}", db_path.display());
            println!("Total events: {}", count);

            if count > 0 {
                let oldest_dt: DateTime<Utc> = Utc.timestamp_opt(oldest, 0).unwrap();
                let newest_dt: DateTime<Utc> = Utc.timestamp_opt(newest, 0).unwrap();

                println!("Oldest: {}", oldest_dt.format("%Y-%m-%d %H:%M:%S UTC"));
                println!("Newest: {}", newest_dt.format("%Y-%m-%d %H:%M:%S UTC"));
            }
        }

        Commands::Verify => {
            let report = log::verify_integrity(&conn).expect("Failed to verify database integrity");
            let issues =
                report.missing_or_invalid_hashes + report.hash_mismatches + report.orphan_chunks;

            println!("Integrity Report:");
            println!("  Total events: {}", report.total_events);
            println!(
                "  Missing/invalid hashes: {}",
                report.missing_or_invalid_hashes
            );
            println!("  Hash mismatches: {}", report.hash_mismatches);
            println!("  Orphan chunks: {}", report.orphan_chunks);

            if issues == 0 {
                println!("  Status: OK");
            } else {
                println!("  Status: FAILED ({} issues)", issues);
                std::process::exit(1);
            }
        }

        Commands::Embed { .. } => {
            eprintln!(
                "⚠️  Embed command temporarily disabled (embedding features not yet implemented)"
            );
            eprintln!("This feature will be available in a future release.");
            println!("To enable embedding support, add the 'embedding' feature to your Cargo.toml");
            std::process::exit(0);
        }

        Commands::SearchSimilar { .. } => {
            eprintln!("⚠️  Search similar command temporarily disabled");
            eprintln!("Semantic similarity search is coming in the next release.");
            println!("Run `mirror-log help` to see other available commands.");
            std::process::exit(0);
        }
    }
}

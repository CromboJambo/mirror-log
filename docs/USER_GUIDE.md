# Mirror-Log User Guide

Welcome to Mirror-Log, your local-first, SQLite-backed append-only event log for capturing thoughts, notes, and data you do not want to lose.

## Table of Contents

1. [Getting Started](#getting-started)
2. [Installation](#installation)
3. [Basic Usage](#basic-usage)
4. [Advanced Features](#advanced-features)
5. [Configuration](#configuration)
6. [Data Management](#data-management)
7. [Integration](#integration)
8. [Troubleshooting](#troubleshooting)
9. [Best Practices](#best-practices)
10. [FAQ](#faq)

## Getting Started

### What is Mirror-Log?

Mirror-Log is an append-only event log designed for:

- **Knowledge Capture**: Store thoughts, notes, and ideas
- **Data Preservation**: Keep important information safe and local
- **Searchability**: Find content quickly with full-text search
- **Chunking**: Break large content into searchable chunks
- **Deduplication**: Detect and track duplicate events
- **Source Tracking**: Know where each event came from

### Core Principles

- **Append-only**: Events are never updated or deleted
- **SQLite is source of truth**: Your data stays local and inspectable
- **No hidden layers**: Direct SQL remains first-class
- **Source-aware logging**: Every event tracks where it came from

## Installation

### From Source

```bash
git clone https://github.com/CromboJambo/mirror-log
cd mirror-log
cargo build --release
```

The binary will be at `target/release/mirror-log`.

### Local Installation

```bash
cargo install --path .
```

### Prerequisites

- Rust toolchain (if building from source)
- SQLite library (bundled with rusqlite on most platforms)

## Basic Usage

### Adding Events

#### From Command Line

```bash
# Simple addition
mirror-log add "Overhead allocation needs review" --source journal

# With metadata
mirror-log add "Meeting notes" --source meetings --meta '{"important": true}'

# From file
mirror-log add-file notes.md --source meetings
```

#### From Stdin

```bash
# Pipe content to stdin
cat ideas.txt | mirror-log stdin --source ideas

# Multiple events
printf "Event 1\nEvent 2\nEvent 3\n" | mirror-log stdin --source stdin_test
```

### Viewing Events

```bash
# Show recent events
mirror-log show --last 10

# Show events from specific source
mirror-log show --source journal --last 5

# Preview with character limit
mirror-log show --last 5 --preview 100
```

### Searching

```bash
# Full event search
mirror-log search "overhead"

# Search within chunked content
mirror-log search "allocation" --chunks

# Preview search results
mirror-log search "meeting" --preview 200
```

### Statistics

```bash
# Show ingestion statistics
mirror-log stats

# Database information
mirror-log info
```

## Advanced Features

### Chunking

Mirror-Log automatically chunks large content for efficient search:

```bash
# Content larger than chunk size will be auto-chunked
mirror-log add "Very long content that exceeds the chunk size threshold..."
```

Chunking is configured by the `chunk_size` parameter (default: 1500 bytes).

### Duplicate Detection

Mirror-Log tracks duplicates using SHA256 hashing:

```bash
# Add duplicate content
mirror-log add "Same content" --source source1
mirror-log add "Same content" --source source2

# Check stats
mirror-log stats
# Shows: total events, unique events, duplicates
```

### Metadata

Store JSON or text metadata with events:

```bash
# JSON metadata
mirror-log add "Event with meta" --source test --meta '{"key": "value"}'

# Text metadata
mirror-log add "Event with meta" --source test --meta "Some text metadata"
```

### Database Management

```bash
# Specify custom database path
mirror-log --db /path/to/custom.db add "Event"

# Default database is mirror.db
```

## Configuration

### Global Options

```bash
# Database path
--db <path>

# Batch size for stdin ingestion (default: 1000)
--batch-size <n>
```

### Command Options

```bash
# Add command
mirror-log add <content> [--source <name>] [--meta <json-or-text>]

# Add-file command
mirror-log add-file <path> [--source <name>] [--meta <json-or-text>]

# Stdin command
mirror-log stdin [--source <name>] [--meta <json-or-text>]

# Show command
mirror-log show [--last <n>] [--source <name>] [--preview <chars>]

# Search command
mirror-log search <term> [--preview <chars>] [--chunks]

# Get command
mirror-log get <event-id>

# Stats command
mirror-log stats

# Info command
mirror-log info
```

## Data Management

### Direct SQLite Access

Mirror-Log uses SQLite as its data store, allowing direct inspection:

```bash
# Open database with sqlite3
sqlite3 mirror.db

# Query events
SELECT datetime(timestamp, 'unixepoch'), source, content
FROM events
ORDER BY timestamp DESC
LIMIT 10;

# Count events by source
SELECT source, COUNT(*)
FROM events
GROUP BY source
ORDER BY COUNT(*) DESC;

# Get statistics
SELECT COUNT(*) AS total,
       COUNT(DISTINCT content_hash) AS unique_events
FROM events;
```

### Database Schema

#### Events Table

- `id TEXT PRIMARY KEY` (UUID)
- `timestamp INTEGER NOT NULL` (event timestamp)
- `source TEXT NOT NULL`
- `content TEXT NOT NULL`
- `meta TEXT NULL`
- `ingested_at INTEGER NOT NULL`
- `content_hash TEXT NULL` (SHA256 for dedupe analytics)

#### Chunks Table

- Stores chunked slices of event content
- Columns: `event_id`, `chunk_index`, offsets, text, timestamp
- Used by `search --chunks` and large-content workflows

#### Additional Tables

- `event_tags`
- `event_links`
- `event_embeddings`
- `enrichment_jobs`

These are for future layering without mutating raw events.

### Backup and Restore

```bash
# Backup database
cp mirror.db mirror.db.backup

# Restore database
cp mirror.db.backup mirror.db
```

## Integration

### Command Line Integration

```bash
# Pipe from other tools
git log --oneline | mirror-log stdin --source git

# Use with cron jobs
0 2 * * * /usr/local/bin/mirror-log add "Daily backup completed" --source automation
```

### Scripting

```bash
#!/bin/bash
# Script to add events from a file

mirror-log add-file "$1" --source "$(basename "$1" .txt)"
```

### API Usage

Mirror-Log provides a Rust library for programmatic access:

```rust
use mirror_log::{log, view, db};

// Initialize database
let db_path = "mirror.db";
let conn = db::init_db(db_path)?;

// Append event
let event_id = log::append(&conn, "source", "content", None)?;

// Get event
let event = view::get_by_id(&conn, &event_id)?;

// Search
let events = view::search(&conn, "search term")?;

// Stats
let (total, unique, oldest, newest) = log::stats(&conn)?;
```

## Troubleshooting

### Common Issues

**Issue**: Database file not found
```
Solution: Use --db flag to specify database path
```

**Issue**: Large files taking too long
```
Solution: Use --batch-size to control stdin ingestion speed
```

**Issue**: Search not finding content
```
Solution: Try --chunks flag to search within chunked content
```

**Issue**: Duplicate events not detected
```
Solution: Check that content_hash is populated, duplicates are allowed
```

### Error Messages

- **"Failed to initialize DB"**: Check database path permissions
- **"Failed to append"**: Check content validity and database connection
- **"Failed to get stats"**: Verify database exists and is accessible

## Best Practices

### 1. Consistent Source Naming

Use consistent source names for better organization:

```bash
mirror-log add "Idea" --source ideas
mirror-log add "Meeting notes" --source meetings
mirror-log add "Code snippets" --source code
```

### 2. Metadata Usage

Use metadata to add context:

```bash
mirror-log add "Important decision" --source decisions --meta '{"priority": "high"}'
```

### 3. Regular Backups

Backup your database regularly:

```bash
# Create backup
cp mirror.db mirror.db.backup-$(date +%Y%m%d)
```

### 4. Search Strategy

- Use specific terms for better results
- Try both full search and chunked search
- Use preview flag to see content before searching

### 5. Database Maintenance

```bash
# Check database size
ls -lh mirror.db

# Optimize database (SQLite feature)
sqlite3 mirror.db "VACUUM;"
```

### 6. Integration with LLMs

Mirror-Log's markdown-ready format works great with LLMs:

```bash
# Export for LLM processing
mirror-log show --last 100 > events.md
```

## FAQ

### Q: Can I delete events?

**A**: No, Mirror-Log is append-only. However, you can create new events with different content or use direct SQL to modify the database (not recommended for regular use).

### Q: How do I migrate to a new database?

**A**: Copy the database file and use it with the new installation:

```bash
cp mirror.db /new/path/mirror.db
```

### Q: What happens if the database gets corrupted?

**A**: Mirror-Log will handle corruption gracefully. The database file should be recreated if needed.

### Q: Can I use multiple databases?

**A**: Yes! Use the `--db` flag to specify different database paths for different projects or purposes.

### Q: How large can the database be?

**A**: SQLite has no practical size limit for most use cases. Mirror-Log has been tested with databases containing millions of events.

### Q: Is my data secure?

**A**: Yes, all data is stored locally on your machine. No data leaves your system unless you explicitly export it.

### Q: Can I export events?

**A**: While not a built-in command, you can use SQLite directly to export events:

```bash
sqlite3 mirror.db "SELECT * FROM events;" > export.csv
```

### Q: How do I reset the database?

**A**: Delete the database file and start fresh:

```bash
rm mirror.db
```

### Q: What's the difference between total and unique events?

**A**: 
- **Total**: Total number of events added
- **Unique**: Number of unique events (based on content_hash)

### Q: Can I use Mirror-Log in production?

**A**: Mirror-Log is designed for personal knowledge management and production use. It's stable and tested, but always backup your data before critical operations.

### Q: How do I contribute?

**A**: See the GitHub repository for contribution guidelines and issues.

## Support and Resources

- **GitHub Repository**: https://github.com/CromboJambo/mirror-log
- **Issues**: Report bugs and request features on GitHub
- **Documentation**: This user guide and README.md

## License

AGPL-3.0-or-later. See LICENSE file for details.

---

Happy logging! 📝
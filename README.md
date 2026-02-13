# mirror-log

An append-only event log for capturing thoughts, notes, and data you do not want to lose.

`mirror-log` is local-first, SQLite-backed, and designed to be boring in the best way: easy to inspect, easy to script, and hard to accidentally lose context.

## What Changed in v0.1.4

- `init_db` now accepts path-like inputs (`&str`, `PathBuf`, etc.) for cleaner integration usage.
- Duplicate events are allowed again (append-only semantics preserved), while hash-based lookup remains indexed for dedupe stats.
- `is_duplicate` now safely returns `false` when no matching hash exists.
- Chunk splitting is more robust and UTF-8 safe for large content.
- Integration tests and CLI-path handling were cleaned up; test and lint pipeline is green.

## Core Principles

- Append-only: events are never updated or deleted.
- SQLite is the source of truth: your data stays local and inspectable.
- No hidden layers: direct SQL remains first-class.
- Source-aware logging: every event tracks where it came from.

## Installation

```bash
git clone https://github.com/CromboJambo/mirror-log
cd mirror-log
cargo build --release
```

Binary output:

- `target/release/mirror-log`

Optional local install:

```bash
cargo install --path .
```

## Quick Start

```bash
# Add one event
mirror-log add "Overhead allocation needs review" --source journal

# Add from file
mirror-log add-file notes.md --source meetings

# Bulk import from stdin (one line = one event)
cat ideas.txt | mirror-log stdin --source ideas

# Show recent
mirror-log show --last 10

# Search full events
mirror-log search "overhead"

# Search chunked content
mirror-log search "allocation" --chunks

# Ingestion stats (total/unique/duplicates)
mirror-log stats

# Database summary
mirror-log info
```

## CLI Overview

Global flags:

- `--db <path>`: SQLite database path (default: `mirror.db`)
- `--batch-size <n>`: stdin ingest batch size (default: `1000`)

Commands:

- `add <content> [--source <name>] [--meta <json-or-text>]`
- `add-file <path> [--source <name>] [--meta <json-or-text>]`
- `stdin [--source <name>] [--meta <json-or-text>]`
- `show [--last <n>] [--source <name>] [--preview <chars>]`
- `search <term> [--preview <chars>] [--chunks]`
- `get <event-id>`
- `stats`
- `info`

## Data Model

Main table: `events`

- `id TEXT PRIMARY KEY` (UUID)
- `timestamp INTEGER NOT NULL` (event timestamp)
- `source TEXT NOT NULL`
- `content TEXT NOT NULL`
- `meta TEXT NULL`
- `ingested_at INTEGER NOT NULL`
- `content_hash TEXT NULL` (SHA256 for dedupe analytics)

Chunk table: `chunks`

- Stores chunked slices of event content (`event_id`, `chunk_index`, offsets, text, timestamp).
- Used by `search --chunks` and large-content workflows.

Additional enrichment tables also exist (`event_tags`, `event_links`, `event_embeddings`, `enrichment_jobs`) for future layering without mutating raw events.

## Direct SQLite Access

```bash
sqlite3 mirror.db
```

```sql
SELECT datetime(timestamp, 'unixepoch'), source, content
FROM events
ORDER BY timestamp DESC
LIMIT 10;
```

```sql
SELECT source, COUNT(*)
FROM events
GROUP BY source
ORDER BY COUNT(*) DESC;
```

```sql
SELECT COUNT(*) AS total,
       COUNT(DISTINCT content_hash) AS unique_events
FROM events;
```

## Development

```bash
cargo fmt
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

## License

AGPL-3.0-or-later. See `LICENSE`.

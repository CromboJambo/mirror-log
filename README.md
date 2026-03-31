```
░  ░░░░  ░░        ░░       ░░░       ░░░░      ░░░       ░░░  ░░░░░░░░░      ░░░░      ░░
▒   ▒▒   ▒▒▒▒▒  ▒▒▒▒▒  ▒▒▒▒  ▒▒  ▒▒▒▒  ▒▒  ▒▒▒▒  ▒▒  ▒▒▒▒  ▒▒  ▒▒▒▒▒▒▒▒  ▒▒▒▒  ▒▒  ▒▒▒▒▒▒▒
▓        ▓▓▓▓▓  ▓▓▓▓▓       ▓▓▓       ▓▓▓  ▓▓▓▓  ▓▓       ▓▓▓  ▓▓▓▓▓▓▓▓  ▓▓▓▓  ▓▓  ▓▓▓   ▓
█  █  █  █████  █████  ███  ███  ███  ███  ████  ██  ███  ███  ████████  ████  ██  ████  █
█  ████  ██        ██  ████  ██  ████  ███      ███  ████  ██        ███      ████      ██

█  ████  ██        ██  ████  ██  ████  ███      ███  ████  ██        ███      ████      ██
█  █  █  █████  █████  ███  ███  ███  ███  ████  ██  ███  ███  ████████  ████  ██  ████  █
▓        ▓▓▓▓▓  ▓▓▓▓▓       ▓▓▓       ▓▓▓  ▓▓▓▓  ▓▓       ▓▓▓  ▓▓▓▓▓▓▓▓  ▓▓▓▓  ▓▓  ▓▓▓   ▓
▒   ▒▒   ▒▒▒▒▒  ▒▒▒▒▒  ▒▒▒▒  ▒▒  ▒▒▒▒  ▒▒  ▒▒▒▒  ▒▒  ▒▒▒▒  ▒▒  ▒▒▒▒▒▒▒▒  ▒▒▒▒  ▒▒  ▒▒▒▒▒▒▒
░  ░░░░  ░░        ░░       ░░░       ░░░░      ░░░       ░░░  ░░░░░░░░░      ░░░░      ░░
```                                                                                          
An append-only event log for capturing thoughts, notes, and data you do not want to lose.
[![CI](https://github.com/CromboJambo/mirror-log/actions/workflows/ci.yml/badge.svg)](https://github.com/CromboJambo/mirror-log/actions/workflows/ci.yml)
[![Release](https://github.com/CromboJambo/mirror-log/actions/workflows/release.yml/badge.svg)](https://github.com/CromboJambo/mirror-log/actions/workflows/release.yml)
[![Crates.io](https://img.shields.io/crates/v/mirror-log.svg)](https://crates.io/crates/mirror-log)
[![Docs.rs](https://docs.rs/mirror-log/badge.svg)](https://docs.rs/mirror-log)
[![License](https://img.shields.io/crates/l/mirror-log.svg)](https://github.com/CromboJambo/mirror-log/blob/main/LICENSE)



`mirror-log` is local-first, SQLite-backed, and designed to be boring in the best way: easy to inspect, easy to script, and hard to accidentally lose context.

The default build is intentionally lean: SQLite is the source of truth, direct SQL stays first-class, and optional embedding work lives behind a feature flag instead of in the core path.

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

# Integrity verification (hash + relational checks)
mirror-log verify

# Generate embeddings for events (optional feature)
# mirror-log embed --source journal

# Search similar events using embeddings (optional feature)
# mirror-log search-similar "overhead allocation" --limit 5
```

## Installation

```bash
# Clone and build
git clone https://github.com/CromboJambo/mirror-log
cd mirror-log
cargo build --release

# Binary location: target/release/mirror-log

# Optional embedding commands
cargo build --release --features embedding

# Or install locally
cargo install --path .
```

## Documentation

- **[User Guide](docs/USER_GUIDE.md)** - Comprehensive documentation with examples, advanced features, and best practices
- **[Canonical Pipeline](docs/CANONICAL_PIPELINE.md)** - Formal order of operations and governance layers
- **[API Documentation](src/lib.rs)** - Library usage for programmatic access

## Core Principles

- **Append-only**: Events are never updated or deleted
- **SQLite is source of truth**: Your data stays local and inspectable
- **No hidden layers**: Direct SQL remains first-class
- **Source-aware logging**: Every event tracks where it came from
- **Optional enrichment**: Embeddings stay outside the default build

## Canonical Pipeline

Mirror-Log ingestion follows one sequence:

1. `capture`
2. `persist`
3. `structure`
4. `enrich` (human-level, via explicit structure)

The nested design structure is:

`law -> principle -> right -> rule -> guideline`

## Data Model

### Main Tables

**Events Table**
- `id TEXT PRIMARY KEY` (UUID)
- `timestamp INTEGER NOT NULL` (event timestamp)
- `source TEXT NOT NULL`
- `content TEXT NOT NULL`
- `meta TEXT NULL`
- `ingested_at INTEGER NOT NULL`
- `content_hash TEXT NULL` (SHA256 for dedupe analytics)

**Chunks Table**
- Stores chunked slices of event content
- Used by `search --chunks` and large-content workflows

### Direct SQLite Access

```bash
sqlite3 mirror.db

# Query events
SELECT datetime(timestamp, 'unixepoch'), source, content
FROM events
ORDER BY timestamp DESC
LIMIT 10;

# Count by source
SELECT source, COUNT(*)
FROM events
GROUP BY source
ORDER BY COUNT(*) DESC;

# Get stats
SELECT COUNT(*) AS total,
       COUNT(DISTINCT content_hash) AS unique_events
FROM events;

# View embeddings (optional feature)
# SELECT e.id, e.content, emb.embedding
# FROM events e
# JOIN event_embeddings emb ON e.id = emb.event_id
# LIMIT 10;
```

## Development

```bash
cargo fmt
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

### Dependency Audit

```bash
# Compare declared, active direct, and full transitive dependency surfaces
scripts/dep-audit.sh

# Audit an optional feature path
scripts/dep-audit.sh --features embedding

# Emit a compact line for mirror-log indexing
MIRROR_LOG_INDEX=1 scripts/dep-audit.sh

# Only append a new audit event when the surface changes
scripts/log-dep-audit.sh

# Log both the default surface and optional feature paths
scripts/log-dep-audit-matrix.sh
```

## License

AGPL-3.0-or-later. See `LICENSE`.

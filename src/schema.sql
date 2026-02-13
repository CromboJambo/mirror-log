-- mirror-log Schema with Ingest-Optimized Design
-- This schema supports high-volume ingestion, deduplication, and rich enrichment

-- ============================================================================
-- Core Events Table
-- ============================================================================
CREATE TABLE IF NOT EXISTS events (
    id TEXT PRIMARY KEY,
    timestamp INTEGER NOT NULL,           -- Event creation timestamp (UTC seconds)
    source TEXT NOT NULL CHECK (length(source) > 0), -- Source identifier (e.g., "cli", "stdin", "file")
    content TEXT NOT NULL,                 -- Raw event content
    meta TEXT,                             -- Optional JSON metadata
    ingested_at INTEGER NOT NULL DEFAULT (unixepoch()),  -- Ingestion timestamp
    content_hash TEXT CHECK (content_hash IS NULL OR length(content_hash) = 64) -- SHA256 hash for deduplication
);

-- ============================================================================
-- Performance Indexes
-- ============================================================================
-- Fast retrieval by timestamp (descending for recent events)
CREATE INDEX IF NOT EXISTS idx_events_ts ON events(timestamp DESC);

-- Composite index for source + timestamp queries
CREATE INDEX IF NOT EXISTS idx_events_source_ts ON events(source, timestamp DESC);

-- Deduplication lookup index (NULL-safe, duplicates allowed)
CREATE INDEX IF NOT EXISTS idx_events_hash ON events(content_hash) WHERE content_hash IS NOT NULL;

-- ============================================================================
-- Chunked Content Table (for large events)
-- ============================================================================
CREATE TABLE IF NOT EXISTS chunks (
    id TEXT PRIMARY KEY,
    event_id TEXT NOT NULL,
    chunk_index INTEGER NOT NULL,
    content TEXT NOT NULL,
    start_offset INTEGER NOT NULL,
    end_offset INTEGER NOT NULL,
    timestamp INTEGER NOT NULL,
    FOREIGN KEY (event_id) REFERENCES events(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_chunks_event ON chunks(event_id);
CREATE INDEX IF NOT EXISTS idx_chunks_time ON chunks(timestamp DESC);

-- ============================================================================
-- Enrichment Tables
-- ============================================================================
-- Event tags: structured metadata
CREATE TABLE IF NOT EXISTS event_tags (
    id TEXT PRIMARY KEY,
    event_id TEXT NOT NULL,
    tag TEXT NOT NULL,
    created_at INTEGER NOT NULL DEFAULT (unixepoch()),
    FOREIGN KEY (event_id) REFERENCES events(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_tags_event ON event_tags(event_id);
CREATE INDEX IF NOT EXISTS idx_tags_tag ON event_tags(tag);

-- Event links: semantic relationships between events
CREATE TABLE IF NOT EXISTS event_links (
    id TEXT PRIMARY KEY,
    from_event_id TEXT NOT NULL,
    to_event_id TEXT NOT NULL,
    relation TEXT NOT NULL,
    created_at INTEGER NOT NULL DEFAULT (unixepoch()),
    FOREIGN KEY (from_event_id) REFERENCES events(id) ON DELETE CASCADE,
    FOREIGN KEY (to_event_id) REFERENCES events(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_links_from ON event_links(from_event_id);
CREATE INDEX IF NOT EXISTS idx_links_to ON event_links(to_event_id);

-- Event embeddings: vector embeddings for AI search
CREATE TABLE IF NOT EXISTS event_embeddings (
    id TEXT PRIMARY KEY,
    event_id TEXT NOT NULL,
    embedding BLOB NOT NULL,  -- Binary vector representation
    model_name TEXT NOT NULL,
    created_at INTEGER NOT NULL DEFAULT (unixepoch()),
    FOREIGN KEY (event_id) REFERENCES events(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_embeddings_event ON event_embeddings(event_id);
CREATE INDEX IF NOT EXISTS idx_embeddings_model ON event_embeddings(model_name);

-- Enrichment jobs: track background enrichment processes
CREATE TABLE IF NOT EXISTS enrichment_jobs (
    id TEXT PRIMARY KEY,
    event_id TEXT NOT NULL,
    job_type TEXT NOT NULL,  -- e.g., "tag", "link", "embed"
    status TEXT NOT NULL DEFAULT 'pending',  -- 'pending', 'running', 'completed', 'failed'
    attempts INTEGER NOT NULL DEFAULT 0,
    result TEXT,  -- JSON result or error message
    updated_at INTEGER NOT NULL DEFAULT (unixepoch()),
    created_at INTEGER NOT NULL DEFAULT (unixepoch()),
    FOREIGN KEY (event_id) REFERENCES events(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_jobs_event ON enrichment_jobs(event_id);
CREATE INDEX IF NOT EXISTS idx_jobs_status ON enrichment_jobs(status);
CREATE INDEX IF NOT EXISTS idx_jobs_type ON enrichment_jobs(job_type);

-- ============================================================================
-- SQLite Performance Pragmas
-- ============================================================================
-- Recommended pragmas for production use
-- Note: These should be set at application startup
-- PRAGMA journal_mode = WAL;          -- Write-Ahead Logging for better concurrency
-- PRAGMA synchronous = NORMAL;        -- Balance between safety and performance
-- PRAGMA temp_store = MEMORY;         -- Use memory for temporary tables
-- PRAGMA page_size = 4096;            -- Optimal page size for most systems
-- PRAGMA mmap_size = 30000000000;      -- Memory-mapped I/O for large databases
-- PRAGMA cache_size = -10000;         -- 10,000 pages (~40MB)
-- PRAGMA foreign_keys = ON;           -- Enforce referential integrity

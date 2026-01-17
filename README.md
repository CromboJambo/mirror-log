mirror-log
An append-only event log for capturing thoughts, notes, and data you don't want to lose.

Philosophy
This is a lab notebook for your brain. Write things down once, never lose them, find them later. No editing history, no second-guessing what you wrote, no normalization that loses context.

Think of it as a foundation for personal knowledge that:

Accepts anything (text, documents, conversation exports)
Never modifies what you wrote
Lets you search and connect ideas later
Stays local, debuggable, and honest
Core Principles
SQLite is the source of truth - Your data, your filesystem, no servers
Append-only - Events are never modified or deleted
No magic - Every query is visible SQL, every function does one thing
Source tracking - Know where everything came from
Debuggable - Open mirror.db in sqlite3 anytime and understand it

Installation

git clone https://github.com/CromboJambo/mirror-log
cd mirror-log
cargo build --release``

The binary will be at target/release/mirror-log.

Optionally, add it to your PATH:
cargo install --path .

Quick Start

# Add a thought
mirror-log add "Overhead allocation needs review" --source journal

# Add from a file
mirror-log add "$(cat meeting-notes.md)" --source meetings

# Bulk import (one line per event)
cat ideas.txt | mirror-log stdin --source ideas

# View recent entries
mirror-log show --last 10

# Search
mirror-log search "overhead"

# Filter by source
mirror-log show --last 20 --source journal

# Database info
mirror-log info
Usage
Adding Events
Single entry:

bash
mirror-log add "Your content here" --source cli
With metadata:

bash
mirror-log add "Meeting notes" \
  --source meetings \
  --meta '{"attendees": ["alice", "bob"], "project": "Q4-planning"}'
From file:

bash
mirror-log add "$(cat document.md)" --source documents
Bulk import from stdin:

bash
cat file.txt | mirror-log stdin --source import
seq 1 100000 | mirror-log stdin --source test  # Fast bulk import
Reading Events
Recent entries:

bash
mirror-log show --last 20
Filter by source:

bash
mirror-log show --last 50 --source journal
Search content:

bash
mirror-log search "cost allocation"
Get specific event:

bash
mirror-log get <event-id>
Database statistics:

bash
mirror-log info
Direct SQLite Access
The database is just SQLite. You can query it directly:

bash
sqlite3 mirror.db

# See recent events
SELECT datetime(timestamp, 'unixepoch'), source, content 
FROM events 
ORDER BY timestamp DESC 
LIMIT 10;

# Count by source
SELECT source, COUNT(*) 
FROM events 
GROUP BY source;

# Search with SQL
SELECT * FROM events 
WHERE content LIKE '%overhead%' 
ORDER BY timestamp DESC;
Schema
sql
CREATE TABLE events (
    id TEXT PRIMARY KEY,          -- UUID
    timestamp INTEGER NOT NULL,   -- Unix epoch
    source TEXT NOT NULL,         -- Where this came from
    content TEXT NOT NULL,        -- The actual data
    meta TEXT                     -- Optional JSON metadata
);
Simple. Visible. No surprises.

Data Model
Every event has:

id: UUID (unique identifier)
timestamp: Unix epoch (when it was logged)
source: String tag (cli, journal, meetings, import, etc.)
content: The actual text/data
meta: Optional JSON for structured metadata
Events are never modified or deleted. The log is append-only.

Use Cases
Personal journal:

bash
mirror-log add "$(date): Thought about variance analysis today" --source journal
Meeting notes:

bash
mirror-log add "$(cat meeting-2024-01-13.md)" --source meetings
Conversation exports:

bash
mirror-log add "$(cat 'claude-chat.md')" --source claude-chat
Git history:

bash
git log --oneline | mirror-log stdin --source git-history
Research clips:

bash
mirror-log add "Interesting paper on ABC costing..." \
  --source research \
  --meta '{"url": "https://...", "tags": ["costing", "ABC"]}'
Performance
Bulk imports use transactions for speed:

10,000 events: ~1 second
100,000 events: ~3 seconds
1,000,000 events: ~30 seconds
SQLite handles millions of events efficiently. Text is tiny.

What's NOT Here (By Design)
❌ No updates or deletes
❌ No migrations (schema is stable)
❌ No AI/embeddings (yet - that's a separate layer)
❌ No web UI (CLI first, UI later)
❌ No sync (local-first)
❌ No normalization (events are atomic)
These features can be built on top without touching the core log.

Future Layers
This is the foundation. Future additions will be separate systems that read from events:

Annotations table - Your thoughts about events (interpretive layer)
Relations table - Connections between events (manual or AI-suggested)
Embeddings - Vector search for semantic similarity
Views - Saved queries and perspectives
Web UI - Read-only interface for browsing
Export tools - Markdown, JSON, etc.
All of these will reference events by ID but never modify them.

Philosophy: The Lab Notebook
This is inspired by the lab notebook principle:

Write in pen, never erase
Date everything
Note the source
Preserve context
Never lose data
Digital tools make it too easy to edit, delete, and lose context. This tool enforces honesty by making the log immutable.

Later analysis, connections, and interpretations happen in separate layers. The original data stays pristine.

License
AGPL-3.0-or-later

This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

This ensures that if anyone runs a modified version of this software as a network service, they must make the source code available to users of that service.

See the LICENSE file for full text.

Contributing
This is a personal tool first. If it's useful to you, great. If you want to contribute, open an issue and let's talk.

Keep it simple. Keep it honest. Keep it debuggable.

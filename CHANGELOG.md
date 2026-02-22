# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.6] - 2026-02-20

### Added
- **Embedding Support**: Added support for semantic embeddings with vector similarity search capabilities
- **New Commands**: 
  - `mirror-log embed` - command to generate and store embeddings for events
  - `mirror-log search-similar` - command to find similar events using cosine similarity
- **API Functions**: 
  - `batch_generate_and_store()` - batch embedding generation and storage
  - `search_similar()` - semantic similarity search function
  - `get_embedding_stats()` - get embedding statistics

### Changed
- **Database Schema**: Extended schema with new tables for embeddings, enrichment jobs, and iteration tracking
- **Performance Improvements**: Added SQLite pragmas for better performance in WAL mode
- **Chunking Logic**: Improved chunking algorithm to handle larger content more efficiently
- **CLI Interface**: Enhanced command-line interface with embedding-related options

### Fixed
- **Duplicate Detection**: Improved duplicate detection logic to properly handle edge cases
- **Database Integrity**: Enhanced integrity verification functions to detect orphaned chunks and hash mismatches
- **Batch Processing**: Fixed issues in stdin batch processing that could cause partial failures

### Deprecated
- None

### Removed
- None

## [0.1.5] - 2026-01-26

### Added
- Initial release of mirror-log with core functionality:
  - Append-only event logging
  - SQLite-backed storage
  - Source-aware logging
  - Chunked content search
  - Duplicate detection using SHA256 hashing
  - Full-text search capabilities
  - Statistics and integrity verification

### Changed
- Core database schema structure
- CLI command interface

### Fixed
- Various minor bugs in event insertion and retrieval logic

### Deprecated
- None

### Removed
- None

[0.1.6]: https://github.com/CromboJambo/mirror-log/compare/v0.1.5...v0.1.6
[0.1.5]: https://github.com/CromboJambo/mirror-log/compare/v0.1.4...v0.1.5

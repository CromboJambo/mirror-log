# Canonical Pipeline

Mirror-Log follows a structured, layered approach to event ingestion and processing. The canonical pipeline ensures consistency, traceability, and proper governance of all events entering the system.

## Overview

The Mirror-Log canonical pipeline consists of four distinct stages:

1. **Capture** - Raw data collection from various sources
2. **Persist** - Data storage in SQLite with integrity checks  
3. **Structure** - Content chunking and organization
4. **Enrich** - Semantic enhancement (optional features)

## Governance Layers

The processing follows nested governance principles, ensuring proper structure at each level:

### Law
- Core system constraints and data integrity rules
- Database schema compliance
- Event timestamp validation

### Principle  
- Fundamental design principles
- Data flow consistency
- Source tracking requirements

### Right
- Access control for different event types
- Permission-based processing
- Feature flag enforcement

### Rule
- Specific business logic rules
- Content validation and sanitization
- Duplicate detection policies

### Guideline
- Best practices for content organization
- Metadata enrichment standards
- User experience considerations

## Stage Details

### Capture Stage (`capture`)
- Raw event data collection from sources (CLI, stdin, files)
- Source identification and metadata attachment  
- Timestamp generation with collision handling
- Input validation and sanitization

### Persist Stage (`persist`)
- SQLite database insertion with proper indexing
- Event ID generation using UUIDv4
- Content hashing for deduplication tracking
- Foreign key relationship maintenance
- Integrity constraint enforcement

### Structure Stage (`structure`) 
- Content chunking for large events (>2KB)
- Paragraph-based splitting strategy
- Chunk offset and index management
- Memory-efficient processing
- Database schema compliance

### Enrich Stage (`enrich`)
- Semantic embedding generation (feature-gated)
- Metadata tagging and linking
- Iteration tracking and feedback integration
- Performance optimization considerations

## Nested Governance Structure

The governance hierarchy ensures proper control flow:

```
law -> principle -> right -> rule -> guideline
```

Each level builds upon the previous one:
- **Law**: Enforces fundamental data integrity
- **Principle**: Maintains design consistency  
- **Right**: Manages access and permissions
- **Rule**: Applies specific processing logic
- **Guideline**: Guides best practices for enrichment

## Processing Flow Example

1. User adds event: `mirror-log add "My thought" --source cli`
2. Capture stage creates UUID, sets timestamp, validates input
3. Persist stage stores in SQLite with content hash  
4. Structure stage checks if chunking needed and processes chunks
5. Enrich stage (if enabled) generates embeddings or other enhancements

## Feature Integration Points

### Embedding Support (`embedding` feature)
- Added at the enrichment stage
- Uses sentence-transformers models by default
- Batch processing for efficiency
- Memory management for large models

### Iteration System (`iteration` feature)  
- Integrated into enrichment pipeline
- Tracks iteration passes per event
- Maintains insight metrics and feedback loops
- Enforces completion criteria

### Attention Layer (`attention` feature)
- Applied at capture stage for recent access tracking
- Manages decay scores and relevance ranking
- Integrates with user interaction patterns

## Error Handling

All stages implement proper error handling:
- Input validation failures
- Database connection issues  
- Processing timeouts or memory constraints
- Feature-specific errors (e.g., model loading)

## Performance Considerations

### Batch Operations
- Bulk inserts for high-throughput scenarios
- Chunked processing to prevent memory spikes
- Asynchronous operations where appropriate

### Memory Management
- Limited batch sizes for stdin ingestion
- Efficient chunking algorithms  
- Caching strategies for repeated processing

### Database Optimization
- Indexes on frequently queried columns
- WAL journal mode for concurrency
- Proper transaction handling

This canonical pipeline ensures Mirror-Log maintains its core principles while providing extensible features through proper governance and layered processing.
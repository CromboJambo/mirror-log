```

# Iteration Module for Mirror-Log

Core functionality for tracking iterative learning cycles with insight measurement.

## Architecture

This module implements the "first-pass fuzziness → iterative mastery" cycle by tracking:
- **Passes**: Each exposure/reflection/re-encoding/application cycle
- **Insight Metrics**: Score and delta per iteration (dy/dx concept)
- **Thresholds**: When to stop iterating based on insight quality
- **Status**: Current state and completion reasons

## Public API

```rust
mod iteration;
mod models;
mod queries;
mod types;

pub use models::*;
pub use queries::*;
pub use types::*;
```

## Key Concepts

### Pass Types
```rust
pub enum PassType {
    Exposure,      // First read/observation
    Reflection,    // Hint/question phase
    ReEncoding,    // Re-encoding and consolidation
    Application,   // Application and synthesis
}
```

### Insight Quality
```rust
pub enum FeedbackQuality {
    Poor,
    Fair,
    Good,
    Excellent,
}
```

### Completion Reasons
```rust
pub enum CompletionReason {
    MaxIterations,
    InsightThreshold,
    DeltaThreshold,
    Manual,
}
```

## Usage Example

```rust
use mirror_log::iteration::*;

// Start a new iteration cycle
let event_id = "uuid-here";
start_iteration_cycle(event_id)?;

// Add an exposure pass
add_pass(event_id, PassType::Exposure, "Read the material")?;

// Add a reflection pass with hint
add_pass_with_hint(
    event_id,
    PassType::Reflection,
    "What's the main concept?",
    Some("The material discusses iterative learning")?
)?;

// Measure insight for this iteration
let insight = measure_insight(event_id, current_iteration)?;
if insight.score < 30 {
    // Too low, continue iterating
    continue;
}

// Check if we should stop
if should_complete(event_id, insight)? {
    complete_iteration(event_id, CompletionReason::InsightThreshold)?;
}
```

## Database Schema Integration

This module uses the following tables (defined in `src/schema.sql`):

- `iteration_passes`: Track each pass type
- `iteration_insight`: Measure insight score and delta
- `iteration_feedback`: Detailed feedback per iteration
- `iteration_thresholds`: Configurable thresholds
- `iteration_status`: Current state tracking
- `iteration_stats`: Aggregated statistics

## Performance Considerations

- All queries use indexed lookups for efficient retrieval
- Aggregations are cached in `iteration_stats` table
- Bulk operations available via `add_pass_batch()`

## Future Extensions

- Support for different learning strategies per material type
- Integration with AI for automated hint generation
- Visualization of iteration curves over time
- Adaptive threshold tuning based on user performance
</arg_value></tool_call>

pub mod chunk;
pub mod db;
pub mod embedding;
pub mod export;
pub mod iteration;
pub mod log;
pub mod pipeline;
pub mod view;

// Re-export commonly used types and functions
pub use log::{
    append, append_batch, append_stdin, is_duplicate, stats, verify_integrity, AppendReceipt,
    IntegrityReport,
};
pub use view::{by_ingestion_time, dedup_stats, find_duplicates, Event};

// Re-export iteration types (only the core types)
pub use iteration::types::*;

// Re-export embedding types and functions
#[cfg(feature = "embedding")]
pub use embedding::{
    batch_generate_and_store, cosine_similarity, generate_embedding, get_embedding,
    get_embedding_stats, init_embedding_service, normalize_vector, search_similar, store_embedding,
    Embedding, EmbeddingError, EmbeddingStats, Similarity,
};

// Re-export iteration types and functions
pub use iteration::{
    get_iteration_passes, get_iteration_status, insert_iteration_pass, update_iteration_status,
    CompletionReason, FeedbackQuality, IterationError, IterationFeedback, IterationInsight,
    IterationPass, IterationStats, IterationStatus, IterationThreshold, PassType,
};

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

// Re-export embedding types and functions
// Temporarily disabled - embedding module has missing dependencies
// pub use embedding::{
//     batch_generate_and_store, cosine_similarity, generate_embedding, get_embedding,
//     get_embedding_stats, init_embedding_service, normalize_vector, search_similar, store_embedding,
//     Embedding, EmbeddingError, EmbeddingStats, Similarity,
// };

// Re-export iteration types (module is currently placeholder)

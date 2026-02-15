pub mod chunk;
pub mod db;
pub mod embedding;
pub mod export;
pub mod log;
pub mod view;

// Re-export commonly used types and functions
pub use log::{
    IntegrityReport, append, append_batch, append_stdin, is_duplicate, stats, verify_integrity,
};
pub use view::{Event, by_ingestion_time, dedup_stats, find_duplicates};

// Re-export embedding types and functions
pub use embedding::{
    Embedding, EmbeddingError, EmbeddingStats, Similarity, VectorSearch, batch_generate_and_store,
    cosine_similarity, generate_embedding, get_embedding, get_embedding_stats,
    init_embedding_service, normalize_vector, search_similar, store_embedding,
};

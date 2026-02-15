use approx::RelativeEq;
use chrono::{DateTime, Utc};
use rusqlite::{Connection, Result as SqliteResult};
use std::collections::{BTreeMap, HashMap, HashSet};
use tokenizers::Tokenizer;

/// Represents a single embedding vector
#[derive(Debug, Clone)]
pub struct Embedding {
    pub id: String,
    pub vector: Vec<f32>,
    pub model_name: String,
    pub created_at: i64,
}

/// Represents a similarity score between two embeddings
#[derive(Debug, Clone)]
pub struct Similarity {
    pub event_id: String,
    pub score: f32,
}

/// Embedding service for mirror-log
pub struct EmbeddingService {
    conn: Connection,
    model_name: String,
    embedding_dim: usize,
    tokenizer: Tokenizer,
}

/// Error types for embedding operations
#[derive(Debug)]
pub enum EmbeddingError {
    TokenizerError(String),
    DatabaseError(SqliteResult),
}

impl std::fmt::Display for EmbeddingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EmbeddingError::TokenizerError(msg) => write!(f, "

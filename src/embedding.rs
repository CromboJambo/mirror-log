use chrono::{DateTime, TimeZone, Utc};
use rusqlite::{Connection, Result as SqliteResult};
use std::collections::HashMap;
use tokenizers::Tokenizer;

/// Represents a single embedding vector
#[derive(Debug, Clone)]
pub struct Embedding {
    pub id: String,
    pub vector: Vec<f32>,
    pub model_name: String,
    pub created_at: i64,
}

/// Represents statistics about embedding operations
#[derive(Debug, Clone)]
pub struct EmbeddingStats {
    pub total_embeddings: i64,
    pub total_events: i64,
    pub model_name: String,
    pub embedding_dim: usize,
    pub average_vector_length: f32,
}

/// Represents a similarity score between two embeddings
#[derive(Debug, Clone)]
pub struct Similarity {
    pub event_id: String,
    pub score: f32,
}

/// Error types for embedding operations
#[derive(Debug)]
pub enum EmbeddingError {
    TokenizerError(String),
    DatabaseError(SqliteResult<()>),
    ModelLoadError(String),
    VectorDimensionMismatch(usize, usize),
    NoEmbeddingsFound,
}

impl std::fmt::Display for EmbeddingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EmbeddingError::TokenizerError(msg) => write!(f, "Tokenizer error: {}", msg),
            EmbeddingError::DatabaseError(e) => write!(f, "Database error: {}", e),
            EmbeddingError::ModelLoadError(msg) => write!(f, "Model load error: {}", msg),
            EmbeddingError::VectorDimensionMismatch(expected, actual) => {
                write!(
                    f,
                    "Vector dimension mismatch: expected {}, got {}",
                    expected, actual
                )
            }
            EmbeddingError::NoEmbeddingsFound => write!(f, "No embeddings found in database"),
        }
    }
}

impl std::error::Error for EmbeddingError {}

/// Embedding service for mirror-log
pub struct EmbeddingService {
    conn: Connection,
    model_name: String,
    embedding_dim: usize,
    tokenizer: Tokenizer,
}

impl EmbeddingService {
    /// Create a new embedding service
    pub fn new(
        conn: Connection,
        model_name: &str,
        tokenizer: Tokenizer,
        embedding_dim: usize,
    ) -> Self {
        Self {
            conn,
            model_name: model_name.to_string(),
            embedding_dim,
            tokenizer,
        }
    }

    /// Initialize a new embedding service with a default model
    pub fn init(
        model_name: &str,
        tokenizer: Tokenizer,
        embedding_dim: usize,
    ) -> Result<Self, EmbeddingError> {
        let conn = Connection::open("mirror.db").map_err(EmbeddingError::DatabaseError)?;
        Ok(Self::new(conn, model_name, tokenizer, embedding_dim))
    }

    /// Generate an embedding for text
    pub fn generate_embedding(&mut self, text: &str) -> Result<Embedding, EmbeddingError> {
        // Tokenize text
        let tokens = self
            .tokenizer
            .encode(text, true)
            .map_err(EmbeddingError::TokenizerError)?;

        // In a real implementation, you would use rust-bert to generate embeddings
        // For now, create a dummy embedding (this would be replaced with actual model inference)
        let vector = vec![0.0f32; self.embedding_dim];

        let now = chrono::Utc::now().timestamp();

        Ok(Embedding {
            id: uuid::Uuid::new_v4().to_string(),
            vector,
            model_name: self.model_name.clone(),
            created_at: now,
        })
    }

    /// Store an embedding in the database
    pub fn store_embedding(
        &self,
        embedding: &Embedding,
        event_id: &str,
    ) -> Result<(), EmbeddingError> {
        // Convert vector to bytes for storage
        let vector_bytes: Vec<u8> = embedding
            .vector
            .iter()
            .flat_map(|&v| v.to_le_bytes())
            .collect();

        let now = chrono::Utc::now().timestamp();

        self.conn
            .execute(
                "INSERT INTO event_embeddings (id, event_id, embedding, model_name, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5)",
                (
                    &embedding.id,
                    event_id,
                    &vector_bytes,
                    &embedding.model_name,
                    now,
                ),
            )
            .map_err(EmbeddingError::DatabaseError)?;

        Ok(())
    }

    /// Get an embedding for an event
    pub fn get_embedding(&self, event_id: &str) -> Result<Embedding, EmbeddingError> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT id, embedding, model_name, created_at
             FROM event_embeddings
             WHERE event_id = ?1",
            )
            .map_err(EmbeddingError::DatabaseError)?;

        let embedding_result = stmt
            .query_row([event_id], |row| {
                let id: String = row.get(0)?;
                let vector_bytes: Vec<u8> = row.get(1)?;
                let model_name: String = row.get(2)?;
                let created_at: i64 = row.get(3)?;

                // Convert bytes back to vector
                let vector: Vec<f32> = vector_bytes
                    .chunks_exact(4)
                    .filter_map(|chunk| chunk.try_into().ok())
                    .flat_map(|bytes| f32::from_le_bytes(bytes))
                    .collect();

                Ok(Embedding {
                    id,
                    vector,
                    model_name,
                    created_at,
                })
            })
            .map_err(EmbeddingError::DatabaseError)?;

        Ok(embedding_result)
    }

    /// Get all embeddings for a specific model
    pub fn get_embeddings_by_model(
        &self,
        model_name: &str,
    ) -> Result<Vec<Embedding>, EmbeddingError> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT id, embedding, model_name, created_at
             FROM event_embeddings
             WHERE model_name = ?1",
            )
            .map_err(EmbeddingError::DatabaseError)?;

        let embeddings = stmt
            .query_map([model_name], |row| {
                let id: String = row.get(0)?;
                let vector_bytes: Vec<u8> = row.get(1)?;
                let model_name: String = row.get(2)?;
                let created_at: i64 = row.get(3)?;

                let vector: Vec<f32> = vector_bytes
                    .chunks_exact(4)
                    .filter_map(|chunk| chunk.try_into().ok())
                    .flat_map(|bytes| f32::from_le_bytes(bytes))
                    .collect();

                Ok(Embedding {
                    id,
                    vector,
                    model_name,
                    created_at,
                })
            })
            .map_err(EmbeddingError::DatabaseError)?
            .filter_map(Result::ok)
            .collect();

        Ok(embeddings)
    }

    /// Calculate cosine similarity between two vectors
    pub fn cosine_similarity(vec1: &[f32], vec2: &[f32]) -> Result<f32, EmbeddingError> {
        if vec1.len() != vec2.len() {
            return Err(EmbeddingError::VectorDimensionMismatch(
                vec1.len(),
                vec2.len(),
            ));
        }

        let dot_product: f32 = vec1.iter().zip(vec2.iter()).map(|(a, b)| a * b).sum();
        let norm1 = vec1.iter().map(|&v| v * v).sum::<f32>().sqrt();
        let norm2 = vec2.iter().map(|&v| v * v).sum::<f32>().sqrt();

        if norm1 == 0.0 || norm2 == 0.0 {
            return Ok(0.0);
        }

        Ok
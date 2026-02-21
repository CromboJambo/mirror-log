use rusqlite::{Connection, Error as SqliteError};
use tokenizers::Tokenizer;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Embedding {
    pub id: String,
    pub vector: Vec<f32>,
    pub model_name: String,
    pub created_at: i64,
}

#[derive(Debug, Clone)]
pub struct EmbeddingStats {
    pub total_embeddings: i64,
    pub total_events: i64,
    pub model_name: String,
    pub embedding_dim: usize,
    pub average_vector_length: f32,
}

#[derive(Debug, Clone)]
pub struct Similarity {
    pub event_id: String,
    pub score: f32,
}

#[derive(Debug)]
pub enum EmbeddingError {
    TokenizerError(String),
    DatabaseError(SqliteError),
    ModelLoadError(String),
    VectorDimensionMismatch(usize, usize),
    NoEmbeddingsFound,
    InvalidEmbeddingData(usize),
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
            EmbeddingError::InvalidEmbeddingData(bytes) => {
                write!(f, "Embedding blob is not valid f32 data ({} bytes)", bytes)
            }
        }
    }
}

impl std::error::Error for EmbeddingError {}

pub struct EmbeddingService {
    conn: Connection,
    model_name: String,
    embedding_dim: usize,
    tokenizer: Tokenizer,
}

impl EmbeddingService {
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

    pub fn init(
        model_name: &str,
        tokenizer: Tokenizer,
        embedding_dim: usize,
    ) -> Result<Self, EmbeddingError> {
        let conn = Connection::open("mirror.db").map_err(EmbeddingError::DatabaseError)?;
        Ok(Self::new(conn, model_name, tokenizer, embedding_dim))
    }

    pub fn generate_embedding(&mut self, text: &str) -> Result<Embedding, EmbeddingError> {
        let encoding = self
            .tokenizer
            .encode(text, true)
            .map_err(|e| EmbeddingError::TokenizerError(e.to_string()))?;
        let mut vector = vec![0.0f32; self.embedding_dim];

        // Deterministic token-bucket embedding as a lightweight baseline.
        for (position, token_id) in encoding.get_ids().iter().enumerate() {
            let idx = (*token_id as usize) % self.embedding_dim;
            let weight = 1.0 + (position % 8) as f32 * 0.125;
            vector[idx] += weight;
        }
        vector = normalize_vector(&vector);

        Ok(Embedding {
            id: Uuid::new_v4().to_string(),
            vector,
            model_name: self.model_name.clone(),
            created_at: chrono::Utc::now().timestamp(),
        })
    }

    pub fn store_embedding(
        &self,
        embedding: &Embedding,
        event_id: &str,
    ) -> Result<(), EmbeddingError> {
        self.conn
            .execute(
                "INSERT INTO event_embeddings (id, event_id, embedding, model_name, created_at)
                 VALUES (?1, ?2, ?3, ?4, ?5)",
                (
                    &embedding.id,
                    event_id,
                    vector_to_bytes(&embedding.vector),
                    &embedding.model_name,
                    embedding.created_at,
                ),
            )
            .map_err(EmbeddingError::DatabaseError)?;
        Ok(())
    }

    pub fn get_embedding(&self, event_id: &str) -> Result<Embedding, EmbeddingError> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT id, embedding, model_name, created_at
                 FROM event_embeddings
                 WHERE event_id = ?1
                 ORDER BY created_at DESC
                 LIMIT 1",
            )
            .map_err(EmbeddingError::DatabaseError)?;

        stmt.query_row([event_id], |row| {
            let id: String = row.get(0)?;
            let vector_bytes: Vec<u8> = row.get(1)?;
            let model_name: String = row.get(2)?;
            let created_at: i64 = row.get(3)?;

            Ok((id, vector_bytes, model_name, created_at))
        })
        .map_err(EmbeddingError::DatabaseError)
        .and_then(|(id, vector_bytes, model_name, created_at)| {
            let vector = bytes_to_vector(&vector_bytes)?;
            Ok(Embedding {
                id,
                vector,
                model_name,
                created_at,
            })
        })
    }

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

        let rows = stmt
            .query_map([model_name], |row| {
                let id: String = row.get(0)?;
                let vector_bytes: Vec<u8> = row.get(1)?;
                let model_name: String = row.get(2)?;
                let created_at: i64 = row.get(3)?;
                Ok((id, vector_bytes, model_name, created_at))
            })
            .map_err(EmbeddingError::DatabaseError)?;

        let raw: Vec<(String, Vec<u8>, String, i64)> = rows
            .collect::<Result<Vec<_>, _>>()
            .map_err(EmbeddingError::DatabaseError)?;

        raw.into_iter()
            .map(|(id, vector_bytes, model_name, created_at)| {
                let vector = bytes_to_vector(&vector_bytes)?;
                Ok(Embedding {
                    id,
                    vector,
                    model_name,
                    created_at,
                })
            })
            .collect()
    }
}

fn vector_to_bytes(vector: &[f32]) -> Vec<u8> {
    vector
        .iter()
        .flat_map(|value| value.to_le_bytes())
        .collect()
}

fn bytes_to_vector(bytes: &[u8]) -> Result<Vec<f32>, EmbeddingError> {
    if !bytes.len().is_multiple_of(4) {
        return Err(EmbeddingError::InvalidEmbeddingData(bytes.len()));
    }

    Ok(bytes
        .chunks_exact(4)
        .map(|chunk| f32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]))
        .collect())
}

pub fn normalize_vector(vector: &[f32]) -> Vec<f32> {
    let norm = vector.iter().map(|v| v * v).sum::<f32>().sqrt();
    if norm == 0.0 {
        return vector.to_vec();
    }
    vector.iter().map(|v| v / norm).collect()
}

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

    Ok(dot_product / (norm1 * norm2))
}

pub fn init_embedding_service(
    model_name: &str,
    tokenizer: Tokenizer,
    embedding_dim: usize,
) -> Result<EmbeddingService, EmbeddingError> {
    EmbeddingService::init(model_name, tokenizer, embedding_dim)
}

pub fn generate_embedding(
    service: &mut EmbeddingService,
    text: &str,
) -> Result<Embedding, EmbeddingError> {
    service.generate_embedding(text)
}

pub fn store_embedding(
    service: &EmbeddingService,
    embedding: &Embedding,
    event_id: &str,
) -> Result<(), EmbeddingError> {
    service.store_embedding(embedding, event_id)
}

pub fn get_embedding(
    service: &EmbeddingService,
    event_id: &str,
) -> Result<Embedding, EmbeddingError> {
    service.get_embedding(event_id)
}

pub fn batch_generate_and_store(
    service: &mut EmbeddingService,
    items: &[(&str, &str)],
) -> Result<Vec<Embedding>, EmbeddingError> {
    let mut generated = Vec::with_capacity(items.len());
    for (event_id, text) in items {
        let embedding = service.generate_embedding(text)?;
        service.store_embedding(&embedding, event_id)?;
        generated.push(embedding);
    }
    Ok(generated)
}

pub fn search_similar(
    service: &EmbeddingService,
    query_vector: &[f32],
    limit: usize,
) -> Result<Vec<Similarity>, EmbeddingError> {
    let mut stmt = service
        .conn
        .prepare(
            "SELECT event_id, embedding
             FROM event_embeddings
             WHERE model_name = ?1",
        )
        .map_err(EmbeddingError::DatabaseError)?;
    let rows = stmt
        .query_map([service.model_name.as_str()], |row| {
            let event_id: String = row.get(0)?;
            let embedding_bytes: Vec<u8> = row.get(1)?;
            Ok((event_id, embedding_bytes))
        })
        .map_err(EmbeddingError::DatabaseError)?;
    let raw = rows
        .collect::<Result<Vec<_>, _>>()
        .map_err(EmbeddingError::DatabaseError)?;

    if raw.is_empty() {
        return Err(EmbeddingError::NoEmbeddingsFound);
    }

    let mut similarities: Vec<Similarity> = raw
        .into_iter()
        .map(|(event_id, bytes)| {
            let vector = bytes_to_vector(&bytes)?;
            let score = cosine_similarity(query_vector, &vector)?;
            Ok(Similarity { event_id, score })
        })
        .collect::<Result<Vec<_>, EmbeddingError>>()?;

    similarities.sort_by(|a, b| b.score.total_cmp(&a.score));
    similarities.truncate(limit);
    Ok(similarities)
}

pub fn get_embedding_stats(service: &EmbeddingService) -> Result<EmbeddingStats, EmbeddingError> {
    let total_embeddings: i64 = service
        .conn
        .query_row(
            "SELECT COUNT(*) FROM event_embeddings WHERE model_name = ?1",
            [service.model_name.as_str()],
            |row| row.get(0),
        )
        .map_err(EmbeddingError::DatabaseError)?;

    let total_events: i64 = service
        .conn
        .query_row("SELECT COUNT(*) FROM events", [], |row| row.get(0))
        .map_err(EmbeddingError::DatabaseError)?;

    let mut stmt = service
        .conn
        .prepare(
            "SELECT embedding
             FROM event_embeddings
             WHERE model_name = ?1",
        )
        .map_err(EmbeddingError::DatabaseError)?;
    let rows = stmt
        .query_map([service.model_name.as_str()], |row| {
            row.get::<_, Vec<u8>>(0)
        })
        .map_err(EmbeddingError::DatabaseError)?;
    let blobs = rows
        .collect::<Result<Vec<_>, _>>()
        .map_err(EmbeddingError::DatabaseError)?;

    let mut total_len = 0.0f32;
    let mut count = 0usize;
    for blob in blobs {
        let vector = bytes_to_vector(&blob)?;
        let length = vector.iter().map(|v| v * v).sum::<f32>().sqrt();
        total_len += length;
        count += 1;
    }

    Ok(EmbeddingStats {
        total_embeddings,
        total_events,
        model_name: service.model_name.clone(),
        embedding_dim: service.embedding_dim,
        average_vector_length: if count == 0 {
            0.0
        } else {
            total_len / count as f32
        },
    })
}

//! Database query functions for the iteration tracking system.
//!
//! This module provides functions to interact with the iteration-related tables
//! defined in the schema, including pass tracking, insight measurement, and
//! status management.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt;

use crate::iteration::types::{CompletionReason, FeedbackQuality, IterationError, IterationResult, IterationPass, IterationStatus, PassType};
use sqlx::{Pool, Sqlite};

/// Represents the state of an iteration cycle.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IterationStatus {
    /// Unique identifier for this status record
    pub id: String,

    /// Event ID being iterated
    pub event_id: String,

    /// Current iteration number
    pub current_iteration: i32,

    /// Current pass type
    pub current_pass_type: Option<PassType>,

    /// Last insight score (0-100)
    pub last_insight_score: Option<u8>,

    /// Last insight delta (negative value, improvement from previous)
    pub last_insight_delta: Option<f64>,

    /// Whether the cycle is complete
    pub is_complete: bool,

    /// Reason for completion
    pub completion_reason: Option<CompletionReason>,

    /// Time when completion occurred (if complete)
    pub completed_at: Option<i64>,

    /// Creation timestamp (UTC seconds)
    pub created_at: i64,
}

/// Represents a single insight measurement for an iteration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IterationInsight {
    /// Unique identifier
    pub id: String,

    /// Event ID being measured
    pub event_id: String,

    /// Iteration number
    pub iteration_number: i32,

    /// Insight score (0-100)
    pub insight_score: u8,

    /// Insight delta (negative value, improvement from previous)
    pub insight_delta: f64,

    /// Quality of feedback
    pub feedback_quality: FeedbackQuality,

    /// Semantic changes (JSON)
    pub semantic_change: Option<String>,

    /// Creation timestamp (UTC seconds)
    pub created_at: i64,
}

/// Represents detailed feedback for a specific iteration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IterationFeedback {
    /// Unique identifier
    pub id: String,

    /// Event ID Option<String>,

    /// Quality of response
    pub response_quality: FeedbackQuality,

    /// Time taken to respond (in seconds)
    pub response_time: Option<i32>,

    /// Creation timestamp (UTC seconds)
    pub created_at: i64,
}

/// Represents aggregated statistics for an event's iteration cycle.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IterationStats {
    /// Unique identifier
    pub id: String,

    /// Event ID
    pub event_id: String,

    /// Total iterations performed
    pub total_iterations: i32,

    /// Total passes performed
    pub total_passes: i32,

    /// Average insight score
    pub average_insight_score: Option<f64>,

    /// Maximum insight score
    pub max_insight_score: Option<u8>,

    /// Minimum insight score
    pub min_insight_score: Option<u8>,

    /// Total improvement (sum of deltas)
    pub total_improvement: Option<f64>,

    /// Average improvement per iteration
    pub avg_improvement: Option<f64>,

    /// Completion time (UTC seconds)
    pub completion_time: Option<i64>,

    /// Creation timestamp (UTC seconds)
    pub created_at: i64,

    /// Last update timestamp (UTC seconds)
    pub updated_at: i64,
}

/// Represents a threshold configuration for iteration stopping conditions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IterationThreshold {
    /// Unique identifier
    pub id: String,

    /// Event ID
    pub event_id: String,

    /// Pass type this threshold applies to
    pub pass_type: PassType,

    /// Maximum number of iterations allowed
    pub max_iterations: i32,

    /// Insight score threshold (stop when score drops below this)
    pub insight_threshold: u8,

    /// Delta threshold (stop when improvement drops below this)
    pub delta_threshold: f64,

    /// Creation timestamp (UTC seconds)
    pub created_at: i64,
}

/// Represents a single pass in an iteration cycle.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IterationPass {
    /// Unique identifier
    pub id: String,

    /// Event ID
    pub event_id: String,

    /// Iteration number
    pub iteration_number: i32,

    /// Type of pass
    pub pass_type: PassType,

    /// Description of the pass
    pub description: Option<String>,

    /// Creation timestamp (UTC seconds)
    pub created_at: i64,
}

/// Error types for iteration operations.
#[derive(Debug, thiserror::Error)]
pub enum IterationError {
    #[error("Event not found: {0}")]
    EventNotFound(String),

    #[error("Invalid iteration number: {0}")]
    InvalidIterationNumber(String),

    #[error("No active iteration cycle for event: {0}")]
    NoActiveCycle(String),

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Invalid threshold configuration: {0}")]
    InvalidThreshold(String),
}

/// Result type for iteration operations.
pub type IterationResult<T> = Result<T, IterationError>;

/// Represents the state of an iteration cycle.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IterationStatus {
    /// Unique identifier for this status record
    pub id: String,

    /// Event ID being iterated
    pub event_id: String,

    /// Current iteration number
    pub current_iteration: i32,

    /// Current pass type
    pub current_pass_type: Option<PassType>,

    /// Last insight score (0-100)
    pub last_insight_score: Option<u8>,

    /// Last insight delta (negative value, improvement from previous)
    pub last_insight_delta: Option<f64>,

    /// Whether the cycle is complete
    pub is_complete: bool,

    /// Reason for completion
    pub completion_reason: Option<CompletionReason>,

    /// Time when completion occurred (if complete)
    pub completed_at: Option<i64>,

    /// Creation timestamp (UTC seconds)
    pub created_at: i64,
}

/// Represents a single insight measurement for an iteration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IterationInsight {
    /// Unique identifier
    pub id: String,

    /// Event ID being measured
    pub event_id: String,

    /// Iteration number
    pub iteration_number: i32,

    /// Insight score (0-100)
    pub insight_score: u8,

    /// Insight delta (negative value, improvement from previous)
    pub insight_delta: f64,

    /// Quality of feedback
    pub feedback_quality: FeedbackQuality,

    /// Semantic changes (JSON)
    pub semantic_change: Option<String>,

    /// Creation timestamp (UTC seconds)
    pub created_at: i64,
}

/// Represents detailed feedback for a specific iteration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IterationFeedback {
    /// Unique identifier
    pub id: String,

    /// Event ID
    pub event_id: String,

    /// Iteration number
    pub iteration_number: i32,

    /// Hint provided at this iteration
    pub hint: String,

    /// User's response
    pub user_response: Option<String>,

    /// Quality of response
    pub response_quality: FeedbackQuality,

    /// Time taken to respond (in seconds)
    pub response_time: Option<i32>,

    /// Creation timestamp (UTC seconds)
    pub created_at: i64,
}

/// Represents aggregated statistics for an event's iteration cycle.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IterationStats {
    /// Unique identifier
    pub id: String,

    /// Event ID
    pub event_id: String,

    /// Total iterations performed
    pub total_iterations: i32,

    /// Total passes performed
    pub total_passes: i32,

    /// Average insight score
    pub average_insight_score: Option<f64>,

    /// Maximum insight score
    pub max_insight_score: Option<u8>,

    /// Minimum insight score
    pub min_insight_score: Option<u8>,

    /// Total improvement (sum of deltas)
    pub total_improvement: Option<f64>,

    /// Average improvement per iteration
    pub avg_improvement: Option<f64>,

    /// Completion time (UTC seconds)
    pub completion_time: Option<i64>,

    /// Creation timestamp (UTC seconds)
    pub created_at: i64,

    /// Last update timestamp (UTC seconds)
    pub updated_at: i64,
}

/// Represents a threshold configuration for iteration stopping conditions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IterationThreshold {
    /// Unique identifier
    pub id: String,

    /// Event ID
    pub event_id: String,

    /// Pass type this threshold applies to
    pub pass_type: PassType,

    /// Maximum number of iterations allowed
    pub max_iterations: i32,

    /// Insight score threshold (stop when score drops below this)
    pub insight_threshold: u8,

    /// Delta threshold (stop when improvement drops below this)
    pub delta_threshold: f64,

    /// Creation timestamp (UTC seconds)
    pub created_at: i64,
}

/// Represents a single pass in an iteration cycle.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IterationPass {
    /// Unique identifier
    pub id: String,

    /// Event ID
    pub event_id: String,

    /// Iteration number
    pub iteration_number: i32,

    /// Type of pass
    pub pass_type: PassType,

    /// Description of the pass
    pub description: Option<String>,

    /// Creation timestamp (UTC seconds)
    pub created_at: i64,
}

/// Error types for iteration operations.
#[derive(Debug, thiserror::Error)]
pub enum IterationError {
    #[error("Event not found: {0}")]
    EventNotFound(String),

    #[error("Invalid iteration number: {0}")]
    InvalidIterationNumber(String),

    #[error("No active iteration cycle for event: {0}")]
    NoActiveCycle(String),

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Invalid threshold configuration: {0}")]
    InvalidThreshold(String),
}

/// Result type for iteration operations.
pub type IterationResult<T> = Result<T, IterationError>;

/// Represents the state of an iteration cycle.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IterationStatus {
    /// Unique identifier for this status record
    pub id: String,

    /// Event ID being iterated
    pub event_id: String,

    /// Current iteration number
    pub current_iteration: i32,

    /// Current pass type
    pub current_pass_type: Option<PassType>,

    /// Last insight score (0-100)
    pub last_insight_score: Option<u8>,

    /// Last insight delta (negative value, improvement from previous)
    pub last_insight_delta: Option<f64>,

    /// Whether the cycle is complete
    pub is_complete: bool,

    /// Reason for completion
    pub completion_reason: Option<CompletionReason>,

    /// Time when completion occurred (if complete)
    pub completed_at: Option<i64>,

    /// Creation timestamp (UTC seconds)
    pub created_at: i64,
}

/// Represents a single insight measurement for an iteration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IterationInsight {
    /// Unique identifier
    pub id: String,

    /// Event ID being measured
    pub event_id: String,

    /// Iteration number
    pub iteration_number: i32,

    /// Insight score (0-100)
    pub insight_score: u8,

    /// Insight delta (negative value, improvement from previous)
    pub insight_delta: f64,

    /// Quality of feedback
    pub feedback_quality: FeedbackQuality,

    /// Semantic changes (JSON)
    pub semantic_change: Option<String>,

    /// Creation timestamp (UTC seconds)
    pub created_at: i64,
}

/// Represents detailed feedback for a specific iteration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IterationFeedback {
    /// Unique identifier
    pub id: String,

    /// Event ID
    pub event_id: String,

    /// Iteration number
    pub iteration_number: i32,

    /// Hint provided at this iteration
    pub hint: String,

    /// User's response
    pub user_response: Option<String>,

    /// Quality of response
    pub response_quality: FeedbackQuality,

    /// Time taken to respond (in seconds)
    pub response_time: Option<i32>,

    /// Creation timestamp (UTC seconds)
    pub created_at: i64,
}

/// Represents aggregated statistics for an event's iteration cycle.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IterationStats {
    /// Unique identifier
    pub id: String,

    /// Event ID
    pub event_id: String,

    /// Total iterations performed
    pub total_iterations: i32,

    /// Total passes performed
    pub total_passes: i32,

    /// Average insight score
    pub average_insight_score: Option<f64>,

    /// Maximum insight score
    pub max_insight_score: Option<u8>,

    /// Minimum insight score
    pub min_insight_score: Option<u8>,

    /// Total improvement (sum of deltas)
    pub total_improvement: Option<f64>,

    /// Average improvement per iteration
    pub avg_improvement: Option<f64>,

    /// Completion time (UTC seconds)
    pub completion_time: Option<i64>,

    /// Creation timestamp (UTC seconds)
    pub created_at: i64,

    /// Last update timestamp (UTC seconds)
    pub updated_at: i64,
}

/// Represents a threshold configuration for iteration stopping conditions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IterationThreshold {
    /// Unique identifier
    pub id: String,

    /// Event ID
    pub event_id: String,

    /// Pass type this threshold applies to
    pub pass_type: PassType,

    /// Maximum number of iterations allowed
    pub max_iterations: i32,

    /// Insight score threshold (stop when score drops below this)
    pub insight_threshold: u8,

    /// Delta threshold (stop when improvement drops below this)
    pub delta_threshold: f64,

    /// Creation timestamp (UTC seconds)
    pub created_at: i64,
}

/// Represents a single pass in an iteration cycle.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IterationPass {
    /// Unique identifier
    pub id: String,

    /// Event ID
    pub event_id: String,

    /// Iteration number
    pub iteration_number: i32,

    /// Type of pass
    pub pass_type: PassType,

    /// Description of the pass
    pub description: Option<String>,

    /// Creation timestamp (UTC seconds)
    pub created_at: i64,
}

/// Error types for iteration operations.
#[derive(Debug, thiserror::Error)]
pub enum IterationError {
    #[error("Event not found: {0}")]
    EventNotFound(String),

    #[error("Invalid iteration number: {0}")]
    InvalidIterationNumber(String),

    #[error("No active iteration cycle for event: {0}")]
    NoActiveCycle(String),

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Invalid threshold configuration: {0}")]
    InvalidThreshold(String),
}

/// Result type for iteration operations.
pub type IterationResult<T> = Result<T, IterationError>;

/// Represents the state of an iteration cycle.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IterationStatus {
    /// Unique identifier for this status record
    pub id: String,

    /// Event ID being iterated
    pub event_id: String,

    /// Current iteration number
    pub current_iteration: i32,

    /// Current pass type
    pub current_pass_type: Option<PassType>,

    /// Last insight score (0-100)
    pub last_insight_score: Option<u8>,

    /// Last insight delta (negative value, improvement from previous)
    pub last_insight_delta: Option<f64>,

    /// Whether the cycle is complete
    pub is_complete: bool,

    /// Reason for completion
    pub completion_reason: Option<CompletionReason>,

    /// Time when completion occurred (if complete)
    pub completed_at: Option<i64>,

    /// Creation timestamp (UTC seconds)
    pub created_at: i64,
}

/// Represents a single insight measurement for an iteration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IterationInsight {
    /// Unique identifier
    pub id: String,

    /// Event ID being measured
    pub event_id: String,

    /// Iteration number
    pub iteration_number: i32,

    /// Insight score (0-100)
    pub insight_score: u8,

    /// Insight delta (negative value, improvement from previous)
    pub insight_delta: f64,

    /// Quality of feedback
    pub feedback_quality: FeedbackQuality,

    /// Semantic changes (JSON)
    pub semantic_change: Option<String>,

    /// Creation timestamp (UTC seconds)
    pub created_at: i64,
}

/// Represents detailed feedback for a specific iteration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IterationFeedback {
    /// Unique identifier
    pub id: String,

    /// Event ID
    pub event_id: String,

    /// Iteration number
    pub iteration_number: i32,

    /// Hint provided at this iteration
    pub hint: String,

    /// User's response
    pub user_response: Option<String>,

    /// Quality of response
    pub response_quality: FeedbackQuality,

    /// Time taken to respond (in seconds)
    pub response_time: Option<i32>,

    /// Creation timestamp (UTC seconds)
    pub created_at: i64,
}

/// Represents aggregated statistics for an event's iteration cycle.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IterationStats {
    /// Unique identifier
    pub id: String,

    /// Event ID
    pub event_id: String,

    /// Total iterations performed
    pub total_iterations: i32,

    /// Total passes performed
    pub total_passes: i32,

    /// Average insight score
    pub average_insight_score: Option<f64>,

    /// Maximum insight score
    pub max_insight_score: Option<u8>,

    /// Minimum insight score
    pub min_insight_score: Option<u8>,

    /// Total improvement (sum of deltas)
    pub total_improvement: Option<f64>,

    /// Average improvement per iteration
    pub avg_improvement: Option<f64>,

    /// Completion time (UTC seconds)
    pub completion_time: Option<i64>,

    /// Creation timestamp (UTC seconds)
    pub created_at: i64,

    /// Last update timestamp (UTC seconds)
    pub updated_at: i64,
}

/// Represents a threshold configuration for iteration stopping conditions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IterationThreshold {
    /// Unique identifier
    pub id: String,

    /// Event ID
    pub event_id: String,

    /// Pass type this threshold applies to
    pub pass_type: PassType,

    /// Maximum number of iterations allowed
    pub max_iterations: i32,

    /// Insight score threshold (stop when score drops below this)
    pub insight_threshold: u8,

    /// Delta threshold (stop when improvement drops below this)
    pub delta_threshold: f64,

    /// Creation timestamp (UTC seconds)
    pub created_at: i64,
}

/// Represents a single pass in an iteration cycle.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IterationPass {
    /// Unique identifier
    pub id: String,

    /// Event ID
    pub event_id: String,

    /// Iteration number
    pub iteration_number: i32,

    /// Type of pass
    pub pass_type: PassType,

    /// Description of the pass
    pub description: Option<String>,

    /// Creation timestamp (UTC seconds)
    pub created_at: i64,
}

/// Error types for iteration operations.
#[derive(Debug, thiserror::Error)]
pub enum IterationError {
    #[error("Event not found: {0}")]
    EventNotFound(String),

    #[error("Invalid iteration number: {0}")]
    InvalidIterationNumber(String),

    #[error("No active iteration cycle for event: {0}")]
    NoActiveCycle(String),

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Invalid threshold configuration: {0}")]
    InvalidThreshold(String),
}

/// Result type for iteration operations.
pub type IterationResult<T> = Result<T, IterationError>;

/// Represents the state of an iteration cycle.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IterationStatus {
    /// Unique identifier for this status record
    pub id: String,

    /// Event ID being iterated
    pub event_id: String,

    /// Current iteration number
    pub current_iteration: i32,

    /// Current pass type
    pub current_pass_type: Option<PassType>,

    /// Last insight score (0-100)
    pub last_insight_score: Option<u8>,

    /// Last insight delta (negative value, improvement from previous)
    pub last_insight_delta: Option<f64>,

    /// Whether the cycle is complete
    pub is_complete: bool,

    /// Reason for completion
    pub completion_reason: Option<CompletionReason>,

    /// Time when completion occurred (if complete)
    pub completed_at: Option<i64>,

    /// Creation timestamp (UTC seconds)
    pub created_at: i64,
}

/// Represents a single insight measurement for an iteration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IterationInsight {
    /// Unique identifier
    pub id: String,

    /// Event ID being measured
    pub event_id: String,

    /// Iteration number
    pub iteration_number: i32,

    /// Insight score (0-100)
    pub insight_score: u8,

    /// Insight delta (negative value, improvement from previous)
    pub insight_delta: f64,

    /// Quality of feedback
    pub feedback_quality: FeedbackQuality,

    /// Semantic changes (JSON)
    pub semantic_change: Option<String>,

    /// Creation timestamp (UTC seconds)
    pub created_at: i64,
}

/// Represents detailed feedback for a specific iteration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IterationFeedback {
    /// Unique identifier
    pub id: String,

    /// Event ID
    pub event_id: String,

    /// Iteration number
    pub iteration_number: i32,

    /// Hint provided at this iteration
    pub hint: String,

    /// User's response
    pub user_response: Option<String>,

    /// Quality of response
    pub response_quality: FeedbackQuality,

    /// Time taken to respond (in seconds)
    pub response_time: Option<i32>,

    /// Creation timestamp (UTC seconds)
    pub created_at: i64,
}

/// Represents aggregated statistics for an event's iteration cycle.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IterationStats {
    /// Unique identifier
    pub id: String,

    /// Event ID
    pub event_id: String,

    /// Total iterations performed
    pub total_iterations: i32,

    /// Total passes performed
    pub total_passes: i32,

    /// Average insight score
    pub average_insight_score: Option<f64>,

    /// Maximum insight score
    pub max_insight_score: Option<u8>,

    /// Minimum insight score
    pub min_insight_score: Option<u8>,

    /// Total improvement (sum of deltas)
    pub total_improvement: Option<f64>,

    /// Average improvement per iteration
    pub avg_improvement: Option<f64>,

    /// Completion time (UTC seconds)
    pub completion_time: Option<i64>,

    /// Creation timestamp (UTC seconds)
    pub created_at: i64,

    /// Last update timestamp (UTC seconds)
    pub updated_at: i64,
}

/// Represents a threshold configuration for iteration stopping conditions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IterationThreshold {
    /// Unique identifier
    pub id: String,

    /// Event ID
    pub event_id: String,

    /// Pass type this threshold applies to
    pub pass_type: PassType,

    /// Maximum number of iterations allowed
    pub max_iterations: i32,

    /// Insight score threshold (stop when score drops below this)
    pub insight_threshold: u8,

    /// Delta threshold (stop when improvement drops below this)
    pub delta_threshold: f64,

    /// Creation timestamp (UTC seconds)
    pub created_at: i64,
}

/// Represents a single pass in an iteration cycle.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IterationPass {
    /// Unique identifier
    pub id: String,

    /// Event ID
    pub event_id: String,

    /// Iteration number
    pub iteration_number: i32,

    /// Type of pass
    pub pass_type: PassType,

    /// Description of the pass
    pub description: Option<String>,

    /// Creation timestamp (UTC seconds)
    pub created_at: i64,
}

/// Error types for iteration operations.
#[derive(Debug, thiserror::Error)]
pub enum IterationError {
    #[error("Event not found: {0}")]
    EventNotFound(String),

    #[error("Invalid iteration number: {0}")]
    InvalidIterationNumber(String),

    #[error("No active iteration cycle for event: {0}")]
    NoActiveCycle(String),

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Invalid threshold configuration: {0}")]
    InvalidThreshold(String),
}

/// Result type for iteration operations.
pub type IterationResult<T> = Result<T, IterationError>;

/// Represents the state of an iteration cycle.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IterationStatus {
    /// Unique identifier for this status record
    pub id: String,

    /// Event ID being iterated
    pub event_id: String,

    /// Current iteration number
    pub current_iteration: i32,

    /// Current pass type
    pub current_pass_type: Option<PassType>,

    /// Last insight score (0-100)
    pub last_insight_score: Option<u8>,

    /// Last insight delta (negative value, improvement from previous)
    pub last_insight_delta: Option<f64>,

    /// Whether the cycle is complete
    pub is_complete: bool,

    /// Reason for completion
    pub completion_reason: Option<CompletionReason>,

    /// Time when completion occurred (if complete)
    pub completed_at: Option<i64>,

    /// Creation timestamp (UTC seconds)
    pub created_at: i64,
}

/// Represents a single insight measurement for an iteration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IterationInsight {
    /// Unique identifier
    pub id: String,

    /// Event ID being measured
    pub event_id: String,

    /// Iteration number
    pub iteration_number: i32,

    /// Insight score (0-100)
    pub insight_score: u8,

    /// Insight delta (negative value, improvement from previous)
    pub insight_delta: f64,

    /// Quality of feedback
    pub feedback_quality: FeedbackQuality,

    /// Semantic changes (JSON)
    pub semantic_change: Option<String>,

    /// Creation timestamp (UTC seconds)
    pub created_at: i64,
}

/// Represents detailed feedback for a specific iteration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IterationFeedback {
    /// Unique identifier
    pub id: String,

    /// Event ID
    pub event_id: String,

    /// Iteration number
    pub iteration_number: i32,

    /// Hint provided at this iteration
    pub hint: String,

    /// User's response
    pub user_response: Option<String>,

    /// Quality of response
    pub response_quality: FeedbackQuality,

    /// Time taken to respond (in seconds)
    pub response_time: Option<i32>,

    /// Creation timestamp (UTC seconds)
    pub created_at: i64,
}

/// Represents aggregated statistics for an event's iteration cycle.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IterationStats {
    /// Unique identifier
    pub id: String,

    /// Event ID
    pub event_id: String,

    /// Total iterations performed
    pub total_iterations: i32,

    /// Total passes performed
    pub total_passes: i32,

    /// Average insight score
    pub average_insight_score: Option<f64>,

    /// Maximum insight score
    pub max_insight_score: Option<u8>,

    /// Minimum insight score
    pub min_insight_score: Option<u8>,

    /// Total improvement (sum of deltas)
    pub total_improvement: Option<f64>,

    /// Average improvement per iteration
    pub avg_improvement: Option<f64>,

    /// Completion time (UTC seconds)
    pub completion_time: Option<i64>,

    /// Creation timestamp (UTC seconds)
    pub created_at: i64,

    /// Last update timestamp (UTC seconds)
    pub updated_at: i64,
}

/// Represents a threshold configuration for iteration stopping conditions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IterationThreshold {
    /// Unique identifier
    pub id: String,

    /// Event ID
    pub event_id: String,

    /// Pass type this threshold applies to
    pub pass_type: PassType,

    /// Maximum number of iterations allowed
    pub max_iterations: i32,

    /// Insight score threshold (stop when score drops below this)
    pub insight_threshold: u8,

    /// Delta threshold (stop when improvement drops below this)
    pub delta_threshold: f64,

    /// Creation timestamp (UTC seconds)
    pub created_at: i64,
}

/// Represents a single pass in an iteration cycle.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IterationPass {
    /// Unique identifier
    pub id: String,

    /// Event ID
    pub event_id: String,

    /// Iteration number
    pub iteration_number: i32,

    /// Type of pass
    pub pass_type: PassType,

    /// Description of the pass
    pub description: Option<String>,

    /// Creation timestamp (UTC seconds)
    pub created_at: i64,
}

/// Error types for iteration operations.
#[derive(Debug, thiserror::Error)]
pub enum IterationError {
    #[error("Event not found: {0}")]
    EventNotFound(String),

    #[error("Invalid iteration number: {0}")]
    InvalidIterationNumber(String),

    #[error("No active iteration cycle for event: {0}")]
    NoActiveCycle(String),

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Invalid threshold configuration: {0}")]
    InvalidThreshold(String),
}

/// Result type for iteration operations.
pub type IterationResult<T> = Result<T, IterationError>;

/// Represents the state of an iteration cycle.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IterationStatus {
    /// Unique identifier for this status record
    pub id: String,

    /// Event ID being iterated
    pub event_id: String,

    /// Current iteration number
    pub current_iteration: i32,

    /// Current pass type
    pub current_pass_type: Option<PassType>,

    /// Last insight score (0-100)
    pub last_insight_score: Option<u8>,

    /// Last insight delta (negative value, improvement from previous)
    pub last_insight_delta: Option<f64>,

    /// Whether the cycle is complete
    pub is_complete: bool,

    /// Reason for completion
    pub completion_reason: Option<CompletionReason>,

    /// Time when completion occurred (if complete)
    pub completed_at: Option<i64>,

    /// Creation timestamp (UTC seconds)
    pub created_at: i64,
}

/// Represents a single insight measurement for an iteration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IterationInsight {
    /// Unique identifier
    pub id: String,

    /// Event ID being measured
    pub event_id: String,

    /// Iteration number
    pub iteration_number: i32,

    /// Insight score (0-100)
    pub insight_score: u8,

    /// Insight delta (negative value, improvement from previous)
    pub insight_delta: f64,

    /// Quality of feedback
    pub feedback_quality: FeedbackQuality,

    /// Semantic changes (JSON)
    pub semantic_change: Option<String>,

    /// Creation timestamp (UTC seconds)
    pub created_at: i64,
}

/// Represents detailed feedback for a specific iteration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IterationFeedback {
    /// Unique identifier
    pub id: String,

    /// Event ID
    pub event_id: String,

    /// Iteration number
    pub iteration_number: i32,

    /// Hint provided at this iteration
    pub hint: String,

    /// User's response
    pub user_response: Option<String>,

    /// Quality of response
    pub response_quality: FeedbackQuality,

    /// Time taken to respond (in seconds)
    pub response_time: Option<i32>,

    /// Creation timestamp (UTC seconds)
    pub created_at: i64,
}

/// Represents aggregated statistics for an event's iteration cycle.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IterationStats {
    /// Unique identifier
    pub id: String,

    /// Event ID
    pub event_id: String,

    /// Total iterations performed
    pub total_iterations: i32,

    /// Total passes performed
    pub total_passes: i32,

    /// Average insight score
    pub average_insight_score: Option<f64>,

    /// Maximum insight score
    pub max_insight_score: Option<u8>,

    /// Minimum insight score
    pub min_insight_score: Option<u8>,

    /// Total improvement (sum of deltas)
    pub total_improvement: Option<f64>,

    /// Average improvement per iteration
    pub avg_improvement: Option<f64>,

    /// Completion time (UTC seconds)
    pub completion_time: Option<i64>,

    /// Creation timestamp (UTC seconds)
    pub created_at: i64,

    /// Last update timestamp (UTC seconds)
    pub updated_at: i64,
}

/// Represents a threshold configuration for iteration stopping conditions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IterationThreshold {
    /// Unique identifier
    pub id: String,

    /// Event ID
    pub event_id: String,

    /// Pass type this threshold applies to
    pub pass_type: PassType,

    /// Maximum number of iterations allowed
    pub max_iterations: i32,

    /// Insight score threshold (stop when score drops below this)
    pub insight_threshold: u8,

    /// Delta threshold (stop when improvement drops below this)
    pub delta_threshold: f64,

    /// Creation timestamp (UTC seconds)
    pub created_at: i64,
}

/// Represents a single pass in an iteration cycle.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IterationPass {
    /// Unique identifier
    pub id: String,

    /// Event ID
    pub event_id: String,

    /// Iteration number
    pub iteration_number: i32,

    /// Type of pass
    pub pass_type: PassType,

    /// Description of the pass
    pub description: Option<String>,

    /// Creation timestamp (UTC seconds)
    pub created_at: i64,
}

/// Error types for iteration operations.
#[derive(Debug, thiserror::Error)]
pub enum IterationError {
    #[error("Event not found: {0}")]
    EventNotFound(String),

    #[error("Invalid iteration number: {0}")]
    InvalidIterationNumber(String),

    #[error("No active iteration cycle for event: {0}")]
    NoActiveCycle(String),

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Invalid threshold configuration: {0}")]
    InvalidThreshold(String),
}

/// Result type for iteration operations.
pub type IterationResult<T> = Result<T, IterationError>;

/// Represents the state of an iteration cycle.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IterationStatus {
    /// Unique identifier for this status record
    pub id: String,

    /// Event ID being iterated
    pub event_id: String,

    /// Current iteration number
    pub current_iteration: i32,

    /// Current pass type
    pub current_pass_type: Option<PassType>,

    /// Last insight score (0-100)
    pub last_insight_score: Option<u8>,

    /// Last insight delta (negative value, improvement from previous)
    pub last_insight_delta: Option<f64>,

    /// Whether the cycle is complete
    pub is_complete: bool,

    /// Reason for completion
    pub completion_reason: Option<CompletionReason>,

    /// Time when completion occurred (if complete)
    pub completed_at: Option<i64>,

    /// Creation timestamp (UTC seconds)
    pub created_at: i64,
}

/// Represents a single insight measurement for an iteration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IterationInsight {
    /// Unique identifier
    pub id: String,

    /// Event ID being measured
    pub event_id: String,

    /// Iteration number
    pub iteration_number: i32,

    /// Insight score (0-100)
    pub insight_score: u8,

    /// Insight delta (negative value, improvement from previous)
    pub insight_delta: f64,

    /// Quality of feedback
    pub feedback_quality: FeedbackQuality,

    /// Semantic changes (JSON)
    pub semantic_change: Option<String>,

    /// Creation timestamp (UTC seconds)
    pub created_at: i64,
}

/// Represents detailed feedback for a specific iteration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IterationFeedback {
    /// Unique identifier
    pub id: String,

    /// Event ID
    pub event_id: String,

    /// Iteration number
    pub iteration_number: i32,

    /// Hint provided at this iteration
    pub hint: String,

    /// User's response
    pub user_response: Option<String>,

    /// Quality of response
    pub response_quality: FeedbackQuality,

    /// Time taken to respond (in seconds)
    pub response_time: Option<i32>,

    /// Creation timestamp (UTC seconds)
    pub created_at: i64,
}

/// Represents aggregated statistics for an event's iteration cycle.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IterationStats {
    /// Unique identifier
    pub id: String,

    /// Event ID
    pub event_id: String,

    /// Total iterations performed
    pub total_iterations: i32,

    /// Total passes performed
    pub total_passes: i32,

    /// Average insight score
    pub average_insight_score: Option<f64>,

    /// Maximum insight score
    pub max_insight_score: Option<u8>,

    /// Minimum insight score
    pub min_insight_score: Option<u8>,

    /// Total improvement (sum of deltas)
    pub total_improvement: Option<f64>,

    /// Average improvement per iteration
    pub avg_improvement: Option<f64>,

    /// Completion time (UTC seconds)
    pub completion_time: Option<i64>,

    /// Creation timestamp (UTC seconds)
    pub created_at: i64,

    /// Last update timestamp (UTC seconds)
    pub updated_at: i64,
}

/// Represents a threshold configuration for iteration stopping conditions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IterationThreshold {
    /// Unique identifier
    pub id: String,

    /// Event ID
    pub event_id: String,

    /// Pass type this threshold applies to
    pub pass_type: PassType,

    /// Maximum number of iterations allowed
    pub max_iterations: i32,

    /// Insight score threshold (stop when score drops below this)
    pub insight_threshold: u8,

    /// Delta threshold (stop when improvement drops below this)
    pub delta_threshold: f64,

    /// Creation timestamp (UTC seconds)
    pub created_at: i64,
}

/// Represents a single pass in an iteration cycle.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IterationPass {
    /// Unique identifier
    pub id: String,

    /// Event ID
    pub event_id: String,

    /// Iteration number
    pub iteration_number: i32,

    /// Type of pass
    pub pass_type: PassType,

    /// Description of the pass
    pub description: Option<String>,

    /// Creation timestamp (UTC seconds)
    pub created_at: i64,
}

/// Error types for iteration operations.
#[derive(Debug, thiserror::Error)]
pub enum IterationError {
    #[error("Event not found: {0}")]
    EventNotFound(String),

    #[error("Invalid iteration number: {0}")]
    InvalidIterationNumber(String),

    #[error("No active iteration cycle for event: {0}")]
    NoActiveCycle(String),

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Invalid threshold configuration: {0}")]
    InvalidThreshold(String),
}

/// Result type for iteration operations.
pub type IterationResult<T> = Result<T, IterationError>;

/// Represents the state of an iteration cycle.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IterationStatus {
    /// Unique identifier for this status record
    pub id: String,

    /// Event ID being iterated
    pub event_id: String,

    /// Current iteration number
    pub current_iteration: i32,

    /// Current pass type
    pub current_pass_type: Option<PassType>,

    /// Last insight score (0-100)
    pub last_insight_score: Option<u8>,

    /// Last insight delta (negative value, improvement from previous)
    pub last_insight_delta: Option<f64>,

    /// Whether the cycle is complete
    pub is_complete: bool,

    /// Reason for completion
    pub completion_reason: Option<CompletionReason>,

    /// Time when completion occurred (if complete)
    pub completed_at: Option<i64>,

    /// Creation timestamp (UTC seconds)
    pub created_at: i64,
}

/// Represents a single insight measurement for an iteration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IterationInsight {
    /// Unique identifier
    pub id: String,

    /// Event ID being measured
    pub event_id: String,

    /// Iteration number
    pub iteration_number: i32,

    /// Insight score (0-100)
    pub insight_score: u8,

    /// Insight delta (negative value, improvement from previous)
    pub insight_delta: f64,

    /// Quality of feedback
    pub feedback_quality: FeedbackQuality,

    /// Semantic changes (JSON)
    pub semantic_change: Option<String>,

    /// Creation timestamp (UTC seconds)
    pub created_at: i64,
}

/// Represents detailed feedback for a specific iteration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IterationFeedback {
    /// Unique identifier
    pub id: String,

    /// Event ID
    pub event_id: String,

    /// Iteration number
    pub iteration_number: i32,

    /// Hint provided at this iteration
    pub hint: String,

    /// User's response
    pub user_response: Option<String>,

    /// Quality of response
    pub response_quality: FeedbackQuality,

    /// Time taken to respond (in seconds)
    pub response_time: Option<i32>,

    /// Creation timestamp (UTC seconds)
    pub created_at: i64,
}

/// Represents aggregated statistics for an event's iteration cycle.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IterationStats {
    /// Unique identifier
    pub id: String,

    /// Event ID
    pub event_id: String,

    /// Total iterations performed
    pub total_iterations: i32,

    /// Total passes performed
    pub total_passes: i32,

    /// Average insight score
    pub average_insight_score: Option<f64>,

    /// Maximum insight score
    pub max_insight_score: Option<u8>,

    /// Minimum insight score
    pub min_insight_score: Option<u8>,

    /// Total improvement (sum of deltas)
    pub total_improvement: Option<f64>,

    /// Average improvement per iteration
    pub avg_improvement: Option<f64>,

    /// Completion time (UTC seconds)
    pub completion_time: Option<i64>,

    /// Creation timestamp (UTC seconds)
    pub created_at: i64,

    /// Last update timestamp (UTC seconds)
    pub updated_at: i64,
}

/// Represents a threshold configuration for iteration stopping conditions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IterationThreshold {
    /// Unique identifier
    pub id: String,

    /// Event ID
    pub event_id: String,

    /// Pass type this threshold applies to
    pub pass_type: PassType,

    /// Maximum number of iterations allowed
    pub max_iterations: i32,

    /// Insight score threshold (stop when score drops below this)
    pub insight_threshold: u8,

    /// Delta threshold (stop when improvement drops below this)
    pub delta_threshold: f64,

    /// Creation timestamp (UTC seconds)
    pub created_at: i64,
}

/// Represents a single pass in an iteration cycle.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IterationPass {
    /// Unique identifier
    pub id: String,

    /// Event ID
    pub event_id: String,

    /// Iteration number
    pub iteration_number: i32,

    /// Type of pass
    pub pass_type: PassType,

    /// Description of the pass
    pub description: Option<String>,

    /// Creation timestamp (UTC seconds)
    pub created_at: i64,
}

/// Error types for iteration operations.
#[derive(Debug, thiserror::Error)]
pub enum IterationError {
    #[error("Event not found: {0}")]
    EventNotFound(String),

    #[error("Invalid iteration number: {0}")]
    InvalidIterationNumber(String),

    #[error("No active iteration cycle for event: {0}")]
    NoActiveCycle(String),

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Invalid threshold configuration: {0}")]
    InvalidThreshold(String),
}

/// Result type for iteration operations.
pub type IterationResult<T> = Result<T, IterationError>;

/// Represents the state of an iteration cycle.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IterationStatus {
    /// Unique identifier for this status record
    pub id: String,

    /// Event ID being iterated
    pub event_id: String,

    /// Current iteration number
    pub current_iteration: i32,

    /// Current pass type
    pub current_pass_type: Option<PassType>,

    /// Last insight score (0-100)
    pub last_insight_score: Option<u8>,

    /// Last insight delta (negative value, improvement from previous)
    pub last_insight_delta: Option<f64>,

    /// Whether the cycle is complete
    pub is_complete: bool,

    /// Reason for completion
    pub completion_reason: Option<CompletionReason>,

    /// Time when completion occurred (if complete)
    pub completed_at: Option<i64>,

    /// Creation timestamp (UTC seconds)
    pub created_at: i64,
}

/// Represents a single insight measurement for an iteration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IterationInsight {
    /// Unique identifier
    pub id: String,

    /// Event ID being measured
    pub event_id: String,

    /// Iteration number
    pub iteration_number: i32,

    /// Insight score (0-100)
    pub insight_score: u8,

    /// Insight delta (negative value, improvement from previous)
    pub insight_delta: f64,

    /// Quality of feedback
    pub feedback_quality: FeedbackQuality,

    /// Semantic changes (JSON)
    pub semantic_change: Option<String>,

    /// Creation timestamp (UTC seconds)
    pub created_at: i64,
}

/// Represents detailed feedback for a specific iteration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IterationFeedback {
    /// Unique identifier
    pub id: String,

    /// Event ID
    pub event_id: String,

    /// Iteration number
    pub iteration_number: i32,

    /// Hint provided at this iteration
    pub hint: String,

    /// User's response
    pub user_response: Option<String>,

    /// Quality of response
    pub response_quality: FeedbackQuality,

    /// Time taken to respond (in seconds)
    pub response_time: Option<i32>,

    /// Creation timestamp (UTC seconds)
    pub created_at: i64,
}

/// Represents aggregated statistics for an event's iteration cycle.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IterationStats {
    /// Unique identifier
    pub id: String,

    /// Event ID
    pub event_id: String,

    /// Total iterations performed
    pub total_iterations: i32,

    /// Total passes performed
    pub total_passes: i32,

    /// Average insight score
    pub average_insight_score: Option<f64>,

    /// Maximum insight score
    pub max_insight_score: Option<u8>,

    /// Minimum insight score
    pub min_insight_score: Option<u8>,

    /// Total improvement (sum of deltas)
    pub total_improvement: Option<f64>,

    /// Average improvement per iteration
    pub avg_improvement: Option<f64>,

    /// Completion time (UTC seconds)
    pub completion_time: Option<i64>,

    /// Creation timestamp (UTC seconds)
    pub created_at: i64,

    /// Last update timestamp (UTC seconds)
    pub updated_at: i64,
}

/// Represents a threshold configuration for iteration stopping conditions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IterationThreshold {
    /// Unique identifier
    pub id: String,

    /// Event ID
    pub event_id: String,

    /// Pass type this threshold applies to
    pub pass_type: PassType,

    /// Maximum number of iterations allowed
    pub max_iterations: i32,

    /// Insight score threshold (stop when score drops below this)
    pub insight_threshold: u8,

    /// Delta threshold (stop when improvement drops below this)
    pub delta_threshold: f64,

    /// Creation timestamp (UTC seconds)
    pub created_at: i64,
}

/// Represents a single pass in an iteration cycle.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IterationPass {
    /// Unique identifier
    pub id: String,

    /// Event ID
    pub event_id: String,

    /// Iteration number
    pub iteration_number: i32,

    /// Type of pass
    pub pass_type: PassType,

    /// Description of the pass
    pub description: Option<String>,

    /// Creation timestamp (UTC seconds)
    pub created_at: i64,
}

/// Error types for iteration operations.
#[derive(Debug, thiserror::Error)]
pub enum IterationError {
    #[error("Event not found: {0}")]
    EventNotFound(String),

    #[error("Invalid iteration number: {0}")]
    InvalidIterationNumber(String),

    #[error("No active iteration cycle for event: {0}")]
    NoActiveCycle(String),

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Invalid threshold configuration: {0}")]
    InvalidThreshold(String),
}

/// Result type for iteration operations.
pub type IterationResult<T> = Result<T, IterationError>;

/// Represents the state of an iteration cycle.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IterationStatus {
    /// Unique identifier for this status record
    pub id: String,

    /// Event ID being iterated
    pub event_id: String,

    /// Current iteration number
    pub current_iteration: i32,

    /// Current pass type
    pub current_pass_type: Option<PassType>,

    /// Last insight score (0-100)
    pub last_insight_score: Option<u8>,

    /// Last insight delta (negative value, improvement from previous)
    pub last_insight_delta: Option<f64>,

    /// Whether the cycle is complete
    pub is_complete: bool,

    /// Reason for completion
    pub completion_reason: Option<CompletionReason>,

    /// Time when completion occurred (if complete)
    pub completed_at: Option<i64>,

    /// Creation timestamp (UTC seconds)
    pub created_at: i64,
}

/// Represents a single insight measurement for an iteration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IterationInsight {
    /// Unique identifier
    pub id: String,

    /// Event ID being measured
    pub event_id: String,

    /// Iteration number
    pub iteration_number: i32,

    /// Insight score (0-100)
    pub insight_score: u8,

    /// Insight delta (negative value, improvement from previous)
    pub insight_delta: f64,

    /// Quality of feedback
    pub feedback_quality: FeedbackQuality,

    /// Semantic changes (JSON)
    pub semantic_change: Option<String>,

    /// Creation timestamp (UTC seconds)
    pub created_at: i64,
}

/// Represents detailed feedback for a specific iteration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IterationFeedback {
    /// Unique identifier
    pub id: String,

    /// Event ID
    pub event_id: String,

    /// Iteration number
    pub iteration_number: i32,

    /// Hint provided at this iteration
    pub hint: String,

    /// User's response
    pub user_response: Option<String>,

    /// Quality of response
    pub response_quality: FeedbackQuality,

    /// Time taken to respond (in seconds)
    pub response_time: Option<i32>,

    /// Creation timestamp (UTC seconds)
    pub created_at: i64,
}

/// Represents aggregated statistics for an event's iteration cycle.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IterationStats {
    /// Unique identifier
    pub id: String,

    /// Event ID
    pub event_id: String,

    /// Total iterations performed
    pub total_iterations: i32,

    /// Total passes performed
    pub total_passes: i32,

    /// Average insight score
    pub average_insight_score: Option<f64>,

    /// Maximum insight score
    pub max_insight_score: Option<u8>,

    /// Minimum insight score
    pub min_insight_score: Option<u8>,

    /// Total improvement (sum of deltas)
    pub total_improvement: Option<f64>,

    /// Average improvement per iteration
    pub avg_improvement: Option<f64>,

    /// Completion time (UTC seconds)
    pub completion_time: Option<i64>,

    /// Creation timestamp (UTC seconds)
    pub created_at: i64,

    /// Last update timestamp (UTC seconds)
    pub updated_at: i64,
}

/// Represents a threshold configuration for iteration stopping conditions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IterationThreshold {
    /// Unique identifier
    pub id: String,

    /// Event ID
    pub event_id: String,

    /// Pass type this threshold applies to
    pub pass_type: PassType,

    /// Maximum number of iterations allowed
    pub max_iterations: i32,

    /// Insight score threshold (stop when score drops below this)
    pub insight_threshold: u8,

    /// Delta threshold (stop when improvement drops below this)
    pub delta_threshold: f64,

    /// Creation timestamp (UTC seconds)
    pub created_at: i64,
}

/// Represents a single pass in an iteration cycle.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IterationPass {
    /// Unique identifier
    pub id: String,

    /// Event ID
    pub event_id: String,

    /// Iteration number
    pub
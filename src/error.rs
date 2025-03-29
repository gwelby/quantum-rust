//! Error handling for quantum field operations
//!
//! This module provides specialized error types for quantum operations
//! with dimensional awareness and phi-harmonic error correction capabilities.

use thiserror::Error;

use crate::constants::{ConsciousnessState, Dimension, Frequency};

/// Quantum error types for dimensional operations and field coherence
#[derive(Error, Debug)]
pub enum QuantumError {
    /// Error related to insufficient field coherence
    #[error("Insufficient field coherence: {current:.4} (required: {required:.4})")]
    InsufficientCoherence {
        /// Current coherence level
        current: f64,
        /// Required coherence level
        required: f64,
    },

    /// Error when attempting operations with an incompatible consciousness state
    #[error("Incompatible consciousness state {state:?} for operation (required: {required_states:?})")]
    IncompatibleState {
        /// Current consciousness state
        state: ConsciousnessState,
        /// Required consciousness states
        required_states: Vec<ConsciousnessState>,
    },

    /// Error when attempting dimensional translation beyond capability
    #[error("Dimensional translation error: cannot translate from {from_dimension:?} to {to_dimension:?}")]
    DimensionalTranslationError {
        /// Source dimension
        from_dimension: Dimension,
        /// Target dimension
        to_dimension: Dimension,
        /// Translation coherence
        coherence: Option<f64>,
    },

    /// Error when frequency resonance is outside the acceptable range
    #[error("Frequency resonance error: {current_frequency:?} does not resonate with {target_frequency:?}")]
    FrequencyResonanceError {
        /// Current frequency
        current_frequency: Frequency,
        /// Target frequency
        target_frequency: Frequency,
    },

    /// Error when field integrity is compromised
    #[error("Quantum field integrity error: {message}")]
    FieldIntegrityError {
        /// Error message
        message: String,
        /// Field integrity level
        integrity: f64,
    },

    /// Error when temporal bridge fails
    #[error("Temporal bridge error: {message}")]
    TemporalBridgeError {
        /// Error message
        message: String,
    },

    /// General quantum operation error
    #[error("Quantum operation error: {message}")]
    OperationError {
        /// Error message
        message: String,
    },
}

/// Result type for quantum operations
pub type QuantumResult<T> = Result<T, QuantumError>;

/// Helper for creating an insufficient coherence error
pub fn insufficient_coherence(current: f64, required: f64) -> QuantumError {
    QuantumError::InsufficientCoherence { current, required }
}

/// Helper for creating an incompatible state error
pub fn incompatible_state(state: ConsciousnessState, required_states: Vec<ConsciousnessState>) -> QuantumError {
    QuantumError::IncompatibleState { state, required_states }
}

/// Helper for creating a dimensional translation error
pub fn dimensional_translation_error(from_dimension: Dimension, to_dimension: Dimension, coherence: Option<f64>) -> QuantumError {
    QuantumError::DimensionalTranslationError { from_dimension, to_dimension, coherence }
}

/// Helper for creating a frequency resonance error
pub fn frequency_resonance_error(current_frequency: Frequency, target_frequency: Frequency) -> QuantumError {
    QuantumError::FrequencyResonanceError { current_frequency, target_frequency }
}

/// Helper for creating a field integrity error
pub fn field_integrity_error(message: impl Into<String>, integrity: f64) -> QuantumError {
    QuantumError::FieldIntegrityError { message: message.into(), integrity }
}

/// Helper for creating a temporal bridge error
pub fn temporal_bridge_error(message: impl Into<String>) -> QuantumError {
    QuantumError::TemporalBridgeError { message: message.into() }
}

/// Helper for creating a general quantum operation error
pub fn operation_error(message: impl Into<String>) -> QuantumError {
    QuantumError::OperationError { message: message.into() }
}
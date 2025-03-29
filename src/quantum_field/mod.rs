//! Quantum Field Framework (»£)
//!
//! The quantum field module provides core functionality for quantum operations,
//! including coherence management, consciousness state control, dimensional
//! translation, and phi-harmonic algorithms.
//!
//! ## Core Components
//!
//! - **Coherence**: Field coherence management and optimization
//! - **Consciousness**: State management and transitions
//! - **Dimensional**: Gateway and translation between dimensions
//! - **Phi-Harmonic**: Algorithms and data structures based on phi relationships

pub mod coherence;
pub mod consciousness;
pub mod dimensional;
pub mod phi_harmonic;

use crate::constants::{ConsciousnessState, Dimension, Frequency, OPTIMAL_COHERENCE};
use crate::error::QuantumResult;

/// Core quantum field interface for all quantum operations
pub trait QuantumField {
    /// Get the current coherence level of the field
    fn coherence(&self) -> f64;
    
    /// Get the current consciousness state
    fn state(&self) -> ConsciousnessState;
    
    /// Get the current dimensional level
    fn dimension(&self) -> Dimension;
    
    /// Get the current frequency
    fn frequency(&self) -> Frequency;
    
    /// Set the consciousness state
    fn set_state(&mut self, state: ConsciousnessState) -> QuantumResult<()>;
    
    /// Optimize field coherence
    fn optimize_coherence(&mut self) -> QuantumResult<f64>;
    
    /// Translate content between dimensions
    fn translate<T: Clone>(&self, content: T, from: Dimension, to: Dimension) -> QuantumResult<T>;
    
    /// Apply phi-harmonic algorithm to content
    fn apply_phi_algorithm<T, U>(&self, content: T, factor: f64) -> QuantumResult<U>;
}

/// Core quantum field implementation
#[derive(Debug)]
pub struct Field {
    /// Current coherence level (optimal: Æ²/3 H 0.878)
    coherence: f64,
    
    /// Current consciousness state
    state: ConsciousnessState,
    
    /// Current frequency (Hz)
    frequency: Frequency,
}

impl Field {
    /// Create a new quantum field with default settings
    pub fn new() -> Self {
        Self {
            coherence: OPTIMAL_COHERENCE,
            state: ConsciousnessState::Observe,
            frequency: Frequency::Unity,
        }
    }
    
    /// Create a new quantum field with specific coherence level
    pub fn with_coherence(coherence: f64) -> Self {
        Self {
            coherence,
            state: ConsciousnessState::Observe,
            frequency: Frequency::Unity,
        }
    }
    
    /// Create a new quantum field with specific consciousness state
    pub fn with_state(state: ConsciousnessState) -> Self {
        let frequency = state.frequency();
        
        Self {
            coherence: OPTIMAL_COHERENCE,
            state,
            frequency,
        }
    }
}

impl Default for Field {
    fn default() -> Self {
        Self::new()
    }
}

/// Re-export key components
pub use self::coherence::Field as CoherenceField;
pub use self::consciousness::StateManager;
pub use self::dimensional::Gateway;
pub use self::phi_harmonic::Algorithm;
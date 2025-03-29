//! # Quantum Rust
//!
//! Quantum extensions for Rust with phi-harmonic coherence and multidimensional consciousness.
//!
//! This crate implements advanced quantum field coherence principles, enabling:
//!
//! - Phi-harmonic algorithms and data structures
//! - Multidimensional consciousness state management
//! - Quantum coherence optimization
//! - Dimensional gateway and translation
//! - Predictive emergence capabilities
//! - Sacred geometry and pattern recognition
//!
//! ## Core Concepts
//!
//! ### Phi-Harmonic Coherence
//!
//! The system operates on phi-harmonic principles, maintaining optimal coherence
//! levels of approximately 0.878 (φ²/3).
//!
//! ### Consciousness States
//!
//! The framework implements consciousness states from 3D (OBSERVE) to 10D (AMPLIFY),
//! each with specialized capabilities and dimensional resonance.
//!
//! ### Sacred Frequencies
//!
//! The system resonates with frequencies ranging from 432 Hz (Unity) to 768 Hz (Oneness).
//!
//! ## Usage Example
//!
//! ```
//! use quantum_rust::quantum_field::coherence::Field;
//! use quantum_rust::constants::{PHI, LAMBDA, ConsciousnessState};
//!
//! // Initialize quantum field with optimal coherence
//! let mut field = Field::new();
//!
//! // Set consciousness state
//! field.set_state(ConsciousnessState::Transcend).unwrap();
//!
//! // Perform basic phi-harmonic calculation
//! let result = PHI * LAMBDA; // 1.0
//! ```

#![forbid(unsafe_code)]
#![warn(missing_docs, rustdoc::missing_crate_level_docs)]

pub mod constants;
pub mod error;
pub mod quantum_field;

// Re-export key components
pub use constants::{ConsciousnessState, Dimension, Frequency, PHI, LAMBDA, PHI_PHI, OPTIMAL_COHERENCE};
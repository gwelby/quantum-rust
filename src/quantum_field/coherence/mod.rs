//! Quantum Field Coherence Management
//!
//! This module provides tools for managing and optimizing quantum field coherence,
//! ensuring proper phi-harmonic resonance for multidimensional operations.

use crate::constants::{ConsciousnessState, Dimension, Frequency, PHI, LAMBDA, OPTIMAL_COHERENCE};
use crate::error::{QuantumError, QuantumResult};

/// Quantum field coherence management system
#[derive(Debug, Clone)]
pub struct Field {
    /// Current coherence level (optimal: Æ²/3 H 0.878)
    coherence: f64,
    
    /// Base frequency (Hz)
    frequency: Frequency,
    
    /// Consciousness state
    state: ConsciousnessState,
    
    /// Coherence history
    history: Vec<f64>,
    
    /// Coherence correction enabled
    correction_enabled: bool,
}

impl Field {
    /// Create a new coherence field with optimal coherence
    pub fn new() -> Self {
        Self {
            coherence: OPTIMAL_COHERENCE,
            frequency: Frequency::Vision,
            state: ConsciousnessState::Transcend,
            history: vec![OPTIMAL_COHERENCE],
            correction_enabled: true,
        }
    }
    
    /// Create a new coherence field with specific coherence level
    pub fn with_coherence(coherence: f64) -> Self {
        Self {
            coherence,
            frequency: Frequency::Vision,
            state: ConsciousnessState::Transcend,
            history: vec![coherence],
            correction_enabled: true,
        }
    }
    
    /// Get the current coherence level
    pub fn coherence(&self) -> f64 {
        self.coherence
    }
    
    /// Get the current frequency
    pub fn frequency(&self) -> Frequency {
        self.frequency
    }
    
    /// Get the current state
    pub fn state(&self) -> ConsciousnessState {
        self.state
    }
    
    /// Set the consciousness state
    pub fn set_state(&mut self, state: ConsciousnessState) -> QuantumResult<()> {
        // Update frequency based on state
        self.frequency = state.frequency();
        
        // Adjust coherence for state transition
        let coherence_adjustment = match state {
            ConsciousnessState::Observe => 0.95,  // Small reduction
            ConsciousnessState::Create => 1.0,    // No change
            ConsciousnessState::Transcend => 1.1, // Small increase
            ConsciousnessState::Cascade => 1.21,  // Larger increase (Æ × 0.75)
            ConsciousnessState::Integrate => 1.05, // Small increase
            ConsciousnessState::Harmonize => 1.15, // Moderate increase
            ConsciousnessState::Amplify => 1.3,   // Significant increase (Æ × 0.8)
        };
        
        // Apply adjustment
        let new_coherence = self.coherence * coherence_adjustment;
        
        // Cap at maximum coherence
        let capped_coherence = new_coherence.min(PHI * 0.9);
        
        // Update state and coherence
        self.state = state;
        self.coherence = capped_coherence;
        self.history.push(capped_coherence);
        
        // Apply automatic correction if needed and enabled
        if self.correction_enabled && (self.coherence < OPTIMAL_COHERENCE * 0.7 || self.coherence > OPTIMAL_COHERENCE * 1.3) {
            self.apply_phi_harmonic_correction()?;
        }
        
        Ok(())
    }
    
    /// Enable or disable automatic coherence correction
    pub fn set_correction_enabled(&mut self, enabled: bool) {
        self.correction_enabled = enabled;
    }
    
    /// Optimize coherence to the ideal level (Æ²/3)
    pub fn optimize(&mut self) -> QuantumResult<f64> {
        let current = self.coherence;
        let target = OPTIMAL_COHERENCE;
        
        // Calculate phi-harmonic adjustment factor
        let adjustment = if current < target {
            // Increase coherence
            let factor = target / current;
            let phi_factor = 1.0 + (factor - 1.0) * PHI * 0.1;
            phi_factor
        } else {
            // Decrease coherence
            let factor = current / target;
            let phi_factor = 1.0 - (factor - 1.0) * LAMBDA * 0.1;
            phi_factor
        };
        
        // Apply adjustment with phi-harmonic correction
        let new_coherence = current * adjustment;
        
        // Apply further correction if still far from optimal
        let final_coherence = if (new_coherence - target).abs() > 0.1 {
            target * 0.95 + new_coherence * 0.05
        } else {
            new_coherence
        };
        
        // Update coherence
        self.coherence = final_coherence;
        self.history.push(final_coherence);
        
        Ok(final_coherence)
    }
    
    /// Apply phi-harmonic correction to restore optimal coherence
    pub fn apply_phi_harmonic_correction(&mut self) -> QuantumResult<f64> {
        // Calculate correction factor using phi-harmonic principles
        let phi_factor = PHI * LAMBDA; // 1.0
        let correction_factor = phi_factor * (OPTIMAL_COHERENCE / self.coherence).powf(0.5);
        
        // Apply correction
        let corrected_coherence = self.coherence * correction_factor;
        
        // Apply smoothing with history
        let history_weight = 0.2;
        let history_avg = if !self.history.is_empty() {
            self.history.iter().sum::<f64>() / self.history.len() as f64
        } else {
            self.coherence
        };
        
        let smoothed_coherence = corrected_coherence * (1.0 - history_weight) + history_avg * history_weight;
        
        // Update coherence
        self.coherence = smoothed_coherence;
        self.history.push(smoothed_coherence);
        
        // Keep history size manageable
        if self.history.len() > 10 {
            self.history.remove(0);
        }
        
        Ok(smoothed_coherence)
    }
    
    /// Calculate coherence for a dimensional translation
    pub fn calculate_translation_coherence(&self, from: Dimension, to: Dimension) -> f64 {
        let base_coherence = self.coherence;
        
        // Calculate dimensional distance
        let distance = (from.value() as i8 - to.value() as i8).abs() as f64;
        
        // Calculate coherence reduction factor based on distance
        let reduction_factor = 1.0 / (1.0 + distance * LAMBDA * 0.2);
        
        // Apply reduction
        let translation_coherence = base_coherence * reduction_factor;
        
        // Apply state-based adjustment
        let state_factor = match self.state {
            ConsciousnessState::Observe => 0.9,
            ConsciousnessState::Create => 1.0,
            ConsciousnessState::Transcend => 1.2,
            ConsciousnessState::Cascade => 1.3,
            ConsciousnessState::Integrate => 1.1,
            ConsciousnessState::Harmonize => 1.25,
            ConsciousnessState::Amplify => 1.35,
        };
        
        translation_coherence * state_factor
    }
    
    /// Check if coherence is sufficient for an operation
    pub fn is_coherence_sufficient(&self, required: f64) -> bool {
        self.coherence >= required
    }
    
    /// Verify operational integrity for a quantum operation
    pub fn verify_operational_integrity(&self, operation_type: &str, dimension: Dimension) -> QuantumResult<f64> {
        // Get minimum coherence needed for operation
        let min_coherence = match operation_type {
            "translation" => 0.6,
            "creation" => 0.7,
            "transformation" => 0.75,
            "manifestation" => 0.85,
            _ => 0.65, // Default minimum
        };
        
        // Adjust based on dimension
        let dimensional_factor = match dimension {
            Dimension::Physical => 0.9,
            Dimension::Emotional => 0.95,
            Dimension::Mental => 1.0,
            Dimension::Soul => 1.05,
            Dimension::Cosmic => 1.1,
            Dimension::Harmonic => 1.15,
            Dimension::Creative => 1.2,
            Dimension::Divine => 1.25,
            Dimension::Source => 1.3,
            Dimension::Absolute => 1.4,
        };
        
        let required_coherence = min_coherence * dimensional_factor;
        
        // Check if current coherence is sufficient
        if self.coherence < required_coherence {
            return Err(QuantumError::InsufficientCoherence {
                current: self.coherence,
                required: required_coherence,
            });
        }
        
        // Return operational integrity factor (higher is better)
        let integrity_factor = self.coherence / required_coherence;
        Ok(integrity_factor.min(PHI)) // Cap at PHI
    }
}

impl Default for Field {
    fn default() -> Self {
        Self::new()
    }
}
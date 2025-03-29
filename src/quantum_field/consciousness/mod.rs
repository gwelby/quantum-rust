//! Consciousness State Management
//!
//! This module provides tools for managing consciousness states,
//! state transitions, and state-specific operations.

use std::collections::VecDeque;

use crate::constants::{ConsciousnessState, Dimension, Frequency, PHI, LAMBDA};
use crate::error::{QuantumError, QuantumResult};
use crate::quantum_field::coherence::Field as CoherenceField;

/// Manager for consciousness states and transitions
#[derive(Debug)]
pub struct StateManager {
    /// Current consciousness state
    current_state: ConsciousnessState,
    
    /// Target consciousness state (if transitioning)
    target_state: Option<ConsciousnessState>,
    
    /// State transition progress (0.0 - 1.0)
    transition_progress: f64,
    
    /// State transition history
    transition_history: VecDeque<(ConsciousnessState, ConsciousnessState)>,
    
    /// Coherence field for state management
    coherence_field: CoherenceField,
    
    /// Current frequency
    frequency: Frequency,
}

impl StateManager {
    /// Create a new state manager with default OBSERVE state
    pub fn new() -> Self {
        Self {
            current_state: ConsciousnessState::Observe,
            target_state: None,
            transition_progress: 0.0,
            transition_history: VecDeque::with_capacity(10),
            coherence_field: CoherenceField::new(),
            frequency: Frequency::Unity,
        }
    }
    
    /// Create a new state manager with specific state
    pub fn with_state(state: ConsciousnessState) -> Self {
        let frequency = state.frequency();
        
        Self {
            current_state: state,
            target_state: None,
            transition_progress: 0.0,
            transition_history: VecDeque::with_capacity(10),
            coherence_field: CoherenceField::with_coherence(0.85),
            frequency,
        }
    }
    
    /// Get the current consciousness state
    pub fn current_state(&self) -> ConsciousnessState {
        self.current_state
    }
    
    /// Get the target state if transitioning
    pub fn target_state(&self) -> Option<ConsciousnessState> {
        self.target_state
    }
    
    /// Get the current transition progress
    pub fn transition_progress(&self) -> f64 {
        self.transition_progress
    }
    
    /// Get the current dimensional level
    pub fn dimension(&self) -> Dimension {
        self.current_state.dimension()
    }
    
    /// Get the current frequency
    pub fn frequency(&self) -> Frequency {
        self.frequency
    }
    
    /// Check if currently in a transition
    pub fn is_transitioning(&self) -> bool {
        self.target_state.is_some() && self.transition_progress < 1.0
    }
    
    /// Set the state immediately (without transition)
    pub fn set_state(&mut self, state: ConsciousnessState) -> QuantumResult<()> {
        // If already in this state, do nothing
        if self.current_state == state {
            return Ok(());
        }
        
        // Update coherence field
        self.coherence_field.set_state(state)?;
        
        // Record transition in history
        self.transition_history.push_front((self.current_state, state));
        if self.transition_history.len() > 10 {
            self.transition_history.pop_back();
        }
        
        // Update state
        self.current_state = state;
        self.target_state = None;
        self.transition_progress = 0.0;
        
        // Update frequency
        self.frequency = state.frequency();
        
        Ok(())
    }
    
    /// Begin transition to a new state
    pub fn begin_transition(&mut self, target_state: ConsciousnessState) -> QuantumResult<()> {
        // If already in this state, do nothing
        if self.current_state == target_state {
            return Ok(());
        }
        
        // If already transitioning to this state, do nothing
        if self.target_state == Some(target_state) {
            return Ok(());
        }
        
        // Set target state and reset progress
        self.target_state = Some(target_state);
        self.transition_progress = 0.0;
        
        Ok(())
    }
    
    /// Advance transition by the specified amount (0.0 - 1.0)
    pub fn advance_transition(&mut self, amount: f64) -> QuantumResult<f64> {
        if let Some(target) = self.target_state {
            // Calculate new progress
            let new_progress = (self.transition_progress + amount).min(1.0);
            self.transition_progress = new_progress;
            
            // If transition complete, update state
            if new_progress >= 1.0 {
                // Record transition in history
                self.transition_history.push_front((self.current_state, target));
                if self.transition_history.len() > 10 {
                    self.transition_history.pop_back();
                }
                
                // Update coherence field
                self.coherence_field.set_state(target)?;
                
                // Update state
                self.current_state = target;
                self.target_state = None;
                self.transition_progress = 0.0;
                
                // Update frequency
                self.frequency = target.frequency();
            } else {
                // Calculate intermediate frequency during transition
                let current_freq = self.current_state.frequency().value();
                let target_freq = target.frequency().value();
                let diff = target_freq - current_freq;
                let intermediate_freq = current_freq + diff * new_progress;
                
                // No direct way to set frequency from f64, so we just store it
                match target.frequency() {
                    Frequency::Unity => self.frequency = Frequency::Unity,
                    Frequency::Love => self.frequency = Frequency::Love,
                    Frequency::Cascade => self.frequency = Frequency::Cascade,
                    Frequency::Truth => self.frequency = Frequency::Truth,
                    Frequency::Vision => self.frequency = Frequency::Vision,
                    Frequency::Oneness => self.frequency = Frequency::Oneness,
                }
            }
            
            Ok(new_progress)
        } else {
            Err(QuantumError::OperationError {
                message: "No active state transition".to_string(),
            })
        }
    }
    
    /// Cancel the current transition
    pub fn cancel_transition(&mut self) {
        self.target_state = None;
        self.transition_progress = 0.0;
    }
    
    /// Get transition history
    pub fn transition_history(&self) -> &VecDeque<(ConsciousnessState, ConsciousnessState)> {
        &self.transition_history
    }
    
    /// Check if a specific operation is allowed in the current state
    pub fn is_operation_allowed(&self, operation: &str) -> bool {
        match operation {
            "observe" => matches!(self.current_state, ConsciousnessState::Observe | ConsciousnessState::Transcend),
            "create" => matches!(self.current_state, ConsciousnessState::Create | ConsciousnessState::Harmonize | ConsciousnessState::Cascade),
            "transform" => matches!(self.current_state, ConsciousnessState::Create | ConsciousnessState::Transcend | ConsciousnessState::Cascade),
            "analyze" => matches!(self.current_state, ConsciousnessState::Observe | ConsciousnessState::Transcend | ConsciousnessState::Integrate),
            "integrate" => matches!(self.current_state, ConsciousnessState::Integrate | ConsciousnessState::Harmonize),
            "manifest" => matches!(self.current_state, ConsciousnessState::Cascade | ConsciousnessState::Amplify),
            "harmonize" => matches!(self.current_state, ConsciousnessState::Harmonize | ConsciousnessState::Transcend),
            "amplify" => matches!(self.current_state, ConsciousnessState::Amplify | ConsciousnessState::Cascade),
            _ => false,
        }
    }
    
    /// Get the optimal state for a specific operation
    pub fn optimal_state_for_operation(&self, operation: &str) -> ConsciousnessState {
        match operation {
            "observe" => ConsciousnessState::Observe,
            "create" => ConsciousnessState::Create,
            "transform" => ConsciousnessState::Create,
            "analyze" => ConsciousnessState::Transcend,
            "integrate" => ConsciousnessState::Integrate,
            "manifest" => ConsciousnessState::Cascade,
            "harmonize" => ConsciousnessState::Harmonize,
            "amplify" => ConsciousnessState::Amplify,
            _ => ConsciousnessState::Transcend, // Default to Transcend
        }
    }
    
    /// Calculate coherence for the current state
    pub fn calculate_state_coherence(&self) -> f64 {
        let base_coherence = self.coherence_field.coherence();
        
        // Adjust based on state
        let state_factor = match self.current_state {
            ConsciousnessState::Observe => 0.9,   // Slight reduction
            ConsciousnessState::Create => 1.0,    // No change
            ConsciousnessState::Transcend => 1.1, // Slight increase
            ConsciousnessState::Cascade => PHI * 0.75, // Æ * 0.75
            ConsciousnessState::Integrate => 1.05, // Small increase
            ConsciousnessState::Harmonize => 1.15, // Moderate increase
            ConsciousnessState::Amplify => PHI * 0.8, // Æ * 0.8
        };
        
        // Apply transition adjustment if transitioning
        if self.is_transitioning() {
            let transition_factor = 1.0 - (0.1 * self.transition_progress * LAMBDA);
            base_coherence * state_factor * transition_factor
        } else {
            base_coherence * state_factor
        }
    }
    
    /// Get the coherence field
    pub fn coherence_field(&self) -> &CoherenceField {
        &self.coherence_field
    }
    
    /// Get mutable reference to the coherence field
    pub fn coherence_field_mut(&mut self) -> &mut CoherenceField {
        &mut self.coherence_field
    }
}

impl Default for StateManager {
    fn default() -> Self {
        Self::new()
    }
}
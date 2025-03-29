//! Unit tests for the consciousness module

#[cfg(test)]
mod tests {
    use crate::quantum_field::consciousness::StateManager;
    use crate::constants::ConsciousnessState;
    
    #[test]
    fn test_new_state_manager_has_observe_state() {
        let manager = StateManager::new();
        assert_eq!(manager.current_state(), ConsciousnessState::Observe);
    }

    #[test]
    fn test_with_state_sets_correct_state() {
        let manager = StateManager::with_state(ConsciousnessState::Transcend);
        assert_eq!(manager.current_state(), ConsciousnessState::Transcend);
    }

    #[test]
    fn test_set_state_changes_state() {
        let mut manager = StateManager::new();
        manager.set_state(ConsciousnessState::Create).unwrap();
        assert_eq!(manager.current_state(), ConsciousnessState::Create);
    }

    #[test]
    fn test_begin_transition_sets_target_state() {
        let mut manager = StateManager::new();
        manager.begin_transition(ConsciousnessState::Transcend).unwrap();
        assert_eq!(manager.target_state(), Some(ConsciousnessState::Transcend));
        assert_eq!(manager.transition_progress(), 0.0);
    }

    #[test]
    fn test_advance_transition_increases_progress() {
        let mut manager = StateManager::new();
        manager.begin_transition(ConsciousnessState::Create).unwrap();
        
        let progress = manager.advance_transition(0.3).unwrap();
        assert_eq!(progress, 0.3);
        assert_eq!(manager.transition_progress(), 0.3);
        
        // State shouldn't change until complete
        assert_eq!(manager.current_state(), ConsciousnessState::Observe);
    }

    #[test]
    fn test_complete_transition_changes_state() {
        let mut manager = StateManager::new();
        manager.begin_transition(ConsciousnessState::Create).unwrap();
        
        // Complete the transition
        manager.advance_transition(1.0).unwrap();
        
        // State should be updated
        assert_eq!(manager.current_state(), ConsciousnessState::Create);
        assert_eq!(manager.target_state(), None);
        assert_eq!(manager.transition_progress(), 0.0);
    }

    #[test]
    fn test_cancel_transition_resets_target() {
        let mut manager = StateManager::new();
        manager.begin_transition(ConsciousnessState::Transcend).unwrap();
        manager.advance_transition(0.5).unwrap();
        
        // Cancel the transition
        manager.cancel_transition();
        
        assert_eq!(manager.target_state(), None);
        assert_eq!(manager.transition_progress(), 0.0);
        assert_eq!(manager.current_state(), ConsciousnessState::Observe); // Unchanged
    }

    #[test]
    fn test_is_operation_allowed() {
        let mut manager = StateManager::new();
        
        // Observe state should allow observe operations
        assert!(manager.is_operation_allowed("observe"));
        
        // Observe state should not allow create operations
        assert!(!manager.is_operation_allowed("manifest"));
        
        // Change to Create state
        manager.set_state(ConsciousnessState::Create).unwrap();
        
        // Create state should allow create operations
        assert!(manager.is_operation_allowed("create"));
    }

    #[test]
    fn test_optimal_state_for_operation() {
        let manager = StateManager::new();
        
        assert_eq!(
            manager.optimal_state_for_operation("observe"), 
            ConsciousnessState::Observe
        );
        
        assert_eq!(
            manager.optimal_state_for_operation("create"), 
            ConsciousnessState::Create
        );
        
        assert_eq!(
            manager.optimal_state_for_operation("manifest"), 
            ConsciousnessState::Cascade
        );
    }

    #[test]
    fn test_calculate_state_coherence() {
        let manager = StateManager::new();
        let base_coherence = manager.calculate_state_coherence();
        
        // Create a manager with a different state
        let transcend_manager = StateManager::with_state(ConsciousnessState::Transcend);
        let transcend_coherence = transcend_manager.calculate_state_coherence();
        
        // Transcend should have higher coherence than Observe
        assert!(transcend_coherence > base_coherence);
    }
}
//! Unit tests for the coherence module

#[cfg(test)]
mod tests {
    use crate::quantum_field::coherence::Field;
    use crate::constants::{ConsciousnessState, Dimension, OPTIMAL_COHERENCE};

    #[test]
    fn test_new_field_has_optimal_coherence() {
        let field = Field::new();
        assert_eq!(field.coherence(), OPTIMAL_COHERENCE);
    }

    #[test]
    fn test_with_coherence_sets_correct_value() {
        let coherence = 0.75;
        let field = Field::with_coherence(coherence);
        assert_eq!(field.coherence(), coherence);
    }

    #[test]
    fn test_set_state_changes_coherence() {
        let mut field = Field::new();
        let initial_coherence = field.coherence();
        
        // Change to a state that increases coherence
        field.set_state(ConsciousnessState::Transcend).unwrap();
        let increased_coherence = field.coherence();
        assert!(increased_coherence > initial_coherence);
        
        // Change to a state that should decrease coherence compared to Transcend state
        // (might still be higher than initial if starting from optimal)
        field.set_state(ConsciousnessState::Observe).unwrap();
        assert!(field.coherence() < increased_coherence);
    }

    #[test]
    fn test_optimize_moves_toward_optimal_coherence() {
        let mut field = Field::with_coherence(0.5); // Well below optimal
        field.optimize().unwrap();
        assert!(field.coherence() > 0.5);
        assert!(field.coherence() <= OPTIMAL_COHERENCE * 1.1); // Allow some wiggle room
    }

    #[test]
    fn test_apply_phi_harmonic_correction_adjusts_coherence() {
        let mut field = Field::with_coherence(0.5); // Well below optimal
        field.apply_phi_harmonic_correction().unwrap();
        assert!(field.coherence() > 0.5);
    }

    #[test]
    fn test_calculate_translation_coherence() {
        let field = Field::new();
        
        // Same dimension should have high coherence
        let same_dim = field.calculate_translation_coherence(
            Dimension::Cosmic, 
            Dimension::Cosmic
        );
        assert!(same_dim > 0.8);
        
        // Distant dimensions should have lower coherence
        let distant_dims = field.calculate_translation_coherence(
            Dimension::Physical, 
            Dimension::Divine
        );
        assert!(distant_dims < same_dim);
    }

    #[test]
    fn test_is_coherence_sufficient() {
        let field = Field::with_coherence(0.7);
        
        // Lower requirement should be sufficient
        assert!(field.is_coherence_sufficient(0.6));
        
        // Higher requirement should not be sufficient
        assert!(!field.is_coherence_sufficient(0.8));
    }

    #[test]
    fn test_verify_operational_integrity() {
        let field = Field::new();
        
        // Low dimension operation should succeed
        let result = field.verify_operational_integrity("translation", Dimension::Physical);
        assert!(result.is_ok());
        
        // High dimension operation with high requirements might fail
        let low_coherence_field = Field::with_coherence(0.5);
        let result = low_coherence_field.verify_operational_integrity("manifestation", Dimension::Divine);
        assert!(result.is_err());
    }
}
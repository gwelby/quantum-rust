//! Unit tests for the quantum field module

#[cfg(test)]
mod tests {
    use crate::quantum_field::Field;
    use crate::constants::{ConsciousnessState, OPTIMAL_COHERENCE, Frequency};
    
    #[test]
    fn test_field_new() {
        let field = Field::new();
        assert_eq!(field.coherence, OPTIMAL_COHERENCE);
        assert!(matches!(field.state, ConsciousnessState::Observe));
        assert!(matches!(field.frequency, Frequency::Unity));
    }
    
    #[test]
    fn test_field_with_coherence() {
        let coherence = 0.75;
        let field = Field::with_coherence(coherence);
        assert_eq!(field.coherence, coherence);
    }
    
    #[test]
    fn test_field_with_state() {
        let field = Field::with_state(ConsciousnessState::Transcend);
        assert!(matches!(field.state, ConsciousnessState::Transcend));
        assert!(matches!(field.frequency, Frequency::Vision));
    }
    
    #[test]
    fn test_field_default() {
        let field = Field::default();
        assert_eq!(field.coherence, OPTIMAL_COHERENCE);
    }
}
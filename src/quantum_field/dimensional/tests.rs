//! Unit tests for the dimensional module

#[cfg(test)]
mod tests {
    use crate::quantum_field::dimensional::{Gateway, translate_simple, DimensionalSignature};
    use crate::constants::{ConsciousnessState, Dimension};
    use crate::error::QuantumResult;

    #[test]
    fn test_new_gateway_has_cosmic_dimension() {
        let gateway = Gateway::new();
        assert_eq!(gateway.current_dimension(), Dimension::Cosmic);
    }

    #[test]
    fn test_with_dimension_sets_correct_dimension() {
        let gateway = Gateway::with_dimension(Dimension::Mental);
        assert_eq!(gateway.current_dimension(), Dimension::Mental);
    }

    #[test]
    fn test_set_consciousness_state_updates_dimension() {
        let mut gateway = Gateway::new();
        gateway.set_consciousness_state(ConsciousnessState::Create).unwrap();
        
        // Create state should set Mental dimension
        assert_eq!(gateway.current_dimension(), Dimension::Mental);
        assert_eq!(gateway.consciousness_state(), ConsciousnessState::Create);
    }

    #[test]
    fn test_navigate_to_changes_dimension() {
        let mut gateway = Gateway::new();
        // First set state to appropriate value to navigate
        gateway.set_consciousness_state(ConsciousnessState::Observe).unwrap();
        gateway.navigate_to(Dimension::Physical).unwrap();
        assert_eq!(gateway.current_dimension(), Dimension::Physical);
    }

    #[test]
    fn test_establish_anchor_creates_anchor() {
        let mut gateway = Gateway::new();
        let anchor_coherence = gateway.establish_anchor().unwrap();
        
        assert!(anchor_coherence > 0.0);
        assert_eq!(
            gateway.anchor_coherence(Dimension::Cosmic),
            anchor_coherence
        );
    }

    #[test]
    fn test_is_dimension_accessible() {
        let gateway = Gateway::new();
        
        // Cosmic dimension (current) should be accessible
        assert!(gateway.is_dimension_accessible(Dimension::Cosmic));
        
        // Physical dimension should be accessible with default coherence
        assert!(gateway.is_dimension_accessible(Dimension::Physical));
    }

    #[test]
    fn test_translate_simple_works() {
        let result = translate_simple("test", Dimension::Physical, Dimension::Mental, 0.8);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "test");
    }

    #[test]
    fn test_gateway_translate() {
        let gateway = Gateway::new();
        
        // Define a custom translator that adds dimension info - using a String parameter
        let translator = |content: String, from: Dimension, to: Dimension, _coherence: f64| -> QuantumResult<String> {
            Ok(format!("{} ({}D→{}D)", content, from.value(), to.value()))
        };
        
        let result = gateway.translate(
            "quantum data".to_string(), 
            Dimension::Physical, 
            Dimension::Mental,
            translator
        ).unwrap();
        
        assert_eq!(result, "quantum data (3D→5D)");
    }

    #[test]
    fn test_dimensional_signature() {
        let signature = DimensionalSignature::new(
            "test data".to_string(), 
            Dimension::Cosmic, 
            0.85
        );
        
        assert_eq!(signature.content(), &"test data".to_string());
        assert_eq!(signature.dimension(), Dimension::Cosmic);
        assert_eq!(signature.coherence(), 0.85);
        assert!(signature.phi_resonance() > 0.0);
    }
}
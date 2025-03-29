//! Quantum Field Operations Example
//!
//! This example demonstrates more advanced quantum field operations
//! including coherence optimization, dimensional translation, and
//! phi-harmonic algorithm application.

use quantum_rust::constants::{ConsciousnessState, Dimension, OPTIMAL_COHERENCE};
use quantum_rust::quantum_field::coherence::Field as CoherenceField;
use quantum_rust::quantum_field::dimensional::{Gateway, translate_simple};
use quantum_rust::quantum_field::phi_harmonic::{
    phi_optimize, phi_harmonic_optimize, PhiHarmonicValues
};

fn main() {
    println!("Quantum Field Operations Example (∇λΣ∞)");
    println!("======================================\n");

    // Part 1: Coherence Optimization
    println!("Part 1: Coherence Optimization");
    println!("-----------------------------");
    
    // Create a field with suboptimal coherence
    let mut field = CoherenceField::with_coherence(0.5);
    println!("Initial coherence: {:.4}", field.coherence());
    
    // Optimize coherence
    println!("Optimizing coherence...");
    let result = field.optimize().expect("Failed to optimize coherence");
    println!("Optimized coherence: {:.4} (optimal: {:.4})", result, OPTIMAL_COHERENCE);
    
    // Apply phi-harmonic correction
    println!("Applying phi-harmonic correction...");
    let corrected = field.apply_phi_harmonic_correction().expect("Failed to apply correction");
    println!("Corrected coherence: {:.4}\n", corrected);

    // Part 2: Dimensional Gateway Operations
    println!("Part 2: Dimensional Gateway Operations");
    println!("-----------------------------------");
    
    // Create a gateway with Cosmic dimension
    let mut gateway = Gateway::with_dimension(Dimension::Cosmic);
    println!("Initial dimension: {:?}", gateway.current_dimension());
    
    // Set consciousness state
    println!("Setting consciousness state to Transcend...");
    gateway.set_consciousness_state(ConsciousnessState::Transcend)
        .expect("Failed to set consciousness state");
    
    // Establish anchor in current dimension
    let anchor = gateway.establish_anchor().expect("Failed to establish anchor");
    println!("Established anchor in {:?} with coherence: {:.4}", 
             gateway.current_dimension(), anchor);
    
    // Navigate to Mental dimension
    println!("Navigating to Mental dimension...");
    gateway.navigate_to(Dimension::Mental).expect("Failed to navigate");
    println!("Current dimension: {:?}", gateway.current_dimension());
    
    // Translation example
    println!("Translating content between dimensions...");
    let content = "Quantum information";
    let result = gateway.translate(
        content, 
        Dimension::Mental, 
        Dimension::Cosmic,
        translate_simple
    ).expect("Failed to translate");
    println!("Translated: \"{}\" → \"{}\"\n", content, result);

    // Part 3: Phi-Harmonic Algorithms
    println!("Part 3: Phi-Harmonic Algorithms");
    println!("----------------------------");
    
    // Phi optimization
    let initial_value = 3.5;
    let target = 5.0;
    let optimized = phi_optimize(initial_value, target, 5);
    println!("Phi optimization: {:.4} → {:.4} (target: {:.4})", 
             initial_value, optimized, target);
    
    // Cost function for phi-harmonic optimization
    let cost_function = |x: f64| (x - target).powi(2);
    
    // Phi-harmonic optimization
    let result = phi_harmonic_optimize(
        initial_value, 
        target, 
        cost_function, 
        10
    ).expect("Failed phi-harmonic optimization");
    println!("Phi-harmonic optimization: {:.4} → {:.4} (target: {:.4})", 
             initial_value, result, target);
    
    // PhiHarmonicValues demonstration
    let values = vec![1.0, 2.0, 3.0, 5.0, 8.0, 13.0];
    let mut harmonic_values = PhiHarmonicValues::new(values, 432.0);
    println!("Initial phi resonance: {:.4}", harmonic_values.phi_resonance());
    
    // Optimize values
    println!("Optimizing values...");
    harmonic_values.optimize(8.0, 5);
    println!("Optimized values: {:?}", harmonic_values.values());
    println!("New phi resonance: {:.4}", harmonic_values.phi_resonance());
    
    // Compress values
    println!("Compressing values...");
    harmonic_values.compress(0.5);
    println!("Compressed values: {:?}", harmonic_values.values());
    println!("Final phi resonance: {:.4}\n", harmonic_values.phi_resonance());
    
    println!("Quantum Field Operations Example Complete");
}
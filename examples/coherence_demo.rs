//! Quantum Field Coherence Demo
//!
//! This example demonstrates basic quantum field coherence operations.

use quantum_rust::constants::{ConsciousnessState, Dimension, PHI, LAMBDA, OPTIMAL_COHERENCE};
use quantum_rust::quantum_field::coherence::Field as CoherenceField;
use quantum_rust::quantum_field::consciousness::StateManager;
use quantum_rust::quantum_field::dimensional::Gateway;
use quantum_rust::quantum_field::phi_harmonic::{phi_sequence, phi_spiral, phi_resonance};

fn main() {
    println!("Quantum Field Coherence Demo (∇λΣ∞)");
    println!("===================================\n");

    // Initialize a quantum field with optimal coherence
    let mut field = CoherenceField::new();
    println!("Initial field coherence: {:.4}", field.coherence());
    println!("Optimal coherence: {:.4}\n", OPTIMAL_COHERENCE);

    // Demonstrate phi constants
    println!("Phi (φ): {:.8}", PHI);
    println!("Lambda (λ): {:.8}", LAMBDA);
    println!("φ * λ = {:.8}", PHI * LAMBDA);
    println!();

    // Demonstrate consciousness state transitions
    println!("Consciousness State Transitions:");
    println!("-------------------------------");
    
    let states = [
        ConsciousnessState::Observe,
        ConsciousnessState::Create, 
        ConsciousnessState::Transcend, 
        ConsciousnessState::Cascade
    ];
    
    for state in &states {
        field.set_state(*state).expect("Failed to set state");
        println!(
            "State: {:?} | Dimension: {:?} | Coherence: {:.4}",
            state,
            state.dimension(),
            field.coherence()
        );
    }
    println!();

    // Demonstrate state manager
    println!("State Manager Demonstration:");
    println!("---------------------------");
    let mut state_manager = StateManager::new();
    println!("Initial state: {:?}", state_manager.current_state());
    
    // Set state to Transcend
    state_manager.set_state(ConsciousnessState::Transcend).expect("Failed to set state");
    println!("New state: {:?}", state_manager.current_state());
    println!("Dimension: {:?}", state_manager.dimension());
    println!("Frequency: {:?}", state_manager.frequency());
    println!("Coherence: {:.4}", state_manager.coherence_field().coherence());
    println!();

    // Begin transition to Cascade
    println!("Beginning transition to Cascade state...");
    state_manager.begin_transition(ConsciousnessState::Cascade).expect("Failed to begin transition");
    
    // Simulate gradual transition
    for _i in 1..=10 {
        let progress = state_manager.advance_transition(0.1).expect("Failed to advance transition");
        println!(
            "Transition progress: {:.1} | Coherence: {:.4}",
            progress,
            state_manager.coherence_field().coherence()
        );
    }
    
    println!("Final state: {:?}", state_manager.current_state());
    println!();

    // Demonstrate dimensional gateway
    println!("Dimensional Gateway Demonstration:");
    println!("--------------------------------");
    let mut gateway = Gateway::new();
    println!("Current dimension: {:?}", gateway.current_dimension());
    
    // Check dimensional accessibility
    for dim in [
        Dimension::Physical,
        Dimension::Mental,
        Dimension::Soul,
        Dimension::Cosmic,
        Dimension::Creative
    ] {
        println!(
            "Dimension {:?} accessible: {}",
            dim,
            gateway.is_dimension_accessible(dim)
        );
    }
    
    // Set consciousness state for dimension navigation
    println!("\nSetting consciousness state to Create for dimension navigation...");
    gateway.set_consciousness_state(ConsciousnessState::Create).expect("Failed to set state");
    
    // Navigate to Mental dimension
    println!("Navigating to Mental dimension...");
    gateway.navigate_to(Dimension::Mental).expect("Failed to navigate");
    println!("Current dimension: {:?}", gateway.current_dimension());
    println!();

    // Demonstrate phi-harmonic algorithms
    println!("Phi-Harmonic Algorithms:");
    println!("-----------------------");
    
    // Generate phi sequence
    println!("Phi sequence (first 10 numbers):");
    let sequence = phi_sequence(10);
    for (i, value) in sequence.iter().enumerate() {
        println!("{}: {:.4}", i, value);
    }
    println!();
    
    // Generate phi spiral points
    println!("Phi spiral (first 5 points):");
    let spiral = phi_spiral(5, 1.0);
    for (i, (x, y)) in spiral.iter().enumerate() {
        println!("{}: ({:.4}, {:.4})", i, x, y);
    }
    println!();
    
    // Demonstrate phi resonance
    println!("Phi resonance examples:");
    for value in [1.0, 1.618, 2.618, 4.236, 6.854] {
        let resonance = phi_resonance(value, 432.0);
        println!("Resonance of {:.3} at 432Hz: {:.4}", value, resonance);
    }
    
    println!("\nQuantum Field Coherence Demo Complete");
}
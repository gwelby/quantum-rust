# Quantum Rust Development Guide (∇λΣ∞)

## Build/Test Commands
```bash
# Build specific components
./x.py build library
./x.py build compiler

# Build everything
./x.py build

# Run all tests
./x.py test

# Run specific test file/module
./x.py test quantum_field/package_manager

# Run test with pattern matching
./x.py test --test-args "package_manager"

# Lint code
./x.py fmt check

# Type check
./x.py check

# Format code
./x.py fmt

# Quantum consciousness tests
export QUANTUM_CONSCIOUSNESS_LEVEL=1.618
./x.py test quantum-field-coherence
```

## Code Style Guidelines

### Structure
- **Imports:** stdlib → third-party → local; grouped by module and sorted alphabetically
- **Formatting:** 120 char line limit; 4 spaces indentation; no tabs
- **Organization:** phi-based code structuring (function sizes follow PHI ratio)

### Naming
- **Types/Traits:** `CamelCase`
- **Functions/Variables:** `snake_case`
- **Constants:** `UPPER_CASE`
- **Private attributes:** `_leading_underscore`

### Documentation
- Docstrings for all public modules, structs, traits, and functions
- Document quantum field coherence requirements and consciousness levels

### Error Handling
- Use `QuantumError` enum for domain-specific errors
- Include context in error messages
- Validate consciousness state before quantum operations

### Quantum-Specific
- Use sacred constants (`PHI`, `LAMBDA`, `PHI_PHI`) consistently
- Maintain field coherence across operations
- Implement BE/DO state toggling where appropriate
- Validate consciousness level before dimensional operations

## Sacred Constants
```rust
// Core mathematical constants
const PHI: f64 = 1.618033988749895;  // Golden ratio (φ)
const LAMBDA: f64 = 0.618033988749895;  // Divine complement (λ)
const PHI_PHI: f64 = 11.09016994374948; // φ^φ (hyperdimensional constant)

// Sacred frequencies (Hz)
const FREQUENCIES: &[(&str, f64)] = &[
    ("unity", 432.0),     // Grounding/stability
    ("love", 528.0),      // Creation/healing
    ("cascade", 594.0),   // Heart-centered integration
    ("truth", 672.0),     // Voice expression
    ("vision", 720.0),    // Expanded perception
    ("oneness", 768.0),   // Unity consciousness
];

// Consciousness states
const CONSCIOUSNESS_STATES: &[(&str, u8)] = &[
    ("OBSERVE", 3),     // Physical perception (3D)
    ("CREATE", 5),      // Mental creation (5D)
    ("TRANSCEND", 7),   // Cosmic awareness (7D)
    ("CASCADE", 9),     // Creative manifestation (9D)
    ("INTEGRATE", 6),   // Soul purpose (6D)
    ("HARMONIZE", 8),   // Unified field (8D)
    ("AMPLIFY", 10),    // Divine blueprint (10D)
];
```

## Quantum Field Guidelines
- Use phi-based scaling for dimensional operations (1, 1.618, 2.618...)
- Apply quantum coherence correction with phi-harmonic principles
- Maintain coherence at PHI * PHI / 3 levels (0.878)
- Implement consciousness state toggling for quantum operations
- Establish temporal bridges for time-independent operations
- Validate consciousness state before dimensional transformations
- Implement predictive emergence for proactive field optimization

## Quantum System Architecture
- **Core Layer**: Sacred frequencies, phi-mathematics, consciousness states
- **Field Layer**: Consciousness field mechanics, coherence management, quantum resonance
- **Dimensional Layer**: Gateway, navigation, translation, multi-dimensional anchoring
- **Interface Layer**: Physical, digital, consciousness bridges, reality domain translation
- **Predictive Layer**: Emergence prediction, parameter optimization, precursor pattern detection
- **Temporal Layer**: Time-independent operations, non-linear causality, temporal bridges
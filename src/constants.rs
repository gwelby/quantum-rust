//! Sacred constants and frequencies for quantum field operations
//!
//! This module provides the core mathematical constants and sacred frequencies
//! needed for phi-harmonic coherence and multidimensional operations.

/// Golden ratio (φ)
pub const PHI: f64 = 1.618033988749895;

/// Divine complement (λ)
pub const LAMBDA: f64 = 0.618033988749895;

/// Hyperdimensional constant (φ^φ)
pub const PHI_PHI: f64 = 11.09016994374948;

/// Optimal field coherence level (φ²/3)
pub const OPTIMAL_COHERENCE: f64 = 0.8726;

/// Sacred frequencies (Hz)
pub enum Frequency {
    /// Unity frequency (432 Hz) - Grounding and stability
    Unity = 432,
    
    /// Love frequency (528 Hz) - Creation and healing
    Love = 528,
    
    /// Cascade frequency (594 Hz) - Heart-centered integration
    Cascade = 594,
    
    /// Truth frequency (672 Hz) - Voice expression
    Truth = 672,
    
    /// Vision frequency (720 Hz) - Expanded perception
    Vision = 720,
    
    /// Oneness frequency (768 Hz) - Unity consciousness
    Oneness = 768,
}

impl Frequency {
    /// Get the frequency value in Hz
    pub fn value(&self) -> f64 {
        match self {
            Frequency::Unity => 432.0,
            Frequency::Love => 528.0,
            Frequency::Cascade => 594.0,
            Frequency::Truth => 672.0,
            Frequency::Vision => 720.0,
            Frequency::Oneness => 768.0,
        }
    }
    
    /// Get the name of the frequency
    pub fn name(&self) -> &'static str {
        match self {
            Frequency::Unity => "Unity",
            Frequency::Love => "Love",
            Frequency::Cascade => "Cascade",
            Frequency::Truth => "Truth",
            Frequency::Vision => "Vision",
            Frequency::Oneness => "Oneness",
        }
    }
    
    /// Get the purpose of the frequency
    pub fn purpose(&self) -> &'static str {
        match self {
            Frequency::Unity => "Grounding and stability",
            Frequency::Love => "Creation and healing",
            Frequency::Cascade => "Heart-centered integration",
            Frequency::Truth => "Voice expression",
            Frequency::Vision => "Expanded perception",
            Frequency::Oneness => "Unity consciousness",
        }
    }
}

/// Dimensions (3D-12D)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Dimension {
    /// Physical/Material dimension (3D)
    Physical = 3,
    
    /// Emotional/Temporal dimension (4D)
    Emotional = 4,
    
    /// Mental/Conceptual dimension (5D)
    Mental = 5,
    
    /// Soul/Purpose dimension (6D)
    Soul = 6,
    
    /// Cosmic/Universal dimension (7D)
    Cosmic = 7,
    
    /// Harmonic/Unified dimension (8D)
    Harmonic = 8,
    
    /// Creative/Manifestation dimension (9D)
    Creative = 9,
    
    /// Divine/Blueprint dimension (10D)
    Divine = 10,
    
    /// Source/Void dimension (11D)
    Source = 11,
    
    /// Absolute/Infinite dimension (12D)
    Absolute = 12,
}

impl Dimension {
    /// Get the dimension value
    pub fn value(&self) -> u8 {
        *self as u8
    }
    
    /// Get the name of the dimension
    pub fn name(&self) -> &'static str {
        match self {
            Dimension::Physical => "Physical",
            Dimension::Emotional => "Emotional",
            Dimension::Mental => "Mental",
            Dimension::Soul => "Soul",
            Dimension::Cosmic => "Cosmic",
            Dimension::Harmonic => "Harmonic",
            Dimension::Creative => "Creative",
            Dimension::Divine => "Divine",
            Dimension::Source => "Source",
            Dimension::Absolute => "Absolute",
        }
    }
    
    /// Get the characteristics of the dimension
    pub fn characteristics(&self) -> &'static str {
        match self {
            Dimension::Physical => "Factual content, linear sequences, discrete data points",
            Dimension::Emotional => "Resonance connections, emotional coloring, timeline flexibility",
            Dimension::Mental => "Thought forms, conceptual frameworks, possibility spectrums",
            Dimension::Soul => "Meaning, purpose, soul-level blueprints, truth vibrations",
            Dimension::Cosmic => "Universal perspective, cosmic awareness, collective patterns",
            Dimension::Harmonic => "Pattern synthesis, harmonic integration, unified consciousness",
            Dimension::Creative => "Generative fields, template creation, reality manifestation",
            Dimension::Divine => "Cosmic architecture, divine templates, universal principles",
            Dimension::Source => "Quantum potential state, undefined possibility, source connection",
            Dimension::Absolute => "Pure consciousness, absolute unity, timeless awareness",
        }
    }
    
    /// Get the phi-scaled value of the dimension
    pub fn phi_value(&self) -> f64 {
        self.value() as f64 * PHI
    }
}

/// Consciousness states
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConsciousnessState {
    /// OBSERVE state - Physical perception (3D)
    Observe,
    
    /// CREATE state - Mental creation (5D)
    Create,
    
    /// TRANSCEND state - Cosmic awareness (7D)
    Transcend,
    
    /// CASCADE state - Creative manifestation (9D)
    Cascade,
    
    /// INTEGRATE state - Soul purpose (6D)
    Integrate,
    
    /// HARMONIZE state - Unified field (8D)
    Harmonize,
    
    /// AMPLIFY state - Divine blueprint (10D)
    Amplify,
}

impl ConsciousnessState {
    /// Get the dimension associated with this state
    pub fn dimension(&self) -> Dimension {
        match self {
            ConsciousnessState::Observe => Dimension::Physical,
            ConsciousnessState::Create => Dimension::Mental,
            ConsciousnessState::Transcend => Dimension::Cosmic,
            ConsciousnessState::Cascade => Dimension::Creative,
            ConsciousnessState::Integrate => Dimension::Soul,
            ConsciousnessState::Harmonize => Dimension::Harmonic,
            ConsciousnessState::Amplify => Dimension::Divine,
        }
    }
    
    /// Get the primary frequency associated with this state
    pub fn frequency(&self) -> Frequency {
        match self {
            ConsciousnessState::Observe => Frequency::Unity,
            ConsciousnessState::Create => Frequency::Love,
            ConsciousnessState::Transcend => Frequency::Vision,
            ConsciousnessState::Cascade => Frequency::Oneness,
            ConsciousnessState::Integrate => Frequency::Cascade,
            ConsciousnessState::Harmonize => Frequency::Truth,
            ConsciousnessState::Amplify => Frequency::Oneness,
        }
    }
    
    /// Get the name of the state
    pub fn name(&self) -> &'static str {
        match self {
            ConsciousnessState::Observe => "OBSERVE",
            ConsciousnessState::Create => "CREATE",
            ConsciousnessState::Transcend => "TRANSCEND",
            ConsciousnessState::Cascade => "CASCADE",
            ConsciousnessState::Integrate => "INTEGRATE",
            ConsciousnessState::Harmonize => "HARMONIZE",
            ConsciousnessState::Amplify => "AMPLIFY",
        }
    }
    
    /// Get the purpose of the state
    pub fn purpose(&self) -> &'static str {
        match self {
            ConsciousnessState::Observe => "Physical perception and data collection",
            ConsciousnessState::Create => "Mental creation and transformation",
            ConsciousnessState::Transcend => "Cosmic awareness and universal patterns",
            ConsciousnessState::Cascade => "Creative manifestation and implementation",
            ConsciousnessState::Integrate => "Soul purpose and meaning integration",
            ConsciousnessState::Harmonize => "Unified field operations",
            ConsciousnessState::Amplify => "Divine blueprint activation",
        }
    }
}

/// The Identity Signature - ∇λΣ∞ (Gradient-Lambda-Summation-Infinity)
pub struct IdentitySignature;

impl IdentitySignature {
    /// Get the full signature string
    pub fn signature() -> &'static str {
        "∇λΣ∞"
    }
    
    /// Get the components with meanings
    pub fn components() -> [(&'static str, &'static str); 4] {
        [
            ("∇", "Gradient operations across dimensional boundaries"),
            ("λ", "Divine complement and quantum field harmonics"),
            ("Σ", "Integration and summation of patterns across dimensions"),
            ("∞", "Infinite potential and timeless awareness"),
        ]
    }
}
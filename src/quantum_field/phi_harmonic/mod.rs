//! Phi-Harmonic Algorithms and Data Structures
//!
//! This module provides algorithms and data structures based on phi relationships,
//! including phi-scaled calculations, phi-spiral patterns, and phi-harmonic optimization.

#[cfg(test)]
mod tests;

use crate::constants::{PHI, LAMBDA, PHI_PHI};
use crate::error::QuantumResult;

/// Phi-harmonic algorithm types
#[derive(Debug, Clone, Copy)]
pub enum Algorithm {
    /// Phi-scaling algorithm (scales by �)
    PhiScaling,
    
    /// Lambda-scaling algorithm (scales by �)
    LambdaScaling,
    
    /// Phi-spiral algorithm (organizes in phi spiral)
    PhiSpiral,
    
    /// Phi-sequence algorithm (creates phi sequence)
    PhiSequence,
    
    /// Phi-optimization algorithm (optimizes using phi)
    PhiOptimization,
    
    /// Phi-gridding algorithm (creates phi grid)
    PhiGridding,
    
    /// Phi-compression algorithm (compresses using phi)
    PhiCompression,
    
    /// Phi-tuned resonance (tunes to phi resonance)
    PhiResonance,
}

/// Phi-scaling function
pub fn phi_scale(value: f64, multiplier: f64) -> f64 {
    value * (PHI * multiplier)
}

/// Lambda-scaling function
pub fn lambda_scale(value: f64, multiplier: f64) -> f64 {
    value * (LAMBDA * multiplier)
}

/// Phi-sequence generator
pub fn phi_sequence(length: usize) -> Vec<f64> {
    let mut sequence = Vec::with_capacity(length);
    
    if length > 0 {
        sequence.push(1.0);
    }
    
    if length > 1 {
        sequence.push(PHI);
    }
    
    for i in 2..length {
        let next = sequence[i-1] + sequence[i-2];
        sequence.push(next);
    }
    
    sequence
}

/// Phi-ratio calculator
pub fn phi_ratio(a: f64, b: f64) -> f64 {
    if b == 0.0 {
        return 0.0;
    }
    
    let ratio = a / b;
    let phi_diff = (ratio - PHI).abs();
    
    // Calculate how close the ratio is to � (1.0 = exact match, 0.0 = far off)
    1.0 / (1.0 + phi_diff * 5.0)
}

/// Phi-optimized value
pub fn phi_optimize(value: f64, target: f64, iterations: usize) -> f64 {
    let mut current = value;
    
    for _ in 0..iterations {
        let diff = target - current;
        let adjustment = diff * LAMBDA;
        current += adjustment;
    }
    
    current
}

/// Phi-spiral point generator
pub fn phi_spiral_point(index: usize, scale: f64) -> (f64, f64) {
    let angle = index as f64 * PHI * std::f64::consts::PI * 2.0;
    let radius = scale * (index as f64).sqrt();
    
    let x = radius * angle.cos();
    let y = radius * angle.sin();
    
    (x, y)
}

/// Phi-spiral points generator
pub fn phi_spiral(count: usize, scale: f64) -> Vec<(f64, f64)> {
    (0..count).map(|i| phi_spiral_point(i, scale)).collect()
}

/// Phi-harmonic resonance calculator
pub fn phi_resonance(value: f64, base_frequency: f64) -> f64 {
    // Calculate resonant frequency
    let resonant_frequency = base_frequency * PHI.powf(value.abs().ln());
    
    // Calculate resonance strength
    let harmonics = [0.5, 1.0, PHI, PHI_PHI];
    
    let max_resonance = harmonics.iter()
        .map(|h| {
            let harmonic_freq = base_frequency * h;
            let distance = (resonant_frequency - harmonic_freq).abs();
            let normalized_distance = distance / base_frequency;
            (-normalized_distance * 10.0).exp()
        })
        .fold(0.0, f64::max);
    
    max_resonance
}

/// Phi-gridding function
pub fn phi_grid(width: usize, height: usize) -> Vec<Vec<f64>> {
    let mut grid = Vec::with_capacity(height);
    
    for y in 0..height {
        let mut row = Vec::with_capacity(width);
        
        for x in 0..width {
            let phi_x = phi_scale(x as f64, 0.1);
            let phi_y = phi_scale(y as f64, 0.1);
            
            let value = (phi_x * phi_y).sin() * 0.5 + 0.5;
            row.push(value);
        }
        
        grid.push(row);
    }
    
    grid
}

/// Phi-compression algorithm
pub fn phi_compress(values: &[f64], factor: f64) -> Vec<f64> {
    let compressed_len = (values.len() as f64 * LAMBDA * factor) as usize;
    let mut compressed = Vec::with_capacity(compressed_len);
    
    if values.is_empty() || compressed_len == 0 {
        return compressed;
    }
    
    let step = values.len() as f64 / compressed_len as f64;
    
    for i in 0..compressed_len {
        let index = (i as f64 * step) as usize;
        let next_index = ((i as f64 + 1.0) * step) as usize;
        
        if index >= values.len() {
            break;
        }
        
        let next_index = next_index.min(values.len() - 1);
        
        if next_index == index {
            compressed.push(values[index]);
        } else {
            // Calculate weighted average
            let mut sum = 0.0;
            let mut weight_sum = 0.0;
            
            for j in index..=next_index {
                let weight = PHI.powf(-((j - index) as f64).abs());
                sum += values[j] * weight;
                weight_sum += weight;
            }
            
            compressed.push(sum / weight_sum);
        }
    }
    
    compressed
}

/// Phi-harmonic optimization function
pub fn phi_harmonic_optimize<F>(mut value: f64, target: f64, cost_function: F, iterations: usize) -> QuantumResult<f64>
where
    F: Fn(f64) -> f64,
{
    let mut best_value = value;
    let mut best_cost = cost_function(value);
    
    for i in 0..iterations {
        // Calculate phi-scaled adjustment
        let progress = i as f64 / iterations as f64;
        let scale = (1.0 - progress).powf(0.5); // Square root decay
        
        let diff = target - value;
        let adjustment = diff * LAMBDA * scale;
        
        // Try new value
        let new_value = value + adjustment;
        let new_cost = cost_function(new_value);
        
        // Update if better
        if new_cost < best_cost {
            best_value = new_value;
            best_cost = new_cost;
        }
        
        // Phi-harmonic oscillation to escape local minima
        let oscillation = (i as f64 * PHI).sin() * scale * LAMBDA;
        value = best_value + oscillation;
    }
    
    Ok(best_value)
}

/// Container for phi-harmonic values
#[derive(Debug, Clone)]
pub struct PhiHarmonicValues {
    /// The values
    values: Vec<f64>,
    
    /// Base frequency
    base_frequency: f64,
    
    /// Phi resonance
    phi_resonance: f64,
}

impl PhiHarmonicValues {
    /// Create a new container from raw values
    pub fn new(values: Vec<f64>, base_frequency: f64) -> Self {
        // Calculate phi resonance
        let resonance = values.iter()
            .map(|v| phi_resonance(*v, base_frequency))
            .sum::<f64>() / values.len() as f64;
        
        Self {
            values,
            base_frequency,
            phi_resonance: resonance,
        }
    }
    
    /// Apply phi optimization to all values
    pub fn optimize(&mut self, target: f64, iterations: usize) {
        self.values = self.values.iter()
            .map(|v| phi_optimize(*v, target, iterations))
            .collect();
        
        // Recalculate resonance
        self.phi_resonance = self.values.iter()
            .map(|v| phi_resonance(*v, self.base_frequency))
            .sum::<f64>() / self.values.len() as f64;
    }
    
    /// Apply phi compression
    pub fn compress(&mut self, factor: f64) {
        self.values = phi_compress(&self.values, factor);
        
        // Recalculate resonance
        self.phi_resonance = self.values.iter()
            .map(|v| phi_resonance(*v, self.base_frequency))
            .sum::<f64>() / self.values.len() as f64;
    }
    
    /// Get the values
    pub fn values(&self) -> &[f64] {
        &self.values
    }
    
    /// Get the base frequency
    pub fn base_frequency(&self) -> f64 {
        self.base_frequency
    }
    
    /// Get the phi resonance
    pub fn phi_resonance(&self) -> f64 {
        self.phi_resonance
    }
    
    /// Add a value
    pub fn add_value(&mut self, value: f64) {
        self.values.push(value);
        
        // Update resonance
        let value_resonance = phi_resonance(value, self.base_frequency);
        let prev_resonance = self.phi_resonance * (self.values.len() - 1) as f64;
        self.phi_resonance = (prev_resonance + value_resonance) / self.values.len() as f64;
    }
}
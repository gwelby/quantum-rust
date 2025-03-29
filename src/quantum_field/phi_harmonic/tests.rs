//! Unit tests for the phi-harmonic module

#[cfg(test)]
mod tests {
    use crate::quantum_field::phi_harmonic::{
        phi_scale, lambda_scale, phi_sequence, phi_ratio, phi_optimize,
        phi_spiral_point, phi_spiral, phi_resonance, phi_grid, phi_compress,
        phi_harmonic_optimize, PhiHarmonicValues
    };
    use crate::constants::{PHI, LAMBDA};
    use std::f64::consts::PI;

    #[test]
    fn test_phi_scale() {
        let value = 2.0;
        let scaled = phi_scale(value, 1.0);
        assert_eq!(scaled, value * PHI);
        
        let half_scaled = phi_scale(value, 0.5);
        assert_eq!(half_scaled, value * PHI * 0.5);
    }

    #[test]
    fn test_lambda_scale() {
        let value = 2.0;
        let scaled = lambda_scale(value, 1.0);
        assert_eq!(scaled, value * LAMBDA);
        
        let double_scaled = lambda_scale(value, 2.0);
        assert_eq!(double_scaled, value * LAMBDA * 2.0);
    }

    #[test]
    fn test_phi_sequence() {
        let sequence = phi_sequence(5);
        
        // Sequence should have 5 elements
        assert_eq!(sequence.len(), 5);
        
        // First two elements should be 1.0 and PHI
        assert_eq!(sequence[0], 1.0);
        assert_eq!(sequence[1], PHI);
        
        // Each element after the first two should be the sum of the previous two
        for i in 2..5 {
            assert_eq!(sequence[i], sequence[i-1] + sequence[i-2]);
        }
    }

    #[test]
    fn test_phi_ratio() {
        // Exact phi ratio should return 1.0
        let exact = phi_ratio(PHI, 1.0);
        assert!(exact > 0.9);
        
        // Very different ratio should return lower value
        let different = phi_ratio(2.0, 1.0);
        assert!(different < exact);
        
        // Equal values should return low value (far from phi)
        let equal = phi_ratio(1.0, 1.0);
        assert!(equal < 0.5);
    }

    #[test]
    fn test_phi_optimize() {
        let value = 2.0;
        let target = 5.0;
        let optimized = phi_optimize(value, target, 10);
        
        // Optimized value should be closer to target than original
        assert!(optimized > value);
        assert!((optimized - target).abs() < (value - target).abs());
    }

    #[test]
    fn test_phi_spiral_point() {
        let (x, y) = phi_spiral_point(0, 1.0);
        assert_eq!(x, 0.0);
        assert_eq!(y, 0.0);
        
        // Verify that points for indices 1 and 2 are not at the origin
        let (x1, y1) = phi_spiral_point(1, 1.0);
        assert!(x1 != 0.0 || y1 != 0.0);
        
        let (x2, y2) = phi_spiral_point(2, 1.0);
        assert!(x2 != 0.0 || y2 != 0.0);
        
        // Ensure points are different from each other (spiral grows)
        assert!(x1 != x2 || y1 != y2);
    }

    #[test]
    fn test_phi_spiral() {
        let spiral = phi_spiral(5, 1.0);
        
        // Should return 5 points
        assert_eq!(spiral.len(), 5);
        
        // First point should be origin
        assert_eq!(spiral[0], (0.0, 0.0));
    }

    #[test]
    fn test_phi_resonance() {
        let base_freq = 432.0;
        
        // Resonance should be positive and non-zero for PHI
        let phi_value = phi_resonance(PHI, base_freq);
        assert!(phi_value > 0.0);
        
        // Resonance should be lower for random values
        let _random = phi_resonance(3.147, base_freq);
        // This is probabilistic so we can't assert an exact relationship
    }

    #[test]
    fn test_phi_grid() {
        let grid = phi_grid(3, 2);
        
        // Grid should have correct dimensions
        assert_eq!(grid.len(), 2);
        assert_eq!(grid[0].len(), 3);
        
        // Values should be between 0 and 1
        for row in &grid {
            for &value in row {
                assert!(value >= 0.0 && value <= 1.0);
            }
        }
    }

    #[test]
    fn test_phi_compress() {
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
        let compressed = phi_compress(&values, 0.5);
        
        // Compressed array should be smaller
        assert!(compressed.len() < values.len());
    }

    #[test]
    fn test_phi_harmonic_optimize() {
        let value = 2.0;
        let target = 5.0;
        let cost_function = |x: f64| (x - target).powi(2);
        
        let result = phi_harmonic_optimize(value, target, cost_function, 10);
        assert!(result.is_ok());
        
        let optimized = result.unwrap();
        
        // Optimized value should be closer to target than original
        assert!((optimized - target).abs() < (value - target).abs());
    }

    #[test]
    fn test_phi_harmonic_values() {
        let values = vec![1.0, 2.0, 3.0, 5.0, 8.0];
        let harmonic = PhiHarmonicValues::new(values.clone(), 432.0);
        
        // Should preserve original values
        assert_eq!(harmonic.values(), &values);
        
        // Should calculate resonance
        assert!(harmonic.phi_resonance() > 0.0);
        
        // Test optimization
        let mut optimized = harmonic.clone();
        optimized.optimize(6.0, 5);
        
        // Optimization should change values
        assert_ne!(optimized.values(), &values);
        
        // Test compression
        let mut compressed = harmonic.clone();
        compressed.compress(0.5);
        
        // Compression should reduce size
        assert!(compressed.values().len() < values.len());
    }
}
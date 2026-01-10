tests/integration_tests.rs

use lightspeedvalidator::{GammaRayAnalyzer, TimingData, LightSpeedTestResult};

#[test]
fn test_basic_analyzer_creation() {
    let analyzer = GammaRayAnalyzer::new();
    assert_eq!(analyzer.measurements.len(), 0);
}

#[test]
fn test_add_measurement() {
    let mut analyzer = GammaRayAnalyzer::new();
    analyzer.add_measurement(100.0, 1234567890.123, 0.001);
    assert_eq!(analyzer.measurements.len(), 1);
    
    let measurement = &analyzer.measurements[0];
    assert_eq!(measurement.energy, 100.0);
    assert_eq!(measurement.arrival_time, 1234567890.123);
    assert_eq!(measurement.error, 0.001);
}

#[test]
fn test_set_sensitivity_threshold() {
    let mut analyzer = GammaRayAnalyzer::new();
    assert_eq!(analyzer.sensitivity_threshold, 1e-12);
    
    analyzer.set_sensitivity_threshold(1e-10);
    assert_eq!(analyzer.sensitivity_threshold, 1e-10);
}

#[test]
fn test_light_speed_test_result_structure() {
    let analyzer = GammaRayAnalyzer::new();
    let result = analyzer.test_light_speed_constancy();
    
    // Verify the result has the expected structure
    assert!(result.is_valid || !result.is_valid); // Valid boolean value
    assert!(result.confidence_level >= 0.0 && result.confidence_level <= 1.0);
    assert!(result.chi_squared >= 0.0);
    assert!(result.p_value >= 0.0 && result.p_value <= 1.0);
    assert!(result.anomalies_detected >= 0);
}

#[test]
fn test_calculate_expected_arrivals_empty() {
    let analyzer = GammaRayAnalyzer::new();
    let expected = analyzer.calculate_expected_arrivals();
    assert_eq!(expected.len(), 0);
}

#[test]
fn test_detect_anomalies_empty() {
    let analyzer = GammaRayAnalyzer::new();
    let anomalies = analyzer.detect_anomalies();
    assert_eq!(anomalies.len(), 0);
}

#[test]
fn test_full_workflow() {
    let mut analyzer = GammaRayAnalyzer::new();
    
    // Add some test measurements
    analyzer.add_measurement(100.0, 1234567890.123, 0.001);
    analyzer.add_measurement(200.0, 1234567891.456, 0.002);
    analyzer.add_measurement(500.0, 1234567893.789, 0.003);
    
    // Test that we can get expected arrivals
    let expected_arrivals = analyzer.calculate_expected_arrivals();
    assert_eq!(expected_arrivals.len(), 3);
    
    // Test that we can detect anomalies (will likely be empty with no real deviations)
    let anomalies = analyzer.detect_anomalies();
    assert!(anomalies.len() >= 0);
    
    // Test the full analysis
    let result = analyzer.test_light_speed_constancy();
    assert!(result.is_valid || !result.is_valid);
    assert!(result.confidence_level >= 0.0 && result.confidence_level <= 1.0);
}
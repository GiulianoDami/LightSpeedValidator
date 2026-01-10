// src/analyzer.rs
use std::collections::HashMap;

/// Represents a single timing measurement from gamma-ray detection
#[derive(Debug, Clone)]
pub struct TimingData {
    pub energy: f64,           // Energy in GeV
    pub arrival_time: f64,     // Arrival time in seconds since Unix epoch
    pub error: f64,            // Measurement error in seconds
}

/// Main analyzer for gamma-ray timing data
pub struct GammaRayAnalyzer {
    measurements: Vec<TimingData>,
    sensitivity_threshold: f64,
    quantum_gravity_model: Option<QuantumGravityModel>,
}

impl GammaRayAnalyzer {
    /// Creates a new analyzer with default settings
    pub fn new() -> Self {
        Self {
            measurements: Vec::new(),
            sensitivity_threshold: 1e-12,
            quantum_gravity_model: None,
        }
    }

    /// Adds a timing measurement to the dataset
    pub fn add_measurement(&mut self, energy: f64, arrival_time: f64, error: f64) {
        self.measurements.push(TimingData { energy, arrival_time, error });
    }

    /// Sets the sensitivity threshold for detecting deviations
    pub fn set_sensitivity_threshold(&mut self, threshold: f64) {
        self.sensitivity_threshold = threshold;
    }

    /// Sets up quantum gravity model simulation
    pub fn enable_quantum_gravity_simulation(&mut self, model: QuantumGravityModel) {
        self.quantum_gravity_model = Some(model);
    }

    /// Performs analysis to test light speed constancy hypothesis
    pub fn test_light_speed_constancy(&self) -> LightSpeedTestResult {
        if self.measurements.is_empty() {
            return LightSpeedTestResult {
                is_valid: true,
                confidence_level: 0.0,
                deviation_estimate: None,
                chi_squared: 0.0,
                p_value: 1.0,
                anomalies_detected: 0,
            };
        }

        // Calculate expected arrival times assuming c = 299792458 m/s
        let expected_times = self.calculate_expected_arrivals();
        
        // Calculate chi-squared statistic
        let mut chi_squared = 0.0;
        let mut total_weighted_deviation = 0.0;
        let mut total_weight = 0.0;
        
        for (i, measurement) in self.measurements.iter().enumerate() {
            if i < expected_times.len() {
                let expected_time = expected_times[i].1;
                let deviation = measurement.arrival_time - expected_time;
                let weight = 1.0 / (measurement.error * measurement.error);
                
                chi_squared += weight * deviation * deviation;
                total_weighted_deviation += weight * deviation;
                total_weight += weight;
            }
        }
        
        let degrees_of_freedom = self.measurements.len().saturating_sub(1);
        let p_value = if degrees_of_freedom > 0 {
            // Simplified p-value calculation using chi-squared distribution
            // In practice, this would use a proper statistical library
            let chi_sq = chi_squared;
            if chi_sq > 0.0 {
                // Approximate p-value (very simplified)
                1.0 - (-chi_sq / 2.0).exp()
            } else {
                1.0
            }
        } else {
            1.0
        };
        
        let confidence_level = 1.0 - p_value;
        
        // Estimate deviation
        let deviation_estimate = if total_weight > 0.0 {
            Some(total_weighted_deviation / total_weight)
        } else {
            None
        };
        
        let anomalies = self.detect_anomalies();
        
        LightSpeedTestResult {
            is_valid: p_value > 0.05, // 95% confidence level
            confidence_level,
            deviation_estimate,
            chi_squared,
            p_value,
            anomalies_detected: anomalies.len(),
        }
    }

    /// Calculates expected arrival times based on light speed assumption
    pub fn calculate_expected_arrivals(&self) -> Vec<(f64, f64)> {
        // Speed of light in vacuum (m/s)
        const SPEED_OF_LIGHT: f64 = 299792458.0;
        
        // For simplicity, assume all photons travel the same distance
        // In reality, this would depend on source distance and geometry
        let base_distance = 1.0; // arbitrary base distance in meters
        
        self.measurements
            .iter()
            .map(|measurement| {
                // Time delay due to energy-dependent propagation (if quantum gravity model is active)
                let time_delay = if let Some(ref model) = self.quantum_gravity_model {
                    // Simplified quantum gravity effect: delay proportional to energy^2
                    let energy_gev = measurement.energy;
                    model.quantum_gravity_effect * energy_gev * energy_gev / (SPEED_OF_LIGHT * SPEED_OF_LIGHT)
                } else {
                    0.0
                };
                
                // Expected arrival time (base + delay)
                let expected_time = measurement.arrival_time - time_delay;
                (measurement.energy, expected_time)
            })
            .collect()
    }

    /// Detects timing anomalies in the data
    pub fn detect_anomalies(&self) -> Vec<AnomalyDetectionResult> {
        if self.measurements.is_empty() {
            return vec![];
        }
        
        let expected_times = self.calculate_expected_arrivals();
        let mut anomalies = Vec::new();
        
        for (i, measurement) in self.measurements.iter().enumerate() {
            if i < expected_times.len() {
                let expected_time = expected_times[i].1;
                let deviation = measurement.arrival_time - expected_time;
                let significance = deviation.abs() / measurement.error;
                
                if significance > 3.0 { // 3 sigma threshold
                    anomalies.push(AnomalyDetectionResult {
                        energy: measurement.energy,
                        measured_time: measurement.arrival_time,
                        expected_time,
                        deviation,
                        significance,
                    });
                }
            }
        }
        
        anomalies
    }
}

/// Quantum gravity model parameters for photon propagation simulations
#[derive(Debug, Clone)]
pub struct QuantumGravityModel {
    pub planck_length: f64,
    pub quantum_gravity_effect: f64,
    pub energy_scale: f64,
}

/// Result of light speed constancy test
#[derive(Debug, Clone)]
pub struct LightSpeedTestResult {
    pub is_valid: bool,
    pub confidence_level: f64,
    pub deviation_estimate: Option<f64>,
    pub chi_squared: f64,
    pub p_value: f64,
    pub anomalies_detected: usize,
}

/// Anomaly detection result
#[derive(Debug, Clone)]
pub struct AnomalyDetectionResult {
    pub energy: f64,
    pub measured_time: f64,
    pub expected_time: f64,
    pub deviation: f64,
    pub significance: f64,
}
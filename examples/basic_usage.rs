use lightspeedvalidator::{GammaRayAnalyzer, TimingData};

fn main() {
    // Create analyzer instance
    let mut analyzer = GammaRayAnalyzer::new();
    
    // Add some sample timing measurements (energy, arrival_time, error)
    analyzer.add_measurement(100.0, 1234567890.123, 0.001);
    analyzer.add_measurement(200.0, 1234567891.456, 0.002);
    analyzer.add_measurement(500.0, 1234567893.789, 0.003);
    
    // Perform statistical analysis for speed of light deviation tests
    let result = analyzer.test_light_speed_constancy();
    
    println!("Analysis result: {:?}", result);
}
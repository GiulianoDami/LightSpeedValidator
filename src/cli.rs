use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// Command-line interface for LightSpeedValidator
#[derive(Parser)]
#[command(name = "lightspeedvalidator")]
#[command(version = "0.1.0")]
#[command(author = "Scientific Computing Team")]
#[command(about = "Analyze gamma-ray timing data to test light speed constancy", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Analyze gamma-ray timing data
    Analyze(AnalyzeArgs),
}

#[derive(clap::Args)]
pub struct AnalyzeArgs {
    /// Input data file path
    #[arg(short, long)]
    pub input: PathBuf,

    /// Output results file path
    #[arg(short, long)]
    pub output: Option<PathBuf>,

    /// Sensitivity threshold for detecting deviations
    #[arg(long, default_value = "1e-12")]
    pub sensitivity: f64,

    /// Enable quantum gravity simulation
    #[arg(long)]
    pub quantum_gravity: bool,

    /// Verbose output
    #[arg(short, long)]
    pub verbose: bool,
}

/// Main CLI executor
pub struct CliExecutor;

impl CliExecutor {
    /// Execute the command-line application
    pub fn execute(args: AnalyzeArgs) -> Result<(), Box<dyn std::error::Error>> {
        use crate::analyzer::{GammaRayAnalyzer, TimingData};
        use crate::data::{CsvImporter, DataImporter};
        use crate::statistics::SpeedOfLightHypothesisTest;
        
        // Load data from file
        let importer = CsvImporter;
        let measurements = importer.load_from_file(&args.input)?;
        
        // Create analyzer
        let mut analyzer = GammaRayAnalyzer::new();
        for measurement in measurements {
            analyzer.add_measurement(measurement.energy, measurement.arrival_time, measurement.error);
        }
        
        // Set sensitivity
        analyzer.set_sensitivity_threshold(args.sensitivity);
        
        // Enable quantum gravity if requested
        if args.quantum_gravity {
            let model = crate::analyzer::QuantumGravityModel {
                planck_length: 1.616e-35,
                quantum_gravity_effect: 1e-20,
                energy_scale: 1e19,
            };
            analyzer.enable_quantum_gravity_simulation(model);
        }
        
        // Run analysis
        let result = analyzer.test_light_speed_constancy();
        
        // Output results
        if let Some(output_path) = args.output {
            use crate::data::{JsonExporter, DataExporter};
            let exporter = JsonExporter;
            exporter.save_to_file(&analyzer.measurements, &output_path)?;
        }
        
        if args.verbose {
            println!("Analysis complete:");
            println!("  - Is valid: {}", result.is_valid);
            println!("  - Confidence level: {:.6}", result.confidence_level);
            println!("  - Chi-squared: {:.6}", result.chi_squared);
            println!("  - P-value: {:.2e}", result.p_value);
            println!("  - Anomalies detected: {}", result.anomalies_detected);
        }
        
        Ok(())
    }
}
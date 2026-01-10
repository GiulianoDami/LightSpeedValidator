//! LightSpeedValidator: A Rust-based tool for analyzing gamma-ray timing data 
//! to test Einstein's speed of light constancy hypothesis.

pub mod analyzer;
pub mod cli;
pub mod data;
pub mod statistics;

// Re-export key types and functions for easy access
pub use analyzer::{GammaRayAnalyzer, TimingData};
pub use statistics::{LightSpeedTestResult, SpeedOfLightHypothesisTest};
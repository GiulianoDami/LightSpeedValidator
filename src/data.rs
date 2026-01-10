use std::path::Path;
use csv::Reader;
use crate::analyzer::TimingData;

/// Supported input formats for gamma-ray data
#[derive(Debug, Clone)]
pub enum DataFormat {
    Csv,
    Json,
    Ascii,
}

/// Trait for data importers
pub trait DataImporter {
    fn load_from_file(&self, path: &Path) -> Result<Vec<TimingData>, Box<dyn std::error::Error>>;
}

/// CSV data importer
pub struct CsvImporter;

impl DataImporter for CsvImporter {
    fn load_from_file(&self, path: &Path) -> Result<Vec<TimingData>, Box<dyn std::error::Error>> {
        let mut reader = Reader::from_path(path)?;
        let mut data = Vec::new();
        
        for result in reader.records() {
            let record = result?;
            let energy: f64 = record.get(0).unwrap_or("0").parse()?;
            let arrival_time: f64 = record.get(1).unwrap_or("0").parse()?;
            let error: f64 = record.get(2).unwrap_or("0").parse()?;
            
            data.push(TimingData { energy, arrival_time, error });
        }
        
        Ok(data)
    }
}

/// Trait for data exporters
pub trait DataExporter {
    fn save_to_file(&self, data: &[TimingData], path: &Path) -> Result<(), Box<dyn std::error::Error>>;
}

/// JSON data exporter
pub struct JsonExporter;

impl DataExporter for JsonExporter {
    fn save_to_file(&self, data: &[TimingData], path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let json_data = serde_json::to_string_pretty(&data)?;
        std::fs::write(path, json_data)?;
        Ok(())
    }
}
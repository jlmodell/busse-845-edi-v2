use std::{path::Path};
use serde::{de::DeserializeOwned, Deserialize};

#[derive(Debug, Deserialize)]
pub struct ContractData {
    pub part: String,
    pub price: f32,
    pub description: String,    
    pub start: String,
    pub end: String,
    pub purpose: String,
}

#[derive(Debug, Deserialize)]
pub struct EndBuyerData {
    pub name: String,
    pub id: String,
    pub address: String,
    pub city: String,
    pub state: String,
    pub zipcode: String,
    pub change: String,
    pub start: String,
    pub end: String,
}

pub fn parse_csv<T>(path: &Path) -> Result<Vec<T>, Box<dyn std::error::Error>> where
    T: DeserializeOwned {
    let mut reader = csv::Reader::from_path(path)?;
    
    let records: Vec<T> = reader
        .deserialize()
        .map(|record| {
            record.expect("Unable to parse CSV")
        })
        .collect();

    Ok(records)
}

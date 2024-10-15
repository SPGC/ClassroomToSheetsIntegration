use serde::{Deserialize, Serialize};
use base64::decode;
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct TestResult {
    pub name: String,
    pub status: String,
    pub score: Option<f64>,
    pub test_code: Option<String>,
    pub filename: Option<String>,
    pub line_no: Option<u32>,
    pub duration: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TestResults {
    pub version: u8,
    pub status: String,
    pub max_score: Option<f64>,
    pub tests: Vec<TestResult>,
}

pub fn parse_results(base64_encoded: &str) -> Result<TestResults, Box<dyn Error>> {
    // Декодирование Base64
    let decoded_bytes = decode(base64_encoded)?;
    let decoded_str = String::from_utf8(decoded_bytes)?;

    // Парсинг JSON
    let test_results: TestResults = serde_json::from_str(&decoded_str)?;
    Ok(test_results)
}

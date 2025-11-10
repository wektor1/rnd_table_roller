use serde::Deserialize;
use serde_json;
use std::error::Error;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct TableItem {
    pub range: Vec<u32>,
    pub value: String,
}

#[derive(Debug, Deserialize)]
pub struct RollTable {
    pub name: String,
    pub description: String,
    pub roll_type: String,
    pub outcome_prefix: String,
    pub items: Vec<TableItem>,
}

pub fn read_from_file<P: AsRef<Path>>(path: P) -> Result<RollTable, Box<dyn Error>> {
    let file_string: String = fs::read_to_string(path)?;
    let json: RollTable =
        serde_json::from_str(file_string.as_str()).expect("JSON was not well-formatted");
    Ok(json)
}

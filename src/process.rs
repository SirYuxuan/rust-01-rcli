use std::fs;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Player {
    name: String,
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit: u8,
}

/**
 * Process the CSV file
 */
pub fn process_csv(input: &str,output:&str) -> anyhow::Result<()> {
    let mut reader = csv::Reader::from_path(input)?;
    let mut ret = Vec::with_capacity(128);
    for result in reader.deserialize() {
        let record: Player = result?;
        ret.push(record);
    }
    fs::write(output, serde_json::to_string_pretty(&ret)?)?;
    Ok(())
}

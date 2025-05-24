use crate::model::EepromMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub fn load_yaml<P: AsRef<Path>>(path: P) -> Result<EepromMap, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let map: EepromMap = serde_yaml::from_reader(reader)?;
    Ok(map)
}
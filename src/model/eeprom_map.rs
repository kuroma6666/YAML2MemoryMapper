use crate::model::{Endianness, Entry};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EepromMap {
    #[allow(dead_code)]
    pub version: u8,
    #[allow(dead_code)]
    pub base_address: u16,
    #[allow(dead_code)]
    pub endianness: Endianness,
    pub entries: Vec<Entry>,
    #[serde(default)]
    pub types: HashMap<String, Vec<Entry>>, // Custom(String) サポート
}

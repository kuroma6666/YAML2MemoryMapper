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
    pub types: HashMap<String, Vec<Entry>>, // for Custom(String) support
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Endianness {
    Little,
    Big,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct Entry {
    pub name: String,
    pub offset: u16,
    #[serde(rename = "type")]
    pub ty: Type,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Type {
    Uint8,
    Uint16,
    Uint32,
    Struct(Vec<Entry>),
    Custom(String),
}
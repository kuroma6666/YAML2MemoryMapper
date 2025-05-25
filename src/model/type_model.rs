use crate::model::Entry;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Type {
    CustomCandidate(String),
    Uint8,
    Uint16,
    Uint32,
    #[serde(rename_all = "snake_case")]
    StructWrapper {
        r#struct: Vec<Entry>,
    },
    #[serde(rename = "custom")]
    Custom(String),
}

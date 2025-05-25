use crate::model::type_model::Type;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct Entry {
    pub name: String,
    pub offset: u16,
    #[serde(rename = "type")]
    pub ty: Type,
    pub description: Option<String>,
}

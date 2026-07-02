pub mod base64;
pub mod yaml;
pub mod json;
pub mod txt;
pub mod csv;

use serde::{Deserialize, Serialize};
use smartstring::alias::String;
use std::collections::HashMap;

/// 解析后的节点
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ParsedNode {
    pub name: String,
    pub protocol: String,
    pub address: String,
    pub port: u16,
    pub uri: String,
    #[serde(default)]
    pub extra: HashMap<String, String>,
}

use crate::{blocks::Block, connections::Connection};
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub blocks: Vec<Block>,
    pub connections: Vec<Connection>,
}

impl Project {
    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string(self)
    }

    pub fn from_json(json: &str) -> Result<Self> {
        serde_json::from_str(json)
    }

    pub async fn save_to_file(&self, path: &Path) -> std::io::Result<()> {
        tokio::fs::write(path, self.to_json().unwrap()).await
    }

    pub async fn load_from_file(path: &Path) -> std::io::Result<Self> {
        let contents = tokio::fs::read_to_string(path).await?;
        Ok(serde_json::from_str(&contents)?)
    }
}

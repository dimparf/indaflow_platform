use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum BlockType {
    #[default]
    Undefined,
    Click,
    MouseMove,
    Screenshot,
    GetCoordinates,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Block {
    pub id: String,
    pub block_type: BlockType,
    pub position: (f32, f32),
    pub properties: HashMap<String, String>,
}

impl Block {
    pub fn new(block_type: BlockType, position: (f32, f32)) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            block_type,
            position,
            properties: HashMap::new(),
        }
    }

    // Методы для создания блоков разных типов
    pub fn new_click() -> Self {
        Self::new(BlockType::Click, (0.0, 0.0))
    }

    pub fn new_mouse_move() -> Self {
        Self::new(BlockType::MouseMove, (0.0, 0.0))
    }

    pub fn new_screenshot() -> Self {
        Self::new(BlockType::Screenshot, (0.0, 0.0))
    }

    pub fn new_coordinates() -> Self {
        Self::new(BlockType::GetCoordinates, (0.0, 0.0))
    }
}

use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct MoveSpeed {
    pub speed: f32,
}

impl MoveSpeed {
    pub fn new(speed: f32) -> Self {
        Self { speed }
    }
}

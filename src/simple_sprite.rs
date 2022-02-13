use super::*;

/// Bounds [`f32::NEG_INFINITY`] or [`f32::INFINITY`] or in range [-1; 1]
type Position = f32;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SpriteCommand {
    None,
    Set {
        sprite: String,
        name: String,
        position: Position,
    },
    Remove {
        name: String,
    },
    FadeIn {
        sprite: String,
        name: String,
        position: Position,
    },
    FadeOut {
        name: String,
    },
    LeftIn {
        sprite: String,
        name: String,
        position: Position,
    },
    LeftOut {
        name: String,
    },
    RightIn {
        sprite: String,
        name: String,
        position: Position,
    },
    RightOut {
        name: String,
    },
    Move {
        name: String,
        position: Position,
    },
}

impl Default for SpriteCommand {
    fn default() -> Self {
        Self::None
    }
}

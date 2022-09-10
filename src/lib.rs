#![allow(dead_code)]

mod simple_sprite;
mod condition;
mod inspect;
pub mod bevy_glue;

use std::collections::HashMap;
use std::fmt::{Debug};
use std::hash::Hasher;
use uuid::Uuid;
use serde::{Serialize, Deserialize};

pub use simple_sprite::*;
pub use condition::*;
use crate::inspect::Inspector;

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct AerugoState {
    pub current: Uuid,
    pub select_story: Vec<(Uuid, String)>,
    pub inspector: Inspector,
    _pre_collected: Option<Vec<Steps>>,
}

impl AerugoState {
    pub fn new(aerugo: &Aerugo) -> Self {
        Self {
            current: aerugo.steps.first().unwrap().id,
            select_story: vec![],
            inspector: Default::default(),
            _pre_collected: None,
        }
    }

    fn apply_jump(
        current: &mut Uuid,
        select_story: &Vec<(Uuid, String)>,
        condition: &Option<Condition>,
        target: &Uuid,
    ) -> bool
    {
        let condition = match condition {
            None => { true }
            Some(c) => { c.resolve(&select_story) }
        };
        if condition {
            *current = *target;
        }
        condition
    }

    fn find_next(current: Uuid, aerugo: &Aerugo) -> Uuid {
        let current_pos = aerugo.steps.iter()
            .position(|s| { s.id == current })
            .unwrap();
        aerugo.steps.get(current_pos + 1).unwrap().id
    }

    pub fn setup(aerugo: &Aerugo) -> AerugoState {
        let mut current = aerugo.steps.get(0).unwrap().id;
        let select_story = Default::default();
        loop {
            let step = aerugo.steps.iter()
                .find(|s| { s.id == current })
                .unwrap();
            match &step.inner {
                Steps::Jump { condition, target } => {
                    if Self::apply_jump(&mut current, &select_story, condition, target) {
                        continue;
                    }
                }
                Steps::None => {}
                _ => { break; }
            }

            current = Self::find_next(current, &aerugo);
        }
        AerugoState { current, select_story, inspector: Default::default(), _pre_collected: None }
    }

    pub fn step(&self, aerugo: &Aerugo) -> Step {
        aerugo.steps.iter().find(|s| { s.id == self.current }).unwrap().clone()
    }

    pub fn next(&mut self, aerugo: &Aerugo) -> Option<()> {
        let current = aerugo.steps
            .iter()
            .position(|s| { s.id == self.current })
            .unwrap();
        match aerugo.steps.get(current + 1) {
            None => { None }
            Some(next) => {
                self.current = next.id;
                Some(())
            }
        }
    }

    // collect graphic commands steps
    pub fn collect(&mut self, aerugo: &Aerugo) -> Vec<Steps> {
        if let Some(collected) = self._pre_collected.take() {
            return collected;
        }

        let mut steps: Vec<Steps> = Default::default();
        loop {
            let step = self.step(&aerugo);
            match &step.inner {
                // region user await steps
                Steps::Text { .. }
                | Steps::Phrase { .. }
                | Steps::ImageSelect { .. } => { break; }
                // endregion
                Steps::Jump { condition, target } => {
                    if Self::apply_jump(&mut self.current, &self.select_story, condition, target) {
                        continue;
                    }
                }
                // region graphic commands steps
                Steps::SpriteNarrator { .. }
                | Steps::Sprite { .. }
                | Steps::Background { .. }
                | Steps::Scene { .. } => {
                    steps.push(step.inner);
                }
                // endregion
                _ => {}
            }

            self.current = Self::find_next(self.current, &aerugo);
        }
        self.inspector.keep(&steps);
        steps
    }

    pub fn select_unique(&mut self, step: Uuid, value: String) {
        let exist = self.select_story
            .iter()
            .rposition(|(s, _)| {
                *s == step
            });
        match exist {
            None => {
                self.select_story.push((step, value));
            }
            Some(i) => {
                self.select_story[i] = (step, value);
            }
        }
    }
}

impl AerugoState {
    pub fn save(&self) -> String {
        ron::to_string(self).unwrap()
    }

    pub fn validate(aerugo: &Aerugo, state: &AerugoState) -> Option<()> {
        if aerugo.steps.iter().find(|s| { s.id == state.current }).is_none() {
            return None;
        }
        Some(())
    }

    pub fn reload(mut self) -> Self {
        self._pre_collected = Some(self.inspector.extract());
        self
    }
}

impl Default for AerugoState {
    fn default() -> Self {
        AerugoState {
            current: Default::default(),
            select_story: vec![],
            inspector: Default::default(),
            _pre_collected: None,
        }
    }
}


#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct Aerugo {
    pub steps: Vec<Step>,
}

impl Default for Aerugo {
    fn default() -> Self {
        Aerugo {
            steps: vec![Step::default()],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Steps {
    Text {
        author: String,
        texts: String,
    },
    Jump {
        condition: Option<Condition>,
        target: Uuid,
    },
    Phrase {
        phrases: Vec<(String, String)>,
    },
    ImageSelect {
        background: String,
        /// (sprite, (x, y))
        options: HashMap<String, (String, (isize, isize))>,
    },
    SpriteNarrator(NarratorCommand),
    Sprite(SpriteCommand),
    Background(BackgroundCommand),
    Scene(SceneCommand),
    None,
}

impl std::hash::Hash for Steps {
    fn hash<H: Hasher>(&self, state: &mut H) {
        ron::to_string(self).unwrap().hash(state)
    }
}

#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
pub struct Step {
    pub id: Uuid,
    pub name: String,
    pub inner: Steps,
}

impl Default for Step {
    fn default() -> Self {
        Step {
            id: Uuid::nil(),
            name: "".to_string(),
            inner: Steps::None,
        }
    }
}

impl Step {
    pub fn new() -> Step {
        Step {
            id: Uuid::new_v4(),
            name: "".to_string(),
            inner: Steps::None,
        }
    }

    pub fn with_inner(mut self, inner: Steps) -> Step {
        self.inner = inner;
        self
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub enum NarratorCommand {
    Set {
        name: String,
        sprite: String,
    },
    Remove {
        name: String,
    },
    Clean,
    #[default]
    None,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub enum CommonAnimation {
    // -1|-----0-----|1
    FadeIn(f32),
    FadeOut,
    LeftIn(f32),
    LeftOut,
    RightIn(f32),
    RightOut,
    Jump,
    Move(f32),
    #[default]
    None,
}

impl std::hash::Hash for CommonAnimation {
    fn hash<H: Hasher>(&self, state: &mut H) {
        ron::to_string(self).unwrap().hash(state)
    }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub enum BackgroundCommand {
    Change {
        new: String,
        animation: Option<CommonAnimation>,
    },
    Shake,
    None,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum SceneCommand {
    Set {
        /// sprite
        name: String,
    },
    Remove,
    Play {
        /// sprite_sheet
        name: String,
        is_loop: bool,
        tile: (usize, usize),
        columns: usize,
        rows: usize,
    },
    Pause,
    Resume,
    Stop,
    None,
}

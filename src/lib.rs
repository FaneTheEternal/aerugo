#![allow(dead_code)]

mod simple_sprite;

use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::hash::Hasher;
use uuid::Uuid;
use serde::{Serialize, Deserialize};

pub use simple_sprite::*;

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub struct AerugoState {
    pub current: Uuid,
    pub select_story: Vec<(Uuid, String)>,
}

impl AerugoState {
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
        AerugoState { current, select_story }
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

impl Aerugo {
    pub fn remove(&mut self, step: &Step) -> &Step {
        let mut index = self.steps
            .iter()
            .position(|s| { s.id == step.id })
            .unwrap();
        self.steps.remove(index);
        if self.steps.is_empty() {
            self.steps.push(Step::new());
        }
        index = if index > 0 { index - 1 } else { 0 };
        self.steps.get(index).unwrap()
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
    SpriteNarrator {
        /// None -> cleanup any active
        sprite: Option<String>,
    },
    Sprite(SpriteCommand),
    Background {
        command: BackgroundCommand,
    },
    Scene {
        command: SceneCommand,
    },
    None,
}

impl std::hash::Hash for Steps {
    fn hash<H: Hasher>(&self, state: &mut H) {
        serde_json::to_string(self).unwrap().hash(state)
    }
}

#[derive(Clone, Hash, Serialize, Deserialize)]
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

impl Debug for Step {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.name.is_empty() {
            write!(f, "{}", self.id)
        } else {
            write!(f, "{} | {}", self.id, self.name)
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

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum Condition {
    True,
    False,
    Check {
        step: Uuid,
        val: String,
    },
    Not(Box<Condition>),
    And(Box<Condition>, Box<Condition>),
    Or(Box<Condition>, Box<Condition>),
    GTE(Vec<Condition>, usize),
    LTE(Vec<Condition>, usize),
}

impl Default for Condition {
    fn default() -> Self {
        Condition::True
    }
}

impl Condition {
    fn resolve(&self, select_story: &Vec<(Uuid, String)>) -> bool {
        match self {
            Condition::True => { true }
            Condition::False => { false }
            Condition::Check { step, val } => {
                select_story.iter().find(|(s, v)| {
                    s == step && v == val
                }).is_some()
            }
            Condition::Not(c) => {
                !c.resolve(select_story)
            }
            Condition::And(l, r) => {
                l.resolve(select_story) && r.resolve(select_story)
            }
            Condition::Or(l, r) => {
                l.resolve(select_story) || r.resolve(select_story)
            }
            Condition::GTE(conditions, count) => {
                conditions.iter().filter(|c| { c.resolve(select_story) }).count() >= *count
            }
            Condition::LTE(conditions, count) => {
                conditions.iter().filter(|c| { c.resolve(select_story) }).count() <= *count
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    None,
}

impl std::hash::Hash for CommonAnimation {
    fn hash<H: Hasher>(&self, state: &mut H) {
        serde_json::to_string(self).unwrap().hash(state)
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
    },
    Pause,
    None,
}

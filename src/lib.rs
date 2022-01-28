#![allow(dead_code)]

use std::fmt::{Debug, Formatter};
use uuid::Uuid;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct AerugoState {
    pub current: Uuid,
    pub select_story: Vec<(Uuid, String)>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum Steps {
    Text {
        author: String,
        texts: String,
    },
    Jump {
        condition: Option<Condition>,
        target: Uuid,
    },
    None,
}

#[derive(Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
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

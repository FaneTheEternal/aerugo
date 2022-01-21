#![allow(dead_code)]

use bevy::utils::Uuid;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct AerugoSave {
    current: Uuid,
    select_story: Vec<(Uuid, String)>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Aerugo {
    steps: Vec<Step>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum Steps {
    Text {
        author: String,
        texts: Vec<String>,
    },
    Jump {
        condition: Option<Condition>,
        target: Uuid,
    },
    None,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Step {
    id: Uuid,
    name: String,
    inner: Steps,
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

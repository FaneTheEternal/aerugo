use std::collections::HashMap;
use super::*;


/// Keep in mind info about all steps, exclude common
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Inspector {
    /// name: (path, position)
    pub sprites: HashMap<String, (String, f32)>,
    /// name: sprite
    pub narrator: HashMap<String, String>,
    pub background: Option<String>,
    pub scene: Option<SceneCommand>,
}

impl Inspector {
    pub fn keep(&mut self, steps: &[Steps]) {
        for step in steps {
            match step {
                Steps::SpriteNarrator(cmd) => {
                    match cmd {
                        NarratorCommand::Clean => {
                            self.narrator.clear();
                        }
                        NarratorCommand::Set { name, sprite } => {
                            self.narrator.insert(name.clone(), sprite.clone());
                        }
                        NarratorCommand::Remove { name } => {
                            self.narrator.remove(name);
                        }
                        NarratorCommand::None => {}
                    }
                }
                Steps::Sprite(cmd) => {
                    match cmd {
                        SpriteCommand::None => {}
                        SpriteCommand::Set { sprite, name, position } => {
                            self.sprites.insert(name.clone(), (sprite.clone(), *position));
                        }
                        SpriteCommand::Remove { name } => {
                            self.sprites.remove(name);
                        }
                        SpriteCommand::FadeIn { sprite, name, position } => {
                            self.sprites.insert(name.clone(), (sprite.clone(), *position));
                        }
                        SpriteCommand::FadeOut { name } => {
                            self.sprites.remove(name);
                        }
                        SpriteCommand::LeftIn { sprite, name, position } => {
                            self.sprites.insert(name.clone(), (sprite.clone(), *position));
                        }
                        SpriteCommand::LeftOut { name } => {
                            self.sprites.remove(name);
                        }
                        SpriteCommand::RightIn { sprite, name, position } => {
                            self.sprites.insert(name.clone(), (sprite.clone(), *position));
                        }
                        SpriteCommand::RightOut { name } => {
                            self.sprites.remove(name);
                        }
                        SpriteCommand::Move { name, position } => {
                            if let Some((sprite, ..)) = self.sprites.remove(name) {
                                self.sprites.insert(name.clone(), (sprite, *position));
                            }
                        }
                    }
                }
                Steps::Background(cmd) => {
                    match cmd {
                        BackgroundCommand::Change { new, .. } => {
                            self.background = Some(new.clone());
                        }
                        BackgroundCommand::Shake => {}
                        BackgroundCommand::None => {}
                    }
                }
                Steps::Scene(cmd) => {
                    self.scene = Some(cmd.clone());
                }
                _ => {}
            }
        }
    }

    pub fn extract(&self) -> Vec<Steps> {
        let mut data: Vec<Steps> = Default::default();
        for (name, (sprite, position)) in &self.sprites {
            data.push(Steps::Sprite(SpriteCommand::Set {
                sprite: sprite.clone(),
                name: name.clone(),
                position: *position,
            }));
        }
        for (name, sprite) in &self.narrator {
            data.push(
                Steps::SpriteNarrator(
                    NarratorCommand::Set {
                        name: name.clone(),
                        sprite: sprite.clone()
                    }
                )
            );
        }
        data.push(match &self.background {
            None => { Steps::Background(BackgroundCommand::None) }
            Some(sprite) => {
                Steps::Background(BackgroundCommand::Change {
                    new: sprite.clone(),
                    animation: None,
                })
            }
        });
        data.push(match &self.scene {
            None => { Steps::Scene(SceneCommand::Remove) }
            Some(cmd) => { Steps::Scene(cmd.clone()) }
        });
        data
    }
}

impl std::hash::Hash for Inspector {
    fn hash<H: Hasher>(&self, state: &mut H) {
        ron::to_string(self).unwrap().hash(state)
    }
}

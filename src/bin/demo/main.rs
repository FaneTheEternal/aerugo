use std::io::Write;
use uuid::Uuid;
use aerugo::*;


macro_rules! str {
    ($s:expr) => {format!("{}", $s)};
}

const SCENARIO_PATH: &str = "scenario.ron";

fn main() {
    let story_flow = Uuid::new_v4();
    let mysterious_stranger = Uuid::new_v4();
    let nps_monologue = Uuid::new_v4();
    let phrase_jump_test = Uuid::new_v4();

    let aerugo = Aerugo {
        steps: vec![
            Step::default(),
            // region simple texts test
            Step {
                id: story_flow,
                name: str!("story flow"),
                inner: Steps::Text {
                    author: str!(""),
                    texts: str!("Story flow"),
                },
            },
            Step {
                id: mysterious_stranger,
                name: str!("mysterious stranger"),
                inner: Steps::Text {
                    author: str!("??? / <?>"),
                    texts: str!("Mysterious stranger"),
                },
            },
            Step {
                id: nps_monologue,
                name: str!("NPS monologue"),
                inner: Steps::Text {
                    author: str!("NPS name"),
                    texts: str!("NPS monologue"),
                },
            },
            // endregion

            // region jump by phrase test
            Step {
                id: phrase_jump_test,
                name: str!("phrase jump test"),
                inner: Steps::Phrase {
                    phrases: vec![
                        (str!("story_flow"), str!("Back to story flow")),
                        (str!("mysterious_stranger"), str!("Back to mysterious stranger")),
                        (str!("nps_monologue"), str!("Back to nps monologue")),
                        (str!("pass"), str!("Pass")),
                    ]
                },
            },
            Step {
                id: Uuid::new_v4(),
                name: str!(""),
                inner: Steps::Jump {
                    condition: Some(Condition::Check { step: phrase_jump_test, val: str!("story_flow") }),
                    target: story_flow,
                },
            },
            Step {
                id: Uuid::new_v4(),
                name: str!(""),
                inner: Steps::Jump {
                    condition: Some(Condition::Check { step: phrase_jump_test, val: str!("mysterious_stranger") }),
                    target: mysterious_stranger,
                },
            },
            Step {
                id: Uuid::new_v4(),
                name: str!(""),
                inner: Steps::Jump {
                    condition: Some(Condition::Check { step: phrase_jump_test, val: str!("nps_monologue") }),
                    target: nps_monologue,
                },
            },
            // endregion

            // region image select test
            // TODO: ImageSelect
            // endregion

            // region sprite narrator test
            Step {
                id: Uuid::new_v4(),
                name: "".to_string(),
                inner: Steps::Text { author: str!(""), texts: str!("Try see narrator sprite_step") },
            },
            Step {
                id: Uuid::new_v4(),
                name: "".to_string(),
                inner: Steps::SpriteNarrator { sprite: Some(str!("textures/char/female_sprite.png")) },
            },
            Step {
                id: Uuid::new_v4(),
                name: "".to_string(),
                inner: Steps::Text { author: str!("Sprite narrator"), texts: str!("Hi!") },
            },
            Step {
                id: Uuid::new_v4(),
                name: "".to_string(),
                inner: Steps::SpriteNarrator { sprite: None },
            },
            Step {
                id: Uuid::new_v4(),
                name: "".to_string(),
                inner: Steps::Text { author: str!(""), texts: str!("Hide them") },
            },
            // endregion

            // region sprite test
            Step {
                id: Uuid::new_v4(),
                name: "".to_string(),
                inner: Steps::Text { author: "".to_string(), texts: str!("Sprite test") },
            },
            Step {
                id: Uuid::new_v4(),
                name: "".to_string(),
                inner: Steps::Sprite(SpriteCommand::FadeIn {
                    sprite: str!("textures/char/female_sprite.png"),
                    name: str!("FadeChan"),
                    position: -0.2,
                }),
            },
            Step {
                id: Uuid::new_v4(),
                name: "".to_string(),
                inner: Steps::Text { author: "".to_string(), texts: str!("Fade in") },
            },
            Step {
                id: Uuid::new_v4(),
                name: "".to_string(),
                inner: Steps::Sprite(SpriteCommand::FadeOut { name: str!("FadeChan") }),
            },
            Step {
                id: Uuid::new_v4(),
                name: "".to_string(),
                inner: Steps::Text { author: "".to_string(), texts: str!("Fade out") },
            },
            Step {
                id: Uuid::new_v4(),
                name: "".to_string(),
                inner: Steps::Sprite(SpriteCommand::LeftIn {
                    sprite: str!("textures/char/female_sprite.png"),
                    name: str!("LeftChan"),
                    position: -0.35,
                }),
            },
            Step {
                id: Uuid::new_v4(),
                name: "".to_string(),
                inner: Steps::Text { author: "".to_string(), texts: str!("Left in") },
            },
            Step {
                id: Uuid::new_v4(),
                name: "".to_string(),
                inner: Steps::Sprite(SpriteCommand::LeftOut { name: str!("LeftChan") }),
            },
            Step {
                id: Uuid::new_v4(),
                name: "".to_string(),
                inner: Steps::Text { author: "".to_string(), texts: str!("Left out") },
            },
            Step {
                id: Uuid::new_v4(),
                name: "".to_string(),
                inner: Steps::Sprite(SpriteCommand::RightIn {
                    sprite: str!("textures/char/female_sprite.png"),
                    name: str!("RightChan"),
                    position: 0.35,
                }),
            },
            Step {
                id: Uuid::new_v4(),
                name: "".to_string(),
                inner: Steps::Text { author: "".to_string(), texts: str!("Right in") },
            },
            Step {
                id: Uuid::new_v4(),
                name: "".to_string(),
                inner: Steps::Sprite(SpriteCommand::RightOut { name: str!("RightChan") }),
            },
            Step {
                id: Uuid::new_v4(),
                name: "".to_string(),
                inner: Steps::Text { author: "".to_string(), texts: str!("Right out") },
            },
            Step {
                id: Uuid::new_v4(),
                name: "".to_string(),
                inner: Steps::Sprite(SpriteCommand::FadeIn {
                    sprite: str!("textures/char/female_sprite.png"),
                    name: str!("MoveChan"),
                    position: 0.7,
                }),
            },
            Step {
                id: Uuid::new_v4(),
                name: "".to_string(),
                inner: Steps::Text { author: "".to_string(), texts: str!("Move prepare") },
            },
            Step {
                id: Uuid::new_v4(),
                name: "".to_string(),
                inner: Steps::Sprite(SpriteCommand::Move { name: str!("MoveChan"), position: -0.7 }),
            },
            Step {
                id: Uuid::new_v4(),
                name: "".to_string(),
                inner: Steps::Text { author: "".to_string(), texts: str!("Wo-o-o-o") },
            },
            Step {
                id: Uuid::new_v4(),
                name: "".to_string(),
                inner: Steps::Sprite(SpriteCommand::FadeOut { name: str!("MoveChan") }),
            },
            Step {
                id: Uuid::new_v4(),
                name: "".to_string(),
                inner: Steps::Text { author: "".to_string(), texts: str!("Leave...") },
            },
            // endregion

            // region background test
            Step {
                id: Uuid::new_v4(),
                name: "".to_string(),
                inner: Steps::Text { author: str!(""), texts: str!("Try set background") },
            },
            Step {
                id: Uuid::new_v4(),
                name: "".to_string(),
                inner: Steps::Background(BackgroundCommand::Change {
                    new: str!("textures/background/pexels-francesco-ungaro.jpg"),
                    animation: None,
                }),
            },
            Step {
                id: Uuid::new_v4(),
                name: "".to_string(),
                inner: Steps::Text { author: str!(""), texts: str!("Looks good") },
            },
            // endregion

            // region scene test
            Step {
                id: Uuid::new_v4(),
                name: "".to_string(),
                inner: Steps::Text { author: str!(""), texts: str!("Try to do simple scene test") },
            },
            Step {
                id: Uuid::new_v4(),
                name: "".to_string(),
                inner: Steps::Scene(SceneCommand::Set { name: str!("textures/scene/simple_scene_test.png") }),
            },
            Step {
                id: Uuid::new_v4(),
                name: "".to_string(),
                inner: Steps::Text { author: str!(""), texts: str!("Looks good") },
            },
            Step {
                id: Uuid::new_v4(),
                name: "".to_string(),
                inner: Steps::Scene(SceneCommand::Remove),
            },
            Step {
                id: Uuid::new_v4(),
                name: "".to_string(),
                inner: Steps::Text { author: str!(""), texts: str!("Remove them better") },
            },
            // endregion

            Step {  // End
                id: Uuid::new_v4(),
                name: str!("End"),
                inner: Steps::Text { author: str!("Narrator"), texts: str!("The End!") },
            },
        ],
    };

    let data = ron::ser::to_string_pretty(&aerugo, Default::default()).unwrap();
    let save_path = std::path::Path::new(SCENARIO_PATH);
    let mut save_file = std::fs::File::options()
        .write(true).create(true).truncate(true)
        .open(save_path)
        .unwrap();

    save_file
        .write_all(data.as_bytes())
        .unwrap();

    println!("Scenario saved successfully to {}", save_path.to_str().unwrap());
}
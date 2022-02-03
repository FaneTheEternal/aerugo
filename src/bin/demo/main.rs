use std::io::Write;
use uuid::Uuid;
use aerugo::*;


macro_rules! str {
    ($s:expr) => {format!("{}", $s)};
}

const SCENARIO_PATH: &str = "scenario.json";

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
            // TODO: SpriteNarrator
            // endregion

            // region sprite test
            // TODO: Sprite
            // endregion

            // region background test
            // TODO: Background
            // endregion

            // region scene test
            // TODO: Scene
            // endregion

            Step {  // End
                id: Uuid::new_v4(),
                name: str!("End"),
                inner: Steps::Text { author: str!("Narrator"), texts: str!("The End!") },
            },
        ],
    };

    let data = serde_json::to_string(&aerugo).unwrap();
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
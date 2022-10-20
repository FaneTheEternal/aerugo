#![feature(path_file_prefix)]

mod obfuscation;

use std::error::Error;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use bevy::utils::default;
use aerugo::*;
use aerugo::international::*;
use sha3::{Digest, Sha3_256};
use uuid::Uuid;
use crate::obfuscation::obfuscation;

const SCENARIO_PATH: &str = "scenario.ron";

fn main() -> Result<(), Box<dyn Error>> {
    let command = std::env::args().nth(1).expect("Command expected");
    match command.as_str() {
        "obfuscation" => { obfuscation()? }
        "fix" => { fix()? }
        "help" => {
            todo!("all commands");
        }
        _ => { panic!("Command not found: {}", command) }
    }
    Ok(())
}

pub fn get_aerugo() -> Aerugo {
    let mut file = std::fs::File::options()
        .read(true).write(true).create(true)
        .open(SCENARIO_PATH)
        .unwrap();
    let mut buff = String::new();
    file.read_to_string(&mut buff).unwrap();
    ron::from_str::<Aerugo>(&buff).unwrap_or_default()
}

pub fn set_aerugo<F: AsRef<Path>>(aerugo: &Aerugo, path: F) {
    let aerugo = ron::ser::to_string_pretty(&aerugo, default()).unwrap();
    fs::write(Path::new(path.as_ref()).join(SCENARIO_PATH), aerugo.as_bytes()).unwrap();
}

pub fn fix() -> Result<(), Box<dyn Error>> {
    let aerugo = get_aerugo();
    let mut internationale = Internationale::load();
    let mut last = Uuid::default();
    for step in &aerugo.steps {
        if !Internationale::is_relevant(&step.inner) {
            continue;
        }
        for def in internationale.defs.iter_mut() {
            if last.is_nil() { break; }
            if def.localized.iter().find(|s| s.id == step.id).is_none() {
                let pos = def.localized.iter().position(|s| s.id == last)
                    .expect(
                        format!("NOT FOUND {:?}::{:?}\n\tTriggered {:?}",
                                def.lang, last, step.id).as_str()
                    );
                def.localized.insert(pos + 1, step.clone());
                println!("FIX {:?} at {:^3} ({})", def.lang, pos + 1, step.id);
            }
        }
        match step.inner {
            Steps::Text { .. } | Steps::Phrase { .. } => {
                last = step.id;
            }
            _ => {}
        }
    }
    set_aerugo(&aerugo, "");
    let base = std::env::current_dir()?
        .join("assets")
        .join("lang");
    for def in internationale.defs {
        fs::write(
            base.join(format!("{:?}.imanity", def.lang).to_lowercase().as_str()),
            ron::ser::to_string_pretty(&def, default())?.as_bytes(),
        )?
    }
    Ok(())
}
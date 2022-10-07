#![feature(path_file_prefix)]

use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use aerugo::*;
use aerugo::international::*;
use sha3::{Digest, Sha3_256};

const SCENARIO_PATH: &str = "scenario.ron";

fn main() {
    let command = std::env::args().nth(1).expect("Command expected");
    match command.as_str() {
        "obfuscation" => { obfuscation() }
        "help" => {
            todo!("all commands");
        }
        _ => { panic!("Command not found: {}", command) }
    }
}

fn get_aerugo() -> Aerugo {
    let mut file = std::fs::File::options()
        .read(true).write(true).create(true)
        .open(SCENARIO_PATH)
        .unwrap();
    let mut buff = String::new();
    file.read_to_string(&mut buff).unwrap();
    ron::from_str::<Aerugo>(&buff).unwrap_or_default()
}

fn obfuscation() {
    let mut aerugo = get_aerugo();
    let dst = std::env::current_dir().unwrap()
        .join("target")
        .join("obfuscated");
    let assets = std::env::current_dir().unwrap()
        .join("assets");
    let new_assets = dst.join("assets");
    copy_folder(&assets, &new_assets).unwrap();
    println!("SHA-3");
    let _sha3f = |s: &mut String| {
        let _s = s.clone();
        let path = Path::new(&_s);
        let name = path.file_prefix().unwrap().to_string_lossy().to_string();
        let new_name = get_sha3(&name);
        let new_path = s.replace(&name, &new_name);
        *s = new_path.clone();
        if new_assets.join(&new_path).exists() {
            let _ = fs::remove_file(path);
            return;
        }
        println!(
            "{} -> {}",
            new_assets.join(path).to_string_lossy(),
            new_assets.join(&new_path).to_string_lossy()
        );
        fs::rename(new_assets.join(path), new_assets.join(&new_path)).unwrap();
    };

    for step in aerugo.steps.iter_mut() {
        match &mut step.inner {
            Steps::Jump { .. } => {}
            Steps::ImageSelect { .. } => {}
            Steps::SpriteNarrator(cmd) => {
                match cmd {
                    NarratorCommand::Set { name, sprite } => {
                        *name = get_sha3(&name);
                        _sha3f(sprite);
                    }
                    NarratorCommand::Remove { name } => {
                        *name = get_sha3(&name);
                    }
                    _ => {}
                }
            }
            Steps::Sprite(cmd) => {
                match cmd {
                    SpriteCommand::None => {}
                    SpriteCommand::Set { sprite, name, .. } => {
                        _sha3f(sprite);
                        _sha3(name);
                    }
                    SpriteCommand::Remove { name } => {
                        _sha3(name);
                    }
                    SpriteCommand::FadeIn { sprite, name, .. } => {
                        _sha3f(sprite);
                        _sha3(name);
                    }
                    SpriteCommand::FadeOut { name } => {
                        _sha3(name);
                    }
                    SpriteCommand::LeftIn { sprite, name, .. } => {
                        _sha3f(sprite);
                        _sha3(name);
                    }
                    SpriteCommand::LeftOut { name } => {
                        _sha3(name);
                    }
                    SpriteCommand::RightIn { sprite, name, .. } => {
                        _sha3f(sprite);
                        _sha3(name);
                    }
                    SpriteCommand::RightOut { name } => {
                        _sha3(name);
                    }
                    SpriteCommand::Move { name, .. } => {
                        _sha3(name);
                    }
                }
            }
            Steps::Background(cmd) => {
                match cmd {
                    BackgroundCommand::Change { new, .. } => {
                        _sha3f(new);
                    }
                    _ => {}
                }
            }
            Steps::Scene(cmd) => {
                match cmd {
                    SceneCommand::Set { name } => {
                        _sha3f(name);
                    }
                    SceneCommand::Remove => {}
                    SceneCommand::Play { name, .. } => {
                        _sha3f(name);
                    }
                    SceneCommand::Pause => {}
                    SceneCommand::Resume => {}
                    SceneCommand::Stop => {}
                    SceneCommand::None => {}
                }
            }
            _ => {}
        }
    }
    let aerugo = ron::to_string(&aerugo).unwrap();
    std::fs::write(dst.join(SCENARIO_PATH), aerugo.as_bytes()).unwrap();
}

fn _sha3(s: &mut String) {
    *s = get_sha3(&s);
}

fn get_sha3(s: &str) -> String {
    let mut hasher = Sha3_256::new();
    hasher.update(s);
    let result = hasher.finalize();
    return result[..].iter().map(|b| format!("{:X}", b)).collect();
}

pub fn copy_folder<U: AsRef<Path>, V: AsRef<Path>>(from: U, to: V) -> Result<(), std::io::Error> {
    let mut stack = Vec::new();
    stack.push(PathBuf::from(from.as_ref()));

    let output_root = PathBuf::from(to.as_ref());
    let input_root = PathBuf::from(from.as_ref()).components().count();

    while let Some(working_path) = stack.pop() {
        println!("process: {:?}", &working_path);

        // Generate a relative path
        let src: PathBuf = working_path.components().skip(input_root).collect();

        // Create a destination if missing
        let dest = if src.components().count() == 0 {
            output_root.clone()
        } else {
            output_root.join(&src)
        };
        if fs::metadata(&dest).is_err() {
            println!(" mkdir: {:?}", dest);
            fs::create_dir_all(&dest)?;
        }

        for entry in fs::read_dir(working_path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else {
                match path.file_name() {
                    Some(filename) => {
                        let dest_path = dest.join(filename);
                        println!("  copy: {:?} -> {:?}", &path, &dest_path);
                        fs::copy(&path, &dest_path)?;
                    }
                    None => {
                        println!("failed: {:?}", path);
                    }
                }
            }
        }
    }

    Ok(())
}
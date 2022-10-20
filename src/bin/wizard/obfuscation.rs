use std::io;
use std::io::Write;
use std::process::Stdio;
use crate::*;

pub fn obfuscation() -> Result<(), Box<dyn Error>> {
    let game = build_game()?;
    let mut aerugo = get_aerugo();
    let dst = std::env::current_dir().unwrap()
        .join("target")
        .join("obfuscated");
    println!("Pre cleaning...");
    fs::remove_dir_all(&dst).unwrap_or_else(|e| println!("{e:?}"));
    let assets = std::env::current_dir().unwrap()
        .join("assets");
    let new_assets = dst.join("assets");
    println!("Copying assets...");
    copy_folder(&assets, &new_assets).unwrap();
    println!("SHA-3...");
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
            path.to_string_lossy(),
            new_path,
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
            Steps::Text { author, texts } => {
                *author = default();
                *texts = default();
            }
            Steps::Phrase { phrases } => {
                phrases.iter_mut().for_each(|phrase| { phrase.1 = default() });
            }
            _ => {}
        }
    }
    println!("Flatten Aerugo...");
    let aerugo = ron::to_string(&aerugo).unwrap();
    fs::write(dst.join(SCENARIO_PATH), aerugo.as_bytes()).unwrap();
    println!("Flatten AerugoImanity...");
    flat_aerugo_imanity(&new_assets)?;
    println!("Cleanup...");
    remove_if_empty(new_assets)?;
    post_build_game(game, dst)?;
    Ok(())
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

pub fn remove_if_empty<F: AsRef<Path>>(folder: F) -> Result<(), std::io::Error> {
    let mut del_self = true;
    for entry in fs::read_dir(&folder)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            del_self = false;
        } else if path.is_dir() {
            remove_if_empty(&path)?;
            if path.exists() {
                del_self = false;
            }
        }
    }
    if del_self {
        fs::remove_dir(folder)?;
    }
    Ok(())
}

pub fn flat_aerugo_imanity<F: AsRef<Path>>(assets: F) -> Result<(), std::io::Error> {
    let base = Path::new(assets.as_ref()).join("lang");
    for entry in std::fs::read_dir(&base).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if fs::metadata(&path)?.is_file() {
            let name = path.strip_prefix(&base).unwrap();
            let name = name.to_string_lossy();
            if let Some(_) = name.strip_suffix(".imanity") {
                let imanity = fs::read_to_string(&path).unwrap();
                let imanity: AerugoImanity = ron::from_str(&imanity).unwrap();
                let imanity = ron::to_string(&imanity).unwrap();
                fs::write(&path, imanity)?;
            }
        }
    }
    Ok(())
}

pub fn build_game() -> Result<std::process::Child, Box<dyn Error>> {
    println!("Building game parallel...");
    let output = std::process::Command::new("cargo")
        .arg("build")
        .args(["--profile", "release-max"])
        .args(["--bin", "game"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    Ok(output)
}

pub fn post_build_game<F: AsRef<Path>>(process: std::process::Child, dst: F) -> Result<(), Box<dyn Error>> {
    let output = process.wait_with_output()?;
    println!("Game build status: {}", output.status);
    io::stdout().write_all(&output.stdout)?;
    io::stderr().write_all(&output.stderr)?;
    let source = std::env::current_dir().unwrap()
        .join("target")
        .join("release-max")
        .join("game.exe");
    fs::copy(source, dst.as_ref().join("game.exe"))?;
    Ok(())
}


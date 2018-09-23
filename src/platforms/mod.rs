mod cargo;
mod npm;

use std::path::PathBuf;
use self::cargo::Cargo;
use self::npm::Npm;
use ::utils::{ScriptSource, Platform};

pub fn populate_platforms(workdir: &PathBuf) -> Vec<Platform> {
    let mut platforms = Vec::new();

    if let Ok(scripts) = Cargo::new(&workdir).get_scripts() {
        platforms.push(Platform { name: "Cargo", scripts })
    }

    if let Ok(scripts) = Npm::new(&workdir).get_scripts() {
        platforms.push(Platform { name: "npm", scripts });
    }

    platforms
}

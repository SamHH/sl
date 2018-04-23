extern crate serde_json;
#[macro_use] extern crate serde_derive;
extern crate termion;

use std::env;
use std::path::PathBuf;
use termion::{color, style};

mod platforms;
use platforms::{ScriptSource, ScriptList};
use platforms::npm::Npm;

struct ScriptGroup {
    name: String,
    scripts: ScriptList,
}

fn main() {
    // First index (0) is relative path of executable, ignore it. See:
    // https://doc.rust-lang.org/std/env/fn.args.html
    let arg: Option<String> = env::args().nth(1);
    let path: PathBuf = match arg {
        Some(p) => PathBuf::from(p),
        None => env::current_dir().unwrap(),
    };

    let mut scripts: Vec<ScriptGroup> = Vec::new();

    let npm_scripts = Npm::new(path).get_scripts();
    if npm_scripts.is_ok() {
        scripts.push(ScriptGroup{ name: String::from("npm"), scripts: npm_scripts.unwrap() });
    }

    for group in scripts {
        println!("{}{}{}", color::Fg(color::Red), group.name, style::Reset);

        for script in group.scripts {
            println!("{}: {}", script.name, script.command);
        }
    }
}

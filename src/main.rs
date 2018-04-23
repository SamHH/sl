extern crate serde_json;
#[macro_use] extern crate serde_derive;

use std::env;
use std::path::PathBuf;
use std::collections::HashMap;

mod platforms;
use platforms::{ScriptSource, ScriptList};
use platforms::npm::Npm;

fn main() {
    // First index (0) is relative path of executable, ignore it. See:
    // https://doc.rust-lang.org/std/env/fn.args.html
    let arg: Option<String> = env::args().nth(1);
    let path: PathBuf = match arg {
        Some(p) => PathBuf::from(p),
        None => env::current_dir().unwrap(),
    };

    let mut scripts: HashMap<String, ScriptList> = HashMap::new();

    let npm_scripts = Npm::new(path).get_scripts();
    if npm_scripts.is_ok() {
        scripts.insert(String::from("npm"), npm_scripts.unwrap());
    }

    println!("{:?}", scripts);
}

extern crate serde_json;
#[macro_use] extern crate serde_derive;

use std::env;
use std::path::PathBuf;
use std::io::{stdin, BufRead};
use std::process::Command;
use std::os::unix::process::CommandExt;

mod platforms;
use platforms::{ScriptSource, ScriptList, Script};
use platforms::npm::Npm;

struct ScriptGroup {
    name: String,
    scripts: ScriptList,
}

#[derive(PartialEq)]
enum Step {
    ChoosingGroup,
    ChoosingScript,
}

fn clear() {
    // println!("{}", clear::All);
}

fn print_groups(groups: &Vec<ScriptGroup>) {
    for (index, group) in groups.iter().enumerate() {
        println!("[{}]: {}", index, group.name);
    }
}

fn print_scripts(scripts: &ScriptList) {
    for (index, script) in scripts.iter().enumerate() {
        println!("[{}]: {} ({})", index, script.name, script.command);
    }
}

fn run_script(script: &Script) -> std::io::Error {
    println!("Running script `{}`: `{}`", script.name, script.command);

    // At this point, if successful, the script will be run and sl will be
    // safely terminated. An error is returned and sl continues running if the
    // exec fails for any reason
    return Command::new("sh").arg("-c").arg(&script.command).exec();
}

fn main() {
    // First index (0) is relative path of executable, ignore it. See:
    // https://doc.rust-lang.org/std/env/fn.args.html
    let arg: Option<String> = env::args().nth(1);
    let path: PathBuf = match arg {
        Some(p) => PathBuf::from(p),
        None => env::current_dir().unwrap(),
    };

    let mut groups: Vec<ScriptGroup> = Vec::new();

    let npm_scripts = Npm::new(path).get_scripts();
    if npm_scripts.is_ok() {
        groups.push(ScriptGroup { name: String::from("npm"), scripts: npm_scripts.unwrap() });
    }

    let stdin = stdin();
    let mut evt = stdin.lock().lines();
    let mut step: Step = Step::ChoosingGroup;
    let mut group_index: usize = 0;

    loop {
        clear();

        match step {
            Step::ChoosingGroup => { print_groups(&groups); },
            Step::ChoosingScript => { print_scripts(&groups[group_index].scripts); },
        };

        let input: Result<i16, _> = evt.next().unwrap().unwrap().parse();

        // If we were unable to parse an integer from the user input, then retry
        if input.is_err() { continue; }

        let index = input.unwrap();

        match step {
            Step::ChoosingGroup => {
                if index < 0 || index >= groups.len() as i16 { continue; }
                else {
                    group_index = index as usize;
                    step = Step::ChoosingScript;
                    continue;
                }
            },
            Step::ChoosingScript => {
                if index < 0 || index >= groups[group_index].scripts.len() as i16 { continue; }
                else {
                    let selected_script = &groups[group_index].scripts[index as usize];

                    run_script(selected_script);

                    break;
                }
            },
        };
    }
}

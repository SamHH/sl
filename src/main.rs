#![feature(range_contains)]

extern crate serde_json;
#[macro_use] extern crate serde_derive;
extern crate termion;

mod platforms;
mod print;
mod utils;

use std::rc::Rc;
use std::path::PathBuf;
use std::io::{stdin, BufRead, Lines, StdinLock};
use std::process::{exit, Command};
use std::os::unix::process::CommandExt;
use print::{info, warn, error, print_platforms, print_scripts};
use platforms::populate_platforms;
use utils::Platform;

type Index = i16;

enum Step {
    ChoosingPlatform,
    ChoosingScript,
    RunningScript,
}

fn get_workdir() -> std::io::Result<PathBuf> {
    use std::env;

    // First index (0) is relative path of executable, ignore it. See:
    // https://doc.rust-lang.org/std/env/fn.args.html
    let arg: Option<String> = env::args().nth(1);
    let path = match arg {
        Some(p) => Ok(PathBuf::from(p)),
        None => env::current_dir(),
    };

    path
}

fn listen_for_index(evt: &mut Lines<StdinLock>) -> Result<Index, std::num::ParseIntError> {
    evt.next().unwrap().unwrap().parse()
}

// Run the script and exit, or if insuccessful then return an error
fn run_script(workdir: &PathBuf, command: &String) -> std::io::Error {
    info(format!("exec: {}", &command));

    return Command::new("sh").arg("-c").current_dir(&workdir).arg(&command).exec();
}

struct State {
    step: Step,
    platforms: Vec<Platform>,
    platform_index: Option<Index>,
    script_index: Option<Index>,
}

fn main() {
    let workdir = Rc::new(get_workdir().unwrap());
    let platforms = populate_platforms(&workdir);

    if platforms.is_empty() {
        warn("No compatible environments found.");

        return;
    }

    // Not reactive, but we'll treat it as if it is in the loop
    let mut state = State {
        step: Step::ChoosingPlatform,
        platforms,
        platform_index: None,
        script_index: None,
    };

    let stdin = stdin();
    let mut evt = stdin.lock().lines();

    loop {
        match state.step {
            Step::ChoosingPlatform => {
                let platforms = &state.platforms;

                print_platforms(platforms);

                let valid_indices = 0..platforms.len() as i16;
                let input = match listen_for_index(&mut evt) {
                    Ok(input) => {
                        if !valid_indices.contains(&input) { continue; }

                        input
                    },
                    Err(_) => {
                        continue;
                    },
                };

                state.platform_index = Some(input);
                state.step = Step::ChoosingScript;
            },
            Step::ChoosingScript => {
                let platform_index = match state.platform_index {
                    Some(index) => index as usize,
                    None => {
                        state.step = Step::ChoosingPlatform;

                        continue;
                    },
                };

                let scripts = &state.platforms[platform_index].scripts;

                print_scripts(scripts);

                let valid_indices = 0..scripts.len() as i16;
                let input = match listen_for_index(&mut evt) {
                    Ok(input) => {
                        if !valid_indices.contains(&input) { continue; }

                        input
                    },
                    Err(_) => {
                        continue;
                    },
                };

                state.script_index = Some(input);
                state.step = Step::RunningScript;
            },
            Step::RunningScript => {
                let platform_index = match state.platform_index {
                    Some(index) => index as usize,
                    None => {
                        state.step = Step::ChoosingPlatform;

                        continue;
                    },
                };

                let script_index = match state.script_index {
                    Some(index) => index as usize,
                    None => {
                        state.step = Step::ChoosingScript;

                        continue;
                    },
                };

                // run_script will not return if successful, so this is solely
                // for failure case
                let run_err = run_script(&workdir, &state.platforms[platform_index].scripts[script_index].command);

                error(format!("Failed to run script. Error: {}", run_err));
                exit(0);
            },
        }
    }
}

use std::fmt::Display;
use yansi::Paint;
use ::utils::{Platform, Script};

fn print_with_prefix<T: Into<String>>(text: T) {
    let prefix = format!("{}", Paint::blue("sr>"));

    println!("{} {}", prefix, text.into());
}

pub fn info<T: Into<String>>(text: T) {
    print_with_prefix(text);
}

pub fn warn<T: Into<String>>(text: T) {
    let with_color = format!("{}", Paint::yellow(text.into()));

    print_with_prefix(with_color);
}

pub fn error<T: Into<String>>(text: T) {
    let with_color = format!("{}", Paint::red(text.into()));

    print_with_prefix(with_color);
}

fn print_item(key: impl Display, item: &String) {
    info(format!("[{}]: {}", key, &item));
}

pub fn print_platforms(platforms: &Vec<Platform>) {
    info(String::from("Platforms:"));

    for (index, platform) in platforms.iter().enumerate() {
        print_item(index, &String::from(platform.name));
    }
}

pub fn print_scripts(scripts: &Vec<Script>) {
    info(String::from("Scripts:"));

    for (index, script) in scripts.iter().enumerate() {
        print_item(index, &script.label);
    }
}

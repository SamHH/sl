use std::io;
use std::path::PathBuf;

// Preferring Vecs over Maps as this app will always be selecting/accessing
// scripts by index; this makes the data easiest to work with

pub struct Script {
    pub label: String,
    pub command: String,
}

pub struct Platform {
    pub name: &'static str,
    pub scripts: Vec<Script>,
}

pub trait ScriptSource {
    fn new(path: &PathBuf) -> Self;
    fn get_scripts(&self) -> io::Result<Vec<Script>>;
}

use std::error::Error;
use std::path::PathBuf;

pub mod npm;

pub struct Script {
    pub name: String,
    pub command: String,
}

pub type ScriptList = Vec<Script>;

pub trait ScriptSource {
    fn new(path: PathBuf) -> Self;
    fn get_scripts(&self) -> Result<ScriptList, Box<Error>>;
}

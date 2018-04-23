use std::error::Error;
use std::collections::HashMap;
use std::path::PathBuf;

pub mod npm;

pub type ScriptList = HashMap<String, String>;

pub trait ScriptSource {
    fn new(path: PathBuf) -> Self;
    fn get_scripts(&self) -> Result<ScriptList, Box<Error>>;
}

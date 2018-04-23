use std::error::Error;
use std::fs::File;
use std::path::PathBuf;
use serde_json;
use platforms::{ScriptSource, ScriptList, Script};

#[derive(Deserialize)]
struct NpmManifest {
    scripts: serde_json::Map<String, serde_json::Value>,
}

pub struct Npm {
    path: PathBuf,
    file_name: String,
}

impl ScriptSource for Npm {
    fn new(path: PathBuf) -> Self {
        Self {
            path,
            file_name: String::from("package.json"),
        }
    }

    fn get_scripts(&self) -> Result<ScriptList, Box<Error>> {
        let full_path: PathBuf = [&self.path, &PathBuf::from(&self.file_name)].iter().collect();

        let file = File::open(&full_path.as_path())?;

        let json: NpmManifest = serde_json::from_reader(file)?;

        let scripts = json.scripts
            .iter()
            .map(|(k,v)| Script { name: k.to_string(), command: v.as_str().unwrap().to_string() })
            .collect();

        return Ok(scripts);
    }
}

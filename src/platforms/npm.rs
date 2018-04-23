use std::error::Error;
use std::fs::File;
use std::path::PathBuf;
use serde_json;
use platforms::{ScriptSource, ScriptList};

#[derive(Serialize, Deserialize)]
struct NpmManifest {
    scripts: ScriptList,
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

        return Ok(json.scripts);
    }
}

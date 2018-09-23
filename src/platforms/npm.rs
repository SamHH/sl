use std::io;
use std::fs::File;
use std::path::PathBuf;
use serde_json::{Map, Value, from_reader};
use ::utils::{ScriptSource, Script};

#[derive(Deserialize)]
struct NpmManifest {
    scripts: Map<String, Value>,
}

pub struct Npm {
    path: PathBuf,
    file_name: &'static str,
}

impl ScriptSource for Npm {
    fn new(path: &PathBuf) -> Self {
        Self {
            path: path.clone(),
            file_name: "package.json",
        }
    }

    fn get_scripts(&self) -> io::Result<Vec<Script>> {
        let full_path: PathBuf = [&self.path, &PathBuf::from(&self.file_name)].iter().collect();

        let file = File::open(&full_path.as_path())?;

        let manifest: NpmManifest = from_reader(file)?;

        let scripts = manifest.scripts
            .iter()
            .map(|(k, _v)| Script {
                label: k.to_string(),
                command: format!("npm run {}", *k),
            })
            .collect();

        Ok(scripts)
    }
}

use std::io;
use std::path::Path;
use std::path::PathBuf;
use ::utils::{ScriptSource, Script};

pub struct Cargo {
    path: PathBuf,
    file_name: &'static str,
}

impl ScriptSource for Cargo {
    fn new(path: &PathBuf) -> Self {
        Self {
            path: path.clone(),
            file_name: "Cargo.toml",
        }
    }

    fn get_scripts(&self) -> io::Result<Vec<Script>> {
        let full_path: PathBuf = [&self.path, &PathBuf::from(&self.file_name)].iter().collect();

        let file_exists = Path::new(&full_path).exists();
        if !file_exists {
          return Err(io::Error::new(io::ErrorKind::NotFound, format!("File `{}` does not exist.", &self.file_name)));
        }

        let scripts = vec![
          Script {
            label: String::from("run"),
            command: String::from("cargo run"),
          },
          Script {
            label: String::from("build"),
            command: String::from("cargo build"),
          },
         ];

        Ok(scripts)
    }
}

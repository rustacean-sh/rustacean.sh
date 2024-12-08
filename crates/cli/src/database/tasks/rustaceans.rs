use std::collections::BTreeMap;
use std::fs::{read_dir, read_to_string, write};
use std::path::{Path, PathBuf};

use anyhow::Result;
use proto::Rustacean;

pub struct RustaceansTask {
    input_dir: PathBuf,
    out_dir: PathBuf,
}

impl RustaceansTask {
    pub fn new<P: AsRef<Path>>(input_dir: P, out_dir: P) -> Self {
        Self {
            input_dir: input_dir.as_ref().to_path_buf(),
            out_dir: out_dir.as_ref().to_path_buf(),
        }
    }

    pub fn perform(&self) -> Result<()> {
        let entries: Vec<PathBuf> = read_dir(&self.input_dir)?
            .map(|entry| entry.map(|e| e.path()))
            .collect::<Result<Vec<_>, std::io::Error>>()?;

        let toml_files: Vec<PathBuf> = entries
            .into_iter()
            .filter(|entry| entry.extension().unwrap_or_default() == "toml")
            .collect();

        let mut rustaceans: BTreeMap<String, Rustacean> = BTreeMap::new();

        for toml_file in toml_files {
            let content = read_to_string(&toml_file)?;
            let rustacean = toml::from_str::<Rustacean>(&content)?;

            rustaceans.insert(rustacean.gh_user.clone(), rustacean);
        }

        let output = bincode::serialize(&rustaceans)?;
        write(self.out_dir.join("rustaceans.bin"), output)?;

        Ok(())
    }
}

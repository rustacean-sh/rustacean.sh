use std::path::PathBuf;

use anyhow::Result;
use clap::Args;

use rustacean_sh::database::{DatabaseTask, Sources};

#[derive(Args, Debug)]
pub struct GenerateOpt {
    /// Path for generated outputs
    #[clap(long)]
    pub out_dir: PathBuf,
    /// Path to "rustaceans" directory
    #[clap(long)]
    pub rustaceans_path: PathBuf,
}

impl GenerateOpt {
    pub fn exec(&self) -> Result<()> {
        let sources = Sources {
            rustaceans_dir_path: self.rustaceans_path.clone(),
        };
        let task = DatabaseTask::new(&self.out_dir, sources);

        task.perform()
    }
}

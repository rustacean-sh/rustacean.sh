mod tasks;

use std::path::{Path, PathBuf};

use anyhow::Result;
use tasks::rustaceans::RustaceansTask;

pub struct Sources {
    pub rustaceans_dir_path: PathBuf,
}

pub struct DatabaseTask {
    out_dir: PathBuf,
    sources: Sources,
}

impl DatabaseTask {
    pub fn new<P: AsRef<Path>>(out_dir: P, sources: Sources) -> Self {
        Self {
            out_dir: out_dir.as_ref().to_path_buf(),
            sources,
        }
    }

    pub fn perform(&self) -> Result<()> {
        let rustaceans_task = RustaceansTask::new(&self.sources.rustaceans_dir_path, &self.out_dir);
        rustaceans_task.perform()
    }
}

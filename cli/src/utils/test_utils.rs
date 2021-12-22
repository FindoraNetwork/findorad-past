use std::{
    env::temp_dir,
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};

pub struct TempDir {
    path: Option<PathBuf>,
}

impl TempDir {
    pub fn new<P: Into<PathBuf>>(path: P) -> Result<TempDir> {
        let p = path.into();
        let dir = temp_dir().as_path().join(p);
        fs::create_dir_all(dir.clone())
            .with_context(|| format!("failed to create directory: {}", dir.display()))?;

        Ok(TempDir { path: Some(dir) })
    }

    pub fn path(&self) -> &Path {
        self.path
            .as_ref()
            .expect("temp dir has already been removed")
    }

    pub fn remove(&mut self) {
        if let Some(p) = &self.path {
            let _ = fs::remove_dir_all(p);
            self.path = None;
        }
    }
}

impl Default for TempDir {
    fn default() -> TempDir {
        TempDir::new("default_temp").unwrap()
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        self.remove();
    }
}

impl AsRef<Path> for TempDir {
    fn as_ref(&self) -> &Path {
        self.path()
    }
}

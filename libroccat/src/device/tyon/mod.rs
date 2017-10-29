use std::path::{Path, PathBuf};

pub struct Tyon {
    path: PathBuf,
}

impl Tyon {
    pub fn new(path: &Path) -> Self {
        Self {
            path: path.to_path_buf(),
        }
    }

    pub fn get_path(&self) -> &Path {
        &self.path
    }
}

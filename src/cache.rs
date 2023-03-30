//! Cache functions, stored in json
//!
//! You either write_cache() or read_cache()
use std::path::PathBuf;
use std::path::Path;
use std::io::Write;
use std::fs;
use std::fs::File;
use std::os::unix::fs::MetadataExt;
use std::time::SystemTime;

use crate::Colors;
use crate::MyLab;
use crate::config::Backend;

use serde::*;
use anyhow::Result;

/// This is to handle the use of multiple backends, needs to be updated manually

/// Used to manage cache, rather than passing arguments in main() a lot
#[derive(Serialize, Deserialize)]
pub struct Cache {
    back: Backend,
    file: PathBuf,
    hash: String,
    path: String,
}

impl Cache {
    pub fn new(filename: PathBuf, backend: Backend) -> Result<Self> {
        let cachepath = match backend {
            Backend::Full    => "~/.cache/wallust/full",
            Backend::Resized => "~/.cache/wallust/resized",
        };
        let md = fs::metadata(&filename)?;
        let birth = if let Ok(o) = md.created() { o } else { panic!("Not Supported") };
        let modif = if let Ok(o) = md.modified() { o } else { panic!("Not Supported") };

        // The following generates a hash name from a filename and it's `stat` attrs
        let hash_name = format!("{}{}{}{}",
            //filename.display(),
            md.ino(),
            //md.file_type(),
            md.len(),
            birth.duration_since(SystemTime::UNIX_EPOCH)?.as_secs(),
            modif.duration_since(SystemTime::UNIX_EPOCH)?.as_secs(),
        );

        Ok(Self {
            back: backend,
            file: filename,
            hash: hash_name,
            path: cachepath.into(),
        })
    }

    pub fn read(&self) -> Result<Colors<MyLab>> {
        let contents = std::fs::read_to_string(&self.path)?;
        Ok(serde_json::from_str(&contents)?)
    }

    pub fn write(&self, colors: &Colors<MyLab>) -> Result<()> {
        Ok(File::create(&self.path)?
            .write_all(
                serde_json::to_string(colors)?
                    .as_bytes()
        )?)
    }

    pub fn is_cached(&self) -> bool {
        if Path::new(&self.path).exists() {
            return true;
        }
        false
    }
}


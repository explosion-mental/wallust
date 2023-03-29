//! Cache functions, stored in json
//!
//! You either write_cache() or read_cache()
use std::path::PathBuf;
use std::path::Path;
use std::fs;
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
        Ok(Self {
            back: backend,
            file: filename,
            hash: "".to_string(),
            path: cachepath.into(),
        })
    }

    pub fn read(&self) -> Result<Colors<MyLab>> {
        let contents = std::fs::read_to_string(&self.path)?;
        Ok(serde_json::from_str(&contents)?)
    }

    pub fn write(colors: &Colors<MyLab>) -> Result<()> {
        println!("{}", serde_json::to_string(colors)?);
        Ok(())
    }

    pub fn is_cached(&self) -> bool {
        if Path::new(&self.path).exists() {
            return true;
        }
        false
    }
}

/// Generates the name the cache file it's gonna use, which is a hash
//TODO
fn cachename(file: &PathBuf) -> Result<String> {
    let md = fs::metadata(&file)?;
    let birth = if let Ok(o) = md.created() { o } else { panic!("Not Supported") };
    let modif = if let Ok(o) = md.modified() { o } else { panic!("Not Supported") };
    Ok(
    format!("{}{}{}{}{}",
        file.display(),
        md.ino(),
        //md.file_type(),
        md.len(),
        birth.duration_since(SystemTime::UNIX_EPOCH)?.as_secs(),
        modif.duration_since(SystemTime::UNIX_EPOCH)?.as_secs(),
    ))
}

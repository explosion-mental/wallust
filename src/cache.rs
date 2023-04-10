//! Cache functions, serde + serde_json
use std::fs;
use std::fs::File;
use std::os::unix::fs::MetadataExt;
use std::io::Write;
use std::time::SystemTime;
use std::path::PathBuf;
use std::path::Path;

use crate::colors::Colors;
use crate::config::Config;

use anyhow::{Result, Context};
use serde::{Serialize, Deserialize};

/// Used to manage cache, rather than passing arguments in main() a lot
#[derive(Serialize, Deserialize)]
pub struct Cache {
    /// Filename that's gonna be cached
    file: PathBuf,
    /// A file "hash" name, for the cache filename
    hash: String,
    /// Path of the cache
    pub path: String,
}

impl Cache {
    /// init cache
    pub fn new(filename: PathBuf, c: &Config) -> Result<Self> {
        let cachepath = format!("{}/{}/{}", shellexpand::tilde("~/.cache/wallust"), c.backend, c.threshold);

        let md = fs::metadata(&filename)?;
        // if these metadata are not avaliable, then we can't cache
        let birth = if let Ok(o) = md.created()  { o } else { anyhow::bail!("Not Supported") };
        let modif = if let Ok(o) = md.modified() { o } else { anyhow::bail!("Not Supported") };

        // The following generates a hash name from a filename and it's `stat` attrs
        let hash_name = format!("{}{}{}{}",
            birth.duration_since(SystemTime::UNIX_EPOCH)?.as_secs(),
            modif.duration_since(SystemTime::UNIX_EPOCH)?.as_secs(),
            //filename.display(),
            md.ino(),
            //md.file_type(),
            md.len(),
        );

        // Create cache dir (with all of it's parents
        fs::create_dir_all(&cachepath)?;

        Ok(Self {
            path: format!("{cachepath}/{hash_name}"),
            file: filename,
            hash: hash_name,
        })
    }

    /// Fetches values from a file present in cache
    pub fn read(&self) -> Result<Colors> {
        let contents = std::fs::read_to_string(&self.path)?;
        Ok(serde_json::from_str(&contents)?)
    }

    /// Write values to cache
    pub fn write(&self, colors: &Colors) -> Result<()> {
        Ok(File::create(&self.path)?
            .write_all(
                serde_json::to_string(colors)
                    .with_context(|| format!("Failed to deserilize from the json cached file: '{}'\n", &self.path))?
                .as_bytes()
            )?)
    }

    /// To determine whether to read from cache or to generate the colors from scratch
    pub fn is_cached(&self) -> bool {
        if Path::new(&self.path).exists() {
            return true;
        }
        false
    }
}

